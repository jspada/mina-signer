//! Mina signer library

#![warn(missing_docs)]

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
        ArithmeticSponge, ArithmeticSpongeParams, PlonkSpongeConstantsBasic, Sponge,
        SpongeConstants,
    },
};

/// Mina network (or blockchain) id
#[derive(Copy, Clone)]
pub enum NetworkId {
    /// Id for all testnets
    TESTNET = 0x00,

    /// Id for mainnet
    MAINNET = 0x01,
}

impl From<NetworkId> for u8 {
    fn from(id: NetworkId) -> u8 {
        id as u8
    }
}

/// Signer interface
pub trait Signer {
    /// Sign an input using keypair
    fn sign<I: Input>(&mut self, kp: Keypair, input: I) -> Signature;

    /// Verify signature by secret key corresponding to public key on input
    fn verify<I: Input>(&mut self, sig: Signature, pub_key: PubKey, input: I) -> bool;
}

/// Create a default signer context on network
pub fn create(network_id: NetworkId) -> impl Signer {
    Schnorr::<PlonkSpongeConstantsBasic>::new(
        ArithmeticSponge::<PallasField, PlonkSpongeConstantsBasic>::new(pasta::fp::params()),
        network_id,
    )
}

/// Create a custom signer context on network using custom sponge parameters
pub fn custom<SC: SpongeConstants>(
    params: ArithmeticSpongeParams<PallasField>,
    network_id: NetworkId,
) -> impl Signer {
    Schnorr::<SC>::new(ArithmeticSponge::<PallasField, SC>::new(params), network_id)
}
