mod sc_tests;
mod dss_tests;
mod uni_tests;

use serde::{Serialize, Deserialize};
use rand::{OsRng, Rng};
use DV;
use std::str;
use openssl::rsa::{Padding, Rsa};
use DV::SigncryptionBlock;
/*
#[test]
fn test_generate_key(){


    let public_key = DV::pkc::generate().0;
    let private_key  = DV::pkc::generate().1;

    assert_ne!(public_key, private_key);
    println!("-------------generate_key---------");
    println!("Private key: {}", String::from_utf8(private_key).unwrap());
    println!("Public key: {}", String::from_utf8(public_key).unwrap());
}




#[test]
fn test_encrypt_then_decrypt_gives_plaintext() {
    let (public_key, private_key) = DV::pkc::generate();

    let mut ad:[u8;16]=[0;16];
    let mut rng=OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut ad);


    //let message=String::from("Hello World");
    let mut sig: Vec<u8> = vec![0; 64];
    let msg = vec![1, 2, 3];
    let ad = vec![1, 2, 3];
    let u:SigncryptionBlock =  SigncryptionBlock {
        signature : sig,
        message: msg,
        ad
    };

    let message =serde_json::to_string(&u).unwrap();
    println!("message is {}", message);
    println!("message size is {}", message.len());

    let ct = DV::pkc::encrypt(public_key, message.as_bytes(), &u.ad);


    let decrypted_data=DV::pkc::decrypt(private_key, &ct[..],  &u.ad);

    let plain_text =str::from_utf8(decrypted_data.as_slice()).unwrap();
    let pt_struct :SigncryptionBlock = serde_json::from_str(&plain_text).unwrap();
    assert_eq!(message,plain_text);
    assert_eq!(u.message, pt_struct.message);
    assert_eq!(u.ad, pt_struct.ad);
    assert_eq!(u.signature, pt_struct.signature);


    println!("-------------encrypt then decrypt---------");
    println!("Original message: {}", message);
    println!("message after encrypt - decrypt: {}", plain_text);
    println!("-----------------------------------------------");


}



#[test]
fn test_as(){
    let passphrase = "rust_by_example";

    let public_key_pem = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDC+Jx89MjzbWw9PPh0dffD+i2c
J7XMioLndImQvQiNJjZ00zyxjgt4+wkual+ZHhH94HIjRIeLI+ncBEjFMa1xIzHT
exz/pvJUCsHNxNK9958zR0E997xxSf3C2Lu8BWtJG348xd5QNzb+R+i963PtcAsQ
fCu+q5gbqqtQEIjlMwIDAQAB
-----END PUBLIC KEY-----";

    let private_key_pem = "-----BEGIN RSA PRIVATE KEY-----
Proc-Type: 4,ENCRYPTED
DEK-Info: AES-128-CBC,43371B6CECDB096AC2A362FD33BF4B07

aIs3x9UBN95VJJFsd1ddYxmwAKQdFE5BJwZVYtidV+cZ4Qpmg9tdBLm8AhF5bVGR
FzAVMxTEFQgwT4o2jH2UxRkRmChwNy6aqdGteDIK6yXQK7//GMmxhbvqMmFzwdof
2E7Jkq3BQQEqMFu2CxRUPUFYRIebEIZSDCD3PoJ6p7a77qwm/KCXCbad/DqtOGkJ
wOkPH5AXLIu02MJfs+vcLswXFMlq7aaUrAv5WGt1SpKz9Co6bplSYDG7JE+906Uw
MIg4XDJTJDKCKyDaPkMydw6StvyNuZfIYUNIofulLci7yoNEGvwQHsHCaHr6n4bt
I4iC9CbkEcPbf06HAWGFfsexeLGf9mU0HVsZi83QdMhWMbOREakFU755AMvTeB8w
IMCNn55nzJlSHooKuvJAmbqBBb4+wqgwnoYQEVZmTDZxqT/eR08Zl9d1QeKB+1fw
gjZmY/10kFLnTKlWGIaLIu60ehbXxZeFbW+m1pF9uHEiIkWgkrHNjKfzWh5EyfhY
vXxWuZH92ZP/nioGzVQr00oSEPLwW1RSoAx3jPuu1EILNu7lFL896CsDZpa1Oigf
OMxk0GhMuKs4H6TlHmx5a0TOAcGYWEbnqXi+KUw7pMPFiEs1/2crFI6QfQx8R7dL
/ohKFvksPExsB196RZ1PFyMdryOr/mCqI4nBT+KzPz4zJF2iTMGq3NFQI2MvW/4g
WMwsyQtIJQviFJpYlQpOVBFaeB69oHJMxfauM8OdEU8yomFl3sAVagNxPfiWsGt4
LRsReK2BDT/pnhhZG96qSsNPwQlrwffBleTy9BGSuHHox6A7GKyVAAOMND/TY1ak
-----END RSA PRIVATE KEY-----";

    let (public_key, private_key) = DV::pkc::generate();


    let mut sig: Vec<u8> = vec![0; 64];
    let msg = vec![1, 2, 3];
    let ad = vec![1, 2, 3];
    let u:SigncryptionBlock =  SigncryptionBlock {
        signature : sig,
        message: msg,
        ad
    };

    let u1 =serde_json::to_string(&u).unwrap();
    println!("u1 is {}", u1);
    println!("u1 size is {}", u1.len());

    let u2 = u1.as_bytes();
    let rsa = Rsa::public_key_from_pem(public_key_pem.as_bytes()).unwrap();
    let mut i = 0;
    let length = 86;
    let mut encrypt_data = Vec::new();
    loop {
        let mut size = u1.len() - i;
        if size > length {
            size = length;
        }
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        let _ = rsa.public_encrypt(&u2[i..i+size], &mut buf, Padding::PKCS1_OAEP).unwrap();
        //let mut enc = buf[0..size].to_vec();
        //encrypt_data.append(&mut enc);
        encrypt_data.append(&mut buf);
        println!("encrypt_data size is {}", encrypt_data.len());
        i = i + length;
        if i >= u1.len() {
            break;
        }
    };
    let mut decrypt_data = Vec::new();
    let mut j = 0;
    let length = 128;
    let rsa = Rsa::private_key_from_pem_passphrase(private_key_pem.as_bytes(), passphrase.as_bytes()).unwrap();
    loop {
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        let bytes = rsa.private_decrypt(&encrypt_data[j..j+length], &mut buf, Padding::PKCS1_OAEP).unwrap();
        let mut enc = buf[0..bytes].to_vec();
        decrypt_data.append(&mut enc);
        j += length;
        if j >= encrypt_data.len(){
            break;
        }
    }
    let pt = String::from_utf8(decrypt_data).unwrap();
    assert_eq!(u1, pt);
}

*/