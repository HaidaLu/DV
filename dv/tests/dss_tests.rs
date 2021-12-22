use dv;
use dv::dss;


#[test]
fn test_sign_then_verify() {
    let seed: &[u8] = &[0x26, 0x27, 0xf6, 0x85, 0x97, 0x15, 0xad, 0x1d, 0xd2, 0x94, 0xdd, 0xc4, 0x76, 0x19, 0x39, 0x31, 0xf1, 0xad, 0xb5, 0x58, 0xf0, 0x93, 0x97, 0x32, 0x19, 0x2b, 0xd1, 0xc0, 0xfd, 0x16, 0x8e, 0x4e];//32位
    let (private_key, public_key) = dss::generate(seed);

    let message = b"Hello, World!";

    let sig = dss::signature(message, &private_key);

    //private_key
    println!("private_key: {:?} ", private_key.to_vec());
    println!("private_key_len :{:? }", private_key.len());

    // public_key
    println!("public_key :{:?}", public_key.to_vec());
    println!("public_key_len :{:?}", public_key.len());

    //signature
    println!("signature:{:?}", sig.to_vec());
    println!("signature_len:{:?}", sig.len());


    assert_eq!(true, dss::verify(message, &public_key, &sig));

}

