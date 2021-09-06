use signer::NetworkId;

#[test]
fn keypair_tests() {
    use signer::Keypair;

    assert_eq!(Keypair::from_sec_key_hex(""), Err("failed to deserialize secret key bytes"));
    assert_eq!(Keypair::from_sec_key_hex("fg"), Err("failed to decode secret key hex"));
    let kp = Keypair::from_sec_key_hex("164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718").unwrap();
    println!("kp.pub_key.x = {}", kp.pub_key.x);
    println!("kp.pub_key.y = {}", kp.pub_key.y);
    println!("kp.address() = {}", kp.address());
}

#[test]
fn signer_test() {
    use signer::{
        Keypair,
        NetworkId,
        Signer,
        Transaction,
    };

    println!("Signer tests");

    let ctx = signer::create(NetworkId::MAINNET);
    let tx = Transaction::new();
    let sig = ctx.sign(Keypair::new(), tx);
}

#[test]
fn custom_signer_test() {
    use oracle::{
        poseidon,
        pasta,
    };

    use signer::{
        Signer,
        Keypair,
        Transaction,
    };

    println!("Custom signer tests");

    let ctx = signer::custom::<poseidon::PlonkSpongeConstants3>(pasta::fp_3::params(), NetworkId::MAINNET);
    let tx = Transaction::new();
    ctx.sign(Keypair::new(), tx);
}
