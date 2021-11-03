//use std::borrow::Borrow;
//use core::slice::SlicePattern;
/**
    1. Initall: set up the hk and initialize the key pair, the two parties' state
    2. Send: Encrypt the plaintext, update the sender key and sender state
    3. Receive: Decrypt the ciphertext, update the receiver key and receiver state.
 */
use rand::{Rng, OsRng};
use crate::otae::{derive_key_pair, encrypt, decrypt};
use crate::state::{state,update_sk, update_rk};
use crate::hash::Sha256;



pub fn initall() -> (&state, &state) {
 //setup a hk (need updated)
 let mut h_k:[u8;32] = [0; 32];
 let mut rng = OsRng::new().ok().unwrap();
 rng.fill_bytes(&mut h_k);

 let mut keys = derive_key_pair;
 let mut st_a = state {
  hk: h_k,
  sk: keys.sk,
  rk: keys.rk,
 };
 let mut st_b = state {
  hk: h_k,
  sk: keys.rk,
  rk: keys.sk,
 };

 (&st_a, &st_b)

}

pub fn send(st: &state, ad: &[u8], pt: &[u8]) -> (&state, &[u8]){

 // encrypt
 let mut ct = encrypt(&st.sk, ad, pt).unwrap().as_slice();
 // update the sender key
 let mut data :[u8] = u8;
 let len1 = st.hk.len();
 let len2 = st.sk.len();
 for i in len1 {
  data[i] = st.hk[i];
 }
 for j in len2 {
  data[j + len1] = st.sk[j];
 }
 let mut new_sk = Sha256::h_eval(&data);

 // update the sender state
 st.update_sk(new_sk);
 //return the tuple
 (st, ct)
}

pub fn receive(st: state, ad: &[u8], ct: &[u8]) -> (bool,&state, &[u8]) {

 //decrypt the cipher
 let mut pt = decrypt(&st.rk, ad, ct).unwrap().as_slice().expect("unable to decrypt the ciphertext");

 //update the receiver key
 let len1 = st.hk.len();
 let len2 = st.rk.len();
 for i in len1 {
  data[i] = st.hk[i];
 }
 for j in len2 {
  data[j + len1] = st.rk[j];
 }
 let mut new_rk = Sha256::h_eval(data);

 //update the state;
 st.update_rk(st, new_rk);
 //return
 (true, &st, pt)

}
