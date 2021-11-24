use DV;

#[test]
fn test_generate_key(){


}

#[test]
fn test_sign_and_verify(){
    //let (private_key, public_key) = DV::keypair(seed); //[U8,64]
    let (private_key, public_key) = DV::generate_sign_key();
    let message = b"This is test message!";

    //
    let sig = DV::signature(message, &private_key); //[U8,64]

    //private_key
    println!("private_key: {:?} ", private_key.to_vec());
    println!("private_key_len :{:? }", private_key.len());

    // public_key
    println!("public_key :{:?}", public_key.to_vec());
    println!("public_key_len :{:?}", public_key.len());

    //signature
    println!("signature:{:?}", sig.to_vec());
    println!("signature_len:{:?}", sig.len());

    // verify
    //println!("is successful?：{:?} ",DV::verify(message, &public_key, &sig));
    assert_eq!(DV::verify(message, &public_key, &sig), true);
}