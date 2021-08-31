use super::*;

pub struct Signature {
    pub rx: PallasField,
    pub s: PallasScalar,
}
pub trait Signer {
    fn sign<I: Input>(self, kp: Keypair, msg: I) -> Signature;
}
