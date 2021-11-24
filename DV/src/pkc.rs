extern crate rsa;
//extern crate pkcs1;
use crypto::sha2;
use rand::OsRng;
use rsa::{PublicKey, PaddingScheme, RSAPrivateKey, RSAPublicKey};

//pkc -> signcryption -> onion -> uni


/** provide fn for RSA asymmetric encryption/decryption
 */


pub fn generate() -> (RSAPrivateKey, RSAPublicKey) {

    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RSAPublicKey::from(&private_key);
    (private_key, public_key)
}


pub fn encrypt(pkr: &[u8], msg: &[u8], ad: &[u8]) -> Vec<u8> {
    let mut rng = OsRng;
    let pkr_key = RSAPublicKey::from_pkcs1(pkr).unwrap();
    let ct = pkr_key.encrypt(&mut rng, PaddingScheme::new_pss(ad), &msg).expect("failed to encrypt");
    ct
}

pub fn decrypt(sk: &[u8], ct: &[u8], ad: &[u8]) -> &[u8] {
    let sk_key = RSAPrivateKey::from_pkcs1(sk).unwrap();
    let msg = sk_key.decrypt(PaddingScheme::new_pss(ad), &ct).expect("failed to decrypt");
    msg.as_slice()
}

