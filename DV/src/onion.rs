use serde::{Serialize, Deserialize};
use crate::signcryption::*;
use crate::otae::{derive_key_pair, encrypt, decrypt};

#[derive(Serialize, Deserialize)]
pub struct OnionSender {
    sks: [u8],
    pkr: [u8],
}
#[derive(Serialize, Deserialize)]
pub struct OnionReceiver {
    skr: [u8],
    pks: [u8],
}
#[derive(Serialize, Deserialize)]
pub struct OnionMessage {
    s: *[u8],    // s designates the new receiver state
    msg: *[u8],  //plaintext
}
#[derive(Serialize, Deserialize)]
pub struct OnionCiphertext {
    ct: [[u8];u8], //2d array
}

pub fn init() -> (&[u8],&[u8]) {
    //mutable
    let sign_key = generate_sign_key();
    let sks = sign_key[0];
    let skr = sign_key[1];
    let cipher_key = generate_cipher_key();
    let pks = cipher_key[0];
    let pkr = cipher_key[1];

    let mut sender = OnionSender{
        sks,
        pkr,
    };
    let mut receiver = OnionReceiver {
        skr,
        pks,
    };
    let s = serde_json::to_string(&sender).expect("unable to encode onion sender").unwrap().as_bytes();
    let r = serde_json::to_string(&receiver).expect("unable to encode onion receiver").unwrap().as_bytes();
    (s, r)
}

pub fn send(s: &[[u8];u8], hk: &[u8], ad: &[u8], msg: &[u8]) -> (&[u8], &[u8]){
    let u = init();
    let mut us = u.0;
    let mut ur = u.1;
    let n = s.len();

    //One-time symmetric encryption
    //let k: &[u8; 16];
    //let ks : &[[u8]; n];
    let plaintext = OnionMessage{
        s:ur,
        msg,
    };
    let pt = serde_json::to_string(&plaintext).unwrap().as_bytes();


    /**
    for i := n - 1; i >= 0; i-- {
    	ad = primitives.Digest(sha256.New(), hk, ad, c[i])

    	var st onionSender
    	if err = binary.Unmarshal(s[i], &st); err != nil {
    		return nil, nil, errors.Wrap(err, "unable to decode onion sender state")
    	}

    	c[i], err = o.sc.signcrypt(st.SKS, st.PKR, ad, ks[i])
    	if err != nil {
    		return nil, nil, errors.Wrap(err, "unable to signcrypt message")
    	}
    }
     */
}

pub fn receive(s: &[[u8];u8], hk: &[u8], ad: &[u8], ct: &[u8]) -> (&[u8], &[u8]){
    let n = s.len();
    let k : &[u8;16] = &[];

    let mut pt = decrypt(k, ad, ct).unwrap().as_slice().expect("unable to decrypt the ciphertext");

}
