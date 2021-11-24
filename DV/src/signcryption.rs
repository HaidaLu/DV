//json -> unsigncrypt
//sign key and cipher key
//extern crate crypto;
extern crate serde_json;
use crypto::*;
use crypto::ed25519::{keypair, signature, verify};
use rsa::{PaddingScheme, PublicKey, PublicKeyParts, RSAPrivateKey, RSAPublicKey};
use crate::pkc;



/**
 generate a key pair of sign_key*/
pub fn generate_sign_key() -> ([u8; 64], [u8; 32]) {
    let seed: &[u8] = &[0x26, 0x27, 0xf6, 0x85, 0x97, 0x15, 0xad, 0x1d, 0xd2, 0x94, 0xdd, 0xc4, 0x76, 0x19, 0x39, 0x31, 0xf1, 0xad, 0xb5, 0x58, 0xf0, 0x93, 0x97, 0x32, 0x19, 0x2b, 0xd1, 0xc0, 0xfd, 0x16, 0x8e, 0x4e];//32位
    keypair(seed)
}

/**
generate a private and public key pair.
*/
pub fn generate_cipher_key() -> (RSAPrivateKey, RSAPublicKey){
    let (private_key, pub_key) = pkc::generate();
    (private_key, pub_key)
}



pub fn sign_crypt(sks: &[u8], pkr: &[u8], ad: &[u8], msg: &[u8]) -> Vec<u8> {
    // use sks to sign the (ad, pt)
    let sks_key = RSAPrivateKey::from_pkcs1(sks).unwrap();
    let sig = sks_key.sign(PaddingScheme::new_pss(ad), msg).expect("Fail to sign").as_slice();
    // use pkr to encrypt the signature
    let ct = pkc::encrypt(pkr, msg, sig);
    ct
}


pub fn unsign_crypt(skr: &[u8], pks: &[u8], sig : &[u8], ct: &[u8]) -> &[u8] {
    let pks_key = RSAPublicKey::from_pkcs1(pks);
    //use skr to decrypt the ct
    let pt = pkc::decrypt(skr, ct, ad);
    //verify and get the message
    //let msg = pks_key.verify(PaddingScheme::new_pss(ad), sig).expect("Fail to verify"); //hashed??
    verify(pt, pks, sig).expect("Fail to verify");
    //return message, remove the ad
    pt
}