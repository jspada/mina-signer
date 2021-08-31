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
pub use signature::{
    Signature,
    Signer,
};
pub use schnorr::Schnorr;

use algebra::{
    BigInteger, // for is_even()
    CanonicalSerialize, 
    PrimeField, // for from_repr()
    ProjectiveCurve, // for into_affine()
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

use blake2::{Blake2b, Digest};

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
