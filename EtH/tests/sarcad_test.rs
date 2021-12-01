use std::io::{BufRead, Read};
//use core::slice::SlicePattern;
use rand::OsRng;
//use rand::distributions::Alphanumeric;
use EtH;
use std::{io, str};
use std::fs::File;
use std::path::Path;
use rand::{Rng, thread_rng};
//use rand::distributions::{Alphanumeric, Uniform, Standard};
//use rand::distributions::Alphanumeric;




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
/*
#[test]
fn send_hundred_messages() {
    let (st_a, st_b) = EtH::initall();

    let message = "Hello!";
    //let message2 = "How are you";
    let mut iv: [u8; 16] = [0; 16];
    let mut rng = OsRng::new().ok().unwrap();
    rng.fill_bytes(&mut iv);
    let mut rng = rand::thread_rng();

    let new_st_a = st_a;
    let new_st_b = st_b;
    let pt = message.as_bytes();
    let mut i = 0;

    loop {


        let mut rng = rand::thread_rng();
        let random_number: u8 = rng.gen();
        let pt = serde_json::to_string(&random_number).unwrap();
        let original_message = serde_json::to_string(&random_number).unwrap();
        let (new_st_a, ct) = EtH::send(new_st_a, &iv, pt.as_bytes());
        let (new_st_b, pt) = EtH::receive(new_st_b, &iv, ct.as_slice());

        let crypt_message = str::from_utf8(pt.as_slice()).unwrap();

        assert_eq!(original_message, crypt_message);
        println!("{} :", i);
        println!("Alice send: {}", random_number);
        println!("Bob receive: {}", crypt_message);


        i+=1;
        if i>=50 { break; }
    }
}

#[test]
fn test_read_file_send_and_receive(){
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut message = String::new();
    file.read_to_string(&mut message).unwrap();


    let (st_a, st_b) = EtH::initall();
    //let message="Hello World!";
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
*/
#[test]
pub fn test_read_line() {
    if let Ok(lines) = read_lines("data.txt") {
        // 使用迭代器，返回一个（可选）字符串
        let mut file = std::fs::File::open("data.txt").unwrap();
        let mut message = String::new();
        file.read_to_string(&mut message).unwrap();

        let (as_st_a, br_st_b) = EtH::initall();
        let (bs_st_a, ar_st_b) = EtH::initall();
        //let message="Hello World!";
        let mut iv:[u8;16]=[0;16];
        let mut iv2:[u8;16] = [0;16];
        let mut rng=OsRng::new().ok().unwrap();
        rng.fill_bytes(&mut iv);
        rng.fill_bytes(&mut iv2);

        let mut i = 0;
        for line in lines {

            if let Ok(message) = line {
                if i % 2 == 0 {
                    let (as_st_a, ct) = EtH::send(as_st_a, &iv, message.as_bytes());
                    let(br_st_b, pt) = EtH::receive(br_st_b, &iv, ct.as_slice());
                    let crypt_message=str::from_utf8(pt.as_slice()).unwrap();
                    println!("Alice send: {}", message);
                    //println!("Bob receive: {}", crypt_message);
                } else {
                    let (bs_st_a, ct) = EtH::send(bs_st_a, &iv, message.as_bytes());
                    let(ar_st_b, pt) = EtH::receive(ar_st_b, &iv, ct.as_slice());
                    let crypt_message=str::from_utf8(pt.as_slice()).unwrap();
                    println!("Bob send: {}", message);
                    //println!("Alice receive: {}", crypt_message);
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