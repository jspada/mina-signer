use signer::NetworkId;

#[test]
fn signer_test() {
    use signer::{
        Keypair,
        NetworkId,
        Signer,
        Transaction,
    };

    let kp = Keypair::from_hex("164244176fddb5d769b7de2027469d027ad428fadcc0c02396e6280142efb718").expect("failed to create keypair");
    let ctx = signer::create(NetworkId::MAINNET);
    let sig = ctx.sign(
        kp,
        Transaction::new()
    );
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

    let ctx = signer::custom::<poseidon::PlonkSpongeConstants3>(pasta::fp_3::params(), NetworkId::MAINNET);
    let tx = Transaction::new();
    ctx.sign(Keypair::rand(), tx);
}
