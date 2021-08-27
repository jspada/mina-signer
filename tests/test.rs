#[test]
fn signer_test() {
    use signer::{
        Signer,
        Keypair,
        Transaction,
    };

    println!("Signer tests");

    let ctx = signer::create();
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

    let ctx = signer::custom::<poseidon::PlonkSpongeConstants3>(pasta::fp_3::params());
    let tx = Transaction::new();
    ctx.sign(Keypair::new(), tx);
}
