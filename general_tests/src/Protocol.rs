
use std::fs::File;
use std::{io, str};
use std::io::{BufRead, Read};
use std::path::Path;
use EtH::{send, State};
use serde::{Serialize,Deserialize};
//use crate::protocol::Protocol::DV;

//const DV_STATE: Vec<u8> = initall(ProtocolType::DV);
//const ETH_STATE: Vec<u8> = initall(ProtocolType::EtH);

pub trait ProtocolTrait {
    fn send(&mut self, ad: &[u8], msg: Vec<u8>) -> (Vec<u8>);
    fn receive(&mut self, ad :&[u8], ct: Vec<u8>) -> (Vec<u8>);
}


/*
trait ProtocolClone {
    fn clone_box(&self) -> Box<dyn Protocol>;
}

impl<T> ProtocolClone for T
    where
        T: 'static + Protocol + Clone + Send,
{
    fn clone_box(&self) -> Box<dyn Protocol> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Protocol> {
    fn clone(&self) -> Box<dyn Protocol> {
        self.clone_box()
    }
}*/

#[derive(Debug, Copy, Clone)]
pub enum ProtocolType {
    DV,
    EtH,
}


//pub fn send(st: & mut Vec<u8>, ad: &[u8], pt: &[u8]) -> (Vec<u8>, Vec<u8>){
//pub fn receive(st:& mut Vec<u8>, ad: &[u8], ct: &[u8]) -> (Vec<u8>, Vec<u8>) {
#[derive(Clone,Serialize,Deserialize)]
pub struct eth {
    //protocol_type: ProtocolType,
    _send_state: Vec<u8>,
    _receive_state: Vec<u8>
}

impl ProtocolTrait for eth {
    fn send(&mut self, ad: &[u8], msg: Vec<u8>) -> ( Vec<u8>){
        let (_new_state, ct) = EtH::send(&mut self._send_state, ad, &msg);
        self._send_state = _new_state;
        //(Box::new(self), ct)
        ct
    }

    fn receive(&mut self, ad :&[u8], ct: Vec<u8>) -> (Vec<u8>){
        let (_new_state, pt) = EtH::receive(&mut self._receive_state, ad, &ct);
        self._receive_state = _new_state;
        pt
    }
}
#[derive(Clone,Serialize,Deserialize)]
pub struct DV {
    //protocol_type: ProtocolType,
    _send_state: Vec<Vec<u8>>,
    _receive_state: Vec<Vec<u8>>,
}

impl ProtocolTrait for DV {
    fn send(&mut self, ad: &[u8], msg: Vec<u8>) -> (Vec<u8>){
        let (_new_state, ct) = dv::onion::send(&mut self._send_state, ad, msg);
        self._send_state = _new_state;
        ct
    }
    fn receive(&mut self, ad :&[u8], ct: Vec<u8>) -> (Vec<u8>){
        let (_new_state, pt) = dv::onion::receive(&mut self._receive_state, ad, ct);
        self._receive_state = _new_state;
        pt
    }
}

pub fn get_protocol_my_type(protocol_type: ProtocolType) -> Box<dyn ProtocolTrait + Send>{
    let state = initall(protocol_type);
    match protocol_type {
        ProtocolType::EtH => {
            println!("EtH: {:?}", state);
            let (_s_state, _r_state) = bincode::deserialize(&state).unwrap();
            Box::new(eth {
                _send_state:_s_state,
                _receive_state: _r_state,
            } )
        },
        ProtocolType::DV =>{
            println!("DV: {:?}", state);
            let (_s_state, _r_state) = bincode::deserialize(&state).unwrap();
            Box::new(DV{
                _send_state:_s_state,
                _receive_state: _r_state,
            })
        },
        _=> panic!("No such protocol")
    }
}


pub fn get_protocol_my_type_known_state(protocol_type: ProtocolType) -> Box<dyn ProtocolTrait + Send>{
    match protocol_type {
        ProtocolType::EtH => {
            let state:Vec<u8> = vec![96, 0, 0, 0, 0, 0, 0, 0, 142, 222, 107, 77, 26, 178, 112, 213, 113, 75, 76, 223, 112, 72, 153, 160, 182, 142, 112, 238, 193, 47, 207, 236, 172, 179, 166, 142, 128, 66, 54, 192, 239, 131, 10, 118, 50, 240, 178, 232, 133, 36, 28, 86, 222, 145, 0, 145, 122, 70, 201, 134, 46, 201, 17, 116, 119, 212, 228, 247, 39, 72, 192, 56, 140, 220, 222, 97, 248, 18, 70, 12, 240, 12, 27, 171, 208, 61, 137, 201, 9, 92, 94, 239, 194, 27, 92, 73, 80, 9, 182, 201, 140, 117, 187, 161, 96, 0, 0, 0, 0, 0, 0, 0, 142, 222, 107, 77, 26, 178, 112, 213, 113, 75, 76, 223, 112, 72, 153, 160, 182, 142, 112, 238, 193, 47, 207, 236, 172, 179, 166, 142, 128, 66, 54, 192, 140, 220, 222, 97, 248, 18, 70, 12, 240, 12, 27, 171, 208, 61, 137, 201, 9, 92, 94, 239, 194, 27, 92, 73, 80, 9, 182, 201, 140, 117, 187, 161, 239, 131, 10, 118, 50, 240, 178, 232, 133, 36, 28, 86, 222, 145, 0, 145, 122, 70, 201, 134, 46, 201, 17, 116, 119, 212, 228, 247, 39, 72, 192, 56];
            println!("test");
            let (_s_state, _r_state) = bincode::deserialize(&state).unwrap();
            Box::new(eth {
                _send_state:_s_state,
                _receive_state: _r_state,
            } )
        },
        ProtocolType::DV =>{
            let state:Vec<u8> = vec![2, 0, 0, 0, 0, 0, 0, 0, 96, 1, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 38, 39, 246, 133, 151, 21, 173, 29, 210, 148, 221, 196, 118, 25, 57, 49, 241, 173, 181, 88, 240, 147, 151, 50, 25, 43, 209, 192, 253, 22, 142, 78, 93, 109, 35, 107, 82, 209, 142, 58, 182, 214, 7, 47, 182, 228, 199, 212, 107, 213, 154, 217, 204, 25, 71, 38, 95, 0, 183, 32, 250, 44, 143, 102, 16, 1, 0, 0, 0, 0, 0, 0, 45, 45, 45, 45, 45, 66, 69, 71, 73, 78, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10, 77, 73, 71, 102, 77, 65, 48, 71, 67, 83, 113, 71, 83, 73, 98, 51, 68, 81, 69, 66, 65, 81, 85, 65, 65, 52, 71, 78, 65, 68, 67, 66, 105, 81, 75, 66, 103, 81, 68, 89, 98, 47, 54, 108, 118, 57, 73, 56, 75, 73, 114, 116, 115, 105, 120, 79, 76, 110, 83, 67, 107, 72, 82, 110, 10, 57, 121, 84, 86, 121, 106, 98, 120, 65, 49, 100, 109, 119, 80, 118, 78, 82, 97, 113, 121, 65, 67, 114, 78, 47, 75, 89, 105, 53, 67, 121, 98, 99, 117, 110, 51, 47, 119, 113, 48, 85, 81, 115, 86, 71, 104, 78, 77, 73, 72, 67, 115, 88, 106, 100, 74, 109, 73, 112, 75, 120, 75, 122, 105, 10, 101, 104, 66, 122, 106, 100, 120, 101, 53, 79, 114, 106, 72, 50, 90, 110, 56, 105, 80, 108, 84, 120, 108, 115, 53, 122, 80, 111, 70, 110, 107, 77, 97, 68, 51, 89, 108, 56, 43, 86, 99, 101, 115, 117, 73, 77, 55, 102, 77, 86, 108, 74, 102, 121, 75, 87, 72, 79, 103, 81, 106, 90, 78, 103, 10, 85, 106, 78, 107, 83, 86, 52, 68, 87, 97, 90, 54, 86, 116, 74, 50, 103, 119, 73, 68, 65, 81, 65, 66, 10, 45, 45, 45, 45, 45, 69, 78, 68, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10, 96, 1, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 38, 39, 246, 133, 151, 21, 173, 29, 210, 148, 221, 196, 118, 25, 57, 49, 241, 173, 181, 88, 240, 147, 151, 50, 25, 43, 209, 192, 253, 22, 142, 78, 93, 109, 35, 107, 82, 209, 142, 58, 182, 214, 7, 47, 182, 228, 199, 212, 107, 213, 154, 217, 204, 25, 71, 38, 95, 0, 183, 32, 250, 44, 143, 102, 16, 1, 0, 0, 0, 0, 0, 0, 45, 45, 45, 45, 45, 66, 69, 71, 73, 78, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10, 77, 73, 71, 102, 77, 65, 48, 71, 67, 83, 113, 71, 83, 73, 98, 51, 68, 81, 69, 66, 65, 81, 85, 65, 65, 52, 71, 78, 65, 68, 67, 66, 105, 81, 75, 66, 103, 81, 68, 102, 84, 82, 79, 79, 68, 98, 79, 77, 117, 65, 53, 66, 115, 74, 107, 107, 68, 97, 52, 83, 122, 120, 72, 119, 10, 50, 73, 52, 78, 89, 84, 84, 57, 107, 83, 102, 113, 74, 78, 79, 51, 114, 75, 70, 50, 74, 57, 55, 78, 89, 66, 66, 121, 106, 47, 115, 88, 89, 84, 86, 71, 110, 102, 89, 47, 78, 110, 101, 120, 84, 110, 73, 114, 72, 51, 116, 117, 43, 108, 119, 122, 53, 97, 121, 102, 72, 49, 68, 82, 10, 106, 86, 51, 86, 86, 66, 109, 75, 69, 86, 77, 121, 68, 100, 119, 47, 75, 112, 116, 68, 69, 121, 47, 68, 121, 113, 52, 50, 115, 104, 118, 77, 68, 80, 43, 102, 109, 108, 102, 69, 70, 69, 84, 72, 106, 101, 49, 52, 105, 56, 56, 112, 48, 79, 107, 81, 53, 50, 106, 48, 48, 87, 88, 90, 10, 70, 103, 117, 118, 54, 114, 83, 90, 119, 105, 74, 70, 77, 69, 115, 117, 71, 119, 73, 68, 65, 81, 65, 66, 10, 45, 45, 45, 45, 45, 69, 78, 68, 32, 80, 85, 66, 76, 73, 67, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10, 2, 0, 0, 0, 0, 0, 0, 0, 167, 3, 0, 0, 0, 0, 0, 0, 119, 3, 0, 0, 0, 0, 0, 0, 45, 45, 45, 45, 45, 66, 69, 71, 73, 78, 32, 82, 83, 65, 32, 80, 82, 73, 86, 65, 84, 69, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10, 77, 73, 73, 67, 88, 81, 73, 66, 65, 65, 75, 66, 103, 81, 68, 89, 98, 47, 54, 108, 118, 57, 73, 56, 75, 73, 114, 116, 115, 105, 120, 79, 76, 110, 83, 67, 107, 72, 82, 110, 57, 121, 84, 86, 121, 106, 98, 120, 65, 49, 100, 109, 119, 80, 118, 78, 82, 97, 113, 121, 65, 67, 114, 78, 10, 47, 75, 89, 105, 53, 67, 121, 98, 99, 117, 110, 51, 47, 119, 113, 48, 85, 81, 115, 86, 71, 104, 78, 77, 73, 72, 67, 115, 88, 106, 100, 74, 109, 73, 112, 75, 120, 75, 122, 105, 101, 104, 66, 122, 106, 100, 120, 101, 53, 79, 114, 106, 72, 50, 90, 110, 56, 105, 80, 108, 84, 120, 108, 115, 10, 53, 122, 80, 111, 70, 110, 107, 77, 97, 68, 51, 89, 108, 56, 43, 86, 99, 101, 115, 117, 73, 77, 55, 102, 77, 86, 108, 74, 102, 121, 75, 87, 72, 79, 103, 81, 106, 90, 78, 103, 85, 106, 78, 107, 83, 86, 52, 68, 87, 97, 90, 54, 86, 116, 74, 50, 103, 119, 73, 68, 65, 81, 65, 66, 10, 65, 111, 71, 65, 97, 98, 101, 83, 98, 48, 49, 90, 109, 65, 55, 74, 104, 84, 54, 120, 67, 50, 76, 47, 102, 56, 75, 108, 83, 82, 55, 98, 102, 66, 49, 119, 90, 112, 113, 87, 78, 49, 100, 116, 53, 112, 55, 118, 117, 70, 65, 67, 109, 71, 105, 101, 107, 89, 53, 89, 66, 51, 110, 67, 10, 116, 71, 74, 55, 122, 81, 87, 69, 51, 48, 76, 69, 83, 68, 118, 85, 76, 83, 48, 55, 51, 104, 80, 71, 47, 56, 89, 74, 76, 47, 105, 48, 81, 55, 105, 118, 52, 111, 86, 113, 85, 72, 51, 90, 81, 47, 87, 102, 112, 68, 89, 90, 99, 79, 117, 117, 104, 56, 52, 70, 53, 76, 47, 112, 10, 106, 103, 90, 108, 118, 86, 98, 118, 90, 112, 98, 74, 102, 121, 78, 115, 82, 49, 87, 68, 114, 84, 108, 101, 52, 117, 81, 78, 121, 76, 106, 100, 83, 86, 118, 90, 73, 99, 107, 109, 118, 104, 51, 107, 109, 49, 107, 67, 81, 81, 68, 50, 49, 118, 75, 66, 121, 116, 107, 49, 114, 101, 101, 52, 10, 53, 50, 67, 118, 112, 82, 106, 83, 117, 54, 83, 122, 55, 89, 88, 56, 52, 89, 47, 122, 50, 112, 81, 117, 98, 81, 81, 86, 117, 114, 75, 72, 53, 122, 116, 52, 66, 52, 78, 53, 104, 56, 83, 118, 97, 67, 66, 76, 79, 121, 108, 88, 97, 78, 47, 51, 76, 65, 48, 76, 120, 119, 73, 43, 10, 104, 55, 116, 82, 117, 52, 88, 47, 65, 107, 69, 65, 52, 72, 103, 51, 115, 75, 87, 73, 49, 114, 47, 73, 75, 109, 112, 106, 67, 56, 105, 79, 88, 97, 99, 116, 69, 105, 104, 74, 102, 120, 100, 70, 122, 121, 70, 81, 103, 85, 114, 119, 122, 89, 78, 51, 97, 117, 122, 89, 47, 112, 55, 99, 10, 88, 77, 112, 87, 49, 106, 55, 56, 71, 98, 114, 81, 43, 77, 72, 54, 112, 119, 67, 106, 74, 106, 47, 57, 79, 90, 110, 49, 75, 103, 102, 51, 102, 81, 74, 66, 65, 80, 80, 82, 113, 67, 78, 97, 109, 105, 108, 121, 121, 83, 81, 85, 115, 75, 86, 117, 51, 57, 69, 119, 100, 109, 85, 103, 10, 74, 113, 120, 43, 112, 98, 72, 75, 52, 51, 110, 107, 82, 120, 101, 114, 110, 98, 79, 117, 84, 85, 105, 110, 113, 48, 57, 57, 77, 50, 80, 76, 53, 55, 74, 73, 69, 118, 76, 55, 98, 100, 98, 53, 49, 84, 48, 102, 54, 51, 67, 87, 112, 65, 110, 56, 119, 69, 77, 67, 81, 71, 76, 118, 10, 50, 101, 118, 56, 75, 107, 73, 116, 75, 98, 97, 52, 83, 89, 117, 113, 68, 69, 88, 118, 74, 52, 104, 51, 82, 86, 51, 70, 68, 106, 110, 114, 102, 121, 90, 100, 116, 54, 70, 57, 114, 113, 98, 115, 87, 103, 66, 119, 87, 77, 82, 49, 85, 113, 122, 47, 43, 102, 73, 70, 83, 53, 73, 53, 10, 80, 90, 100, 97, 83, 111, 55, 84, 97, 88, 78, 52, 68, 110, 49, 71, 73, 97, 107, 67, 81, 81, 67, 117, 118, 50, 79, 112, 71, 57, 106, 66, 65, 76, 104, 110, 53, 56, 100, 79, 83, 72, 67, 78, 55, 52, 117, 50, 113, 73, 114, 104, 81, 72, 99, 70, 81, 101, 117, 48, 48, 102, 72, 90, 10, 110, 90, 72, 65, 111, 101, 117, 84, 73, 88, 115, 57, 78, 71, 104, 106, 116, 83, 66, 116, 119, 54, 68, 118, 106, 102, 120, 69, 75, 70, 79, 90, 53, 86, 111, 65, 115, 83, 71, 49, 98, 71, 79, 98, 10, 45, 45, 45, 45, 45, 69, 78, 68, 32, 82, 83, 65, 32, 80, 82, 73, 86, 65, 84, 69, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10, 32, 0, 0, 0, 0, 0, 0, 0, 93, 109, 35, 107, 82, 209, 142, 58, 182, 214, 7, 47, 182, 228, 199, 212, 107, 213, 154, 217, 204, 25, 71, 38, 95, 0, 183, 32, 250, 44, 143, 102, 167, 3, 0, 0, 0, 0, 0, 0, 119, 3, 0, 0, 0, 0, 0, 0, 45, 45, 45, 45, 45, 66, 69, 71, 73, 78, 32, 82, 83, 65, 32, 80, 82, 73, 86, 65, 84, 69, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10, 77, 73, 73, 67, 88, 65, 73, 66, 65, 65, 75, 66, 103, 81, 68, 102, 84, 82, 79, 79, 68, 98, 79, 77, 117, 65, 53, 66, 115, 74, 107, 107, 68, 97, 52, 83, 122, 120, 72, 119, 50, 73, 52, 78, 89, 84, 84, 57, 107, 83, 102, 113, 74, 78, 79, 51, 114, 75, 70, 50, 74, 57, 55, 78, 10, 89, 66, 66, 121, 106, 47, 115, 88, 89, 84, 86, 71, 110, 102, 89, 47, 78, 110, 101, 120, 84, 110, 73, 114, 72, 51, 116, 117, 43, 108, 119, 122, 53, 97, 121, 102, 72, 49, 68, 82, 106, 86, 51, 86, 86, 66, 109, 75, 69, 86, 77, 121, 68, 100, 119, 47, 75, 112, 116, 68, 69, 121, 47, 68, 10, 121, 113, 52, 50, 115, 104, 118, 77, 68, 80, 43, 102, 109, 108, 102, 69, 70, 69, 84, 72, 106, 101, 49, 52, 105, 56, 56, 112, 48, 79, 107, 81, 53, 50, 106, 48, 48, 87, 88, 90, 70, 103, 117, 118, 54, 114, 83, 90, 119, 105, 74, 70, 77, 69, 115, 117, 71, 119, 73, 68, 65, 81, 65, 66, 10, 65, 111, 71, 66, 65, 74, 117, 80, 118, 111, 101, 47, 48, 56, 50, 56, 105, 51, 73, 72, 55, 83, 102, 73, 118, 47, 81, 113, 102, 88, 48, 106, 52, 66, 74, 67, 72, 103, 66, 111, 70, 122, 53, 117, 50, 87, 43, 106, 117, 47, 102, 115, 88, 114, 77, 69, 115, 88, 67, 101, 68, 88, 97, 69, 10, 75, 102, 70, 65, 75, 107, 82, 118, 86, 89, 77, 88, 43, 48, 69, 47, 100, 87, 89, 105, 49, 65, 106, 122, 103, 65, 85, 116, 82, 69, 77, 105, 80, 50, 116, 65, 70, 48, 88, 73, 54, 43, 65, 50, 54, 51, 98, 66, 50, 76, 119, 98, 100, 114, 101, 80, 122, 71, 75, 104, 112, 106, 57, 69, 10, 86, 67, 77, 90, 97, 104, 48, 88, 72, 103, 67, 105, 52, 109, 76, 49, 52, 43, 99, 113, 77, 50, 47, 113, 111, 99, 67, 53, 89, 106, 104, 68, 116, 73, 54, 119, 55, 56, 105, 116, 97, 79, 84, 80, 119, 56, 100, 66, 65, 107, 69, 65, 56, 52, 79, 56, 51, 114, 69, 57, 121, 68, 66, 71, 10, 84, 101, 71, 83, 117, 57, 73, 114, 118, 78, 65, 117, 43, 57, 68, 90, 115, 100, 106, 67, 99, 80, 111, 57, 70, 104, 80, 54, 80, 77, 83, 108, 83, 120, 52, 77, 99, 50, 98, 90, 102, 110, 108, 115, 80, 85, 56, 105, 52, 101, 74, 104, 76, 90, 54, 117, 86, 102, 98, 104, 54, 104, 103, 72, 10, 81, 88, 54, 105, 53, 76, 112, 82, 118, 119, 74, 66, 65, 79, 114, 65, 66, 109, 48, 85, 73, 117, 117, 113, 121, 109, 68, 53, 70, 97, 55, 85, 67, 84, 100, 116, 52, 71, 47, 110, 76, 72, 76, 53, 103, 88, 81, 86, 102, 74, 112, 43, 51, 116, 100, 103, 113, 103, 57, 52, 85, 57, 90, 101, 10, 55, 48, 122, 81, 99, 81, 81, 87, 113, 109, 55, 65, 86, 88, 54, 49, 110, 108, 118, 106, 87, 56, 57, 100, 56, 85, 48, 116, 51, 120, 75, 118, 65, 113, 85, 67, 81, 72, 71, 80, 65, 54, 114, 78, 43, 77, 79, 56, 84, 50, 122, 79, 80, 115, 69, 55, 49, 52, 50, 114, 102, 57, 50, 75, 10, 110, 90, 112, 107, 80, 80, 105, 50, 109, 88, 111, 77, 81, 117, 118, 72, 119, 70, 86, 120, 89, 57, 107, 70, 73, 51, 80, 112, 90, 43, 75, 72, 88, 118, 114, 55, 117, 78, 109, 118, 89, 66, 108, 83, 107, 106, 79, 54, 108, 78, 101, 107, 71, 73, 54, 86, 110, 76, 48, 67, 81, 66, 53, 74, 10, 43, 73, 112, 75, 100, 116, 99, 66, 105, 109, 116, 73, 117, 69, 116, 43, 103, 70, 72, 117, 107, 50, 55, 99, 51, 82, 100, 89, 113, 105, 78, 89, 81, 109, 79, 77, 121, 121, 80, 118, 68, 81, 74, 66, 47, 69, 57, 69, 111, 73, 118, 65, 117, 86, 82, 86, 117, 70, 102, 104, 54, 83, 89, 89, 10, 79, 80, 54, 83, 76, 49, 53, 113, 79, 75, 66, 69, 115, 117, 117, 117, 79, 83, 69, 67, 81, 66, 116, 52, 82, 77, 106, 80, 55, 78, 50, 78, 57, 51, 87, 90, 76, 77, 88, 88, 109, 52, 77, 72, 43, 108, 112, 103, 101, 100, 57, 75, 120, 54, 75, 100, 121, 53, 54, 83, 106, 56, 51, 48, 10, 69, 53, 113, 77, 52, 70, 55, 56, 76, 43, 114, 98, 115, 88, 112, 51, 100, 47, 117, 106, 78, 117, 105, 106, 106, 111, 69, 71, 117, 53, 72, 83, 57, 87, 74, 68, 65, 50, 53, 98, 68, 101, 103, 61, 10, 45, 45, 45, 45, 45, 69, 78, 68, 32, 82, 83, 65, 32, 80, 82, 73, 86, 65, 84, 69, 32, 75, 69, 89, 45, 45, 45, 45, 45, 10, 32, 0, 0, 0, 0, 0, 0, 0, 93, 109, 35, 107, 82, 209, 142, 58, 182, 214, 7, 47, 182, 228, 199, 212, 107, 213, 154, 217, 204, 25, 71, 38, 95, 0, 183, 32, 250, 44, 143, 102];
            let (_s_state, _r_state) = bincode::deserialize(&state).unwrap();
            Box::new(DV{
                _send_state:_s_state,
                _receive_state: _r_state,
            })
        },
        _=> panic!("No such protocol")
    }
}


pub fn initall(protocol: ProtocolType) -> (Vec<u8>) {
    match protocol {
        ProtocolType::EtH => {
            let _eth_state = EtH::initall();
            let state = bincode::serialize(&_eth_state).unwrap();
            state
        },
        ProtocolType::DV => {
            let (s1, r1) = dv::onion::init();
            let (s2, r2) = dv::onion::init();
            let _s_state = vec![s1, s2];
            let _r_state = vec![r1, r2];
            let _dv_state = (_s_state, _r_state);
            let state = bincode::serialize(&_dv_state).unwrap();
            state
        },
    }
}

/*
enum ProtocolWrapper {
    EtHProtocol(eth),
    DVProtocol(DV),
}

impl Protocol for ProtocolWrapper {
    fn send(&self, ad: &[u8], msg: Vec<u8>) -> (Vec<u8>){
        match self {
            ProtocolWrapper::EtHProtocol(mut eth) => eth.send(ad, ct),
            ProtocolWrapper::DVProtocol(mut dv) => dv.send(ad, ct),
        }
    }
    fn receive(&self, ad :&[u8], ct: Vec<u8>) -> (Vec<u8>) {
        match self {
            ProtocolWrapper::EtHProtocol(mut eth) => eth.receive(ad, ct),
            ProtocolWrapper::DVProtocol(mut dv) => dv.receive(ad, ct),
        }
    }
}
*/














/*
// Protocol bundles all ARCAD protocol instances under a common interface.
type Protocol interface {
	Init() (alice, bob User, err error)
	Send(user User, ad, msg []byte) (ct []byte, err error)
	Receive(user User, ad, ct []byte) (msg []byte, err error)
}
*/



/*
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
            let (sender_input, receiver_input) = initall(Protocol::DV); //只有这里能用枚举
            let (mut _s_state, _ad, _iv):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&sender_input).unwrap();
            let (mut _r_state, _ad, _iv):(Vec<String>, [u8; 16], [u8; 16]) = serde_json::from_str(&receiver_input).unwrap();
            let (mut _new_s_state, ct) = dv::onion::send(&mut _s_state, &_ad, Vec::from(message), &_iv);
            let (mut _new_r_state, pt) = dv::onion::receive(&mut _r_state, &_ad, ct, &_iv);
            let msg = String::from_utf8(pt).unwrap();
            msg
        }

        Protocol::EtH => {
            let (sender_input, receiver_input) = initall(Protocol::EtH); //只有这里能用枚举
            let (_st_a, _iv): (State, [u8;16]) = serde_json::from_str(&sender_input).unwrap();
            let (_st_b, _iv): (State, [u8;16]) = serde_json::from_str(&receiver_input).unwrap();
            let (_new_st_a, ct) = EtH::send(_st_a, &_iv, message.as_bytes());
            let(_new_st_b, pt) = EtH::receive(_st_b, &_iv, ct.as_slice());
            let msg = String::from_utf8(pt).unwrap();
            msg
        }
    }
    send()
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

}*/