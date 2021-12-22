use std::io::Read;
use dv::{OnionSender, OnionReceiver,  OnionMessage, OnionCiphertext};
use dv::onion;
use std::str;
use rand::{OsRng, Rng};
use primitives;
use dv::signcryption;

#[test]
fn test_init() {
    let message = b"Hello, World!";
    let ad:[u8; 32] = [0; 32];

    let (s, r) = dv::onion::init();
    let sender: OnionSender = bincode::deserialize(&s).unwrap();


    let receiver : OnionReceiver =  bincode::deserialize(&r).unwrap();


    let sk_s = sender.sks;
    let pk_s = sender.pkr;

    let sk_r = receiver.skr;
    let pk_r = receiver.pks;

    let ct = dv::sc_encrypt(&sk_s, &pk_s, &ad, message);
    println!("first, sign the message with sign secret key and get the signature");
    println!("encrypt the signcryptionBlock with cipher public key, then get the ciphertext");
    //println!("ciphertext is: {:?}", ct);

    let pt = dv::sc_decrypt(&sk_r, &pk_r, &ad, &ct[..]).unwrap();
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

    let ad :[u8;32] = [0;32];
    println!("ad is {:?}", ad);
    let mut i = 0;
    let mut ad = ad.to_vec();
    loop {
        ad = primitives::Sha256::h_eval(&ad).to_vec();
        println!("ad is {:?}", ad);
        let _new_ad = ad.to_vec().as_slice();
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
    let (s1, r1) = dv::onion::init();
    let (s2, r2) = dv::onion::init();
    let (s3, r3) = dv::onion::init();
    let (s4, r4) = dv::onion::init();
    let mut s_state = vec![s1, s2, s3, s4];

    // define a message
    let message = Vec::from("HelloWorld");


    //ad
    let ad = dv::generate_iv();


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

    //encrypt the OnionMessage
    let mut c_lists = Vec::new();

    //add the encrypted onionMessage to the last position
    let cn = primitives::encrypt_aes_256_cbc(&pt, &k, &ad).expect("unable to encrypt the ciphertext! ");
    c_lists.insert(0, cn);

    //handle the previous state
    //-----------
    let mut j = 0;
    //let mut ad = ad.to_vec();
    //let mut ad = ad.to_vec();
    loop {
        //hash the ad

        //ad = primitives::Sha256::h_eval(&ad).to_vec();

        let ss = s_state.pop().unwrap();
        // 4- 3- 2-1


        //decode the sender state and get the key
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
    let ciphertext = bincode::serialize(&onion_ciphertext).expect("unable to encode ciphertext");

    let mut r_state = vec![r1, r2, r3, r4];
    //decode and get the onion Ciphertext

    let mut ciphertext_decrypt: OnionCiphertext = bincode::deserialize(&ciphertext).expect("unable to decode the ciphertext");

    let n = r_state.len();
    let mut k: [u8; 32] = [0; 32];

    //the last ciphertext is the Encrypted Onion Message.
    let last_ciphertext = ciphertext_decrypt.ct.pop().unwrap();
    let mut i = 0;
    //let mut ad = ad.to_vec();
    loop {
        let state = r_state.pop().unwrap();

        let receiver_state: OnionReceiver = bincode::deserialize(&state).expect("unable to decode onion receiver state");
      //  ad = primitives::Sha256::h_eval(&ad).to_vec();

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


    //let pt =  primitives::decrypt_aes_256_cbc(&last_ciphertext, &k, &iv).unwrap();
    let pt =  primitives::decrypt_aes_256_cbc(&last_ciphertext, &k, &ad).unwrap();

    //let pt = String::from_utf8(pt2).unwrap();

    let m : OnionMessage = bincode::deserialize(&pt).expect("unable to decode onion message");


    //let m_after_decrypt = m.msg;
    let msg2 = String::from_utf8(m.msg.to_vec()).unwrap();
    println!("message is :{}", msg2);
    let msg1 = String::from("HelloWorld");
    assert_eq!(msg1, msg2);

}
#[test]
fn test_encrypt() {
    //let message: [u8; 3136] = [0; 3136];
    //println!("message size is {}", message.len());
    let msg = "HelloWorld";
    let message = Vec::from(msg);
    let keypair = dv::derive_key_pair();
    let key = keypair.sk;
    let iv = dv::generate_iv();
    println!("iv is {:?}", iv);
    let ct = dv::encrypt_aes_256_cbc(&message, &key, &iv).ok().unwrap();


    println!("iv is {:?}", iv);
    let pt = dv::decrypt_aes_256_cbc(&ct[..], &key, &iv).ok().unwrap();
    let msg2 = String::from_utf8(pt).unwrap();
    let msg1 = String::from("HelloWorld");
    assert_eq!(msg2, msg1);
}


#[test]
fn test_uni_send_receive() {

    let (_s1, _r1) = dv::onion::init();
    let (_s2, _r2) = dv::onion::init();
    let (_s3, _r3) = dv::onion::init();
    let (_s4, _r4) = dv::onion::init();
    let mut _s_state = vec![_s1, _s2, _s3, _s4];
    let mut _r_state = vec![_r1, _r2, _r3, _r4];


    let ad = dv::generate_iv();


    let mut i = 0;
    loop {
        let message = Vec::from("2");
        let (mut _new_s_state, ct2) = onion::send(&mut _s_state, &ad, message);
        let (mut _new_r_state, pt2) = onion::receive(&mut _r_state, &ad, ct2);
        let msg2 = String::from_utf8(pt2.to_vec()).unwrap();
        assert_eq!(msg2, "2");
        println!("Alice send: 2");
        println!("Bob receive: {}", msg2);
        _s_state = _new_s_state;
        _r_state = _new_r_state;
        i+=1;
       if i >= 10 {break;}
    }
}
