use std::ops::Neg;

use super::*;

use algebra::{
    BigInteger,      // for is_even()
    PrimeField,      // for from_repr()
    ProjectiveCurve, // for into_affine()
    Zero,
};

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

pub struct Schnorr<SC: SpongeConstants> {
    pub sponge: ArithmeticSponge<PallasField, SC>,
    pub network_id: NetworkId,
}

impl<SC: SpongeConstants> Signer for Schnorr<SC> {
    fn sign<I: Input>(&mut self, kp: Keypair, input: I) -> Signature {
        let k: PallasScalar = self.blinding_hash(&kp, input);
        let r: PallasPoint = PallasPoint::prime_subgroup_generator().mul(k).into_affine();
        let k: PallasScalar = if r.y.into_repr().is_even() { k } else { -k };

        let e: PallasScalar = self.message_hash(&kp.pub_key, r.x, input);
        let s: PallasScalar = k + e * kp.sec_key;

        Signature::new(r.x, s)
    }

    fn verify<I: Input>(&mut self, sig: Signature, pub_key: PubKey, input: I) -> bool {
        let ev: PallasScalar = self.message_hash(&pub_key, sig.rx, input);

        let sv: PallasPoint = PallasPoint::prime_subgroup_generator()
            .mul(sig.s)
            .into_affine();
        let rv: PallasPoint = sv + pub_key.mul(ev).neg().into_affine();

        !rv.infinity && rv.y.into_repr().is_even() && rv.x == sig.rx
    }
}

impl<SC: SpongeConstants> Schnorr<SC> {
    pub fn new(sponge: ArithmeticSponge<PallasField, SC>, network_id: NetworkId) -> Self {
        Schnorr::<SC> { sponge, network_id }
    }

    fn domain_bytes<I>(&self, input: I) -> Vec<u8>
    where
        I: Input,
    {
        let mut domain_string = input.domain_string(self.network_id);
        // Domain prefixes have a max length of 20 and are padded with '*'
        domain_string = &domain_string[..std::cmp::min(domain_string.len(), 20)];
        let mut bytes = format!("{:*<20}", domain_string).as_bytes().to_vec();
        bytes.resize(32, 0);

        bytes
    }

    fn blinding_hash<I>(&mut self, kp: &Keypair, input: I) -> PallasScalar
    where
        I: Input,
    {
        let mut hasher = VarBlake2b::new(32).unwrap();

        let mut roi: ROInput = input.to_roinput();
        roi.append_field(kp.pub_key.x);
        roi.append_field(kp.pub_key.y);
        roi.append_scalar(kp.sec_key);
        roi.append_bytes(vec![self.network_id.into()]);

        hasher.update(roi.to_bytes());

        let mut bytes = [0; 32];
        hasher.finalize_variable(|out| bytes.copy_from_slice(out));
        bytes[bytes.len() - 1] &= 0b0011_1111; // drop top two bits

        PallasScalar::from_random_bytes(&bytes[..]).expect("failed to create scalar from bytes")
    }

    fn message_hash<I>(&mut self, pub_key: &PubKey, rx: PallasField, input: I) -> PallasScalar
    where
        I: Input,
    {
        let mut roi: ROInput = input.to_roinput();
        roi.append_field(pub_key.x);
        roi.append_field(pub_key.y);
        roi.append_field(rx);

        // Set sponge initial state (explicitly init state so signer context can be reused)
        self.sponge.state = vec![PallasField::zero(); self.sponge.state.len()];
        self.sponge
            .absorb(&[PallasField::from_bytes(self.domain_bytes(input))]);
        self.sponge.squeeze();

        // Absorb random oracle input
        self.sponge.absorb(&roi.to_fields()[..]);

        // Squeeze and convert from field element to scalar
        PallasScalar::from_repr(self.sponge.squeeze().into_repr().into())
    }
}
