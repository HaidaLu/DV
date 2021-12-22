
use std::sync::{Arc,mpsc};
use std::sync::mpsc::{Receiver, Sender};
use chrono::Local;
use crate::Protocol::{ProtocolTrait, ProtocolType};
pub trait SendAndReceive {
    fn send(&self, s:&mut Vec<Vec<u8>>, ad: &[u8], msg: Vec<u8>) ->(Vec<Vec<u8>>);
    fn receive(&self, s:Vec<Vec<u8>>, ad: &[u8]);
}


pub struct User<'a> {
    pub name: &'a str,
    pub send_channel: Sender<Vec<u8>>,
    pub receive_channel: Receiver<Vec<u8>>,
}

impl<'a> SendAndReceive for User<'a>{
    fn send(&self, s : &mut Vec<Vec<u8>>, ad: &[u8], msg: Vec<u8>) ->(Vec<Vec<u8>>) {
        let (_new_state, ct) = dv::onion::send(s, &ad, msg);
        self.send_channel.send(ct).unwrap();
        (_new_state)
    }


    fn receive(&self, mut s: Vec<Vec<u8>>, ad: &[u8]){
        for ciphertext in &self.receive_channel {
            let (mut _new_r_state, pt) = dv::onion::receive(&mut s, ad, ciphertext);
            s = _new_r_state;
            let dt = Local::now();
            println!("{} received: {:?} at {}", self.name, pt, dt.format("%Y-%m-%d %H:%M:%S").to_string());
        }
    }
}