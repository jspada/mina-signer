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

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum NetworkId {
    TESTNET,
    MAINNET,
}

impl Into<u8> for NetworkId {
    fn into(self) -> u8 {
        self as u8
    }
}

pub trait Signer {
    fn sign<I: Input>(self, kp: Keypair, msg: I) -> Signature;
}

pub fn create(network_id: NetworkId) -> impl Signer {
    return Schnorr::<PlonkSpongeConstants>::new(
        ArithmeticSponge::<PallasField, PlonkSpongeConstants>::new(pasta::fp::params()),
        network_id,
    );
}

pub fn custom<SC: SpongeConstants>(params: ArithmeticSpongeParams<PallasField>, network_id: NetworkId) -> impl Signer {
    return Schnorr::<SC>::new(
        ArithmeticSponge::<PallasField, SC>::new(params),
        network_id,
    );
}
