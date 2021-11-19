pub mod domain;
pub mod keypair;
pub mod pubkey;
pub mod roinput;
pub mod schnorr;
pub mod signature;

pub use domain::{FieldHelpers, PallasField, PallasPoint, PallasScalar, ScalarHelpers};
pub use keypair::Keypair;
pub use pubkey::{CompressedPubKey, PubKey, PubKeyHelpers};
pub use roinput::{Input, ROInput};
pub use schnorr::Schnorr;
pub use signature::Signature;

use oracle::{
    pasta,
    poseidon::{
        ArithmeticSponge, ArithmeticSpongeParams, PlonkSpongeConstants, Sponge, SpongeConstants,
    },
};

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum NetworkId {
    TESTNET = 0x00,
    MAINNET = 0x01,
}

impl From<NetworkId> for u8 {
    fn from(id: NetworkId) -> u8 {
        id as u8
    }
}

pub trait Signer {
    fn sign<I: Input>(&mut self, kp: Keypair, input: I) -> Signature;
    fn verify<I: Input>(&mut self, sig: Signature, pub_key: PubKey, input: I) -> bool;
}

pub fn create(network_id: NetworkId) -> impl Signer {
    Schnorr::<PlonkSpongeConstants>::new(
        ArithmeticSponge::<PallasField, PlonkSpongeConstants>::new(pasta::fp::params()),
        network_id,
    )
}

pub fn custom<SC: SpongeConstants>(
    params: ArithmeticSpongeParams<PallasField>,
    network_id: NetworkId,
) -> impl Signer {
    Schnorr::<SC>::new(ArithmeticSponge::<PallasField, SC>::new(params), network_id)
}
