//! Mina signature structure and associated helpers

use std::fmt;

use crate::{BaseField, FieldHelpers, ScalarField, ScalarHelpers};

/// Signature structure
#[derive(Clone, Copy, Eq, fmt::Debug, PartialEq)]
pub struct Signature {
    /// Field component
    pub rx: BaseField,

    /// Scalar component
    pub s: ScalarField,
}

impl Signature {
    /// Create a new signature
    pub fn new(rx: BaseField, s: ScalarField) -> Self {
        Self { rx, s }
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rx.to_hex(), self.s.to_hex())
    }
}
