pub mod domain;
pub mod keypair;
pub mod roinput;
pub mod transaction;
pub mod signature;
pub mod schnorr;

pub use domain::*;

pub use keypair::Keypair;
pub use roinput::{
    Input,
    ROInput,
};
pub use transaction::Transaction;
pub use signature::Signature;
pub use schnorr::Schnorr;

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

pub trait Signer {
    fn sign<I: Input>(self, kp: Keypair, msg: I) -> Signature;
}

pub fn create() -> impl Signer {
    return Schnorr::<PlonkSpongeConstants> {
        sponge: ArithmeticSponge::<PallasField, PlonkSpongeConstants>::new(pasta::fp::params()),
    };
}

pub fn custom<SC: SpongeConstants>(params: ArithmeticSpongeParams<PallasField>) -> impl Signer {
    return Schnorr::<SC> {
        sponge: ArithmeticSponge::<PallasField, SC>::new(params),
    };
}
