//extern crate pkcs1;

use crypto::sha2;
/** rsa-oaep */
use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme, RSAPrivateKey, RSAPublicKey};
use pkcs1::{FromRsaPublicKey, FromRsaPrivateKey};
use rand::{Rng,OsRng};

//may need to change to u[8] format
pub fn generate() -> (&[u8], &[u8]) {

    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    let pri_key = from_pkcs1_private_key(private_key);
    let pub_key = from_pkcs1_public_key(public_key);
    (private_key, public_key)

}

pub fn encrypt(pkr: &[u8], msg: &[u8], ad: &[u8]) -> &[u8]{
    let mut rng = OsRng;
    //let padding = PaddingScheme::new_oaep::<sha2::Sha256>();
    let pkr_key = RSAPublicKey::from_pkcs1(pkr).unwrap();
    let ct = pkr_key.encrypt(&mut rng, PaddingScheme::new_pss(ad), &msg).expect("failed to encrypt");
    ct.as_slice()
}

pub fn decrypt(sk: &[u8], ct: &[u8], ad: &[u8]) -> &[u8] {
    let sk_key = RSAPrivateKey::from_pkcs1(sk).unwrap();
    let msg = sk_key.decrypt(PaddingScheme::new_pss(ad), &ct).expect("failed to decrypt");
    msg;
}