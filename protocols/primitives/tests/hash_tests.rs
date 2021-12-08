use primitives;



#[test]
fn test_update_key() {
    let keypair = primitives::derive_key_pair();
    let sk = keypair.sk;
    let rk = keypair.rk;
    let new_sk = primitives::Sha256::h_eval(&sk);

    let new_rk = primitives::Sha256::h_eval(&rk);

    assert_ne!(sk, new_sk);
    assert_ne!(rk, new_rk);
    println!("previous sk is {:?}", sk);
    println!("previous rk is {:?}", rk);
    println!("new sk is {:?}", new_sk);
    println!("new rk is {:?}", new_rk);
}

#[test]
fn same_input_same_hash() {
    let keypair = primitives::derive_key_pair();
    let sk = keypair.sk;
    assert_eq!(primitives::Sha256::h_eval(&sk), primitives::Sha256::h_eval(&sk));
}

