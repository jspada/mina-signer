use signer::{
    Input,
    ROInput
};

#[derive(Clone, Copy)]
pub struct Transaction {
}

impl Input for Transaction {
    fn to_roinput(self) -> ROInput {
        return ROInput::new();
    }
}

impl Transaction {
    pub fn new() -> Self {
        return Transaction { };
    }
}