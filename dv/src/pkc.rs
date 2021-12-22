use openssl::rsa::{Rsa, Padding};


//a public-key cryptosystem PKC

// RSA-OAEP
pub fn generate() -> (Vec<u8>, Vec<u8>) {
//pub fn generate() -> ([u8;128], [u8;128]) {

    let rsa = Rsa::generate(1024).unwrap();
    //let private_key = rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), passphrase.as_bytes()).unwrap().as_slice();
    let private_key = rsa.private_key_to_pem().unwrap();
    //let v = rsa.private_key_to_pem().unwrap();
    //let private_key:[u8;128] = v.try_into().unwrap_or_else(|v: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 128, v.len()));


    let public_key = rsa.public_key_to_pem().unwrap();
    //let u = rsa.private_key_to_pem().unwrap();
    //let public_key:[u8;128] = u.try_into().unwrap_or_else(|u: Vec<u8>| panic!("Expected a Vec of length {} but it was {}", 128, u.len()));


    //let private_keya =  String::from_utf8(private_key).unwrap().as_bytes();
    //println!("Public key: {}", String::from_utf8(public_key).unwrap());
    (public_key,private_key)
}

pub fn encrypt(pk: Vec<u8>, msg: &[u8], _ad: &[u8]) -> Vec<u8> {
    // Encrypt with public key

    let rsa = Rsa::public_key_from_pem(String::from_utf8(pk).unwrap().as_bytes()).unwrap();

    let mut i = 0;
    let length = 86;
    let mut encrypt_data = Vec::new();

    loop {
        let mut size = msg.len() - i;
        if size > length {
            size = length;
        }
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        let _ = rsa.public_encrypt(&msg[i..i+size], &mut buf, Padding::PKCS1_OAEP).unwrap();
        encrypt_data.append(&mut buf);
        i += length;
        if i>= msg.len() {
            break;
        }
    }
    //let _ = rsa.public_encrypt(msg, &mut buf, Padding::PKCS1_OAEP).unwrap();
    //panics if to is smaller than self.size()
    //println!("Encrypted: {:?}", ct);
    encrypt_data
    //encrypt_data
}

pub fn decrypt(sk: Vec<u8>, ct: &[u8], _ad: &[u8]) -> Vec<u8> {
    let mut decrypt_data = Vec::new();
    let mut i = 0;
    let length = 128;
    let rsa = Rsa::private_key_from_pem(String::from_utf8(sk).unwrap().as_bytes()).unwrap();
    loop{
        let mut buf: Vec<u8> = vec![0; rsa.size() as usize];
        let bytes = rsa.private_decrypt(&ct[i..i+length], &mut buf, Padding::PKCS1_OAEP).unwrap();
        //let pt = String::from_utf8(buf[0..bytes].to_vec()).unwrap();
        let mut enc = buf[0..bytes].to_vec();
        decrypt_data.append(&mut enc);
        i += length;
        if i >= ct.len() {
            break;
        }
        //pt.into_bytes()
    }
    decrypt_data
}
