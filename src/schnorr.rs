use super::*;

use algebra::{
    BigInteger, // for is_even()
    CanonicalSerialize as _,
    PrimeField, // for from_repr()
    ProjectiveCurve, // for into_affine()
};

use blake2::{Blake2b, Digest};

pub struct Schnorr<SC: SpongeConstants> {
    pub sponge: ArithmeticSponge<PallasField, SC>,
    pub network_id: NetworkId,
}

impl<SC: SpongeConstants> Signer for Schnorr<SC> {
    fn sign<I: Input>(mut self, kp: Keypair, input: I) -> Signature {
        println!("sign");

        let k: PallasScalar = self.blinding_hash(&kp, input);
        let r: PallasPoint = PallasPoint::prime_subgroup_generator().mul(k).into_affine();
        let k: PallasScalar = if r.y.0.is_even() { k } else { -k };

        let e: PallasScalar = self.message_hash(&kp.pub_key);
        let s: PallasScalar = k + e * kp.sec_key;
        return Signature::new(r.x, s);
    }
}

impl<SC: SpongeConstants> Schnorr<SC> {
    pub fn new(sponge: ArithmeticSponge<PallasField, SC>, network_id: NetworkId) -> Self {
        return Schnorr::<SC> {
            sponge,
            network_id,
        }
    }

    fn blinding_hash<I>(&mut self, kp: &Keypair, input: I) -> PallasScalar where I: Input {
        let mut hasher: Blake2b = Blake2b::new();

        let mut roi: ROInput = input.to_roinput();
        roi.append_field(kp.pub_key.x);
        roi.append_field(kp.pub_key.y);
        roi.append_scalar(kp.sec_key);
        roi.append_bytes(vec!(self.network_id.into()));

        // TODO: derive_message
        let mut bytes: Vec<u8> = vec![];
        kp.sec_key.into_repr()
            .serialize(&mut bytes)
            .expect("failed to serialize secret key");

        hasher.update(bytes);

        // TODO: need to swap from little-endian to big-endian?
        return PallasScalar::from_random_bytes(
                   &hasher.finalize()[..31]
               )
               .expect("failed to create scalar from bytes");
    }

    fn message_hash(&mut self, pub_key: &keypair::PubKey) -> PallasScalar {
        // TODO: hash_message
        self.sponge.absorb(&[pub_key.x]);
        // Squeeze and convert from field element to scalar
        return PallasScalar::from_repr(self.sponge.squeeze().into_repr().into());
    }
}