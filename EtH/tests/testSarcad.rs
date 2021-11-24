//use core::slice::SlicePattern;
use rand::{OsRng, Rng};
use EtH;
use std::str;

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
    /*
    let(new_st_b, pt) = EtH::receive(st_b, &iv, ct);

    let crypt_message=str::from_utf8(pt.as_slice()).unwrap();


    assert_eq!(message,crypt_message);
    println!("message is: {}", message);
    println!("message after encrypt-decrypt is: {}", crypt_message);*/
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
