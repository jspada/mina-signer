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
        self.add_bytes(x.to_le_bytes().to_vec());
    }

    fn add_u64(&mut self, x: u64) -> () {
        self.add_bytes(x.to_le_bytes().to_vec());
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
        roi.add_bytes(vec![0x01]);
        assert!(roi.bits.len() == 8);
        assert!(roi.bits.as_raw_slice() == [0x01]);
    }

    #[test]
    fn add_two_bytes() {
        let mut roi: ROInput = ROInput::new();
        roi.add_bytes(vec![0x10, 0xac]);
        assert!(roi.bits.len() == 16);
        assert!(roi.bits.as_raw_slice() == [0x10, 0xac]);
    }

    #[test]
    fn add_five_bytes() {
        let mut roi: ROInput = ROInput::new();
        roi.add_bytes(vec![0x10, 0xac, 0x01, 0xeb, 0xca]);
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
        roi.add_bytes(vec![0x01]);
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

    #[test]
    fn add_two_scalars_and_byte() {
        use algebra::UniformRand;
        println!("{}", PallasScalar::rand(&mut rand_core::OsRng).to_string().to_lowercase());

        let scalar1 = PallasScalar::from_hex("1fbdb2a799ae51d482cd433f6e7a8c26c34c329eba7f74cbc7e18c5b4f6fdb60").expect("failed to create scalar");
        let scalar2 = PallasScalar::from_hex("07cd3821f100189d9ceffe2acfd88e4b409fb94e5a3ee2f358ebbc06b17577fe").expect("failed to create scalar");
        let mut roi: ROInput = ROInput::new();
        roi.add_scalar(scalar1);
        roi.add_bytes(vec![0x2a]);
        roi.add_scalar(scalar2);
        assert!(roi.bits.len() == 518);
        assert!(roi.bits.as_raw_slice() == [
            0x60, 0xdb, 0x6f, 0x4f, 0x5b, 0x8c, 0xe1, 0xc7,
            0xcb, 0x74, 0x7f, 0xba, 0x9e, 0x32, 0x4c, 0xc3,
            0x26, 0x8c, 0x7a, 0x6e, 0x3f, 0x43, 0xcd, 0x82,
            0xd4, 0x51, 0xae, 0x99, 0xa7, 0xb2, 0xbd, 0x1f,
            0x15, 0xff, 0xbb, 0xba, 0x58, 0x03, 0xde, 0x75,
            0xac, 0x79, 0x71, 0x1f, 0x2d, 0xa7, 0xdc, 0x4f,
            0xa0, 0x25, 0x47, 0xec, 0x67, 0x15, 0xff, 0x77,
            0xce, 0x4e, 0x0c, 0x80, 0xf8, 0x10, 0x9c, 0xe6,
            0x03
        ]);
    }

    #[test]
    fn add_u32() {
        let mut roi: ROInput = ROInput::new();
        roi.add_u32(1984u32);
        assert!(roi.bits.len() == 32);
        assert!(roi.bits.as_raw_slice() == [0xc0, 0x07, 0x00, 0x00]);
    }

    #[test]
    fn add_two_u32_and_bit() {
        let mut roi: ROInput = ROInput::new();
        roi.add_u32(1729u32);
        roi.add_bit(false);
        roi.add_u32(u32::MAX);
        assert!(roi.bits.len() == 65);
        assert!(roi.bits.as_raw_slice() == [0xc1, 0x06, 0x00, 0x00, 0xfe, 0xff, 0xff, 0xff, 0x01]);
    }

    #[test]
    fn add_u64() {
        let mut roi: ROInput = ROInput::new();
        roi.add_u64(6174u64);
        assert!(roi.bits.len() == 64);
        assert!(roi.bits.as_raw_slice() == [0x1e, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn add_two_u64_and_bits() {
        let mut roi: ROInput = ROInput::new();
        roi.add_bit(true);
        roi.add_u64(u64::MAX/6174u64);
        roi.add_bit(false);
        roi.add_u64(u64::MAX/1111u64);
        assert!(roi.bits.len() == 130);
        assert!(roi.bits.as_raw_slice() == [0xe1, 0x29, 0x89, 0xd6, 0xcb, 0x3a, 0x15, 0x00, 0x08, 0x17, 0xc4, 0x9b, 0x04, 0xf4, 0xeb, 0x00, 0x00]);
    }

    #[test]
    fn all_1() {
        let mut roi: ROInput = ROInput::new();
        roi.add_bit(true);
        roi.add_scalar(PallasScalar::from_hex("02a1d7e72199794b83c8b4d44c480fd7a38d1736345acfa9d28c1cb25d75d101").expect("failed to create scalar"));
        roi.add_u64(18446744073709551557);
        roi.add_bytes(vec![0xba, 0xdc, 0x0f, 0xfe]);
        roi.add_scalar(PallasScalar::from_hex("1018176fd80cfa457b14deebea52a67f28d86fa73d43d089445225b1e98701e7").expect("failed to create scalar"));
        roi.add_bit(false);
        roi.add_u32(2147483647);
        roi.add_bit(true);
        println!("{}", roi.bits.len());
        assert!(roi.bits.len() == 641);
        assert!(roi.bits.as_raw_slice() == [
            0x03, 0xa2, 0xeb, 0xba, 0x64, 0x39, 0x18, 0xa5,
            0x53, 0x9f, 0xb5, 0x68, 0x6c, 0x2e, 0x1a, 0x47,
            0xaf, 0x1f, 0x90, 0x98, 0xa8, 0x69, 0x91, 0x07,
            0x97, 0xf2, 0x32, 0x43, 0xce, 0xaf, 0x43, 0x05,
            0xc5, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xba, 0xdc, 0x0f, 0xfe, 0xe7, 0x01, 0x87, 0xe9,
            0xb1, 0x25, 0x52, 0x44, 0x89, 0xd0, 0x43, 0x3d,
            0xa7, 0x6f, 0xd8, 0x28, 0x7f, 0xa6, 0x52, 0xea,
            0xeb, 0xde, 0x14, 0x7b, 0x45, 0xfa, 0x0c, 0xd8,
            0x6f, 0x17, 0x18, 0x10, 0xff, 0xff, 0xff, 0x7f,
            0x01
        ]);
    }

    #[test]
    fn transaction_bits() {
        let mut roi = ROInput::new();
        roi.add_u64(1000000);       // fee
        roi.add_u64(1);             // fee token
        roi.add_bit(true);          // fee payer pk odd
        roi.add_u32(0);             // nonce
        roi.add_u32(u32::MAX);      // valid_until
        roi.add_bytes(vec![0; 34]); // memo
        roi.add_bit(false);         // tags[0]
        roi.add_bit(false);         // tags[1]
        roi.add_bit(false);         // tags[2]
        roi.add_bit(true);          // sender pk odd
        roi.add_bit(false);         // receiver pk odd
        roi.add_u64(1);             // token_id
        roi.add_u64(10000000000);   // amount
        roi.add_bit(false);         // token_locked
        roi.add_scalar(PallasScalar::from_hex("2d6d5f0550d4a730ddba8d2d53be94380e89093cf6758e277a0bca17307a21de").expect("failed to create scalar"));
        roi.add_bytes(vec![0x01]);
        assert!(roi.bits.len() == 862);
        assert!(roi.bits.as_raw_slice() == [
            0x40, 0x42, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0xfe, 0xff, 0xff, 0xff,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0xf9, 0x02, 0x95, 0x00,
            0x00, 0x00, 0x00, 0xef, 0x10, 0x3d, 0x98, 0x0b,
            0xe5, 0x05, 0xbd, 0x13, 0xc7, 0x3a, 0x7b, 0x9e,
            0x84, 0x44, 0x07, 0x1c, 0x4a, 0xdf, 0xa9, 0x96,
            0x46, 0xdd, 0x6e, 0x98, 0x53, 0x6a, 0xa8, 0x82,
            0xaf, 0xb6, 0x56, 0x00
        ])
    }
}