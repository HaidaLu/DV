
use rand;
use std::ops::Sub;
use ibe::kiltz_vahlis_one::*;
use ibe::waters::*;
use num_primes::{ Generator,Verification, BigUint as biguint};
use num_bigint::{BigUint, BigInt, RandBigInt};
use num_traits::{Zero, One, ToPrimitive};
use sha2::{Digest, Sha512};
use bls12_381::*;
mod kudss;
pub mod dss;
mod big_array;
mod kupke;
mod sch;
mod fsdss;
mod hibe;
mod util;




#[test]
fn main () {

}
/*
let (a, b) = kudss::generate();
println!("{:?}", a);
println!("{:?}", b);

let (c, d) = kudss::generate();
println!("{:?}", c);
println!("{:?}", d);

let e = vec![1];
let mut d: KuDssPublicKey = bincode::deserialize(&c).unwrap();
//d.delta.append(e);

let mut f : Vec<Vec<u8>> = vec![];
f.push(e);
print!("{:?}", f);
*//*
    let mut m1:Vec<u8> = vec![0];
    let mut m2:Vec<u8> = vec![1, 2, 3, 4];
    m1.append(&mut m2);
    print!("{:?}", msg);

    let mut vec = vec![1, 2, 3];
    //println!("{:?}", vec)
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);
    print!("{:?}", vec);

    let mut vectest:Vec<Vec<u8>>;
    vectest.push(vec);
    vectest.push(m1);

    let c = vectest.get(1).unwrap();

    let test = vec![1, 2, 3, 5, 6, 1, 2];
    let size = test.len();
    let j = 7;
    for i in j {
        let m = test.get(i);
        println!("{}", m);
    }
    let p = Generator::new_uint(512 / 2);
    let q = Generator::new_uint(512 / 2);
    //println!("{}", p);
    //println!("{}", q);
    let a = num_primes::BigUint::new(vec![4]);
    let b = p % a;
    println!("{}", b);

    let mut p: BigUint;
    let mut q: BigUint;
    loop {
        p = Generator::new_uint(512 / 2);
        q = Generator::new_uint(512 / 2);

        let rp = p.clone() % BigUint::new(vec![4]);
        let rq = q.clone() % BigUint::new(vec![4]);
        if rp == BigUint::new(vec![3]) && rq == BigUint::new(vec![3]){
            break;
        }
    }*/
//println!("{}", p % BigUint::new(vec![4]));
//println!("{} ",q % BigUint::new(vec![4]));
//let N = p * q;
//let y:i32 = 2;
//let pow_y = y.pow(3);
//println!("{}", pow_y);



