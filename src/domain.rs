//! Signer domain and helpers
//!
//! Shorthands and helpers for base and scalar field elements

use ark_ec::AffineCurve;
use ark_ff::PrimeField; // for into_repr()

use mina_curves::pasta::pallas as Pallas;
pub use Pallas::Affine as PallasPoint;

/// Base field
pub type PallasField = <PallasPoint as AffineCurve>::BaseField;
/// Scalar field
pub type PallasScalar = <PallasPoint as AffineCurve>::ScalarField;

use ark_serialize::{CanonicalDeserialize as _, CanonicalSerialize as _};

/// Base field element helpers
pub trait FieldHelpers {
    /// Create a field from bytes
    fn from_bytes(bytes: Vec<u8>) -> PallasField;

    /// Create a field from hex
    fn from_hex(hex: &str) -> Result<PallasField, &str>;

    /// Serialize to bytes
    fn to_bytes(self) -> Vec<u8>;

    /// Serialize to String
    fn to_string(self) -> String;
}

impl FieldHelpers for PallasField {
    fn from_hex(hex: &str) -> Result<PallasField, &str> {
        let bytes: Vec<u8> = hex::decode(hex).map_err(|_| "Failed to decode field hex")?;

        PallasField::deserialize(&mut &bytes[..]).map_err(|_| "Failed to deserialize field bytes")
    }

    fn from_bytes(bytes: Vec<u8>) -> PallasField {
        PallasField::deserialize(&mut &bytes[..]).expect("failed to deserialize field")
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        self.into_repr()
            .serialize(&mut bytes)
            .expect("Failed to serialize field");

        bytes
    }

    fn to_string(self) -> String {
        let mut bytes = self.to_bytes();
        bytes.reverse();

        hex::encode(bytes)
    }
}

/// Scalar field element helpers
// TODO: Combine into single Helpers trait (why did rust require two?!)
pub trait ScalarHelpers {
    /// Deserialize from hex
    fn from_hex(hex: &str) -> Result<PallasScalar, &str>;

    /// Serialize to bytes
    fn to_bytes(self) -> Vec<u8>;

    /// Serialize to String
    fn to_string(self) -> String;
}

impl ScalarHelpers for PallasScalar {
    fn from_hex(hex: &str) -> Result<PallasScalar, &str> {
        let mut bytes: Vec<u8> = hex::decode(hex).map_err(|_| "Failed to decode scalar hex")?;
        bytes.reverse(); // mina scalars hex format is in big-endian order

        PallasScalar::deserialize(&mut &bytes[..]).map_err(|_| "Failed to deserialize scalar bytes")
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        self.into_repr()
            .serialize(&mut bytes)
            .expect("failed to serialize scalar");

        bytes
    }

    fn to_string(self) -> String {
        let mut bytes = self.to_bytes();
        bytes.reverse();

        hex::encode(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_from_hex() {
        assert_eq!(
            PallasField::from_hex(""),
            Err("Failed to deserialize field bytes")
        );
        assert_eq!(
            PallasField::from_hex(
                "1428fadcf0c02396e620f14f176fddb5d769b7de2027469d027a80142ef8f07"
            ),
            Err("Failed to decode field hex")
        );
        assert_eq!(
            PallasField::from_hex(
                "0f5314f176fddb5d769b7de2027469d027ad428fadcf0c02396e6280142efb7d8"
            ),
            Err("Failed to decode field hex")
        );
        assert_eq!(
            PallasField::from_hex(
                "g64244176fddb5d769b7de2027469d027ad428fadcf0c02396e6280142efb7d8"
            ),
            Err("Failed to decode field hex")
        );
        assert_eq!(
            PallasField::from_hex(
                "0cdaf334e9632268a5aa959c2781fb32bf45565fe244ae42c849d3fdc7c644fd"
            ),
            Err("Failed to deserialize field bytes")
        );

        assert_eq!(
            PallasField::from_hex(
                "2eaedae42a7461d5952d27b97ecad068b698ebb94e8a0e4c45388bb613de7e08"
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn scalar_from_hex() {
        assert_eq!(
            PallasScalar::from_hex(""),
            Err("Failed to deserialize scalar bytes")
        );
        assert_eq!(
            PallasScalar::from_hex(
                "1428fadcf0c02396e620f14f176fddb5d769b7de2027469d027a80142ef8f07"
            ),
            Err("Failed to decode scalar hex")
        );
        assert_eq!(
            PallasScalar::from_hex(
                "0f5314f176fddb5d769b7de2027469d027ad428fadcf0c02396e6280142efb7d8"
            ),
            Err("Failed to decode scalar hex")
        );
        assert_eq!(
            PallasScalar::from_hex(
                "g64244176fddb5d769b7de2027469d027ad428fadcf0c02396e6280142efb7d8"
            ),
            Err("Failed to decode scalar hex")
        );
        assert_eq!(
            PallasScalar::from_hex(
                "dd4244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718"
            ),
            Err("Failed to deserialize scalar bytes")
        );

        assert_eq!(
            PallasScalar::from_hex(
                "238344cc01fd5d8cfc7c69cc4a7497bcdb3cb9810d0f8b571615dc3da2433cc2"
            )
            .is_ok(),
            true
        );
    }
}
