//! Mina signer library for verification and signing
//!
//! **Example**
//!
//! ```
//! use signer::{Input, Keypair, NetworkId, ROInput, Signer};
//!
//! #[derive(Clone, Copy)]
//! struct Thing {
//!     foo: u32,
//!     bar: u64,
//! }
//!
//! impl Input for Thing {
//!     fn to_roinput(self) -> ROInput {
//!         let mut roi = ROInput::new();
//!
//!         roi.append_u32(self.foo);
//!         roi.append_u64(self.bar);
//!
//!         roi
//!     }
//!
//!     fn domain_string(self, network_id: NetworkId) -> &'static str {
//!        match network_id {
//!            NetworkId::MAINNET => "ThingSigMainnet",
//!            NetworkId::TESTNET => "ThingSigTestnet",
//!        }
//!    }
//! }
//!
//! let kp = Keypair::rand();
//! let thang = Thing { foo: 31, bar: 45 };
//!
//! let mut ctx = signer::create(NetworkId::TESTNET);
//! let sig = ctx.sign(kp, thang);
//! assert_eq!(ctx.verify(sig, kp.public, thang), true);
//! ```
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

/// Mina network (or blockchain) identifier
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

/// Signer interface for signing [Inputs](Input) and verifying [Signatures](Signature) using [Keypairs](Keypair) and [PubKeys](PubKey)
pub trait Signer {
    /// Sign `input` (see [Input]) using keypair `kp` and return the corresponding signature.
    fn sign<I: Input>(&mut self, kp: Keypair, input: I) -> Signature;

    /// Verify that the signature `sig` on `input` (see [Input]) is signed with the secret key corresponding to `pub_key`.
    /// Return `true` if the signature is valid and `false` otherwise.
    fn verify<I: Input>(&mut self, sig: Signature, pub_key: PubKey, input: I) -> bool;
}

/// Create a default signer context for network instance identified by `network_id`
pub fn create(network_id: NetworkId) -> impl Signer {
    Schnorr::<PlonkSpongeConstantsBasic>::new(
        ArithmeticSponge::<PallasField, PlonkSpongeConstantsBasic>::new(pasta::fp::params()),
        network_id,
    )
}

/// Create a custom signer context for network instance identified by `network_id` using custom sponge parameters `params`
pub fn custom<SC: SpongeConstants>(
    params: ArithmeticSpongeParams<PallasField>,
    network_id: NetworkId,
) -> impl Signer {
    Schnorr::<SC>::new(ArithmeticSponge::<PallasField, SC>::new(params), network_id)
}
