use std::ops::{Neg};

use algebra::{BigInteger, PrimeField};

use sha2::{Digest, Sha256};
use bs58;

use crate::domain::*;

const MINA_ADDRESS_LEN: usize = 55;

pub type PubKey = PallasPoint;

#[derive(Clone, Copy)]
pub struct CompressedPubKey {
    pub x: PallasField,
    pub is_odd: bool,
}

pub trait PubKeyHelpers {
    fn to_compressed(self) -> CompressedPubKey;
    fn to_address(self) -> String;
    fn from_address(b58: &str) -> Result<PubKey, &'static str>;
}

impl PubKeyHelpers for PubKey {
    fn to_compressed(self) -> CompressedPubKey {
        return CompressedPubKey {
            x: self.x,
            is_odd: !self.y.into_repr().is_even(),
        };
    }

    fn to_address(self) -> String {
        let mut raw: Vec<u8> = vec![
            0xcb, // version for base58 check
            0x01, // non_zero_curve_point version
            0x01, // compressed_poly version
        ];

        // pub key x-coordinate
        raw.extend(self.x.to_bytes());

        // pub key y-coordinate parity
        raw.push(!self.y.into_repr().is_even() as u8); // TODO: confirm is_even() like this is correct

        // 4-byte checksum
        let hash = Sha256::digest(&Sha256::digest(&raw[..])[..]);
        raw.extend(&hash[..4]);

        return bs58::encode(raw).into_string();
    }

    fn from_address(address: &str) -> Result<Self, &'static str> {
        if address.len() != MINA_ADDRESS_LEN {
            return Err("Invalid address length");
        }

        let bytes = bs58::decode(address).into_vec().or_else(
            |_| Err("Invalid address encoding")
        )?;

        let (raw, checksum) = (&bytes[..bytes.len()-4], &bytes[bytes.len()-4..]);
        let hash = Sha256::digest(&Sha256::digest(&raw[..])[..]);
        if checksum != &hash[..4] {
            return Err("Invalid address checksum");
        }

        let (version, x_bytes, y_parity) = (&raw[..3], &raw[3..bytes.len()-5], raw[bytes.len()-5] == 0x01);
        if version != [0xcb, 0x01, 0x01] {
            return Err("Invalid address version info");
        }

        let x = PallasField::from_bytes(x_bytes.to_vec());
        let mut pt = PallasPoint::get_point_from_x(x, y_parity).ok_or(
            "Invalid address x-coordinate"
        )?;

        if pt.y.into_repr().is_even() == y_parity {
            pt.y = pt.y.neg();
        }

        return Ok(pt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_address() {
        macro_rules! assert_from_address_check {
            ($address:expr) => {
                let pk = PubKey::from_address($address).expect("failed to create pubkey");
                assert_eq!(pk.to_address(), $address);
            };
        }

        assert_from_address_check!("B62qnzbXmRNo9q32n4SNu2mpB8e7FYYLH8NmaX6oFCBYjjQ8SbD7uzV");
        assert_from_address_check!("B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt");
        assert_from_address_check!("B62qoG5Yk4iVxpyczUrBNpwtx2xunhL48dydN53A2VjoRwF8NUTbVr4");
        assert_from_address_check!("B62qrKG4Z8hnzZqp1AL8WsQhQYah3quN1qUj3SyfJA8Lw135qWWg1mi");
        assert_from_address_check!("B62qoqiAgERjCjXhofXiD7cMLJSKD8hE8ZtMh4jX5MPNgKB4CFxxm1N");
        assert_from_address_check!("B62qkiT4kgCawkSEF84ga5kP9QnhmTJEYzcfgGuk6okAJtSBfVcjm1M");
    }
}