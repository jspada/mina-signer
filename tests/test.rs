use algebra::PrimeField;

pub mod transaction;

#[test]
fn signer_test() {
    use signer::{
        PubKey,
        PubKeyHelpers,
        Keypair,
        NetworkId,
        Signer,
    };

    use transaction::Transaction;

    {
        let kp = Keypair::from_hex("164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718").expect("failed to create keypair");
        let tx = Transaction::new_payment(kp.pub_key,
            PubKey::from_address("B62qicipYxyEHu7QjUqS7QvBipTs5CzgkYZZZkPoKVYBu6tnDUcE9Zt").expect("invalid address"),
            1729000000000,
            2000000000,
            16,
        )
        .valid_until(271828)
        .memo_str("Hello Mina!");

        println!("fee_payer   = {}", tx.fee_payer_pk.to_address());
        println!("source      = {}", tx.source_pk.to_address());
        println!("receiver    = {}", tx.receiver_pk.to_address());
        println!("amount      = {}", tx.amount);
        println!("fee         = {}", tx.fee);
        println!("fee_token   = {}", tx.fee_token);
        println!("token_id    = {}", tx.token_id);
        println!("token_locked= {}", tx.token_locked);
        println!("valid_until = {}", tx.valid_until);
        println!("memo        = {}", std::str::from_utf8(&tx.memo.to_vec()[2..]).unwrap());

        let ctx = signer::create(NetworkId::TESTNET);
        let sig = ctx.sign(kp, tx);

        // GOOD
        // 11a36a8dfe5b857b95a2a7b7b17c62c3ea33411ae6f4eb3a907064aecae353c6
        // 0794f1d0288322fe3f8bb69d6fabd4fd7c15f8d09f8783b2f087a80407e299af

        // BAD
        // ab0234bb3706300ff37310d627940c2d894ae563c3197e6b2ba2b500732eeb01
        // c2953ee2f90e28105736ca42e52f81df3001f2d3c315407e13d9aca8c6416137

        println!("sig.rx = {}", sig.rx.to_string());
        println!("sig.s  = {}", sig.s.into_repr());
        println!("sig    = {}", sig.to_string());

    }
}

#[test]
fn custom_signer_test() {
    use oracle::{
        poseidon,
        pasta,
    };

    use signer::{
        Keypair,
        NetworkId,
        Signer,
    };

    use transaction::Transaction;

    let kp = Keypair::rand();
    let ctx = signer::custom::<poseidon::PlonkSpongeConstants3>(pasta::fp_3::params(), NetworkId::MAINNET);
    let tx = Transaction::new_payment(kp.pub_key, kp.pub_key, 2049, 1, 0);
    ctx.sign(kp, tx);
}
