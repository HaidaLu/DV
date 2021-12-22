use dv;
use dv::{pkc, SigncryptionBlock};
//use dv::{generate_sign_key, generate_cipher_key, sc_encrypt, sc_decrypt};
use std::str;


#[test]
fn test_generate_cipher_key(){


    let public_key = dv::generate_cipher_key().0;
    let private_key  = dv::generate_cipher_key().1;

    assert_ne!(public_key, private_key);
    println!("-------------generate cihper key---------");
    println!("Private key: {}", String::from_utf8(private_key.to_vec()).unwrap());
    println!("Public key: {}", String::from_utf8(public_key.to_vec()).unwrap());
}

#[test]
fn test_generate_sign_key(){
    let (public_key, private_key) = dv::generate_sign_key();
    println!("-------------generate sign key---------");
    println!("Private key: {:?}", public_key);
    println!("Public key: {:?}", private_key);
}



#[test]
fn test_serialize_signcryption_block() {
    let sig: [u8; 64] = [0; 64];
    let msg = vec![1, 2, 3];
    let ad = vec![1, 2, 3];
    let u:SigncryptionBlock =  SigncryptionBlock {
        signature : sig.to_vec(),
        message: msg,
        ad
    };
    let u1 = serde_json::to_string(&u).unwrap();
    println!("struct SigncryptionBlock serializes into byte array {}", u1);
    let ud: SigncryptionBlock = serde_json::from_str(&u1).unwrap();
    assert_eq!(u.message, ud.message);
    assert_eq!(u.ad, ud.ad);
    assert_eq!(u.signature, ud.signature);
}


#[test]
fn test_struct_encrypt() {
    let (public_key, private_key) = dv::pkc::generate();
    let sig: [u8; 64] = [0; 64];
    let msg = vec![1, 2, 3];
    let ad = vec![1, 2, 3];
    let u:SigncryptionBlock =  SigncryptionBlock {
        signature : sig.to_vec(),
        message: msg,
        ad
    };

    let u1 = serde_json::to_string(&u).unwrap();

    println!("u1 is {:?}", u1);
    println!("u1 size is {}", u1.len());
    let ct = pkc::encrypt(public_key.to_vec(), u1.as_bytes(), &u.ad);
    println!("u1 is {:?}", u1);



    let decrypted_data=dv::pkc::decrypt(private_key.to_vec(), &ct[..],  &u.ad);
    let plain_text =str::from_utf8(decrypted_data.as_slice()).unwrap();
    let pt_struct :SigncryptionBlock = serde_json::from_str(&plain_text).unwrap();
    assert_eq!(u1,plain_text);
    assert_eq!(u.message, pt_struct.message);
    assert_eq!(u.ad, pt_struct.ad);
    assert_eq!(u.signature, pt_struct.signature);
}



#[test]
fn test_encrypt_decrypt() {


    let message = b"Hello, World!";
    // message -> dss.sign

    let ad:[u8; 32] = [0; 32];

    let (pk_r, sk_s) = dv::generate_sign_key();

    let (pk_s, sk_r) = dv::generate_cipher_key();
    let ct = dv::sc_encrypt(&sk_s, &pk_s, &ad, message);
    println!("first, sign the message with sign secret key and get the signature");
    println!("encrypt the signcryptionBlock with cipher public key, then get the ciphertext");
    //println!("ciphertext is: {:?}", ct);

    let pt = dv::sc_decrypt(&sk_r, &pk_r, &ad, &ct[..]).unwrap();
    let plain_text =str::from_utf8(&pt).unwrap();
    println!("first, decrypt the ciphertext with cipher secret key, and get the signcryptionBlock");
    println!("then build the struct, use signature and message to verify");
    println!("if verified, get the message");
    let msg = str::from_utf8(message).unwrap();
    assert_eq!(msg, plain_text);
    println!("original message is {}", msg);
    println!("verify!! and get the message: {}", plain_text);

}


