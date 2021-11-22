//! Mina signature structure and associated helpers

use std::fmt;

use crate::{FieldHelpers, PallasField, PallasScalar, ScalarHelpers};

/// Signature structure
#[derive(Clone, Copy)]
pub struct Signature {
    /// Field component
    pub rx: PallasField,

    /// Scalar component
    pub s: PallasScalar,
}

impl Signature {
    /// Create a new signature
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
