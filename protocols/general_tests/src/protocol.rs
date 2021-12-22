use std::fs::File;
use std::{io, str};
use std::io::{BufRead, Read};
use std::path::Path;
use EtH::State;



#[derive(Debug, Copy, Clone)]
pub enum Protocol {
    DV,
    EtH,
}

pub fn initall(protocol: Protocol) -> (String, String) {
    match protocol {
        Protocol::DV => {
            let (s1, r1) = dv::onion::init();
            let (s2, r2) = dv::onion::init();
            let s_state = vec![s1, s2];
            let r_state = vec![r1, r2];
            let ad = dv::generate_iv();
            let iv = dv::generate_iv();
            let sender_input = (s_state, ad, iv);
            let receiver_input = (r_state, ad, iv);
            (serde_json::to_string(&sender_input).unwrap(), serde_json::to_string(&receiver_input).unwrap())
        },
        Protocol::EtH => {
            let (_st_a, _st_b) = EtH::initall();
            let iv:[u8;16]= dv::generate_iv();
            let sender_input = (_st_a, iv);
            let receiver_input = (_st_b, iv);
            (serde_json::to_string(&sender_input).unwrap(), serde_json::to_string(&receiver_input).unwrap())
        },
    }
}

pub fn send_then_receive(message:String, protocol: Protocol) -> String{
    match protocol {
        Protocol::DV => {
            let (sender_input, receiver_input) = initall(Protocol::DV);
            let (mut _s_state, _ad, _iv):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&sender_input).unwrap();
            let (mut _r_state, _ad, _iv):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&receiver_input).unwrap();
            let (mut _new_s_state, ct) = dv::onion::send(&mut _s_state, &_ad, Vec::from(message), &_iv);
            let (mut _new_r_state, pt) = dv::onion::receive(&mut _r_state, &_ad, ct, &_iv);
            let msg = String::from_utf8(pt).unwrap();
            msg
        }
        Protocol::EtH => {
            let (sender_input, receiver_input) = initall(Protocol::EtH);
            let (_st_a, _iv): (State, [u8;16]) = serde_json::from_str(&sender_input).unwrap();
            let (_st_b, _iv): (State, [u8;16]) = serde_json::from_str(&receiver_input).unwrap();
            let (_new_st_a, ct) = EtH::send(_st_a, &_iv, message.as_bytes());
            let(_new_st_b, pt) = EtH::receive(_st_b, &_iv, ct.as_slice());
            let msg = String::from_utf8(pt).unwrap();
            msg
        }
    }
}

pub fn send_hundreds_of_messages(protocol: Protocol) {
    match protocol {
        Protocol::DV => {
            let (sender_input, receiver_input) = initall(Protocol::DV);
            let (mut _s_state, _ad, _iv):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&sender_input).unwrap();
            let (mut _r_state, _ad, _iv):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&receiver_input).unwrap();

            let mut i = 0;
            loop{
                let message = String::from(i.to_string());
                let (mut _new_s_state, ct) = dv::onion::send(&mut _s_state, &_ad, Vec::from(message.as_bytes()), &_iv);
                let (mut _new_r_state, pt) = dv::onion::receive(&mut _r_state, &_ad, ct, &_iv);
                let msg = String::from_utf8(pt).unwrap();
                println!("Alice send: {}", message);
                println!("Bob receive: {}", msg);
                _s_state = _new_s_state;
                _r_state = _new_r_state;
                i+=1;
                if i>= 100 {break;}
            }
        }
        Protocol::EtH => {
            let (sender_input, receiver_input) = initall(Protocol::EtH);
            let (mut _st_a, _iv): (State, [u8; 16]) = serde_json::from_str(&sender_input).unwrap();
            let (mut _st_b, _iv): (State, [u8; 16]) = serde_json::from_str(&receiver_input).unwrap();
            let mut i = 0;
            loop {
                let message = String::from(i.to_string());
                let (_new_st_a, ct) = EtH::send(_st_a, &_iv, message.as_bytes());
                let (_new_st_b, pt) = EtH::receive(_st_b, &_iv, ct.as_slice());
                let msg = String::from_utf8(pt).unwrap();
                //println!("{} :", i);
                println!("Alice send: {}", message);
                println!("Bob receive: {}", msg);
                i+=1;
                _st_a = _new_st_a;
                _st_b = _new_st_b;
                if i >= 100 {break;}
            }
        }

    }

}

pub fn send_hundreds_of_random_messages(protocol: Protocol){
    match protocol {
        Protocol::DV => {
            let (sender_input, receiver_input) = initall(Protocol::DV);
            let (mut _s_state, _ad, _iv):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&sender_input).unwrap();
            let (mut _r_state, _ad, _iv):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&receiver_input).unwrap();

            let mut i = 0;
            loop{
                let message = primitives::generate_random_string();
                let (mut _new_s_state, ct) = dv::onion::send(&mut _s_state, &_ad, Vec::from(message.as_bytes()), &_iv);
                let (mut _new_r_state, pt) = dv::onion::receive(&mut _r_state, &_ad, ct, &_iv);
                let msg = String::from_utf8(pt).unwrap();
                println!("Alice send: {}", message);
                println!("Bob receive: {}", msg);
                _s_state = _new_s_state;
                _r_state = _new_r_state;
                i+=1;
                if i>= 100 {break;}
            }
        }
        Protocol::EtH => {
            let (sender_input, receiver_input) = initall(Protocol::EtH);
            let (mut _st_a, _iv): (State, [u8; 16]) = serde_json::from_str(&sender_input).unwrap();
            let (mut _st_b, _iv): (State, [u8; 16]) = serde_json::from_str(&receiver_input).unwrap();
            let mut i = 0;
            loop {
                let message = primitives::generate_random_string();
                //let message = String::from(i.to_string());
                let (_new_st_a, ct) = EtH::send(_st_a, &_iv, message.as_bytes());
                let (_new_st_b, pt) = EtH::receive(_st_b, &_iv, ct.as_slice());
                let msg = String::from_utf8(pt).unwrap();
                //println!("{} :", i);
                println!("Alice send: {}", message);
                println!("Bob receive: {}", msg);
                i+=1;
                _st_a = _new_st_a;
                _st_b = _new_st_b;
                if i >= 100 {break;}
            }
        }

    }
}

pub fn read_file_send_and_receive(protocol: Protocol){
    let mut file = std::fs::File::open("data2.txt").unwrap();
    let mut message = String::new();
    let message_dup = message.clone();
    file.read_to_string(&mut message).unwrap();

    match protocol {
        Protocol::DV => {
            let (sender_input, receiver_input) = initall(Protocol::DV);
            let (mut _s_state, _ad, _iv):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&sender_input).unwrap();
            let (mut _r_state, _ad, _iv):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&receiver_input).unwrap();

            let (mut _new_s_state, ct) = dv::onion::send(&mut _s_state, &_ad, Vec::from(message), &_iv);
            let (mut _new_r_state, pt) = dv::onion::receive(&mut _r_state, &_ad, ct, &_iv);
            let msg = String::from_utf8(pt).unwrap();
            println!("Alice send: {}", message_dup);
            println!("Bob receive: {}", msg);
        }
        Protocol::EtH => {
            let (sender_input, receiver_input) = initall(Protocol::EtH);
            let (_st_a, _iv): (State, [u8;16]) = serde_json::from_str(&sender_input).unwrap();
            let (_st_b, _iv): (State, [u8;16]) = serde_json::from_str(&receiver_input).unwrap();

            let (_new_st_a, ct) = EtH::send(_st_a, &_iv, message.as_bytes());
            let(_new_st_b, pt) = EtH::receive(_st_b, &_iv, ct.as_slice());
            let msg = String::from_utf8(pt).unwrap();
            println!("Alice send: {}", message_dup);
            println!("Bob receive: {}", msg);
        }
    }
}

pub fn read_line_communicate(protocol: Protocol) {
    if let Ok(lines) = read_lines("data.txt") {
        // read file
        let mut file = std::fs::File::open("data.txt").unwrap();
        let mut message = String::new();
        file.read_to_string(&mut message).unwrap();


        match protocol {
            Protocol::EtH => {
                let (sender_input, receiver_input) = initall(Protocol::EtH);
                let (mut _as_st_a, _iv): (State, [u8;16]) = serde_json::from_str(&sender_input).unwrap();
                let (mut _br_st_b, _iv): (State, [u8;16]) = serde_json::from_str(&receiver_input).unwrap();

                let (sender_input, receiver_input) = initall(Protocol::EtH);
                let (mut _bs_st_a, _iv2): (State, [u8;16]) = serde_json::from_str(&sender_input).unwrap();
                let (mut _ar_st_b, _iv2): (State, [u8;16]) = serde_json::from_str(&receiver_input).unwrap();

                //let _iv:[u8;16]= EtH::generate_iv();
                //let _iv2 = EtH::generate_iv();

                let mut i = 0;

                for line in lines {
                    if let Ok(message) = line {
                        if i % 2 == 0 {
                            let (as_st_a, ct) = EtH::send(_as_st_a, &_iv, message.as_bytes());
                            let(br_st_b, pt) = EtH::receive(_br_st_b, &_iv, ct.as_slice());
                            let crypt_message= str::from_utf8(pt.as_slice()).unwrap();
                            _as_st_a = as_st_a;
                            _br_st_b = br_st_b;
                            println!("Alice send: {}", message);
                            println!("Bob receive: {}", crypt_message);

                        } else {
                            let (bs_st_a, ct) = EtH::send(_bs_st_a, &_iv2, message.as_bytes());
                            let(ar_st_b, pt) = EtH::receive(_ar_st_b, &_iv2, ct.as_slice());
                            let crypt_message=str::from_utf8(pt.as_slice()).unwrap();
                            println!("Bob send: {}", message);
                            println!("Alice receive: {}", crypt_message);
                            _bs_st_a = bs_st_a;
                            _ar_st_b = ar_st_b;
                        }
                        i+=1;
                        println!("---");
                    }

                }
            }
            Protocol::DV => {

                let (sender_input, receiver_input) = initall(Protocol::DV);
                let (mut _as_state, _ad, _iv):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&sender_input).unwrap();
                let (mut _br_state, _ad, _iv):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&receiver_input).unwrap();

                let (sender_input, receiver_input) = initall(Protocol::DV);
                let (mut _bs_state, _ad2, _iv2):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&sender_input).unwrap();
                let (mut _ar_state, _ad2, _iv2):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&receiver_input).unwrap();

                let mut i = 0;
                for line in lines {
                    if let Ok(message) = line {
                        if i % 2 == 0 {
                            let mes_dup = message.clone();
                            let (mut _new_as_state, ct) = dv::onion::send(&mut _as_state, &_ad, Vec::from(message), &_iv);
                            let (mut _new_br_state, pt) = dv::onion::receive(&mut _br_state, &_ad, ct, &_iv);
                            let crypt_message = str::from_utf8(pt.as_slice()).unwrap();
                            _as_state = _new_as_state;
                            _br_state = _new_br_state;
                            println!("Alice send: {}", mes_dup);
                            println!("Bob receive: {}", crypt_message);
                        } else {
                            let mes_dup = message.clone();
                            let (mut _new_bs_state, ct) = dv::onion::send(&mut _bs_state, &_iv2, Vec::from(message), &_iv2);
                            let (mut _new_ar_state, pt) = dv::onion::receive(&mut _ar_state, &_iv2, ct, &_iv2);
                            let crypt_message = str::from_utf8(pt.as_slice()).unwrap();
                            _bs_state = _new_bs_state;
                            _ar_state = _new_ar_state;
                            println!("Bob send: {}", mes_dup);
                            println!("Alice receive: {}", crypt_message);
                        }
                        i += 1;
                        println!("---");
                    }
                }

            }
        }

    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

}