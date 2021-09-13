pub use algebra::AffineCurve;
pub use algebra::Field;

use mina_curves::pasta::pallas as Pallas;
pub use Pallas::Affine as PallasPoint;
pub type PallasField = <PallasPoint as AffineCurve>::BaseField;
pub type PallasScalar = <PallasPoint as AffineCurve>::ScalarField;

use algebra::{
    CanonicalSerialize as _,
    PrimeField, // for into_repr()
    CanonicalDeserialize as _,
};

pub trait FieldHelpers {
    fn from_hex(hex: &str) -> Result<PallasField, &str>;
    fn to_bytes(self) -> Vec<u8>;
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
            |_| Err("Failed to deserialize scalar bytes")
        );
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        self.into_repr()
            .serialize(&mut bytes)
            .expect("Failed to serialize field"); // TODO: OK error handling?
            return bytes;
    }
}

// TODO: Combine into single Helpers trait (why did rust require two?!)
pub trait ScalarHelpers {
    fn from_hex(hex: &str) -> Result<PallasScalar, &str>;
    fn to_bytes(self) -> Vec<u8>;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex() {
        assert_eq!(PallasScalar::from_hex(""), Err("Invalid scalar hex length"));
        assert_eq!(PallasScalar::from_hex("1428fadcf0c02396e620f14f176fddb5d769b7de2027469d027a80142ef8f07"), Err("Invalid scalar hex length"));
        assert_eq!(PallasScalar::from_hex("0f5314f176fddb5d769b7de2027469d027ad428fadcf0c02396e6280142efb7d8"), Err("Invalid scalar hex length"));
        assert_eq!(PallasScalar::from_hex("g64244176fddb5d769b7de2027469d027ad428fadcf0c02396e6280142efb7d8"), Err("Failed to decode scalar hex"));
        assert_eq!(PallasScalar::from_hex("dd4244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718"), Err("Failed to deserialize scalar bytes"));
    }
}