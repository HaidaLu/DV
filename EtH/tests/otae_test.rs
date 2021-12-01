//    public void testSecretKeyIsAES() throws NoSuchAlgorithmException {
//    public void testSameStringGivesSameKey() throws NoSuchAlgorithmException {
//    public void testEncodeKeyEqualsString() throws NoSuchAlgorithmException {
//    public void testGeneratedKeysEqual() throws NoSuchAlgorithmException {

use rand::{OsRng, Rng};
use EtH;
use std::str;
use EtH::{decrypt_aes_256_ctr, encrypt_aes_256_ctr};


/*
#[test]
fn session_keys_are_unique(){
    let keypair = EtH::derive_key_pair();
    let sk = keypair.sk;
    let rk = keypair.rk;
    assert_ne!(sk, rk);
    println!("sk is: {:?}",sk);
    println!("rk is: {:?}", rk);
}

#[test]
fn test_key_length() {
    let keypair = EtH::derive_key_pair();
    let sk = keypair.sk;
    let rk = keypair.rk;
    assert_eq!(sk.len(), 32);
    assert_eq!(rk.len(), 32);
}



#[test]
fn test_aes256_cbc_encrypt_then_decrypt_gives_plaintext() {

    let message="Hello World!";
    let keypair = EtH::derive_key_pair();
    let mut key = keypair.sk;
    let mut iv:[u8;16]=[0;16];

    let mut rng=OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut iv);
    let ct = EtH::encrypt_aes_256_cbc(message.as_bytes(), &key, &iv).ok().unwrap();
    let pt = EtH::decrypt_aes_256_cbc(&ct[..], &key, &iv).ok().unwrap();
    let crypt_message=str::from_utf8(pt.as_slice()).unwrap();
    assert_eq!(message,crypt_message);
    println!("message is: {}", message);
    println!("message after encrypt-decrypt is: {}", crypt_message);
}

#[test]
fn test_aes256_ctr_encrypt_then_decrypt_gives_plaintext() {
    let message="Hello World";

    let keypair = EtH::derive_key_pair();
    let mut key = keypair.sk;
    let mut ad:[u8;16]=[0;16];
    let mut rng=OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut ad);

    let cipher_text = encrypt_aes_256_ctr(message.as_bytes(), &key, &ad).ok().unwrap();
    let decrypted_data=decrypt_aes_256_ctr(&cipher_text[..], &key, &ad).ok().unwrap();

    let plain_text =str::from_utf8(decrypted_data.as_slice()).unwrap();
    assert_eq!(message, plain_text);
    println!("message is: {}", message);
    //println!("message after encrypt is {}", str::from_utf8(ct.as_slice()).unwrap());
    println!("message after encrypt-decrypt is: {}", plain_text);


}

 */