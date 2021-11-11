pub use algebra::AffineCurve;
pub use algebra::Field;

use mina_curves::pasta::pallas as Pallas;
pub use Pallas::Affine as PallasPoint;
pub type PallasField = <PallasPoint as AffineCurve>::BaseField;
pub type PallasScalar = <PallasPoint as AffineCurve>::ScalarField;

use algebra::{
    CanonicalSerialize as _,
    CanonicalDeserialize as _,
    PrimeField, // for into_repr()
};

pub trait FieldHelpers {
    fn from_bytes(bytes: Vec<u8>) -> PallasField;
    fn from_hex(hex: &str) -> Result<PallasField, &str>;
    fn to_bytes(self) -> Vec<u8>;
    fn to_string(self) -> String;
}

impl FieldHelpers for PallasField {
    fn from_hex(hex: &str) -> Result<PallasField, &str> {
        if hex.len() != 64 {
            return Err("Invalid field hex length");
        }

        let bytes: Vec<u8> = hex::decode(hex).or_else(
        |_| Err("Failed to decode field hex")
        )?;

        return PallasField::deserialize(&mut &bytes[..]).or_else(
            |_| Err("Failed to deserialize field bytes")
        );
    }

    fn from_bytes(bytes: Vec<u8>) -> PallasField {
        return PallasField::deserialize(&mut &bytes[..]).expect("failed to deserialize field");
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        self.into_repr()
            .serialize(&mut bytes)
            .expect("Failed to serialize field"); // TODO: OK error handling?
        return bytes;
    }

    fn to_string(self) -> String {
        let mut bytes = self.to_bytes();
        bytes.reverse();
        return hex::encode(bytes);
    }
}

// TODO: Combine into single Helpers trait (why did rust require two?!)
pub trait ScalarHelpers {
    fn from_hex(hex: &str) -> Result<PallasScalar, &str>;
    fn to_bytes(self) -> Vec<u8>;
    fn to_string(self) -> String;
}

impl ScalarHelpers for PallasScalar {
    fn from_hex(hex: &str) -> Result<PallasScalar, &str> {
        if hex.len() != 64 {
            return Err("Invalid scalar hex length");
        }

        let mut bytes: Vec<u8> = hex::decode(hex).or_else(
        |_| Err("Failed to decode scalar hex")
        )?;
        bytes.reverse(); // mina scalars hex format is in big-endian order

        return PallasScalar::deserialize(&mut &bytes[..]).or_else(
            |_| Err("Failed to deserialize scalar bytes")
        );
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        self.into_repr()
            .serialize(&mut bytes)
            .expect("failed to serialize scalar"); // TODO: OK error handling?
        return bytes;
    }

    fn to_string(self) -> String {
        let mut bytes = self.to_bytes();
        bytes.reverse();
        return hex::encode(bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_from_hex() {
        assert_eq!(PallasField::from_hex(""), Err("Invalid field hex length"));
        assert_eq!(PallasField::from_hex("1428fadcf0c02396e620f14f176fddb5d769b7de2027469d027a80142ef8f07"), Err("Invalid field hex length"));
        assert_eq!(PallasField::from_hex("0f5314f176fddb5d769b7de2027469d027ad428fadcf0c02396e6280142efb7d8"), Err("Invalid field hex length"));
        assert_eq!(PallasField::from_hex("g64244176fddb5d769b7de2027469d027ad428fadcf0c02396e6280142efb7d8"), Err("Failed to decode field hex"));
        assert_eq!(PallasField::from_hex("0cdaf334e9632268a5aa959c2781fb32bf45565fe244ae42c849d3fdc7c644fd"), Err("Failed to deserialize field bytes"));

        assert_eq!(PallasField::from_hex("2eaedae42a7461d5952d27b97ecad068b698ebb94e8a0e4c45388bb613de7e08").is_ok(), true);
    }

    #[test]
    fn scalar_from_hex() {
        assert_eq!(PallasScalar::from_hex(""), Err("Invalid scalar hex length"));
        assert_eq!(PallasScalar::from_hex("1428fadcf0c02396e620f14f176fddb5d769b7de2027469d027a80142ef8f07"), Err("Invalid scalar hex length"));
        assert_eq!(PallasScalar::from_hex("0f5314f176fddb5d769b7de2027469d027ad428fadcf0c02396e6280142efb7d8"), Err("Invalid scalar hex length"));
        assert_eq!(PallasScalar::from_hex("g64244176fddb5d769b7de2027469d027ad428fadcf0c02396e6280142efb7d8"), Err("Failed to decode scalar hex"));
        assert_eq!(PallasScalar::from_hex("dd4244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718"), Err("Failed to deserialize scalar bytes"));

        assert_eq!(PallasScalar::from_hex("238344cc01fd5d8cfc7c69cc4a7497bcdb3cb9810d0f8b571615dc3da2433cc2").is_ok(), true);
    }
}