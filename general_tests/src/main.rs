mod user;
mod Protocol;
mod udp;

use std::thread;
use std::time::Duration;
use std::sync::{Arc, mpsc};
use bincode::Options;
use crate::user::{SendAndReceive, User};
use chrono::prelude::*;
use serde::Serialize;

extern crate chrono;
use Protocol::*;

use std::env;
use std::net::UdpSocket;
use std::str;

const LISTEN_PORT: u16 = 8900;
const SRC_PORT: u16 = 8901;



fn main() {

    send_hundreds_of_random_messages_test(ProtocolType::EtH);
    //user_communication_test(ProtocolType::DV);
}

/*
fn main() {
    let mut protocol = Protocol::get_protocol_my_type_known_state(ProtocolType::DV);
    let ad =  [124, 191, 209, 81, 8, 70, 95, 59, 129, 120, 174, 120, 243, 82, 222, 46];
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let dest_addr = args[1].clone();
        let num_of_msgs = args[2].clone();
        let num_of_messages  = num_of_msgs.parse::<usize>().unwrap();
        let mut i = 0;
        loop {
            println!("send: {}", i);
            let ciphertext = protocol.send(&ad, vec![i]);
            //println!("send cipher: {:?}", ciphertext);
            //let data_to_send = String::from_utf8(ciphertext).unwrap();
            let src_addr = format!("0.0.0.0:{}", SRC_PORT);
            println!("binding to {} for sending", src_addr.as_str());
            let socket = UdpSocket::bind(src_addr).expect("bind should succeed");

            socket.set_broadcast(true).expect("set_broadcast to true should succeed");

            //println!("broadcasting to {} data of {}", dest_addr.as_str(), data_to_send.as_str());
            socket
                .send_to(&ciphertext, format!("{}:{}", dest_addr.as_str(), LISTEN_PORT))
                .expect("couldn't send data");
            i += 1;
            if i > num_of_messages as u8 {
                break;
            }
        }

    } else {
        let listen_addr = format!("0.0.0.0:{}", LISTEN_PORT);
        let socket = UdpSocket::bind(listen_addr.as_str()).expect("bind should succeed");
        loop {
            let mut buf = [0; 1504];
            let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("read should succeed");
            let filled_buf = &mut buf[..number_of_bytes];
            let cipher = filled_buf.to_vec();
            //rintln!("received cipher: {:?}", cipher);
            let pt = protocol.receive(&ad, cipher);
            let msg = String::from_utf8(pt).unwrap();
            println!("received: {:?}", msg);
        }
    }
}
*/

pub fn send_hundreds_of_random_messages_test(p: ProtocolType) {
    let mut protocol = Protocol::get_protocol_my_type(p);
    let ad = dv::generate_ad();

    let mut i = 0;
    loop{
        let message = String::from(i.to_string());
        let ct = protocol.send(&ad, Vec::from(message.as_bytes()));
        let pt = protocol.receive(&ad, ct.clone());

        let msg = String::from_utf8(pt).unwrap();
        println!("Alice send: {:?}", message);
        println!("Bob receive: {:?}", msg);
        i+=1;
        if i>= 100 {break;}
    }
}

pub fn user_communication_test(p: ProtocolType) {
    let (_s1, _r1) = dv::onion::init();
    let (_s2, _r2) = dv::onion::init();
    let (_s3, _r3) = dv::onion::init();
    let (_s4, _r4) = dv::onion::init();
    let mut _a_s_state = vec![_s1, _s2];
    let mut _b_r_state = vec![_r1, _r2];
    let mut _b_s_state = vec![_s3, _s4];
    let mut _a_r_state = vec![_r3, _r4];

    let ad = dv::generate_iv();
    let ad2 = dv::generate_iv();

    let (a_s, b_r) = mpsc::channel();
    let (b_s, a_r) = mpsc::channel();
    let alice = User {
        name: "Alice",
        send_channel: a_s,
        receive_channel: a_r,
    };
    let bob = User {
        name: "Bob",
        send_channel: b_s,
        receive_channel: b_r,
    };
    //alice is in spawn thread
    thread::spawn(move || {
        for i in 1..50 {
            let dt = Local::now();
            println!("alice sent {} at {}", i, dt.format("%Y-%m-%d %H:%M:%S").to_string());
            let _new_s_state = alice.send(&mut _a_s_state, &ad, vec![i]);
            _a_s_state = _new_s_state;
            thread::sleep(Duration::from_secs(1));
        }
        alice.receive(_a_r_state, &ad2);
    });
    // bob is in main thread
    for i in 1..50 {
        let dt = Local::now();
        println!("bob sent at {}", dt.format("%Y-%m-%d %H:%M:%S").to_string());
        let _new_s_state = bob.send(&mut _b_s_state, &ad2, vec![i]);
        _b_s_state = _new_s_state;
        thread::sleep(Duration::from_secs(1));
    }
    bob.receive(_b_r_state, &ad);
}
    /*
pub fn user_test() {
    let ad = dv::generate_iv();
    let (a_s, b_r)  = mpsc::channel();
    let(b_s, a_r) = mpsc::channel();
    let alice = User{
        name: "Alice",
        send_channel: a_s,
        receive_channel: a_r,
    };
    let bob = User{
        name: "Bob",
        send_channel: b_s,
        receive_channel: b_r,
    };
    //let mut protocol = Protocol::get_protocol_my_type(ProtocolType::DV);
    thread::spawn(move || {
        for i in 1..10 {
            let dt = Local::now();
            println!("alice sent at {}", dt.format("%Y-%m-%d %H:%M:%S").to_string());
            alice.send();
        //}
        thread::sleep(Duration::from_secs(1));
        })


    for receivedCipher in bob.receive_channel {
        //let(mut protocol, ct) :(ProtocolStruct, Vec<u8>)  = bincode::deserialize(&receivedCipher).unwrap();
        let receivedMessage = receivedCipher
       // let pt = protocol.receive(&ad, receivedCipher);
        let dt = Local::now();
        println!("{} received: {:?} at {}", bob.name, pt, dt.format("%Y-%m-%d %H:%M:%S").to_string());
    }




    //let mut protocol = Protocol::get_protocol_my_type(ProtocolType::DV);
    let ad = dv::generate_iv();
    let (_Sender, _Receiver)  = mpsc::channel();

    thread::spawn(move || {
        for i in 1..10 {
            let dt = Local::now();
            println!("alice sent at {}", dt.format("%Y-%m-%d %H:%M:%S").to_string());
            let ct = protocol.send(&ad, &msg);
            Sender.send();
        //}
        thread::sleep(Duration::from_secs(1));
        })


    for receivedCipher in bob.receive_channel {
       // let pt = protocol.receive(&ad, receivedCipher);
        let dt = Local::now();
        println!("Bob received: {:?} at {}", pt, dt.format("%Y-%m-%d %H:%M:%S").to_string());
    }

    //bob.receive(protocol, &ad);



}*/

    /*
fn test(msg: &[u8]) -> &[u8] {

    //let button = Button{
    //    width: 32,
    //    height: 15,
    //    label : String::from("HelloWorld"),
    //    loc: msg
    //};
    //let a = bincode::serialize(&button).expect("fail");
    let b:Button = bincode::deserialize(&msg).expect("fail");
    //let c = *b.loc;
    b.loc

}

#[derive(Serialize, Deserialize, Debug)]
pub struct Button<'a> {
    pub width: u32,
    pub height: u32,
    pub label: String,
    pub loc : &'a [u8]
}

*/


/*
#[derive(Debug)]
pub struct sendMessge {
protocol: Box<dyn ProtocolTrait + Send>,
message:Vec<u8>,
}
*/
/*
//bug
fn protocol_test(){
let mut protocol = Protocol::get_protocol_my_type(ProtocolType::DV);
let ad = dv::generate_iv();
let (_sender, _receiver)  = mpsc::channel();

thread::spawn(move || {
    for i in 1..10 {
        let dt = Local::now();
        println!("alice sent at {}", dt.format("%Y-%m-%d %H:%M:%S").to_string());
        let ct = protocol.send(&ad, vec![i]);
        _sender.send(ct);
        thread::sleep(Duration::from_secs(1));
    }});



for receivedCipher in _receiver {
    let pt = protocol.receive(&ad, receivedCipher);
    let dt = Local::now();
    println!("Bob received: {:?} at {}", pt, dt.format("%Y-%m-%d %H:%M:%S").to_string());
}
}
*/