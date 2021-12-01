use EtH;
use EtH::{update_sk, update_rk};
use EtH::Sha256;


/*
#[test]
fn test_update_sk(){
    let (st_a, st_b) = EtH::initall();
    let new_key_pair = EtH::derive_key_pair();
    let new_st_a = update_sk(st_a,new_key_pair.sk);//此时st_a 不能在用了
    assert_eq!(new_st_a.hk, st_b.hk);
    assert_ne!(new_st_a.sk, st_b.rk);
    assert_eq!(new_st_a.rk, st_b.sk);
}

#[test]
fn test_update_rk(){
    let (st_a, st_b) = EtH::initall();
    let new_key_pair = EtH::derive_key_pair();
    let new_st_a = update_rk(st_a,new_key_pair.rk);//此时st_a 不能在用了
    assert_eq!(new_st_a.hk, st_b.hk);
    assert_eq!(new_st_a.sk, st_b.rk);
    assert_ne!(new_st_a.rk, st_b.sk);
}


/**
    alice use h_eval to update its sk'
    bob use h_eval to update its rk'
    sk' = rk'
*/
#[test]
fn test_send_and_receiver_update() {
    let (st_a, st_b) = EtH::initall();
    let new_sk = Sha256::h_eval(&st_a.sk);
    let new_rk = Sha256::h_eval(&st_b.rk);
    let new_st_a = update_sk(st_a,new_sk);
    let new_st_b = update_rk(st_b, new_rk);
    assert_eq!(new_st_a.hk, new_st_b.hk);
    assert_eq!(new_st_a.sk, new_st_b.rk);
    assert_eq!(new_st_a.rk, new_st_b.sk);
}

*/