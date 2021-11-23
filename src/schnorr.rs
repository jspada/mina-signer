//! Mina Schnorr signature scheme
//!
//! An implementation of the singer interface for the Mina signature algorithm
//!
//! Details: <https://github.com/MinaProtocol/mina/blob/develop/docs/specs/signatures/description.md>

use ark_ec::{
    AffineCurve,     // for prime_subgroup_generator()
    ProjectiveCurve, // for into_affine()
};
use ark_ff::{
    BigInteger, // for is_even()
    Field,      // for from_random_bytes()
    PrimeField, // for from_repr()
    Zero,
};
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use oracle::{
    poseidon::SpongeConstants,
    rndoracle::{ArithmeticSponge, Sponge},
};
use std::ops::Neg;

use crate::{
    BaseField, CurvePoint, FieldHelpers, Input, Keypair, NetworkId, PubKey, ROInput, ScalarField,
    Signature, Signer,
};

/// Schnorr signer context for the Mina signature algorithm
///
/// For details about the signature algorithm please see [crate::schnorr]
pub struct Schnorr<SC: SpongeConstants> {
    sponge: ArithmeticSponge<BaseField, SC>,
    network_id: NetworkId,
}

impl<SC: SpongeConstants> Signer for Schnorr<SC> {
    fn sign<I: Input>(&mut self, kp: Keypair, input: I) -> Signature {
        let k: ScalarField = self.blinding_hash(&kp, input);
        let r: CurvePoint = CurvePoint::prime_subgroup_generator().mul(k).into_affine();
        let k: ScalarField = if r.y.into_repr().is_even() { k } else { -k };

        let e: ScalarField = self.message_hash(&kp.public, r.x, input);
        let s: ScalarField = k + e * kp.secret;

        Signature::new(r.x, s)
    }

    fn verify<I: Input>(&mut self, sig: Signature, public: PubKey, input: I) -> bool {
        let ev: ScalarField = self.message_hash(&public, sig.rx, input);

        let sv: CurvePoint = CurvePoint::prime_subgroup_generator()
            .mul(sig.s)
            .into_affine();
        let rv: CurvePoint = sv + public.mul(ev).neg().into_affine();

        !rv.infinity && rv.y.into_repr().is_even() && rv.x == sig.rx
    }
}

impl<SC: SpongeConstants> Schnorr<SC> {
    /// Create a new Schnorr signer context for network instance `network_id` using arithmetic sponge defined by `sponge`.
    pub fn new(sponge: ArithmeticSponge<BaseField, SC>, network_id: NetworkId) -> Self {
        Schnorr::<SC> { sponge, network_id }
    }

    fn domain_bytes<I>(&self, input: I) -> Vec<u8>
    where
        I: Input,
    {
        let mut domain_string = input.domain_string(self.network_id);
        // Domain prefixes have a max length of 20 and are padded with '*'
        assert!(domain_string.len() <= 20);
        domain_string = &domain_string[..std::cmp::min(domain_string.len(), 20)];
        let mut bytes = format!("{:*<20}", domain_string).as_bytes().to_vec();
        bytes.resize(32, 0);

        bytes
    }

    fn blinding_hash<I>(&mut self, kp: &Keypair, input: I) -> ScalarField
    where
        I: Input,
    {
        let mut hasher = VarBlake2b::new(32).unwrap();

        let mut roi: ROInput = input.to_roinput();
        roi.append_field(kp.public.x);
        roi.append_field(kp.public.y);
        roi.append_scalar(kp.secret);
        roi.append_bytes(&[self.network_id.into()]);

        hasher.update(roi.to_bytes());

        let mut bytes = [0; 32];
        hasher.finalize_variable(|out| bytes.copy_from_slice(out));
        bytes[bytes.len() - 1] &= 0b0011_1111; // drop top two bits

        ScalarField::from_random_bytes(&bytes[..]).expect("failed to create scalar from bytes")
    }

    fn message_hash<I>(&mut self, pub_key: &PubKey, rx: BaseField, input: I) -> ScalarField
    where
        I: Input,
    {
        let mut roi: ROInput = input.to_roinput();
        roi.append_field(pub_key.x);
        roi.append_field(pub_key.y);
        roi.append_field(rx);

        // Set sponge initial state (explicitly init state so signer context can be reused)
        self.sponge.state = vec![BaseField::zero(); self.sponge.state.len()];
        self.sponge
            .absorb(&[BaseField::from_bytes(&self.domain_bytes(input))]);
        self.sponge.squeeze();

        // Absorb random oracle input
        self.sponge.absorb(&roi.to_fields());

        // Squeeze and convert from base field element to scalar field element
        // Since the difference in modulus between the two fields is < 2^125, w.h.p., a
        // random value from one field will fit in the other field.
        ScalarField::from_repr(self.sponge.squeeze().into_repr()).expect("failed to create scalar")
    }
}
