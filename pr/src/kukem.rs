use crate::hibe;
use bincode;
// kuKEMPublicKey is composed of the HIBE public parameters and associated data
// that acts as the HIBE public key.
pub struct KuKemPublicKey {
    pk: Vec<u8>,
    a : Vec<Vec<u8>>,
}

//generate creates a new ku-KEM key pair from a given seed
pub fn generate(seed: [u8;48]) -> (Vec<u8>, Vec<u8>) {
    let (params, root) = hibe::setup(seed);
    let sk = hibe::extract(root, vec![]);
    
    let kukemPublicKey = KuKemPublicKey {
        pk: params,
        a: vec![]
    };
    let pk = bincode::serialize(&kukemPublicKey).unwrap();
    (pk, sk)
}

//updatePublicKey updates the ku-KEM public key
pub fn update_public_key(pk: Vec<u8>, ad: Vec<u8>) -> Vec<u8> {
    let mut kukem_public_key: KuKemPublicKey = bincode::deserialize(&pk).unwrap();
    kukem_public_key.a.push(ad);
    bincode::serialize(&kukem_public_key).unwrap()
}

//updateSecretKey udates the ku-KEM secret key.
pub fn update_secret_key(sk: Vec<u8>, ad: Vec<u8>) -> Vec<u8>{
    //hibe::extract(sk, ad)
}

//encrypt generates a new key and encapsulate it in a ciphertext
pub fn encrypt(pk : &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let mut kukem_public_key: KuKemPublicKey = bincode::deserialize(&pk).unwrap();
    hibe::encrypt(kukem_public_key.pk, vec![],kukem_public_key.a)
}

pub fn decrypt(sk: Vec<u8>, ct: Vec<u8>) -> Vec<u8> {
    hibe::decrypt(sk, vec![], ct)
}