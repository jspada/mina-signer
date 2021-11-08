use super::*;

use algebra::{
    BigInteger, // for is_even()
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
        let k: PallasScalar = self.blinding_hash(&kp, input);
        let r: PallasPoint = PallasPoint::prime_subgroup_generator().mul(k).into_affine();
        let k: PallasScalar = if r.y.0.is_even() { k } else { -k };

        let e: PallasScalar = self.message_hash(&kp.pub_key, r.x, input);
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

        hasher.update(roi.to_bytes());

        // TODO: need to swap from little-endian to big-endian?
        return PallasScalar::from_random_bytes(
                   &hasher.finalize()[..31]
               )
               .expect("failed to create scalar from bytes");
    }

    fn message_hash<I>(&mut self, pub_key: &keypair::PubKey, rx: PallasField, input: I) -> PallasScalar where I: Input {
        let mut roi: ROInput = input.to_roinput();
        roi.append_field(pub_key.x);
        roi.append_field(pub_key.y);
        roi.append_field(rx);

        self.sponge.absorb(&roi.to_fields()[..]);
        // Squeeze and convert from field element to scalar
        return PallasScalar::from_repr(self.sponge.squeeze().into_repr().into());
    }
}