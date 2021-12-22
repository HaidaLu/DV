use crate::{dss, pkc};
use serde::{Serialize, Deserialize};
use bincode;
use std::str;


// a naive signcryption SC from PKC and DSS

#[derive(Serialize, Deserialize, Debug)]
pub struct SigncryptionBlock {
    pub signature: Vec<u8>,
    pub message: Vec<u8>,
    pub ad: Vec<u8>
}


/**
generate a key pair of sign_key*/
pub fn generate_sign_key() -> ([u8; 32], [u8; 64]) {
    let seed: &[u8] = &[0x26, 0x27, 0xf6, 0x85, 0x97, 0x15, 0xad, 0x1d, 0xd2, 0x94, 0xdd, 0xc4, 0x76, 0x19, 0x39, 0x31, 0xf1, 0xad, 0xb5, 0x58, 0xf0, 0x93, 0x97, 0x32, 0x19, 0x2b, 0xd1, 0xc0, 0xfd, 0x16, 0x8e, 0x4e];//32位
    let (secret_key, public_key) = dss::generate(seed);
    (public_key, secret_key)
}

/**
generate a private and public key pair.
 */
//pub fn generate_cipher_key() -> (Vec<u8>, Vec<u8>){
pub fn generate_cipher_key() -> (Vec<u8>, Vec<u8>){
    pkc::generate()
}


// SC.Enc = PKC.Enc(pk, (pt, DSS.Sign(sks, (ad, pt))
pub fn sc_encrypt(sks: &[u8], pkr: &[u8], ad: &[u8], msg: &[u8]) -> Vec<u8> {

    let sig = dss::signature(msg, sks);

    let block = SigncryptionBlock{
        signature:sig.to_vec(),
        message: msg.to_vec(),
        ad: ad.to_vec(),
    };
    let b = bincode::serialize(&block).unwrap();
    let ct = pkc::encrypt(pkr.to_vec(), &b, ad);
    ct
}



pub fn sc_decrypt(skr: &[u8], pks: &[u8], ad : &[u8], ct: &[u8]) -> Option<Vec<u8>> {
  
    let dec = pkc::decrypt(skr.to_vec(), ct, ad); 


    let block: SigncryptionBlock = bincode::deserialize(&dec).unwrap();

    if dss::verify(&block.message, pks, &block.signature) {
        Some(block.message)
    } else {
        None
    }
}


