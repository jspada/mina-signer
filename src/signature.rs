use super::*;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Signature {
    pub rx: PallasField,
    pub s: PallasScalar,
}

impl Signature {
    pub fn new(rx: PallasField, s: PallasScalar) -> Self {
        Self { rx, s }
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rx.to_string(), self.s.to_string())
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.rx == other.rx && self.s == other.s
    }
}

impl Eq for Signature {}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Signature")
            .field("rx", &self.rx)
            .field("s", &self.s)
            .finish()
    }
}
