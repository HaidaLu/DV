use std::io::{BufRead, Read};
use rand::OsRng;
use EtH;
use std::{io, str};
use std::fs::File;
use std::path::Path;
use rand::Rng;





/*
#[test]
fn initall_test_alice_sender_key_equals_bob_receive_key() {
    let (st_a, st_b) = EtH::initall();
    let key1 = st_a.sk;
    let key2 = st_a.rk;
    let key3 = st_b.sk;
    let key4 = st_b.rk;
    assert_eq!(key1, key4);
    assert_eq!(key2, key3);
}

#[test]
fn test_send() {
    let (st_a, st_b) = EtH::initall();
    let message="Hello World!";
    let mut iv:[u8;16]=[0;16];
    let mut rng=OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut iv);
    let (new_st_a, ct) = EtH::send(st_a, &iv, message.as_bytes());
    assert_ne!(ct.as_slice(), message.as_bytes());
}

#[test]
fn test_send_and_receive(){
    let (st_a, st_b) = EtH::initall();
    let message="Hello World!";
    let mut iv:[u8;16]=[0;16];
    let mut rng=OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut iv);
    let (new_st_a, ct) = EtH::send(st_a, &iv, message.as_bytes());
    let(new_st_b, pt) = EtH::receive(st_b, &iv, ct.as_slice());
    let crypt_message=str::from_utf8(pt.as_slice()).unwrap();

    assert_eq!(message,crypt_message);
    println!("Alice send: {}", message);
    println!("Bob receive: {}", crypt_message);
    println!("----------")

}

#[test]
fn send_two_messages(){
    let (st_a, st_b) = EtH::initall();
    let message="Hello bob!";
    let message2 = "How are you";
    let mut iv:[u8;16]=[0;16];
    let mut rng=OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut iv);
    let (new_st_a, ct) = EtH::send(st_a, &iv, message.as_bytes());
    let (sec_st_a, ct2) = EtH::send(new_st_a, &iv, message2.as_bytes());
    let(new_st_b, pt) = EtH::receive(st_b, &iv, ct.as_slice());
    let(sec_st_a, pt2) = EtH::receive(new_st_b, &iv, ct2.as_slice());
    let crypt_message=str::from_utf8(pt.as_slice()).unwrap();
    let crypt_message2 = str::from_utf8(pt2.as_slice()).unwrap();


    assert_eq!(message,crypt_message);
    println!("Alice send: {}", message);
    println!("Bob receive: {}", crypt_message);
    assert_eq!(message2,crypt_message2);
    println!("Alice send: {}", message2);
    println!("Bob receive: {}", crypt_message2);
    println!("----------")
}

#[test]
fn two_sending(){
    let (st_a, st_b) = EtH::initall();
    let message="Hello bob!";
    let message2 = "Hello Alice";
    let mut iv:[u8;16]=[0;16];
    let mut rng=OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut iv);
    let (new_st_a, ct) = EtH::send(st_a, &iv, message.as_bytes());
    let(new_st_b, pt) = EtH::receive(st_b, &iv, ct.as_slice());

    let (sec_st_a, ct2) = EtH::send(new_st_b, &iv, message2.as_bytes());
    let(sec_st_a, pt2) = EtH::receive(new_st_a, &iv, ct2.as_slice());

    let crypt_message=str::from_utf8(pt.as_slice()).unwrap();
    let crypt_message2 = str::from_utf8(pt2.as_slice()).unwrap();


    assert_eq!(message,crypt_message);
    println!("Alice send: {}", message);
    println!("Bob receive: {}", crypt_message);
    assert_eq!(message2,crypt_message2);
    println!("Bob send: {}", message2);
    println!("Alice receive: {}", crypt_message2);

    println!("----------")
}

*/
//sent hundreds of messages
#[test]
fn send_hundred_same_messages() {
    let (mut st_a, mut st_b) = EtH::initall();
    let message = "test";
    let iv = EtH::generate_iv();
    let pt = message.as_bytes();
    let mut i = 0;

    loop {
        let (_new_st_a, ct) = EtH::send(&mut st_a, &iv, pt);
        let (_new_st_b, pt) = EtH::receive(&mut st_b, &iv, ct.as_slice());
        let crypt_message = str::from_utf8(pt.as_slice()).unwrap();
        assert_eq!(message, crypt_message);
        println!("{} :", i);
        println!("Alice send: {}", message);
        println!("Bob receive: {}", crypt_message);
        i+=1;
        st_a = _new_st_a;
        st_b = _new_st_b;
        if i>=100 { break; }
    }
}




#[test]
fn send_hundred_random_messages() {
    let (mut st_a,mut st_b) = EtH::initall();

    let iv= EtH::generate_iv();


    let mut i = 0;

    loop {
        let original_message = EtH::generate_random_string();
        let (mut _new_st_a, ct) = EtH::send(&mut st_a, &iv, original_message.as_bytes());
        let (mut _new_st_b, pt) = EtH::receive(&mut st_b, &iv, ct.as_slice());
        let crypt_message = str::from_utf8(pt.as_slice()).unwrap();
        assert_eq!(original_message, crypt_message);
        println!("{} :", i);
        println!("Alice send: {}", original_message);
        println!("Bob receive: {}", crypt_message);
        st_a = _new_st_a;
        st_b = _new_st_b;

        i+=1;
        if i>=50 { break; }
    }
}

#[test]
fn test_read_file_send_and_receive(){
    let mut file = std::fs::File::open("data2.txt").unwrap();
    let mut message = String::new();
    file.read_to_string(&mut message).unwrap();


    let (mut st_a, mut st_b) = EtH::initall();
    let iv  = EtH::generate_iv();


    let (_st_a, ct) = EtH::send(&mut st_a, &iv, message.as_bytes());
    let (_st_b, pt) = EtH::receive(&mut st_b, &iv, ct.as_slice());
    let crypt_message=str::from_utf8(pt.as_slice()).unwrap();

    assert_eq!(message,crypt_message);
    println!("Alice send from file: {}", message);
    println!("Bob receive: {}", crypt_message);
    println!("----------")

}



#[test]
pub fn test_read_line() {
    if let Ok(lines) = read_lines("data.txt") {
        // read file
        let mut file = std::fs::File::open("data.txt").unwrap();
        let mut message = String::new();
        file.read_to_string(&mut message).unwrap();

        //create alice_send_bob_receive ratchet
        //create bob_send_alice_receive ratchet
        let (mut as_st_a, mut br_st_b) = EtH::initall();
        let (mut bs_st_a, mut ar_st_b) = EtH::initall();

        let iv:[u8;16]= EtH::generate_iv();
        let iv2 = EtH::generate_iv();

        let mut i = 0;

        for line in lines {
            if let Ok(message) = line {
                if i % 2 == 0 {
                    let (mut _as_st_a, ct) = EtH::send(&mut as_st_a, &iv, message.as_bytes());
                    let(mut _br_st_b, pt) = EtH::receive(&mut br_st_b, &iv, ct.as_slice());
                    let crypt_message=str::from_utf8(pt.as_slice()).unwrap();
                    println!("Alice send: {}", message);
                    println!("Bob receive: {}", crypt_message);
                    as_st_a = _as_st_a;
                    br_st_b = _br_st_b;

                } else {
                    let (mut _bs_st_a, ct) = EtH::send(&mut bs_st_a, &iv2, message.as_bytes());
                    let(mut _ar_st_b, pt) = EtH::receive(&mut ar_st_b, &iv2, ct.as_slice());
                    let crypt_message=str::from_utf8(pt.as_slice()).unwrap();
                    println!("Bob send: {}", message);
                    println!("Alice receive: {}", crypt_message);
                    bs_st_a = _bs_st_a;
                    ar_st_b = _ar_st_b;
                }
                i+=1;
                println!("---");
            }

        }
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}


