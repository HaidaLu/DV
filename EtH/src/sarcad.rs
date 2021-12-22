//use std::borrow::Borrow;
//use core::slice::SlicePattern;

/**
1. Initall: set up the hk and initialize the key pair, the two parties' State
2. Send: Encrypt the plaintext, update the sender key and sender State
3. Receive: Decrypt the ciphertext, update the receiver key and receiver State.
 */

use rand::{Rng, OsRng};
use primitives::Sha256;
use primitives::{derive_key_pair, encrypt_aes_256_cbc, decrypt_aes_256_cbc};
use crate::state::{State,update_sk, update_rk};

pub fn initall() -> (Vec<u8>, Vec<u8>) {
    //setup a hk (need updated)
    let mut h_k:[u8;32] = [0; 32];
    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut h_k);

    let keys = derive_key_pair();
    let st_a = State {
        hk: h_k,
        sk: keys.sk,
        rk: keys.rk,
    };
    let st_b = State {
        hk: h_k,
        sk: keys.rk,
        rk: keys.sk,
    };
    let _st_a = bincode::serialize(&st_a).unwrap();
    let _st_b = bincode::serialize(&st_b).unwrap();

    (_st_a, _st_b)

}


pub fn send(st: & mut Vec<u8>, ad: &[u8], pt: &[u8]) -> (Vec<u8>, Vec<u8>){

    let _st:State = bincode::deserialize(&st).unwrap();
    //encrypt
    let sender_key = _st.sk;
    let ct = encrypt_aes_256_cbc(pt, &sender_key, &ad).ok().unwrap();

    //update the sender key

    //update the sender state
    let new_sk = Sha256::h_eval(&_st.sk);
    //let new_rk = Sha256::h_eval(&st_b.rk);
    let new_st = update_sk(_st, new_sk);
    let _new_st = bincode::serialize(&new_st).unwrap();
    //return the tuple
    (_new_st, ct)
}


pub fn receive(st:& mut Vec<u8>, ad: &[u8], ct: &[u8]) -> (Vec<u8>, Vec<u8>) {

    let _st:State = bincode::deserialize(&st).unwrap();
    //decrypt the cipher
    let receive_key = _st.rk;
    let pt = decrypt_aes_256_cbc(&ct, &receive_key, &ad).ok().unwrap();

    //update the receiver key
    /*
    let len1 = st.hk.len();
    let len2 = st.rk.len();
    for i in len1 {
     data[i] = st.hk[i];
    }
    for j in len2 {
     data[j + len1] = st.rk[j];
    }*/
    let new_rk = Sha256::h_eval(&_st.rk);

    //update the State;
    let new_st = update_rk(_st, new_rk);
    //return
    let _new_st = bincode::serialize(&new_st).unwrap();
    (_new_st, pt)

}
