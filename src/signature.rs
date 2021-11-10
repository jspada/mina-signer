use super::*;

pub struct Signature {
    pub rx: PallasField,
    pub s: PallasScalar,
}

impl Signature {
    pub fn new(rx: PallasField, s: PallasScalar) -> Self {
        Self { rx, s }
    }

    pub fn to_string(self) -> String {
        return self.rx.to_string() + &self.s.to_string();
    }
}
