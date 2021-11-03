
//json -> unsigncrypt
//sign key and cipher key

//extern crate crypto;
extern crate serde_json;
use crypto::*;
use crypto::ed25519::{keypair, signature, verify};
use rsa::{PaddingScheme, PublicKey, PublicKeyParts, RSAPrivateKey, RSAPublicKey};
use crate::asymmetric;
pub struct sign_cryption_block {
    ad: [u8],
    message: [u8],
    signature: [u8],
}

pub fn generate_sign_key() -> ([u8; 64], [u8; 32]) {
    //change to rand
    let seed: &[u8] = &[0x26, 0x27, 0xf6, 0x85, 0x97, 0x15, 0xad, 0x1d, 0xd2, 0x94, 0xdd, 0xc4, 0x76, 0x19, 0x39, 0x31, 0xf1, 0xad, 0xb5, 0x58, 0xf0, 0x93, 0x97, 0x32, 0x19, 0x2b, 0xd1, 0xc0, 0xfd, 0x16, 0x8e, 0x4e];//32位
    keypair(seed)

}

// to be implemented
pub fn generate_cipher_key() -> (&[u8], &[u8]){
    let (private_key, pub_key) = asymmetric::generate();
    (private_key, pub_key)
}
//
//
pub fn sign_crypt(sks: &[u8], pkr: &[u8], ad: &[u8], msg: &[u8]) -> &[u8]{
//pub fn sign_crypt(sks: &RSAPrivateKey, pkr: &RSAPublicKey, ad: &[u8], msg: &[u8]) -> &[u8]  {
    let sks_key = RSAPrivateKey::from_pkcs1(sks).unwrap();
    let sig = sks_key.sign(PaddingScheme::new_pss(ad), msg).expect("Fail to sign").as_slice();
    //signcryptionBlock
    let block = sign_cryption_block{
        ad: *ad,
        message: *msg,
        signature: *sig,
    };
    //let v: sign_cryption_block = serde_json::from_slice(&block);

    let ct = asymmetric::encrypt(pkr, &block.message, &block.ad);
    ct
}
//
pub fn unsign_crypt(skr: &[u8], pks: &[u8], sig : &[u8], ct: &[u8]) -> &[u8] {
    let pks_key = RSAPublicKey::from_pkcs1(pks);
    //decrypt the signature
    let sig = asymmetric::decrypt(skr, ct, ad);
    //let msg = pks_key.verify(PaddingScheme::new_pss(ad), ,sig).expect("Fail to verify"); //hashed??

    verify(msg, pks, sig).expect("Fail to verify");
    //return message
    msg;
}