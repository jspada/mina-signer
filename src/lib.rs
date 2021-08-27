pub mod domain;
pub mod keypair;
pub mod roinput;
pub mod transaction;
pub mod signature;

pub use domain::*;

pub use keypair::Keypair;
pub use roinput::{
    Input,
    ROInput,
};
pub use transaction::Transaction;
pub use signature::Signature;

use algebra::{
    BigInteger, // for is_even()
    CanonicalSerialize, 
    PrimeField, // for from_repr()
    ProjectiveCurve, // for into_affine()
    UniformRand,
};

use oracle::{
    pasta,
    poseidon::{
        Sponge,
        ArithmeticSponge,
        SpongeConstants,
        ArithmeticSpongeParams,
        PlonkSpongeConstants,
    },
};

use blake2::{Blake2b, Digest, digest::generic_array::GenericArray};

pub trait Signer {
    fn sign<I: Input>(self, kp: Keypair, msg: I) -> Signature;
}

pub struct Context<SC: SpongeConstants> {
    pub sponge: ArithmeticSponge<PallasField, SC>,
    pub hasher: Blake2b,
}

pub fn create() -> impl Signer {
    return Context::<PlonkSpongeConstants> {
        sponge: ArithmeticSponge::<PallasField, PlonkSpongeConstants>::new(pasta::fp::params()),
        hasher: Blake2b::new(),
    };
}

pub fn custom<SC: SpongeConstants>(params: ArithmeticSpongeParams<PallasField>) -> impl Signer {
    return Context::<SC> {
        sponge: ArithmeticSponge::<PallasField, SC>::new(params),
        hasher: Blake2b::new(),
    };
}

impl<SC: SpongeConstants> Signer for Context<SC> {
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

impl<SC: SpongeConstants> Context<SC> {
    fn blinding_hash(&mut self, kp: &Keypair) -> PallasScalar {
        // TODO: derive_message
        let mut bytes: Vec<u8> = vec![];
        kp.sec_key.into_repr()
            .serialize(&mut bytes)
            .expect("failed to serialize secret key");
        
        self.hasher.update(bytes);

        // TODO: need to swap from little-endian to big-endian?
        return PallasScalar::from_random_bytes(
                   &self.hasher.clone().finalize()[..31]
               )
               .expect("failed to create scalar from bytes");
    }

    fn message_hash(&mut self, pub_key: &keypair::PubKey) -> PallasScalar {
        self.sponge.absorb(&[pub_key.x]);
        // Squeeze and convert from field element to scalar
        return PallasScalar::from_repr(self.sponge.squeeze().into_repr().into());
    }
}
