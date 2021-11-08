pub mod transaction;

#[test]
fn signer_test() {
    use signer::{
        Keypair,
        NetworkId,
        Signer,
    };

    use transaction::Transaction;

    let kp = Keypair::from_hex("164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718").expect("failed to create keypair");
    let ctx = signer::create(NetworkId::MAINNET);
    let sig = ctx.sign(
        kp,
        Transaction::new_payment(kp.pub_key, kp.pub_key, 2049, 0, 1),
    );
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
    let tx = Transaction::new_payment(kp.pub_key, kp.pub_key, 2049, 0, 1);
    ctx.sign(kp, tx);
}
