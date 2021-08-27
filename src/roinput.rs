use super::*;

pub trait Input {
    fn to_roinput(self) -> ROInput;
}

pub struct ROInput {
    fields: Vec<PallasField>,
    bits:   Vec<u8>
}

impl ROInput {
    pub fn new() -> Self {
        return ROInput { fields: vec![], bits: vec![] };
    }

    fn add_field(&mut self, f: PallasField) -> () {

    }

    fn add_scalar(&mut self, s: PallasField) -> () {

    }

    fn add_bit(&mut self, b: bool) -> () {

    }

    fn add_bytes(&mut self, bytes: Vec<u8>) -> () {

    }

    fn add_u32(&mut self, x: u32) -> () {

    }

    fn add_u64(&mut self, x: u64) -> () {

    }

    fn derive_message(&mut self) -> Vec<u8> {
        return vec![];
    }

    fn hash_message(&mut self) -> Vec<PallasField> {
        return vec![];
    }
}
