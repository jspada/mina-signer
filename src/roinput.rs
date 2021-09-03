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

    pub fn add_field(&mut self, f: PallasField) -> () {
        self.fields.push(f);
    }

    pub fn add_scalar(&mut self, s: PallasScalar) -> () {

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
