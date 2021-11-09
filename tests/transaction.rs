use signer::{
    Input,
    ROInput,
    PubKey,
    PubKeyHelpers,
    CompressedPubKey,
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

        roi
    }
}

impl Transaction {
    pub fn new_payment(from: PubKey, to: PubKey, amount: u64, fee: u64, nonce: u32) -> Self {
        Transaction {
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
        }
    }

    pub fn valid_until(mut self, global_slot: u32) -> Self {
        self.valid_until = global_slot;

        self
    }

    pub fn memo(mut self, memo: [u8; MEMO_BYTES]) -> Self {
        self.memo = memo;

        self
    }

    pub fn memo_str(mut self, memo: &str) -> Self {
        let memo = format!("{:\0<34}", memo); // Pad user-supplied memo with zeros
        self.memo.copy_from_slice(&memo.as_bytes()[..std::cmp::min(memo.len(), MEMO_BYTES)]);
        // Anything beyond MEMO_BYTES is truncated

        self
    }
}


#[cfg(test)]
mod tests {
    use signer::Keypair;

    use super::*;

    #[test]
    fn transaction_memo() {
        let kp = Keypair::from_hex("164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718").expect("failed to create keypair");

        let tx = Transaction::new_payment(kp.pub_key, kp.pub_key, 0, 0, 0);
        assert_eq!(tx.memo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        // Memo length < max memo length
        let tx = tx.memo([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0, 0, 0, 0, 0,
                                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34]);
        assert_eq!(tx.memo, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0, 0, 0, 0, 0,
                             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34]);


        // Memo > max memo length (truncate)
        let tx = tx.memo([8, 92, 15, 51, 52, 53, 54, 55, 56, 57, 48, 49, 50, 51, 2, 31,
                                     54, 55, 4, 57, 48, 49, 50, 51, 52, 53, 54, 55, 6, 71, 48, 49, 2, 3]);
        assert_eq!(tx.memo, [8, 92, 15, 51, 52, 53, 54, 55, 56, 57, 48, 49, 50, 51, 2, 31,
                             54, 55, 4, 57, 48, 49, 50, 51, 52, 53, 54, 55, 6, 71, 48, 49, 2, 3]);
    }

    #[test]
    fn transaction_memo_str() {
        let kp = Keypair::from_hex("164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718").expect("failed to create keypair");

        let tx = Transaction::new_payment(kp.pub_key, kp.pub_key, 0, 0, 0);
        assert_eq!(tx.memo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        // Memo length < max memo length
        let tx = tx.memo_str("Hello Mina!");
        assert_eq!(tx.memo, [72, 101, 108, 108, 111, 32, 77, 105, 110, 97, 33, 0, 0, 0, 0, 0,
                             0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);


        // Memo > max memo length (truncate)
        let tx = tx.memo_str("012345678901234567890123456789012345");
        assert_eq!(tx.memo, [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 49, 50, 51, 52, 53, 54,
                             55, 56, 57, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 49, 50, 51]);
    }
}
