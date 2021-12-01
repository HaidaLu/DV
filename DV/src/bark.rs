/*

use crate::signcryption;

pub struct sk {
    sks: [u8;64],
    skr: Vec<u8>
}
pub struct pk {
    pkr: [u8;32],
    pks: Vec<u8>
}
pub fn generate() -> (sk, pk) {
    let (pk_r, sk_s) = signcryption::generate_sign_key();
    let (pk_s, sk_r) = signcryption::generate_cipher_key();
    //sks sc_encrypt 中签名， skr  sc_decrypt中RSA解密
    //let sk = (sk_s, sk_r);
    let sk = sk{
        sks: sk_s,
        skr: sk_r,
    };
    let pk = pk{
        pkr : pk_r,
        pks : pk_s
    };
    (sk, pk)
}

pub fn Init(sk_a: sk, pk_b : pk )


 */