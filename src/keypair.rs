use algebra::{
    ProjectiveCurve, // for into_affine()
    UniformRand,
};

use crate::domain::*;

pub type SecKey = PallasScalar;
pub type PubKey = PallasPoint;

pub struct Keypair {
    pub sec_key: SecKey,
    pub pub_key: PubKey,
}

impl Keypair {
    pub fn new() -> Self {
        let sec_key: PallasScalar = PallasScalar::rand(&mut rand_core::OsRng);
        let pub_key: PallasPoint = PallasPoint::prime_subgroup_generator().mul(sec_key).into_affine();
        return Keypair { sec_key: sec_key, pub_key: pub_key};
    }
}