// HIBE

// kupke implements the key-updatable public-key encryption scheme based on a HIBE
use rand::{OsRng, Rng};
use crate::hibe;
use crate::hibe::*;
// kuPKEPublicKey bundles the public key material.
#[derive(Serialize, Deserialize)]
pub struct KuPKEPublicKey {
    pub pk : [u8; 32], //pk is the HIBE public parameters
    pub i: Vec<Vec<u8>>, // I is an array of associated data
}

// kuPKECiphertext bundles the two HIBE ciphertext parts.
#[derive(Serialize, Deserialize)]
pub struct KuPKECiphertext {
    pub c1: Vec<u8>,
    pub c2: Vec<u8>,
}

//generate creates a fresh public/private key pair.
pub fn generate() -> (Vec<u8>, Vec<u8>) {

    let mut seed: [u8; 16] = [0; 16];
    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut seed);

    /** hibe*/
    //params, root = k.hibe.Setup(seed[:])

    //Extract a first entity since root PKG is not able to perform encryptins
    // This differs from the specification in the paper where the root PKGs is returned
    //sk, err = k.hibe.Extract(root, []byte{})

    let ku_pke_public_key = KuPKEPublicKey {
        pk: params,
        i :  vec![vec![0]],
    };
    let pk = bincode::serialize(&ku_pke_public_key).unwrap();
    (sk, pk)
}

// update_public_key creates a new public key.
pub fn update_public_key(pk: Vec<u8>, delta: Vec<u8>) -> Vec<u8> {
    let mut public: KuPKEPublicKey = bincode::deserialize(&pk).unwrap();
    public.i.push(delta);
    let new_public = bincode::serialize(&public).unwrap();
    new_public
}

// updatePrivateKey creates a new private key
pub fn update_private_key(sk: Vec<u8>, delta: Vec<u8>) -> Vec<u8> {
    /** */
    hibe::extract(sk, delta)
}

//encrypt enciphers a message with a given public key.
pub fn encrypt(pk: Vec<u8>, msg: Vec<u8>) -> Vec<u8> {
    let mut public: KuPKEPublicKey = bincode::deserialize(&pk).unwrap();

    /** hibe encrypt*/
    //let (c1, c2) = hibe::Encrypt(public.pk, msg, public.i)
    let ciphertext =  KuPKECiphertext {
        c1,
        c2,
    };
    bincode::serialize(&ciphertext).unwrap()
}

//decrypt deciphers a ciphertext with a given secret key.
pub fn decrypt(sk: Vec<u8>, ct: Vec<u8>) -> Vec<u8> {
    let mut ciphertext = bincode::deserialize(&ct).unwrap();
    /** hibe decrypt*/
    //return k.hibe.Decrypt(sk, ciphertext.c1, ciphertext.c2)
}


