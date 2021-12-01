use std::io::Read;
//use core::slice::SlicePattern;
//use core::slice::SlicePattern;
use DV::{OnionSender, OnionReceiver, init, OnionMessage, OnionCiphertext};
use DV::onion;
use std::str;
use rand::{OsRng, Rng};
use DV::hash;
use DV::otae;
use DV::signcryption;
/*
#[test]
fn test_init() {
    let message = b"Hello, World!";
    let ad:[u8; 32] = [0; 32];

    let (s, r) = DV::onion::init();
    //let send = s.to_vec();
    let s_str =str::from_utf8(s.as_bytes()).unwrap();
    let sender : OnionSender = serde_json::from_str(&s_str).unwrap();

    //let receive = r.to_vec();
    let r_str = str::from_utf8(r.as_bytes()).unwrap();
    let receiver : OnionReceiver = serde_json::from_str(&r_str).unwrap();

    let sk_s = sender.sks;
    let pk_s = sender.pkr;

    let sk_r = receiver.skr;
    let pk_r = receiver.pks;

    let ct = DV::sc_encrypt(&sk_s, pk_s, &ad, message);
    println!("first, sign the message with sign secret key and get the signature");
    println!("encrypt the signcryptionBlock with cipher public key, then get the ciphertext");
    //println!("ciphertext is: {:?}", ct);

    let pt = DV::sc_decrypt(sk_r, &pk_r, &ad, &ct[..]).unwrap();
    let plain_text =str::from_utf8(pt.as_slice()).unwrap();
    println!("first, decrypt the ciphertext with cipher secret key, and get the signcryptionBlock");
    println!("then build the struct, use signature and message to verify");
    println!("if verified, get the message");
    let msg = str::from_utf8(message).unwrap();
    assert_eq!(msg, plain_text);
    println!("original message is {}", msg);
    println!("verify!! and get the message: {}", plain_text);
}


#[test]
fn test_xor() {
    //DV::uni::send();
    //let mut v = Vec::new();
    //v.insert(0, "d");
    //v.insert(0, "c");
    //v.insert(0, "b");
    //v.insert(0, "a");
    //println!("v is {:?}", v);

    let mut ad :[u8;32] = [0;32];
    println!("ad is {:?}", ad);
    let mut i = 0;
    let mut ad = ad.to_vec();
    loop {
        ad = hash::Sha256::h_eval(&ad).to_vec();
        println!("ad is {:?}", ad);
        let new_ad = ad.to_vec().as_slice();
        i+=1;
        if i == 5 {
            break;
        }
    }
}



#[test]
fn send_and_receive() {
    //Sender
    //create a s_state
    let (s1, r1) = DV::onion::init();
    let (s2, r2) = DV::onion::init();
    let (s3, r3) = DV::onion::init();
    let (s4, r4) = DV::onion::init();
    let mut s_state = vec![s1, s2, s3, s4];

    // define a message
    let message = Vec::from("HelloWorld");


    //ad
    let mut ad = DV::generate_iv();
    let mut iv= DV::generate_iv();


    //----------Send process---------
    // generate a new state pair.
    let(s5, r5) = DV ::onion::init();

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
        s: r5,
        msg: message,
    };

    //encode the onionMessage
    let pt = serde_json::to_string(&om).expect("unable to encode onion message");

    //encrypt the OnionMessage
    let mut c_lists = Vec::new();

    //add the encrypted onionMessage to the last position
    let cn = otae::encrypt_aes_256_cbc(pt.as_bytes(), &k, &iv).expect("unable to encrypt the ciphertext! ");
    c_lists.insert(0, cn);

    //handle the previous state
    let mut j = 0;
    let mut ad = ad.to_vec();
    loop {
        //hash the ad
        ad = hash::Sha256::h_eval(&ad).to_vec();
        let ss = s_state.pop().unwrap();
        // 4- 3- 2-1


        //decode the sender state and get the key
        let st : OnionSender = serde_json::from_str(&ss).expect("unable to decode onion sender");
        //use sks and pkr to SC.enc ad and each k.
        let encrypt_k = signcryption::sc_encrypt(&st.sks, st.pkr, &ad, ks.pop().unwrap().as_slice());
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
    let ciphertext = serde_json::to_string(&onion_ciphertext).expect("unable to encode ciphertext");


    let mut r_state = vec![r1, r2, r3, r4];
    //decode and get the onion Ciphertext
    let mut ciphertext_decrypt : OnionCiphertext = serde_json::from_str(&ciphertext).expect("unable to decode the ciphertext");

    let n = r_state.len();
    let mut k: [u8; 32] = [0; 32];

    //the last ciphertext is the Encrypted Onion Message.
    let last_ciphertext = ciphertext_decrypt.ct.pop().unwrap();
    let mut i = 0;
    let mut ad = ad.to_vec();
    loop {
        let state = r_state.pop().unwrap();
        let receiver_state: OnionReceiver = serde_json::from_str(&state).expect("unable to decode onion receiver state");
        ad = hash::Sha256::h_eval(&ad).to_vec();

        //SC.dec each ciphertext and get the previous key.
        let tmp = signcryption::sc_decrypt(receiver_state.skr, &receiver_state.pks, &ad, ciphertext_decrypt.ct.pop().unwrap().as_slice()).expect("unable to decrypt onion ciphertext");

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
    let pt2 =  otae::decrypt_aes_256_cbc(&last_ciphertext, &k, &iv).unwrap();

    let pt = String::from_utf8(pt2).unwrap();
    let m : OnionMessage = serde_json::from_str(&pt).expect("unable to decode onion message");
    //let m_after_decrypt = m.msg;
    let msg2 = String::from_utf8(m.msg).unwrap();
    println!("message is :{}", msg2);
    let msg1 = String::from("HelloWorld");
    assert_eq!(msg1, msg2);

}
*/
/* have changed
#[test]
fn test_aes_encrypt() {
    let msg = "HelloWorld";
    let message2 = Vec::from(msg);
    let mut message = Vec::new();

    let ct = otae::encrypt_aes_256_cbc(&message2, &k, &ad).unwrap();
    message.insert(0, ct);
    message.insert(0, Vec::from("4"));
    message.insert(0, Vec::from("3"));
    message.insert(0, Vec::from("2"));
    message.insert(0, Vec::from("1"));
    let mut ad:[u8;16]=[0;16];
    let mut rng=OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut ad);
    let ad = ad.to_vec();
    let mut k :[u8; 32] = [0;32];

    let mut i = 0;
    loop {
        let mut tmp :[u8; 32] = [0;32];
        let mut rng = OsRng::new().ok().unwrap();
        rng.fill_bytes(&mut tmp);
        for j in 0..32 {
            k[j] = k[j] ^ tmp[j];
        }
        i += 1;
        if i >= 4 {
            break;
        }
    }
    //let ct = otae::encrypt_aes_256_cbc(&message, &k, &ad).unwrap();
    let ct = message.pop().unwrap();
    let pt = otae::decrypt_aes_256_cbc(&ct, &k, &ad).unwrap();
    let pt_str = String::from_utf8(pt).unwrap();

    assert_eq!(pt_str, "HelloWorld");

}*/
/*
#[test]
fn test_encrypt() {
    //let message: [u8; 3136] = [0; 3136];
    //println!("message size is {}", message.len());
    let msg = "HelloWorld";
    let message = Vec::from(msg);
    let keypair = DV::derive_key_pair();
    let mut key = keypair.sk;
    let mut iv = DV::generate_iv();
    println!("iv is {:?}", iv);
    let ct = DV::encrypt_aes_256_cbc(&message, &key, &iv).ok().unwrap();


    println!("iv is {:?}", iv);
    let pt = DV::decrypt_aes_256_cbc(&ct[..], &key, &iv).ok().unwrap();
    let msg2 = String::from_utf8(pt).unwrap();
    let msg1 = String::from("HelloWorld");
    assert_eq!(msg2, msg1);
}
*/
/*
#[test]
fn test_uni_send_receive() {

    let (s1, r1) = DV::onion::init();
    let (s2, r2) = DV::onion::init();
    let (s3, r3) = DV::onion::init();
    let (s4, r4) = DV::onion::init();
    let mut s_state = vec![s1, s2, s3, s4];
    let mut r_state = vec![r1, r2, r3, r4];
    let mut ad = DV::generate_iv();
    let mut iv = DV::generate_iv();


    let message = Vec::from("1");
    let (s_state, ct) = onion::send(s_state, &ad, Vec::from(message), &iv);
    let (r_state, pt) = onion::receive(r_state, &ad, ct, &iv);
    let msg = String::from_utf8(pt).unwrap();
    assert_eq!(msg, "1");
    println!("Alice send: 1");
    println!("Bob receive: {}", msg);


    let message = Vec::from("2");
    let (s_state, ct2) = onion::send(s_state, &ad, Vec::from(message), &iv);
    let (r_state, pt2) = onion::receive(r_state, &ad, ct2, &iv);
    let msg2 = String::from_utf8(pt2).unwrap();
    assert_eq!(msg2, "2");
    println!("Alice send: 2");
    println!("Bob receive: {}", msg2);

    let message = Vec::from("3");
    let (s_state, ct2) = onion::send(s_state, &ad, Vec::from(message), &iv);
    let (r_state, pt2) = onion::receive(r_state, &ad, ct2, &iv);
    let msg3 = String::from_utf8(pt2).unwrap();
    assert_eq!(msg3, "3");
    println!("Alice send: 3");
    println!("Bob receive: {}", msg3);

    let message = Vec::from("4");
    let (s_state, ct2) = onion::send(s_state, &ad, Vec::from(message), &iv);
    let (r_state, pt2) = onion::receive(r_state, &ad, ct2, &iv);
    let msg4 = String::from_utf8(pt2).unwrap();
    assert_eq!(msg4, "4");
    println!("Alice send: 4");
    println!("Bob receive: {}", msg4);

    let message = Vec::from("5");
    let (s_state, ct2) = onion::send(s_state, &ad, Vec::from(message), &iv);
    let (r_state, pt2) = onion::receive(r_state, &ad, ct2, &iv);
    let msg5 = String::from_utf8(pt2).unwrap();
    assert_eq!(msg5, "5");
    println!("Alice send: 5");
    println!("Bob receive: {}", msg5);

    let message = Vec::from("6");
    let (s_state, ct2) = onion::send(s_state, &ad, Vec::from(message), &iv);
    let (r_state, pt2) = onion::receive(r_state, &ad, ct2, &iv);
    let msg6 = String::from_utf8(pt2).unwrap();
    assert_eq!(msg6, "6");
    println!("Alice send: 6");
    println!("Bob receive: {}", msg6);


}*/
/*

pub fn test(mut a: Vec<u8>) -> (Vec<u8>, i32) {
    let mut b  = Vec::new();
    let mut c = 0;
    loop {
        if a.len() == 0 { break; }
        let d = a.pop();
        c = c+d;
        b.push(d.unwrap());
    }
    (b, c)
}

#[test]
fn borrow_test() {
    let mut a = vec![1,2,3,4,5];
    let mut i = 0;
    let c = 0;
    loop {
        (a, c) = test(a);
        println!("{}",c);
        if i >=5 {break;}
    }
}

*/

#[test]
fn test_read_file () {

    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut message = String::new();
    file.read_to_string(&mut message).unwrap();


    let (s1, r1) = DV::onion::init();
    let (s2, r2) = DV::onion::init();
    let (s3, r3) = DV::onion::init();
    let (s4, r4) = DV::onion::init();
    let mut s_state = vec![s1, s2, s3, s4];
    let mut r_state = vec![r1, r2, r3, r4];
    let mut ad = DV::generate_iv();
    let mut iv = DV::generate_iv();


    let (s_state, ct) = onion::send(s_state, &ad, Vec::from(message), &iv);
    let (r_state, pt) = onion::receive(r_state, &ad, ct, &iv);
    let msg = String::from_utf8(pt).unwrap();
    assert_eq!(msg, "This is a text file");
    println!("Alice send: This is a text file");
    println!("Bob receive: {}", msg);
}