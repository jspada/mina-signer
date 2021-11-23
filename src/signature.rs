//! Mina signature structure and associated helpers

use std::fmt;

use crate::{FieldHelpers, PallasField, PallasScalar, ScalarHelpers};

/// Signature structure
#[derive(Clone, Copy, Eq, fmt::Debug, PartialEq)]
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
        write!(f, "{}{}", self.rx.to_hex(), self.s.to_hex())
    }
}
