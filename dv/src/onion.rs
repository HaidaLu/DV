use crate::signcryption;
use serde::{Serialize, Deserialize};
use rand::{Rng, OsRng};


//OnionSender is the onion sender state.
#[derive(Serialize, Deserialize, Debug)]
pub struct OnionSender {
    //sks, pkr
    pub sks: Vec<u8>,
    pub pkr: Vec<u8>,
}

//OnionReceiver is the onion receiver state.
#[derive(Serialize, Deserialize, Debug)]
pub struct OnionReceiver {
    // skr, pks
    pub skr : Vec<u8>,
    pub pks : Vec<u8>,
}

//OnionMessage bundles the plaintext material.
#[derive(Serialize, Deserialize, Debug)]
pub struct OnionMessage {

    pub s : Vec<u8>,// S designates the new receiver state.
    pub msg : Vec<u8>,// Msg is the plaintext.
    //pub msg : &'a [u8]
}

//OnionCiphertext bundles the onion ciphertext array.
#[derive(Serialize, Deserialize, Debug)]
pub struct OnionCiphertext {
    pub ct : Vec<Vec<u8>>,
}
/*
#[derive(Serialize, Deserialize, Debug)]
pub struct SenderState{
    pub ss : Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiverState{
    pub rs : Vec<Vec<u8>>,
}

*/
pub fn init() -> (Vec<u8>, Vec<u8>) {
    let (pk_r, sk_s) = signcryption::generate_sign_key();
    let (pk_s, sk_r) = signcryption::generate_cipher_key();
    let sender = OnionSender {
        sks: sk_s.to_vec(),
        pkr: pk_s,
    };
    let s = bincode::serialize(&sender).expect("unable to encode onion sender");


    let receiver = OnionReceiver {
        skr: sk_r,
        pks: pk_r.to_vec(),
    };
    let r = bincode::serialize(&receiver).expect("unable to encode onion receiver");

    (s, r)
}

//send implements the onion send procedure.

pub fn send(s : &mut Vec<Vec<u8>>, ad: &[u8], msg: Vec<u8>) -> (Vec<Vec<u8>>, Vec<u8>) {

    let mut new_s =  Vec::new();

    //generate a new state pair
    let (_us, ur) = init();

    // one-time symmetric
    // initial a key and a ks to keep record of previous keys
    let mut k :[u8; 32] = [0; 32];
    let mut ks : Vec<Vec<u8>> = Vec::new();
    let n = s.len();
    let mut i = 0;

    // k = k1 xor k2 xor k3 ...
    // append k1 k2 k3... to ks.
    loop {
        let mut tmp : [u8; 32] = [0;32];
        let mut rng = OsRng::new().ok().unwrap();
        rng.fill_bytes(&mut tmp);
        for j in 0..32 {
            k[j] = k[j] ^ tmp[j];
        }
        //ks.append(& mut tmp.to_vec());
        ks.append(& mut vec![tmp.to_vec()]);
        //ks.append(& mut Vec::from(tmp));
        i += 1;
        if i >= n {
            break;
        }
    }

    //define a onionMessage (receive state + message)
    let om = OnionMessage{
        s: ur,
        msg,
    };

    //encode-encrypt the onionMessage and add the encrypted onionMessage to the last position
    let pt = bincode::serialize(&om).expect("unable to encode onion message");

    let mut c = Vec::new();
    let cn = primitives::encrypt_aes_256_cbc(pt.as_slice(), &k, &ad).expect("unable to encrypt the plaintext!");
    c.insert(0, cn);

    //handle the previous state
    let mut j = 0;
    //let mut ad = ad.to_vec();
    loop {
        //hash the ad
      //  ad = primitives::Sha256::h_eval(&ad).to_vec();
        let ss = s.pop().unwrap();
        //---------------------new_s----------------------
        new_s.insert(0, ss);
        //decode the sender state and get the key

        let st : OnionSender = bincode::deserialize((&new_s.get(0).unwrap())).expect("unable to decode onion sender");

        let cn = signcryption::sc_encrypt(&st.sks,&st.pkr, &ad, ks.pop().unwrap().as_slice());
        c.insert(0, cn);
        j += 1;
        if j >= n {
            break;
        }
    }
    //encode the onion Ciphertext
    let onion_ciphertext = OnionCiphertext{
        ct: c
    };
    let ct = bincode::serialize(&onion_ciphertext).expect("unable to encode ciphertext");
    //new_s.push(us);
    (new_s, ct)
}


//receive invokes the onion receive routine.
pub fn receive(s : & mut Vec<Vec<u8>>, ad: &[u8], ct: Vec<u8>) -> (Vec<Vec<u8>>, Vec<u8>) {

    let mut new_r = Vec::new();
    //let mut s = rs.rs;
    //decode and get the onion Ciphertext



    let mut oc : OnionCiphertext = bincode::deserialize(&ct).expect("unable to decode onion ciphertext");

    let n = s.len();
    let mut k :[u8; 32] = [0; 32];
    let mut i = 0;
    //let mut ad = ad.to_vec();

    //the last ciphertext is the Encrypted Onion Message.
    let last_ciphertext = oc.ct.pop().unwrap();
    loop {
        let state = s.pop().unwrap();
        new_r.insert(0, state);

        //------------------
        let receiver_state: OnionReceiver = bincode::deserialize(&new_r.get(0).unwrap()).expect("unable to decode onion receiver state");
       // ad = primitives::Sha256::h_eval(&ad).to_vec();

        //SC.dec each ciphertext and get the previous key.
        let tmp = signcryption::sc_decrypt(&receiver_state.skr,&receiver_state.pks, &ad, oc.ct.pop().unwrap().as_slice()).expect("unable to decrypt onion ciphertext");
        //get the symmetric k
        for j in 0..32 {
            k[j] = k[j] ^ tmp[j];
        }

        i += 1;
        if i >= n {
            break;
        }
    }
    //decrypt and get the onion Message.
    let pt = primitives::decrypt_aes_256_cbc(&last_ciphertext, &k, &ad).expect("unable to encrypt the plaintext!");

    let m : OnionMessage = bincode::deserialize(&pt).expect("unable to decode onion message");

    //let msg: &'a [u8] = m.msg;
    (new_r, m.msg)// 如果返回的是引用 m.msg是一个引用，那么 pt在函数结束后就removed了，m.msg也没了
}

