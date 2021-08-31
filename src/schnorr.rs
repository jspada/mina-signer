use super::*;

pub struct Schnorr<SC: SpongeConstants> {
    pub sponge: ArithmeticSponge<PallasField, SC>,
}

impl<SC: SpongeConstants> Signer for Schnorr<SC> {
    fn sign<I: Input>(mut self, kp: Keypair, input: I) -> Signature {
        println!("sign");

        let k: PallasScalar = self.blinding_hash(&kp);
        let r: PallasPoint = PallasPoint::prime_subgroup_generator().mul(k).into_affine();
        let k: PallasScalar = if r.y.0.is_even() { k } else { -k };

        let e: PallasScalar = self.message_hash(&kp.pub_key);
        let s: PallasScalar = k + e * kp.sec_key;
        return Signature { rx: r.x, s: s };
    }
}

impl<SC: SpongeConstants> Schnorr<SC> {
    fn blinding_hash(&mut self, kp: &Keypair) -> PallasScalar {
        let mut hasher: Blake2b = Blake2b::new();

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