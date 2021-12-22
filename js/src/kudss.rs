

use rand::{Rng,OsRng};
use serde::{Serialize, Deserialize};
use bincode;
use crate::big_array::BigArray;
use crate::dss;
use crate::fsdss;


#[derive(Serialize, Deserialize)]
pub struct KuDssPublicKey {
    pub vk : Vec<u8>, //vk is the fs-DSS public key
    pub delta: Vec<Vec<u8>>, // delta is an array of associated data
    pub i : i8, //i is the current period of a live kuDSS instance
}
#[derive(Serialize, Deserialize)]
pub struct KuDssPrivateKey {
    #[serde(with = "BigArray")]
    pub sk : Vec<u8>,
    pub sigma : Vec<Vec<u8>>,
    pub i : i8,
}
#[derive(Serialize, Deserialize)]
pub struct KuDssSignature {
    #[serde(with = "BigArray")]
    pub signature : Vec<u8>,
    pub sigma : Vec<Vec<u8>>,
    pub i : i8,
}


// func
// generate crates a fresh public/private key pair
pub fn generate() -> (Vec<u8>, Vec<u8>){

    let (fsk, fpk) = fsdss::generate();

    let ku_dss_public_key = KuDssPublicKey {
        vk : fpk,
        delta : vec![vec![]],
        i : 0,
    };
    let ku_dss_private_key = KuDssPrivateKey {
        sk : fsk,
        sigma : vec![vec![]],
        i : 0
    };
    let pk = bincode::serialize(&ku_dss_public_key).unwrap();
    let sk = bincode::serialize(&ku_dss_private_key).unwrap();
    (pk, sk)
}



// update_public_key evolves the public key into the next protocol period
pub fn update_public_key(pk: Vec<u8>, d: Vec<u8>) -> Vec<u8>{
    let mut ku_dss_public_key: KuDssPublicKey = bincode::deserialize(&pk).unwrap();
    ku_dss_public_key.delta.push(d);
    ku_dss_public_key.i += 1;
    let new_public = bincode::serialize(&ku_dss_public_key).unwrap();
    new_public
}



// updatePrivateKey evolves the private key into the next protocol period
pub fn update_private_key(sk: Vec<u8>, mut delta: Vec<u8>) -> Vec<u8> {
    let mut ku_dss_private_key :KuDssPrivateKey = bincode::deserialize(&sk).unwrap();
    let mut msg: Vec<u8> = vec![0];
    msg.append(&mut delta);

    let s = fsdss::sign(ku_dss_private_key.sk.clone(), msg);
    //s
    ku_dss_private_key.sigma.push(s.to_vec());

    //update sk? based on forward DSS
    let udp = fsdss::update(ku_dss_private_key.sk.clone());
    ku_dss_private_key.sk = udp;
    ku_dss_private_key.i += 1;
    let new_private = bincode::serialize(&ku_dss_private_key).unwrap();
    new_private
}



//sign creates a signature of a message with a given private key
pub fn sign(sk: Vec<u8>, mut msg: Vec<u8>) -> Vec<u8>{
    let mut ku_dss_private_key:KuDssPrivateKey = bincode::deserialize(&sk).unwrap();

    //sig, err := k.signature.Sign(private.SK, append([]byte{1}, msg...))
    let mut message: Vec<u8> = vec![1];
    message.append(&mut msg);


    let sig = fsdss::sign(message.clone(), ku_dss_private_key.sk.clone());

    let ku_dss_signature = KuDssSignature {
        signature: sig,
        sigma: ku_dss_private_key.sigma,
        i : ku_dss_private_key.i
    };
    bincode::serialize(&ku_dss_signature).unwrap()
}

//verify checks the validity f a signature.
pub fn verify(pk: Vec<u8>, mut msg: Vec<u8>, sig: Vec<u8>) {
    let ku_dss_public_key : KuDssPublicKey = bincode::deserialize(&pk).unwrap();
    let ku_dss_signature : KuDssSignature = bincode::deserialize(&sig).unwrap();
    if ku_dss_signature.i != ku_dss_public_key.i {
    }

    /*
    plaintext := append([]byte{1}, msg...)
	if err := k.signature.Verify(public.VK, plaintext, signature.Signature); err != nil {
		return err
	}
    */
    let mut plaintext: Vec<u8> = vec![1];
    plaintext.append(&mut msg);

    if fsdss::verify(plaintext, ku_dss_public_key.vk, ku_dss_signature.signature) {
        Some(block.message)
    } else {
        None
        //return;
    }

    //let d_clone = ku_dss_public_key.delta;
    for i in ku_dss_public_key.i-1 {
        let mut delta = vec![0];
        //let a= d_clone.get(i).unwrap();
        delta.append(ku_dss_public_key.delta.get(i));
        let sigma = ku_dss_signature.signature.get(i);
        if dss::verify(&delta, &ku_dss_public_key.vk, &sigma) {
            Some(block.message)
        } else {
            None
        }
    }


}

