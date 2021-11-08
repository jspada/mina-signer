use signer::{
    Input,
    ROInput,
    keypair::{
        CompressedPubKey,
        PubKey,
        PubKeyHelpers
    }
};

const MEMO_BYTES: usize = 34;
const TAG_BITS: usize = 3;
const PAYMENT_TX_TAG: [bool; TAG_BITS] = [false; TAG_BITS];

#[derive(Clone, Copy)]
pub struct Transaction {
    // Common
    fee: u64,
    fee_token: u64,
    fee_payer_pk: CompressedPubKey,
    nonce: u32,
    valid_until: u32,
    memo: [u8; MEMO_BYTES],
    // Body
    tag: [bool; TAG_BITS],
    source_pk: CompressedPubKey,
    receiver_pk: CompressedPubKey,
    token_id: u64,
    amount: u64,
    token_locked: bool,
}

impl Input for Transaction {
    fn to_roinput(self) -> ROInput {
        let mut roi = ROInput::new();

        roi.append_field(self.fee_payer_pk.x);
        roi.append_field(self.source_pk.x);
        roi.append_field(self.receiver_pk.x);

        roi.append_u64(self.fee);
        roi.append_u64(self.fee_token);
        roi.append_bit(self.fee_payer_pk.is_odd);
        roi.append_u32(self.nonce);
        roi.append_u32(self.valid_until);
        roi.append_bytes(self.memo.to_vec());

        for tag_bit in self.tag {
            roi.append_bit(tag_bit);
        }

        roi.append_bit(self.source_pk.is_odd);
        roi.append_bit(self.receiver_pk.is_odd);
        roi.append_u64(self.token_id);
        roi.append_u64(self.amount);
        roi.append_bit(self.token_locked);

        return roi;
    }
}

impl Transaction {
    pub fn new_payment(from: PubKey, to: PubKey, amount: u64, nonce: u32, fee: u64) -> Self {
        return Transaction {
            fee: fee,
            fee_token: 1,
            fee_payer_pk: from.to_compressed(),
            nonce: nonce,
            valid_until: u32::MAX,
            memo: [0; MEMO_BYTES],
            tag: PAYMENT_TX_TAG,
            source_pk: from.to_compressed(),
            receiver_pk: to.to_compressed(),
            token_id: 1,
            amount: amount,
            token_locked: false,
        };
    }

    pub fn valid_until(mut self, global_slot: u32) -> Self {
        self.valid_until = global_slot;
        return self;
    }

    pub fn memo(mut self, memo: [u8; MEMO_BYTES]) -> Self {
        self.memo = memo;
        return self;
    }
}