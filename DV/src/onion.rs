
use crypto::symmetriccipher;
use rand::{OsRng, Rng};
use crate::otae::*;
use crate::hash::Sha256;
use crate::signcryption::*;

pub fn onion_encrypt (hk: &[u8], s:&[[u8];u8], mut ad: &[u8], pt: &[u8]) -> [&[u8]; {0}] {

    //pick k -> k1, k2, .... kn
    let mut sym_key: [u8; 16] = [0; 16];
    let mut s_rng = OsRng::new().ok().unwrap();
    s_rng.fill_bytes(&mut sym_key);

    const N: usize = s.len();
    let mut ct : [&[u8]; N] = [&[0]; N];
    let mut n = N;

    ct[n] = encrypt_aes_256_ctr(pt, &sym_key, ad).expect("cannot encrypt!").as_slice();

    for i in n - 1 { // 要改，应该是倒序
        ad = &Sha256::h_eval(ad);
        ct[i] = sign_crypt(s.sks, s.pkr, ad, key);
    }
    ct
}

pub fn onion_decrypt(hk: &[u8], s:&[[u8];u8], mut ad: &[u8], ct: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {

    // parse ct-> ad_{n+1}
    let k:[u8;16] = [0;16];
    const N: usize = s.len();
    let mut ct: [&[u8]; N] = [&[0]; N];
    let mut n = N;


    //for i = n downto 1 , need to modified
    for i in n {
        //ad = &Sha256::h_eval(ad);
        ad = &Sha256::h_eval(ad);
        k[i] = unsign_crypt(s.skr, s.pks, ad,ct[i]);
    }
    let pt = decrypt_aes_256_cbc(ct.0, &k, ad);
    pt

}