use super::*;

use algebra::{
    PrimeField, // for size_in_bits()
};

use bitvec::{
    prelude::*,
    view::AsBits,
};

pub trait Input {
    fn to_roinput(self) -> ROInput;
}

pub struct ROInput {
    pub fields: Vec<PallasField>,
    pub bits:   BitVec<Lsb0, u8>
}

impl ROInput {
    pub fn new() -> Self {
        return ROInput { fields: vec![], bits: BitVec::new() };
    }

    pub fn add_field(&mut self, f: PallasField) -> () {
        self.fields.push(f);
    }

    pub fn add_scalar(&mut self, s: PallasScalar) -> () {
        // mina scalars are 255 bytes
        let bytes = s.to_bytes(); // TODO: Combine these two into one-liner
        let bits = &bytes.as_bits::<Lsb0>()[..PallasScalar::size_in_bits()];

        self.bits.extend(bits);
    }

    fn add_bit(&mut self, b: bool) -> () {
        self.bits.push(b);
    }

    pub fn add_bytes(&mut self, bytes: Vec<u8>) -> () {
        self.bits.extend_from_bitslice(bytes.as_bits::<Lsb0>());
    }

    fn add_u32(&mut self, x: u32) -> () {

    }

    fn add_u64(&mut self, x: u64) -> () {

    }

    fn to_bytes(&mut self) -> Vec<u8> {
        return vec![];
    }

    fn to_fields(&mut self) -> Vec<PallasField> {
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_bit() {
        let mut roi: ROInput = ROInput::new();
        roi.add_bit(true);
        assert!(roi.bits.len() == 1);
        assert!(roi.bits.as_raw_slice() == [0x01]);
    }

    #[test]
    fn add_two_bits() {
        let mut roi: ROInput = ROInput::new();
        roi.add_bit(false);
        roi.add_bit(true);
        assert!(roi.bits.len() == 2);
        assert!(roi.bits.as_raw_slice() == [0x02]);
    }

    #[test]
    fn add_five_bits() {
        let mut roi: ROInput = ROInput::new();
        roi.add_bit(false);
        roi.add_bit(true);
        roi.add_bit(false);
        roi.add_bit(false);
        roi.add_bit(true);
        assert!(roi.bits.len() == 5);
        assert!(roi.bits.as_raw_slice() == [0x12]);
    }

    #[test]
    fn add_byte() {
        let mut roi: ROInput = ROInput::new();
        roi.add_bytes(vec!(0x01));
        assert!(roi.bits.len() == 8);
        assert!(roi.bits.as_raw_slice() == [0x01]);
    }

    #[test]
    fn add_two_bytes() {
        let mut roi: ROInput = ROInput::new();
        roi.add_bytes(vec!(0x10, 0xac));
        assert!(roi.bits.len() == 16);
        assert!(roi.bits.as_raw_slice() == [0x10, 0xac]);
    }

    #[test]
    fn add_five_bytes() {
        let mut roi: ROInput = ROInput::new();
        roi.add_bytes(vec!(0x10, 0xac, 0x01, 0xeb, 0xca));
        assert!(roi.bits.len() == 40);
        assert!(roi.bits.as_raw_slice() == [0x10, 0xac, 0x01, 0xeb, 0xca]);
    }

    #[test]
    fn add_scalar() {
        let scalar = PallasScalar::from_hex("164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718").expect("failed to create scalar");
        let mut roi: ROInput = ROInput::new();
        roi.add_scalar(scalar);
    }

    #[test]
    fn add_scalar_and_byte() {
        let scalar = PallasScalar::from_hex("164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718").expect("failed to create scalar");
        let mut roi: ROInput = ROInput::new();
        roi.add_scalar(scalar);
        roi.add_bytes(vec!(0x01));
        assert!(roi.bits.len() == 263);
        assert!(roi.bits.as_raw_slice() == [0x18, 0xb7, 0xef, 0x42, 0x01, 0x28, 0xe6, 0x96, 0x23, 0xc0, 0xc0, 0xdc, 0xfa, 0x28, 0xd4, 0x7a, 0x02, 0x9d, 0x46, 0x27, 0x20, 0xde, 0xb7, 0x69, 0xd7, 0xb5, 0xdd, 0x6f, 0x17, 0x44, 0x42, 0x96, 0x00]);
    }

    #[test]
    fn add_two_scalars() {
        let scalar1 = PallasScalar::from_hex("164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718").expect("failed to create scalar");
        let scalar2 = PallasScalar::from_hex("05e84aeb13bfe967e9c5e842b795cbdbabff0e4e1348752741e35b8348e9b1a1").expect("failed to create scalar");
        let mut roi: ROInput = ROInput::new();
        roi.add_scalar(scalar1);
        roi.add_scalar(scalar2);
        assert!(roi.bits.len() == 510);
        assert!(roi.bits.as_raw_slice() == [
            0x18, 0xb7, 0xef, 0x42, 0x01, 0x28, 0xe6, 0x96,
            0x23, 0xc0, 0xc0, 0xdc, 0xfa, 0x28, 0xd4, 0x7a,
            0x02, 0x9d, 0x46, 0x27, 0x20, 0xde, 0xb7, 0x69,
            0xd7, 0xb5, 0xdd, 0x6f, 0x17, 0x44, 0x42, 0x96,
            0xd0, 0xd8, 0x74, 0xa4, 0xc1, 0xad, 0xf1, 0xa0,
            0x93, 0x3a, 0xa4, 0x09, 0x27, 0x87, 0xff, 0xd5,
            0xed, 0xe5, 0xca, 0x5b, 0x21, 0xf4, 0xe2, 0xf4,
            0xb3, 0xf4, 0xdf, 0x89, 0x75, 0x25, 0xf4, 0x02
        ]);
    }
}