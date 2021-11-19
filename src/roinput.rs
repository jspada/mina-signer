use super::*;

use algebra::PrimeField;

use bitvec::{prelude::*, view::AsBits};

pub trait Input: Copy {
    fn to_roinput(self) -> ROInput;
    // The domain string length must be <= 20 (TODO: Limit at compile time)
    fn domain_string(self, network_id: NetworkId) -> &'static str;
}

pub struct ROInput {
    pub fields: Vec<PallasField>,
    pub bits: BitVec<Lsb0, u8>,
}

impl ROInput {
    pub fn new() -> Self {
        ROInput {
            fields: vec![],
            bits: BitVec::new(),
        }
    }

    pub fn append_field(&mut self, f: PallasField) {
        self.fields.push(f);
    }

    pub fn append_scalar(&mut self, s: PallasScalar) {
        // mina scalars are 255 bytes
        let bytes = s.to_bytes(); // TODO: Combine these two into one-liner
        let bits = &bytes.as_bits::<Lsb0>()[..PallasScalar::size_in_bits()];
        self.bits.extend(bits);
    }

    pub fn append_bit(&mut self, b: bool) {
        self.bits.push(b);
    }

    pub fn append_bytes(&mut self, bytes: Vec<u8>) {
        self.bits.extend_from_bitslice(bytes.as_bits::<Lsb0>());
    }

    pub fn append_u32(&mut self, x: u32) {
        self.append_bytes(x.to_le_bytes().to_vec());
    }

    pub fn append_u64(&mut self, x: u64) {
        self.append_bytes(x.to_le_bytes().to_vec());
    }

    pub fn to_bytes(&mut self) -> Vec<u8> {
        let mut bits: BitVec<Lsb0, u8> = self.fields.iter().fold(BitVec::new(), |mut acc, fe| {
            acc.extend_from_bitslice(
                &fe.to_bytes().as_bits::<Lsb0>()[..PallasField::size_in_bits()],
            );

            acc
        });

        bits.extend(self.bits.iter());

        return bits.as_raw_slice().to_vec();
    }

    pub fn to_fields(&mut self) -> Vec<PallasField> {
        let mut fields: Vec<PallasField> = self.fields.clone();

        let bits_as_fields = self
            .bits
            .chunks(PallasField::size_in_bits() - 1)
            .into_iter()
            .fold(vec![], |mut acc, chunk| {
                // Clone chunk into a clean copy so that the subsequent
                // as_raw_slice() only contains the chunk's bits.
                //
                // A call to chunk.as_raw_slice() returns the bitvec's
                // underlying memory as bytes, rather than a clean copy
                // with only the chunk's bits set.  If chunk's size is
                // not a byte-multiple, then the raw slice contains bits
                // from the adjacent chunk.
                //
                // To get around this we explicitly allocate a new bitvec of
                // the appropriate size, zero it and then copy the chunk's bits
                // into it using clone_from_bitslice().
                //
                // N.B. BitVec::from_bitslice() doesn't work because it
                // appears to copy chunk's raw memory into the new bitvec.

                let mut bv = BitVec::<Lsb0, u8>::new();
                bv.resize(chunk.len(), false);
                bv.clone_from_bitslice(chunk);

                // extend to the size of a field;
                bv.resize(PallasField::size_in_bits(), false);

                acc.push(PallasField::from_bytes(bv.as_raw_slice().to_vec()));

                acc
            });

        fields.extend(bits_as_fields);

        fields
    }
}

impl Default for ROInput {
    fn default() -> Self {
        ROInput::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_bit() {
        let mut roi: ROInput = ROInput::new();
        roi.append_bit(true);
        assert!(roi.bits.len() == 1);
        assert!(roi.bits.as_raw_slice() == [0x01]);
    }

    #[test]
    fn append_two_bits() {
        let mut roi: ROInput = ROInput::new();
        roi.append_bit(false);
        roi.append_bit(true);
        assert!(roi.bits.len() == 2);
        assert!(roi.bits.as_raw_slice() == [0x02]);
    }

    #[test]
    fn append_five_bits() {
        let mut roi: ROInput = ROInput::new();
        roi.append_bit(false);
        roi.append_bit(true);
        roi.append_bit(false);
        roi.append_bit(false);
        roi.append_bit(true);
        assert!(roi.bits.len() == 5);
        assert!(roi.bits.as_raw_slice() == [0x12]);
    }

    #[test]
    fn append_byte() {
        let mut roi: ROInput = ROInput::new();
        roi.append_bytes(vec![0x01]);
        assert!(roi.bits.len() == 8);
        assert!(roi.bits.as_raw_slice() == [0x01]);
    }

    #[test]
    fn append_two_bytes() {
        let mut roi: ROInput = ROInput::new();
        roi.append_bytes(vec![0x10, 0xac]);
        assert!(roi.bits.len() == 16);
        assert!(roi.bits.as_raw_slice() == [0x10, 0xac]);
    }

    #[test]
    fn append_five_bytes() {
        let mut roi: ROInput = ROInput::new();
        roi.append_bytes(vec![0x10, 0xac, 0x01, 0xeb, 0xca]);
        assert!(roi.bits.len() == 40);
        assert!(roi.bits.as_raw_slice() == [0x10, 0xac, 0x01, 0xeb, 0xca]);
    }

    #[test]
    fn append_scalar() {
        let scalar = PallasScalar::from_hex(
            "164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718",
        )
        .expect("failed to create scalar");
        let mut roi: ROInput = ROInput::new();
        roi.append_scalar(scalar);
        assert_eq!(roi.bits.len(), 255);
        assert_eq!(
            roi.bits.as_raw_slice(),
            [
                0x18, 0xb7, 0xef, 0x42, 0x01, 0x28, 0xe6, 0x96, 0x23, 0xc0, 0xc0, 0xdc, 0xfa, 0x28,
                0xd4, 0x7a, 0x02, 0x9d, 0x46, 0x27, 0x20, 0xde, 0xb7, 0x69, 0xd7, 0xb5, 0xdd, 0x6f,
                0x17, 0x44, 0x42, 0x16
            ]
        );
        assert_eq!(
            roi.to_bytes(),
            [
                0x18, 0xb7, 0xef, 0x42, 0x01, 0x28, 0xe6, 0x96, 0x23, 0xc0, 0xc0, 0xdc, 0xfa, 0x28,
                0xd4, 0x7a, 0x02, 0x9d, 0x46, 0x27, 0x20, 0xde, 0xb7, 0x69, 0xd7, 0xb5, 0xdd, 0x6f,
                0x17, 0x44, 0x42, 0x16
            ]
        );
    }

    #[test]
    fn append_scalar_and_byte() {
        let scalar = PallasScalar::from_hex(
            "164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718",
        )
        .expect("failed to create scalar");
        let mut roi: ROInput = ROInput::new();
        roi.append_scalar(scalar);
        roi.append_bytes(vec![0x01]);
        assert!(roi.bits.len() == 263);
        assert!(
            roi.bits.as_raw_slice()
                == [
                    0x18, 0xb7, 0xef, 0x42, 0x01, 0x28, 0xe6, 0x96, 0x23, 0xc0, 0xc0, 0xdc, 0xfa,
                    0x28, 0xd4, 0x7a, 0x02, 0x9d, 0x46, 0x27, 0x20, 0xde, 0xb7, 0x69, 0xd7, 0xb5,
                    0xdd, 0x6f, 0x17, 0x44, 0x42, 0x96, 0x00
                ]
        );
    }

    #[test]
    fn append_two_scalars() {
        let scalar1 = PallasScalar::from_hex(
            "164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718",
        )
        .expect("failed to create scalar");
        let scalar2 = PallasScalar::from_hex(
            "05e84aeb13bfe967e9c5e842b795cbdbabff0e4e1348752741e35b8348e9b1a1",
        )
        .expect("failed to create scalar");
        let mut roi: ROInput = ROInput::new();
        roi.append_scalar(scalar1);
        roi.append_scalar(scalar2);
        assert!(roi.bits.len() == 510);
        assert!(
            roi.bits.as_raw_slice()
                == [
                    0x18, 0xb7, 0xef, 0x42, 0x01, 0x28, 0xe6, 0x96, 0x23, 0xc0, 0xc0, 0xdc, 0xfa,
                    0x28, 0xd4, 0x7a, 0x02, 0x9d, 0x46, 0x27, 0x20, 0xde, 0xb7, 0x69, 0xd7, 0xb5,
                    0xdd, 0x6f, 0x17, 0x44, 0x42, 0x96, 0xd0, 0xd8, 0x74, 0xa4, 0xc1, 0xad, 0xf1,
                    0xa0, 0x93, 0x3a, 0xa4, 0x09, 0x27, 0x87, 0xff, 0xd5, 0xed, 0xe5, 0xca, 0x5b,
                    0x21, 0xf4, 0xe2, 0xf4, 0xb3, 0xf4, 0xdf, 0x89, 0x75, 0x25, 0xf4, 0x02
                ]
        );
    }

    #[test]
    fn append_two_scalars_and_byte() {
        let scalar1 = PallasScalar::from_hex(
            "1fbdb2a799ae51d482cd433f6e7a8c26c34c329eba7f74cbc7e18c5b4f6fdb60",
        )
        .expect("failed to create scalar");
        let scalar2 = PallasScalar::from_hex(
            "07cd3821f100189d9ceffe2acfd88e4b409fb94e5a3ee2f358ebbc06b17577fe",
        )
        .expect("failed to create scalar");
        let mut roi: ROInput = ROInput::new();
        roi.append_scalar(scalar1);
        roi.append_bytes(vec![0x2a]);
        roi.append_scalar(scalar2);
        assert!(roi.bits.len() == 518);
        assert!(
            roi.bits.as_raw_slice()
                == [
                    0x60, 0xdb, 0x6f, 0x4f, 0x5b, 0x8c, 0xe1, 0xc7, 0xcb, 0x74, 0x7f, 0xba, 0x9e,
                    0x32, 0x4c, 0xc3, 0x26, 0x8c, 0x7a, 0x6e, 0x3f, 0x43, 0xcd, 0x82, 0xd4, 0x51,
                    0xae, 0x99, 0xa7, 0xb2, 0xbd, 0x1f, 0x15, 0xff, 0xbb, 0xba, 0x58, 0x03, 0xde,
                    0x75, 0xac, 0x79, 0x71, 0x1f, 0x2d, 0xa7, 0xdc, 0x4f, 0xa0, 0x25, 0x47, 0xec,
                    0x67, 0x15, 0xff, 0x77, 0xce, 0x4e, 0x0c, 0x80, 0xf8, 0x10, 0x9c, 0xe6, 0x03
                ]
        );
    }

    #[test]
    fn append_u32() {
        let mut roi: ROInput = ROInput::new();
        roi.append_u32(1984u32);
        assert!(roi.bits.len() == 32);
        assert!(roi.bits.as_raw_slice() == [0xc0, 0x07, 0x00, 0x00]);
    }

    #[test]
    fn append_two_u32_and_bit() {
        let mut roi: ROInput = ROInput::new();
        roi.append_u32(1729u32);
        roi.append_bit(false);
        roi.append_u32(u32::MAX);
        assert!(roi.bits.len() == 65);
        assert!(roi.bits.as_raw_slice() == [0xc1, 0x06, 0x00, 0x00, 0xfe, 0xff, 0xff, 0xff, 0x01]);
    }

    #[test]
    fn append_u64() {
        let mut roi: ROInput = ROInput::new();
        roi.append_u64(6174u64);
        assert!(roi.bits.len() == 64);
        assert!(roi.bits.as_raw_slice() == [0x1e, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn append_two_u64_and_bits() {
        let mut roi: ROInput = ROInput::new();
        roi.append_bit(true);
        roi.append_u64(u64::MAX / 6174u64);
        roi.append_bit(false);
        roi.append_u64(u64::MAX / 1111u64);
        assert!(roi.bits.len() == 130);
        assert!(
            roi.bits.as_raw_slice()
                == [
                    0xe1, 0x29, 0x89, 0xd6, 0xcb, 0x3a, 0x15, 0x00, 0x08, 0x17, 0xc4, 0x9b, 0x04,
                    0xf4, 0xeb, 0x00, 0x00
                ]
        );
    }

    #[test]
    fn all_1() {
        let mut roi: ROInput = ROInput::new();
        roi.append_bit(true);
        roi.append_scalar(
            PallasScalar::from_hex(
                "02a1d7e72199794b83c8b4d44c480fd7a38d1736345acfa9d28c1cb25d75d101",
            )
            .expect("failed to create scalar"),
        );
        roi.append_u64(18446744073709551557);
        roi.append_bytes(vec![0xba, 0xdc, 0x0f, 0xfe]);
        roi.append_scalar(
            PallasScalar::from_hex(
                "1018176fd80cfa457b14deebea52a67f28d86fa73d43d089445225b1e98701e7",
            )
            .expect("failed to create scalar"),
        );
        roi.append_bit(false);
        roi.append_u32(2147483647);
        roi.append_bit(true);

        assert!(roi.bits.len() == 641);
        assert!(
            roi.bits.as_raw_slice()
                == [
                    0x03, 0xa2, 0xeb, 0xba, 0x64, 0x39, 0x18, 0xa5, 0x53, 0x9f, 0xb5, 0x68, 0x6c,
                    0x2e, 0x1a, 0x47, 0xaf, 0x1f, 0x90, 0x98, 0xa8, 0x69, 0x91, 0x07, 0x97, 0xf2,
                    0x32, 0x43, 0xce, 0xaf, 0x43, 0x05, 0xc5, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                    0xff, 0xba, 0xdc, 0x0f, 0xfe, 0xe7, 0x01, 0x87, 0xe9, 0xb1, 0x25, 0x52, 0x44,
                    0x89, 0xd0, 0x43, 0x3d, 0xa7, 0x6f, 0xd8, 0x28, 0x7f, 0xa6, 0x52, 0xea, 0xeb,
                    0xde, 0x14, 0x7b, 0x45, 0xfa, 0x0c, 0xd8, 0x6f, 0x17, 0x18, 0x10, 0xff, 0xff,
                    0xff, 0x7f, 0x01
                ]
        );
    }

    #[test]
    fn transaction_bits() {
        let mut roi = ROInput::new();
        roi.append_u64(1000000); // fee
        roi.append_u64(1); // fee token
        roi.append_bit(true); // fee payer pk odd
        roi.append_u32(0); // nonce
        roi.append_u32(u32::MAX); // valid_until
        roi.append_bytes(vec![0; 34]); // memo
        roi.append_bit(false); // tags[0]
        roi.append_bit(false); // tags[1]
        roi.append_bit(false); // tags[2]
        roi.append_bit(true); // sender pk odd
        roi.append_bit(false); // receiver pk odd
        roi.append_u64(1); // token_id
        roi.append_u64(10000000000); // amount
        roi.append_bit(false); // token_locked
        roi.append_scalar(
            PallasScalar::from_hex(
                "2d6d5f0550d4a730ddba8d2d53be94380e89093cf6758e277a0bca17307a21de",
            )
            .expect("failed to create scalar"),
        );
        roi.append_bytes(vec![0x01]);
        assert_eq!(roi.bits.len(), 862);
        assert_eq!(
            roi.bits.as_raw_slice(),
            [
                0x40, 0x42, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xfe, 0xff, 0xff, 0xff, 0x01, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf9, 0x02,
                0x95, 0x00, 0x00, 0x00, 0x00, 0xef, 0x10, 0x3d, 0x98, 0x0b, 0xe5, 0x05, 0xbd, 0x13,
                0xc7, 0x3a, 0x7b, 0x9e, 0x84, 0x44, 0x07, 0x1c, 0x4a, 0xdf, 0xa9, 0x96, 0x46, 0xdd,
                0x6e, 0x98, 0x53, 0x6a, 0xa8, 0x82, 0xaf, 0xb6, 0x56, 0x00
            ]
        )
    }

    #[test]
    fn append_field() {
        let mut roi = ROInput::new();
        roi.append_field(
            PallasField::from_hex(
                "2eaedae42a7461d5952d27b97ecad068b698ebb94e8a0e4c45388bb613de7e08",
            )
            .expect("failed to create field"),
        );

        assert_eq!(
            roi.to_bytes(),
            [
                0x2e, 0xae, 0xda, 0xe4, 0x2a, 0x74, 0x61, 0xd5, 0x95, 0x2d, 0x27, 0xb9, 0x7e, 0xca,
                0xd0, 0x68, 0xb6, 0x98, 0xeb, 0xb9, 0x4e, 0x8a, 0x0e, 0x4c, 0x45, 0x38, 0x8b, 0xb6,
                0x13, 0xde, 0x7e, 0x08
            ]
        );
    }

    #[test]
    fn append_two_fields() {
        let mut roi = ROInput::new();
        roi.append_field(
            PallasField::from_hex(
                "0cdaf334e9632268a5aa959c2781fb32bf45565fe244ae42c849d3fdc7c6441d",
            )
            .expect("failed to create field"),
        );
        roi.append_field(
            PallasField::from_hex(
                "2eaedae42a7461d5952d27b97ecad068b698ebb94e8a0e4c45388bb613de7e08",
            )
            .expect("failed to create field"),
        );

        assert_eq!(
            roi.to_bytes(),
            [
                0x0c, 0xda, 0xf3, 0x34, 0xe9, 0x63, 0x22, 0x68, 0xa5, 0xaa, 0x95, 0x9c, 0x27, 0x81,
                0xfb, 0x32, 0xbf, 0x45, 0x56, 0x5f, 0xe2, 0x44, 0xae, 0x42, 0xc8, 0x49, 0xd3, 0xfd,
                0xc7, 0xc6, 0x44, 0x1d, 0x17, 0x57, 0x6d, 0x72, 0x15, 0xba, 0xb0, 0xea, 0xca, 0x96,
                0x93, 0x5c, 0x3f, 0x65, 0x68, 0x34, 0x5b, 0xcc, 0xf5, 0x5c, 0x27, 0x45, 0x07, 0xa6,
                0x22, 0x9c, 0x45, 0xdb, 0x09, 0x6f, 0x3f, 0x04
            ]
        );
    }

    #[test]
    fn append_three_fields() {
        let mut roi = ROInput::new();
        roi.append_field(
            PallasField::from_hex(
                "1f3f142986041b54427aa2032632e34df2fa9bde9bce70c04c5034266619e529",
            )
            .expect("failed to create field"),
        );
        roi.append_field(
            PallasField::from_hex(
                "37f4433b85e753a91a1d79751645f1448954c433f9492e36a933ca7f3df61a04",
            )
            .expect("failed to create field"),
        );
        roi.append_field(
            PallasField::from_hex(
                "6cf4772d3e1aab98a2b514b73a4f6e0df1fb4f703ecfa762196b22c26da4341c",
            )
            .expect("failed to create field"),
        );

        assert_eq!(
            roi.to_bytes(),
            [
                0x1f, 0x3f, 0x14, 0x29, 0x86, 0x04, 0x1b, 0x54, 0x42, 0x7a, 0xa2, 0x03, 0x26, 0x32,
                0xe3, 0x4d, 0xf2, 0xfa, 0x9b, 0xde, 0x9b, 0xce, 0x70, 0xc0, 0x4c, 0x50, 0x34, 0x26,
                0x66, 0x19, 0xe5, 0xa9, 0x1b, 0xfa, 0xa1, 0x9d, 0xc2, 0xf3, 0xa9, 0x54, 0x8d, 0x8e,
                0xbc, 0x3a, 0x8b, 0xa2, 0x78, 0xa2, 0x44, 0x2a, 0xe2, 0x99, 0xfc, 0x24, 0x17, 0x9b,
                0xd4, 0x19, 0xe5, 0xbf, 0x1e, 0x7b, 0x0d, 0x02, 0x1b, 0xfd, 0x5d, 0x8b, 0x8f, 0xc6,
                0x2a, 0xa6, 0x68, 0x2d, 0xc5, 0xad, 0xce, 0x93, 0x5b, 0x43, 0xfc, 0xfe, 0x13, 0x9c,
                0xcf, 0xf3, 0xa9, 0x58, 0xc6, 0x9a, 0x88, 0x70, 0x1b, 0x29, 0x0d, 0x07
            ]
        );
    }

    #[test]
    fn append_field_and_scalar() {
        let mut roi = ROInput::new();
        roi.append_field(
            PallasField::from_hex(
                "64cde530327a36fcb88b6d769adca9b7c5d266e7d0042482203f3fd3a0d71721",
            )
            .expect("failed to create field"),
        );
        roi.append_scalar(
            PallasScalar::from_hex(
                "32648cf979e395ef5bc9274ca67bd6049bbdc511eed73f78db55a4dad0554360",
            )
            .expect("failed to create scalar"),
        );

        assert_eq!(
            roi.to_bytes(),
            [
                0x64, 0xcd, 0xe5, 0x30, 0x32, 0x7a, 0x36, 0xfc, 0xb8, 0x8b, 0x6d, 0x76, 0x9a, 0xdc,
                0xa9, 0xb7, 0xc5, 0xd2, 0x66, 0xe7, 0xd0, 0x04, 0x24, 0x82, 0x20, 0x3f, 0x3f, 0xd3,
                0xa0, 0xd7, 0x17, 0x21, 0xb0, 0xa1, 0x2a, 0x68, 0x6d, 0xd2, 0xaa, 0x6d, 0xbc, 0x9f,
                0x6b, 0xf7, 0x88, 0xe2, 0xde, 0x4d, 0x02, 0xeb, 0x3d, 0x53, 0xa6, 0x93, 0xe4, 0xad,
                0xf7, 0xca, 0xf1, 0xbc, 0x7c, 0x46, 0x32, 0x19
            ]
        );
    }

    #[test]
    fn append_field_bit_and_scalar() {
        let mut roi = ROInput::new();
        roi.append_field(
            PallasField::from_hex(
                "d897c7a8b811d8acd3eeaa4adf42292802eed80031c2ad7c8989aea1fe94322c",
            )
            .expect("failed to create field"),
        );
        roi.append_bit(false);
        roi.append_scalar(
            PallasScalar::from_hex(
                "282bd473ffc218d8e46c83060fa56c058f5076cae0abb291893cb5b8c66c5879",
            )
            .expect("failed to create scalar"),
        );

        assert_eq!(
            roi.to_bytes(),
            [
                0xd8, 0x97, 0xc7, 0xa8, 0xb8, 0x11, 0xd8, 0xac, 0xd3, 0xee, 0xaa, 0x4a, 0xdf, 0x42,
                0x29, 0x28, 0x02, 0xee, 0xd8, 0x00, 0x31, 0xc2, 0xad, 0x7c, 0x89, 0x89, 0xae, 0xa1,
                0xfe, 0x94, 0x32, 0x2c, 0x79, 0x58, 0x6c, 0xc6, 0xb8, 0xb5, 0x3c, 0x89, 0x91, 0xb2,
                0xab, 0xe0, 0xca, 0x76, 0x50, 0x8f, 0x05, 0x6c, 0xa5, 0x0f, 0x06, 0x83, 0x6c, 0xe4,
                0xd8, 0x18, 0xc2, 0xff, 0x73, 0xd4, 0x2b, 0x28
            ]
        );
    }

    #[test]
    fn to_bytes() {
        let mut roi = ROInput::new();
        roi.append_field(
            PallasField::from_hex(
                "a5984f2bd00906f9a86e75bfb4b2c3625f1a0d1cfacc1501e8e82ae7041efc14",
            )
            .expect("failed to create field"),
        );
        roi.append_field(
            PallasField::from_hex(
                "8af0bc770d49a5b9fcabfcdd033bab470b2a211ef80b710efe71315cfa818c0a",
            )
            .expect("failed to create field"),
        );
        roi.append_bit(false);
        roi.append_u32(314u32);
        roi.append_scalar(
            PallasScalar::from_hex(
                "238344cc01fd5d8cfc7c69cc4a7497bcdb3cb9810d0f8b571615dc3da2433cc2",
            )
            .expect("failed to create scalar"),
        );

        assert_eq!(
            roi.to_bytes(),
            [
                0xa5, 0x98, 0x4f, 0x2b, 0xd0, 0x09, 0x06, 0xf9, 0xa8, 0x6e, 0x75, 0xbf, 0xb4, 0xb2,
                0xc3, 0x62, 0x5f, 0x1a, 0x0d, 0x1c, 0xfa, 0xcc, 0x15, 0x01, 0xe8, 0xe8, 0x2a, 0xe7,
                0x04, 0x1e, 0xfc, 0x14, 0x45, 0x78, 0xde, 0xbb, 0x86, 0xa4, 0xd2, 0x5c, 0xfe, 0x55,
                0xfe, 0xee, 0x81, 0x9d, 0xd5, 0xa3, 0x05, 0x95, 0x10, 0x0f, 0xfc, 0x85, 0x38, 0x07,
                0xff, 0xb8, 0x18, 0x2e, 0xfd, 0x40, 0x46, 0x05, 0x9d, 0x00, 0x00, 0x00, 0x61, 0x9e,
                0x21, 0xd1, 0x1e, 0xee, 0x0a, 0x8b, 0xab, 0xc5, 0x87, 0x86, 0xc0, 0x5c, 0x9e, 0x6d,
                0xde, 0x4b, 0x3a, 0x25, 0xe6, 0x34, 0x3e, 0x7e, 0xc6, 0xae, 0xfe, 0x00, 0x66, 0xa2,
                0xc1, 0x11
            ]
        );
    }

    #[test]
    fn to_fields_1_scalar() {
        let mut roi = ROInput::new();
        roi.append_scalar(
            PallasScalar::from_hex(
                "25a6f0854fb5a0e31efa844f50788cbc162b0998708806c040f663ffd86d495d",
            )
            .expect("failed to create scalar"),
        );

        assert_eq!(
            roi.to_bytes(),
            [
                0x5d, 0x49, 0x6d, 0xd8, 0xff, 0x63, 0xf6, 0x40, 0xc0, 0x06, 0x88, 0x70, 0x98, 0x09,
                0x2b, 0x16, 0xbc, 0x8c, 0x78, 0x50, 0x4f, 0x84, 0xfa, 0x1e, 0xe3, 0xa0, 0xb5, 0x4f,
                0x85, 0xf0, 0xa6, 0x25
            ]
        );

        assert_eq!(
            roi.to_fields(),
            [
                PallasField::from_hex(
                    "5d496dd8ff63f640c006887098092b16bc8c78504f84fa1ee3a0b54f85f0a625"
                )
                .expect("failed to create field"),
                PallasField::from_hex(
                    "0000000000000000000000000000000000000000000000000000000000000000"
                )
                .expect("failed to create field"),
            ]
        );
    }

    #[test]
    fn to_fields_1_scalar_2_bits() {
        let mut roi = ROInput::new();
        roi.append_scalar(
            PallasScalar::from_hex(
                "3073e402aa9e8a5e710f0723d1daa690efe6b0f666733d0e0d7b418c1c96a9e8",
            )
            .expect("failed to create scalar"),
        );
        roi.append_bit(false);
        roi.append_bit(true);

        assert_eq!(
            roi.to_bytes(),
            [
                0xe8, 0xa9, 0x96, 0x1c, 0x8c, 0x41, 0x7b, 0x0d, 0x0e, 0x3d, 0x73, 0x66, 0xf6, 0xb0,
                0xe6, 0xef, 0x90, 0xa6, 0xda, 0xd1, 0x23, 0x07, 0x0f, 0x71, 0x5e, 0x8a, 0x9e, 0xaa,
                0x02, 0xe4, 0x73, 0x30, 0x01
            ]
        );

        assert_eq!(
            roi.to_fields(),
            [
                PallasField::from_hex(
                    "e8a9961c8c417b0d0e3d7366f6b0e6ef90a6dad123070f715e8a9eaa02e47330"
                )
                .expect("failed to create field"),
                PallasField::from_hex(
                    "0400000000000000000000000000000000000000000000000000000000000000"
                )
                .expect("failed to create field"),
            ]
        );
    }

    #[test]
    fn to_fields_2_scalars() {
        let mut roi = ROInput::new();
        roi.append_scalar(
            PallasScalar::from_hex(
                "22dd2d351da264714087f6cf7610450828f54a2021fdc86b0dc27ec1d2255ce0",
            )
            .expect("failed to create scalar"),
        );
        roi.append_scalar(
            PallasScalar::from_hex(
                "0fdf36dd85d3c42fee85c86e36642551efc1a6ff0d32e01888507894b3db56c3",
            )
            .expect("failed to create scalar"),
        );

        assert_eq!(
            roi.to_bytes(),
            [
                0xe0, 0x5c, 0x25, 0xd2, 0xc1, 0x7e, 0xc2, 0x0d, 0x6b, 0xc8, 0xfd, 0x21, 0x20, 0x4a,
                0xf5, 0x28, 0x08, 0x45, 0x10, 0x76, 0xcf, 0xf6, 0x87, 0x40, 0x71, 0x64, 0xa2, 0x1d,
                0x35, 0x2d, 0xdd, 0xa2, 0x61, 0xab, 0xed, 0x59, 0x4a, 0x3c, 0x28, 0x44, 0x0c, 0x70,
                0x99, 0x86, 0x7f, 0xd3, 0xe0, 0xf7, 0xa8, 0x12, 0x32, 0x1b, 0x37, 0xe4, 0x42, 0xf7,
                0x17, 0xe2, 0xe9, 0xc2, 0x6e, 0x9b, 0xef, 0x07
            ]
        );

        assert_eq!(
            roi.to_fields(),
            [
                PallasField::from_hex(
                    "e05c25d2c17ec20d6bc8fd21204af52808451076cff687407164a21d352ddd22"
                )
                .expect("failed to create field"),
                PallasField::from_hex(
                    "86adb66729f1a01031c0651afe4d83dfa34ac86cdc900bdd5f88a70bbb6dbe1f"
                )
                .expect("failed to create field"),
                PallasField::from_hex(
                    "0000000000000000000000000000000000000000000000000000000000000000"
                )
                .expect("failed to create field"),
            ]
        );
    }

    #[test]
    fn to_fields_2_bits_scalar_u32() {
        let mut roi = ROInput::new();
        roi.append_bit(true);
        roi.append_bit(false);
        roi.append_scalar(
            PallasScalar::from_hex(
                "0de1cf4144188f0c6fdc1aea7e752727928344f67dac801a25063b23de349668",
            )
            .expect("failed to create scalar"),
        );
        roi.append_u32(834803);

        assert_eq!(
            roi.to_bytes(),
            [
                0xa1, 0x59, 0xd2, 0x78, 0x8f, 0xec, 0x18, 0x94, 0x68, 0x00, 0xb2, 0xf6, 0xd9, 0x13,
                0x0d, 0x4a, 0x9e, 0x9c, 0xd4, 0xf9, 0xa9, 0x6b, 0x70, 0xbf, 0x31, 0x3c, 0x62, 0x10,
                0x05, 0x3d, 0x87, 0x37, 0xe6, 0x79, 0x19, 0x00, 0x00
            ]
        );

        assert_eq!(
            roi.to_fields(),
            [
                PallasField::from_hex(
                    "a159d2788fec18946800b2f6d9130d4a9e9cd4f9a96b70bf313c6210053d8737"
                )
                .expect("failed to create field"),
                PallasField::from_hex(
                    "98e7650000000000000000000000000000000000000000000000000000000000"
                )
                .expect("failed to create field"),
            ]
        );
    }

    #[test]
    fn to_fields_2_bits_field_scalar() {
        let mut roi = ROInput::new();
        roi.append_bit(false);
        roi.append_bit(true);
        roi.append_field(
            PallasField::from_hex(
                "90926b620ad09ed616d5df158504faed42928719c58ae619d9eccc062f920411",
            )
            .expect("failed to create field"),
        );
        roi.append_scalar(
            PallasScalar::from_hex(
                "0de1cf4144188f0c6fdc1aea7e752727928344f67dac801a25063b23de349668",
            )
            .expect("failed to create scalar"),
        );

        assert_eq!(
            roi.to_bytes(),
            [
                0x90, 0x92, 0x6b, 0x62, 0x0a, 0xd0, 0x9e, 0xd6, 0x16, 0xd5, 0xdf, 0x15, 0x85, 0x04,
                0xfa, 0xed, 0x42, 0x92, 0x87, 0x19, 0xc5, 0x8a, 0xe6, 0x19, 0xd9, 0xec, 0xcc, 0x06,
                0x2f, 0x92, 0x04, 0x11, 0xd1, 0x2c, 0x69, 0xbc, 0x47, 0x76, 0x0c, 0x4a, 0x34, 0x00,
                0x59, 0xfb, 0xec, 0x89, 0x06, 0x25, 0x4f, 0x4e, 0xea, 0xfc, 0xd4, 0x35, 0xb8, 0xdf,
                0x18, 0x1e, 0x31, 0x88, 0x82, 0x9e, 0xc3, 0x1b
            ]
        );

        assert_eq!(
            roi.to_fields(),
            [
                PallasField::from_hex(
                    "90926b620ad09ed616d5df158504faed42928719c58ae619d9eccc062f920411"
                )
                .expect("failed to create field"),
                PallasField::from_hex(
                    "a259d2788fec18946800b2f6d9130d4a9e9cd4f9a96b70bf313c6210053d8737"
                )
                .expect("failed to create field"),
                PallasField::from_hex(
                    "0000000000000000000000000000000000000000000000000000000000000000"
                )
                .expect("failed to create field"),
            ]
        );
    }

    #[test]
    fn transaction_test_1() {
        let mut roi = ROInput::new();
        roi.append_field(
            PallasField::from_hex(
                "41203c6bbac14b357301e1f386d80f52123fd00f02197491b690bddfa742ca22",
            )
            .expect("failed to create field"),
        ); // fee payer
        roi.append_field(
            PallasField::from_hex(
                "992cdaf29ffe15b2bcea5d00e498ed4fffd117c197f0f98586e405f72ef88e00",
            )
            .expect("failed to create field"),
        ); // source
        roi.append_field(
            PallasField::from_hex(
                "3fba4fa71bce0dfdf709d827463036d6291458dfef772ff65e87bd6d1b1e062a",
            )
            .expect("failed to create field"),
        ); // receiver
        roi.append_u64(1000000); // fee
        roi.append_u64(1); // fee token
        roi.append_bit(true); // fee payer pk odd
        roi.append_u32(0); // nonce
        roi.append_u32(u32::MAX); // valid_until
        roi.append_bytes(vec![0; 34]); // memo
        roi.append_bit(false); // tags[0]
        roi.append_bit(false); // tags[1]
        roi.append_bit(false); // tags[2]
        roi.append_bit(true); // sender pk odd
        roi.append_bit(false); // receiver pk odd
        roi.append_u64(1); // token_id
        roi.append_u64(10000000000); // amount
        roi.append_bit(false); // token_locked
        assert_eq!(roi.bits.len() + roi.fields.len() * 255, 1364);
        assert_eq!(
            roi.to_bytes(),
            [
                0x41, 0x20, 0x3c, 0x6b, 0xba, 0xc1, 0x4b, 0x35, 0x73, 0x01, 0xe1, 0xf3, 0x86, 0xd8,
                0x0f, 0x52, 0x12, 0x3f, 0xd0, 0x0f, 0x02, 0x19, 0x74, 0x91, 0xb6, 0x90, 0xbd, 0xdf,
                0xa7, 0x42, 0xca, 0xa2, 0x4c, 0x16, 0x6d, 0xf9, 0x4f, 0xff, 0x0a, 0x59, 0x5e, 0xf5,
                0x2e, 0x00, 0x72, 0xcc, 0xf6, 0xa7, 0xff, 0xe8, 0x8b, 0xe0, 0x4b, 0xf8, 0xfc, 0x42,
                0x43, 0xf2, 0x82, 0x7b, 0x17, 0x7c, 0x47, 0xc0, 0x8f, 0xee, 0xd3, 0xe9, 0x86, 0x73,
                0x43, 0xff, 0x7d, 0x02, 0xf6, 0x89, 0x11, 0x8c, 0x8d, 0x75, 0x0a, 0x05, 0xd6, 0xf7,
                0xfb, 0xdd, 0x8b, 0xbd, 0xd7, 0x61, 0x6f, 0xdb, 0x86, 0x87, 0x81, 0x0a, 0x48, 0xe8,
                0x01, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20,
                0x00, 0x00, 0x00, 0xc0, 0xff, 0xff, 0xff, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x5f, 0xa0, 0x12, 0x00,
                0x00, 0x00, 0x00
            ]
        );

        assert_eq!(
            roi.to_fields(),
            [
                PallasField::from_hex(
                    "41203c6bbac14b357301e1f386d80f52123fd00f02197491b690bddfa742ca22"
                )
                .expect("failed to create field"),
                PallasField::from_hex(
                    "992cdaf29ffe15b2bcea5d00e498ed4fffd117c197f0f98586e405f72ef88e00"
                )
                .expect("failed to create field"),
                PallasField::from_hex(
                    "3fba4fa71bce0dfdf709d827463036d6291458dfef772ff65e87bd6d1b1e062a"
                )
                .expect("failed to create field"),
                PallasField::from_hex(
                    "40420f0000000000010000000000000001000000feffffff0100000000000000"
                )
                .expect("failed to create field"),
                PallasField::from_hex(
                    "0000000000000000000000000000000000000000000000000000400100000000"
                )
                .expect("failed to create field"),
                PallasField::from_hex(
                    "00000000902f5009000000000000000000000000000000000000000000000000"
                )
                .expect("failed to create field"),
            ]
        );
    }
}
