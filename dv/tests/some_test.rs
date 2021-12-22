use std::io::Read;
use dv::{OnionSender, OnionReceiver,  OnionMessage, OnionCiphertext};
use dv::onion;
use std::str;
use rand::{OsRng, Rng};
use primitives;
use dv::signcryption;

#[test]
pub fn test(){
//Sender
//create a s_state
let (s1, r1) = dv::onion::init();
let (s2, r2) = dv::onion::init();
let (s3, r3) = dv::onion::init();
let (s4, r4) = dv::onion::init();
let mut s_state = vec![s1, s2, s3, s4];

// define a message
let message = Vec::from("HelloWorld");


//ad
let ad = dv::generate_iv();
let iv= dv::generate_iv();


//----------Send process---------
// generate a new state pair.
let(_s5, _r5) = dv ::onion::init();

//initial key and ks
let mut k :[u8; 32] = [0; 32];
let mut ks : Vec<Vec<u8>> = Vec::new();
let n:usize = s_state.len();
let mut i = 0;


// k = k1 xor k2 xor k3 ...
// append k1 k2 k3... to ks.
loop {
let mut tmp :[u8; 32] = [0;32];
let mut rng = OsRng::new().ok().unwrap();
rng.fill_bytes(&mut tmp);
for j in 0..32 {
k[j] = k[j] ^ tmp[j];
}

ks.append(&mut vec![tmp.to_vec()]);
i += 1;
if i >= n {
break;
}
}


//define a onionMessage (receive state + message)
let om = OnionMessage{
s: _r5,
msg: message,
};

//encode the onionMessage
let pt = bincode::serialize(&om).expect("unable to encode onion message");
//let pt = serde_json::to_string(&om).expect("unable to encode onion message");

//encrypt the OnionMessage
let mut c_lists = Vec::new();

//add the encrypted onionMessage to the last position
let cn = primitives::encrypt_aes_256_cbc(&pt, &k, &iv).expect("unable to encrypt the ciphertext! ");
c_lists.insert(0, cn);

//handle the previous state
let mut j = 0;
let mut ad = ad.to_vec();
loop {
//hash the ad
ad = primitives::Sha256::h_eval(&ad).to_vec();
let ss = s_state.pop().unwrap();
// 4- 3- 2-1


//decode the sender state and get the key
//let st : OnionSender = serde_json::from_str(&ss).expect("unable to decode onion sender");
let st : OnionSender = bincode::deserialize(&ss).expect("unable to decode onion sender");


//use sks and pkr to SC.enc ad and each k.
let encrypt_k = signcryption::sc_encrypt(&st.sks, &st.pkr, &ad, ks.pop().unwrap().as_slice());
//insert to c_lists
c_lists.insert(0, encrypt_k);
j+=1;
if j>= n {
break;
}
}

//encode the onion Ciphertext
let onion_ciphertext = OnionCiphertext{
ct: c_lists
};
//let ciphertext = serde_json::to_string(&onion_ciphertext).expect("unable to encode ciphertext");
let ciphertext = bincode::serialize(&onion_ciphertext).expect("unable to encode ciphertext");

let mut r_state = vec![r1, r2, r3, r4];
//decode and get the onion Ciphertext

//let mut ciphertext_decrypt : OnionCiphertext = serde_json::from_str(&ciphertext).expect("unable to decode the ciphertext");
let mut ciphertext_decrypt: OnionCiphertext = bincode::deserialize(&ciphertext).expect("unable to decode the ciphertext");

let n = r_state.len();
let mut k: [u8; 32] = [0; 32];

//the last ciphertext is the Encrypted Onion Message.
let last_ciphertext = ciphertext_decrypt.ct.pop().unwrap();
let mut i = 0;
let mut ad = ad.to_vec();
loop {
let state = r_state.pop().unwrap();

let receiver_state: OnionReceiver = bincode::deserialize(&state).expect("unable to decode onion receiver state");
//let receiver_state: OnionReceiver = serde_json::from_str(&state).expect("unable to decode onion receiver state");
ad = primitives::Sha256::h_eval(&ad).to_vec();

//SC.dec each ciphertext and get the previous key.


let tmp = signcryption::sc_decrypt(&receiver_state.skr, &receiver_state.pks, &ad, ciphertext_decrypt.ct.pop().unwrap().as_slice()).expect("unable to decrypt onion ciphertext");

//get the symmetric k
for j in 0..32 {
k[j] = k[j] ^ tmp[j];
}

i+=1;
if i>=n {
break;
}
}
//decrypt and get the onion Message.
let pt =  primitives::decrypt_aes_256_cbc(&last_ciphertext, &k, &iv).unwrap();

//let pt = String::from_utf8(pt2).unwrap();

let m : OnionMessage = bincode::deserialize(&pt).expect("unable to decode onion message");


//let m : OnionMessage = serde_json::from_str(&pt).expect("unable to decode onion message");
//let m_after_decrypt = m.msg;
let msg2 = String::from_utf8(m.msg).unwrap();
println!("message is :{}", msg2);
let msg1 = String::from("HelloWorld");
assert_eq!(msg1, msg2);
}