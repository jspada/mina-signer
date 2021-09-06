use super::*;

use algebra::{
    CanonicalSerialize as _,
    PrimeField, // for from_repr()
};

use bitvec::{prelude::*, view::AsBits};

pub trait Input {
    fn to_roinput(self) -> ROInput;
}

pub struct ROInput {
    pub fields: Vec<PallasField>,
    pub bits:   Vec<u8>
}

impl ROInput {
    pub fn new() -> Self {
        return ROInput { fields: vec![], bits: vec![] };
    }

    pub fn add_field(&mut self, f: PallasField) -> () {
        self.fields.push(f);
    }

    pub fn add_scalar(&mut self, s: PallasScalar) -> () {
        let mut bytes: Vec<u8> = vec![];
        s.into_repr()
            .serialize(&mut bytes)
            .expect("failed to serialize scalar");

        let bits = bytes.as_bits::<Lsb0>();
        println!("bits = {}", bits);

        self.bits.extend(&bytes);
    }

    fn add_bit(&mut self, b: bool) -> () {

    }

    pub fn add_bytes(&mut self, bytes: Vec<u8>) -> () {

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
