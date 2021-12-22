use std::ops::Mul;
use ibe::kiltz_vahlis_one::{setup as SetUp};
use crate::util::*;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use irmaseal_curve::{G1Affine, G1Projective, G2Affine, G2Projective, Gt, pairing, Scalar};
use rand::{Rng, RngCore};
use sha2::digest::Update;
use sha2::{Digest, Sha512};
use subtle::{Choice, ConditionallySelectable, CtOption};
use crate::dss::generate;

const MAX_DEPTH: usize = 10;

pub struct Params {
    g: G1Affine,
    g1: G1Affine,
    g2: G1Affine,
    g3: G1Affine,
    h: [G1Affine;MAX_DEPTH]

}

pub struct Entity {
    id: Vec<Vec<u8>>,
    g: G1Affine,
    g3: G1Affine,
    h: [G1Affine;MAX_DEPTH],
    ao: G1Affine,
    a1: G1Affine,
    b: [u8; MAX_DEPTH]
}

pub struct Ciphertext {

}

//Setup establishes the public parameters and generates a root entity PKG.
pub fn setup(seed: [u8; 48]) {
    use core::ops::Mul;
    let mut rng = rand::thread_rng();
    let g = generate_g1(seed, 1);
    let alpha:u8 = rng.gen();
    let g1 = g.mul(alpha);
    let g2 = generate_g1(seed, 3);
    let g3 = generate_g1(seed,4);


    let r:u8 = rng.gen();
    let mut a0: G1Affine = generate_g1(g2.mul(alpha), 0);
    a0 = a0 + generate_g1(g3.mul(r), 0);
    A1: G1Affine = generate_g1(g.mul(r), 0);

    let mut h:[G1Affine;MAX_DEPTH] = [g; MAX_DEPTH];
    let mut b:[u8; MAX_DEPTH] = [0; MAX_DEPTH];
    let m = 0;
    loop {
        h[i] =  generate_g1(seed, i+6);
        b[i] = generate_g1(h[i].mul(r), 0);
        if m == MAX_DEPTH {
            break;
        }
    }
    let p =  Params{
        g,g1,g2,g3,h
    };
    let id = vec![vec![0u8]];
    let e = Entity{
        id, g, g3, h, ao, a1, b,
    };




}


pub fn extract(ancestor: Vec<u8>, id: Vec<u8>) -> Vec<u8> {
    vec![0]
}

pub fn encrypt(params: Vec<u8>, message: Vec<u8>, id: Vec<Vec<u8>>) {

}

pub fn decrypt(entity: Vec<u8>, c1: Vec<u8>, c2: Vec<u8>) {

}

pub fn hash_fun (mut buf: [u8; 48], mut i:usize) -> [u8; 48] {
    //let mut hasher = sha2::Sha512::default();
    let mut rng = rand::thread_rng();
    //let mut buf = [0u8;64];

    loop {
        rng.fill_bytes(&mut buf);
        i -= 1;
        if i <= 0 {
            break;
        }
    }
    buf
}

pub fn generate_g1 (mut buf:[u8; 48], mut i:usize) -> G1Affine {
    let gp = G1Projective::generator();
    let xs = hash_fun(buf, i);
    /** */
    //let x = Scalar::from_bytes_wide(&xs);
    let g: G1Affine = gp.mul(x).into();
    g
}
/*
pub fn generate_g2 (mut buf:[u8; 64], mut i: usize) -> G2Affine{
    let gp = G2Projective::generator();
    let xs = hash_fun(buf, i);
    let x = Scalar::from_bytes_wide(&xs);
    let g : G2Affine = gp.mul(x).into();
    g
}*/

pub struct ParamsPacket {
    g: [u8;48],
    g1: [u8;48],
    g2: [u8;48],
    g3: [u8;48],
    h: [[u8;48]; MAX_DEPTH]
}

pub fn marshal_bincode(p: Params) -> Vec<u8> {
    let len = p.h.len();
    let i = 0;
    h: [[u8;48]; MAX_DEPTH] = [[0; 48]; MAX_DEPTH];
    loop {
            h[i] = p.h[i].to_compressed();
        if i >= len {
            break;
        }
    }
    let packet = ParamsPacket {
        g: p.g.to_compressed(),
        g1: p.g1.to_compressed(),
        g2: p.g2.to_compressed(),
        g3: p.g3.to_compressed(),
        h,
    };
    bincode::serialize(&packet).unwrap()
}

pub fn unmarshal_bincode(p: Vec<u8>) {
    let packet:ParamsPacket = bincode::deserialize(&p).unwrap();
    let g = generate_g1(packet.g, 0);
    let g1 = generate_g1(packet.g1, 0);
    let g2 = generate_g1(packet.g2, 0);
    let g3 = generate_g1(packet.g3, 0);
    let len = packet.h.len();
    let i = 0;
    let mut h: [G1Affine; MAX_DEPTH] = [g; MAX_DEPTH];
    loop {
        h[i] = generate_g1(packet.g, 0);
        if i >= len {
            break;
        }
    }
    let p = Params{
        g, g1, g2, g3, h
    };
    //return p;

    //let
}