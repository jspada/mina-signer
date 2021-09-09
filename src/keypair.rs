use algebra::{
    BigInteger,
    CanonicalDeserialize as _,
    CanonicalSerialize as _,
    PrimeField,
    ProjectiveCurve, // for into_affine()
    UniformRand,
};

use sha2::{Digest, Sha256};
use bs58;

use crate::domain::*;

pub type SecKey = PallasScalar;
pub type PubKey = PallasPoint;

#[derive(Debug, Copy, Clone, PartialEq)]
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

    pub fn from_sec_key_hex(sec_key_hex: &str) -> Result<Self, &str> {
        if sec_key_hex.len() != 64 {
            return Err("invalid secret key hex length")
        }

        let mut sec_key_bytes: Vec<u8> = match hex::decode(sec_key_hex) {
            Ok(v) => v,
            Err(_) => return Err("failed to decode secret key hex")
        };
        sec_key_bytes.reverse(); // mina secret key hex format is in big-endian order

        let sec_key: PallasScalar = match PallasScalar::deserialize(&mut &sec_key_bytes[..]) {
            Ok(v) => v,
            Err(_) => return Err("failed to deserialize secret key bytes")
        };

        let pub_key: PallasPoint = PallasPoint::prime_subgroup_generator().mul(sec_key).into_affine();
        return Ok(Keypair { sec_key: sec_key, pub_key: pub_key});
    }

    pub fn address(self) -> String {
        let mut raw: Vec<u8> = vec![
            0xcb, // version for base58 check
            0x01, // non_zero_curve_point version
            0x01, // compressed_poly version
        ];

        // pub key x-coordinate
        println!("x = {}", self.pub_key.x.into_repr());

        let mut bytes: Vec<u8> = vec![];
        self.pub_key.x
            .serialize(&mut bytes)
            .expect("failed to serialize scalar");
        println!("pub_key.x bytes = {:x?}", bytes);
        raw.extend(&bytes);

        // pub key y-coordinate parity
        raw.push(!self.pub_key.y.into_repr().is_even() as u8);
        println!("y is odd {}", !self.pub_key.y.into_repr().is_even());

        // 4-byte checksum
        println!("raw = {:x?}", raw);
        let hash1 = Sha256::digest(&raw[..]);
        let hash2 = Sha256::digest(&hash1[..]);
        println!("digest 1 = {:x?}", hash1);
        println!("digest 2 = {:x?}", hash2);
        println!("digest 2 = {:x?}", &hash2[..4]);
        raw.extend(&hash2[..4]);
        println!("raw = {:x?}", raw);

        return bs58::encode(raw).into_string();
    }
}