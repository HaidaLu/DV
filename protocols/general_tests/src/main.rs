use crate::protocol::{Protocol};
pub mod protocol;
pub use dv::*;
pub use EtH::*;
pub use primitives::*;

fn main() {

    //send_then_receive_test(Protocol::DV);
    //send_then_receive_test(Protocol::EtH);
    //send_hundreds_of_messages_test(Protocol::EtH);
    //send_hundreds_of_messages_test(Protocol::DV);
    //send_hundreds_of_random_messages_test(Protocol::DV);
    //send_hundreds_of_random_messages_test(Protocol::EtH);
    //read_file_send_and_receive_test(Protocol::DV);
    //read_file_send_and_receive_test(Protocol::EtH);
    //read_line_communicate_test(Protocol::EtH);
    read_line_communicate_test(Protocol::DV);
}


pub fn send_then_receive_test(protocol: Protocol) {
    let message = "hello".to_string();
    let dup_message = message.clone();
    let receive_message = protocol::send_then_receive(message, protocol);
    println!("message to be sent by {:?} is:{}", protocol, dup_message);
    println!("message after being received is:{}", receive_message);
}

pub fn send_hundreds_of_messages_test(protocol: Protocol) {
    protocol::send_hundreds_of_messages(protocol);
}


pub fn send_hundreds_of_random_messages_test(protocol: Protocol){
    protocol::send_hundreds_of_random_messages(protocol);
}


pub fn read_file_send_and_receive_test(protocol: Protocol){
    protocol::read_file_send_and_receive(protocol);
}


pub fn read_line_communicate_test(protocol: Protocol) {
    protocol::read_line_communicate(protocol);
}