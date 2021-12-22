use std::collections::HashMap;
use crypto::digest::Digest;
use crypto::sha2::Sha512;
use rand::Rng;
use crate::kukem;
use crate::signature::{generate, signature, verify};

const SEED_SIZE: i32 = 16;
const CHAINING_KEY_SIZE: usize = 16;
const SESSION_KEY_SIZE: usize = 16;

pub struct User {
    receiver: Receiver,
    sender: Sender,
    name: String,
}

pub struct Receiver {
    sk: HashMap<u8, Vec<u8>>,
    e0: u8,
    e1: u8,
    r: u8,
    l: HashMap<u8, Vec<u8>>,
    sgk: [u8;32],
    k: [u8;16],
    t: Vec<u8>,
}

pub struct Sender {
    pk: HashMap<u8, Vec<u8>>,
    e0: u8,
    e1: u8,
    s: u8,
    l: HashMap<u8, Vec<u8>>,
    vfk: [u8; 64],
    k: [u8;16],
    t: Vec<u8>,
}

pub fn init() -> (User,User) {
    //1. generate two sets of signature key pairs
    let (vfka, sgka) = generate();
    let (vfkb, sgkb) = generate();

    //2. generate two sets of key-updatable KEM key pairs.
    let mut seed = [0u8; 48];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut seed);

    let (pka, ska) = kukem::generate(seed);
    let (pkb, skb) = kukem::generate(seed);

    //3. Generate two chaining keys.
    let mut k_a: [u8; CHAINING_KEY_SIZE] = [0; CHAINING_KEY_SIZE];
    let mut k_b: [u8; CHAINING_KEY_SIZE] = [0; CHAINING_KEY_SIZE];
    rng.fill_bytes(&mut k_a);
    rng.fill_bytes(&mut k_b);

    //4. create sub-states for user a and b
    let mut sa_map = HashMap::new();
    sa_map.insert(0, pkb);
    let mut ls_map = HashMap::new();
    let s_a = Sender{
        pk: sa_map ,
        e0: 0, e1: 0,
        s: 0,
        l: ls_map,
        vfk: vfkb,
        k: k_b,
        t: vec![]
    };
    let mut ra_map = HashMap::new();
    ra_map.insert(0, ska);
    let mut lr_map = HashMap::new();
    let r_a = Receiver{
        sk: ra_map,
        e0: 0, e1: 0, r: 0,
        l: l_map,
        sgk: sgka,
        k: k_a,
        t: vec![]
    };

    let u_a = User{
        receiver: r_a,
        sender: s_a,
        name: "alice".to_string()
    };

    let mut sb_map = HashMap::new();
    sb_map.insert(0, pka);
    let mut ls_map = HashMap::new();
    let s_b = Sender{
        pk: sb_map,
        e0: 0, e1: 0,
        s: 0,
        l: ls_map,
        vfk: vfka,
        k: k_a,
        t: vec![]
    };
    let mut rb_map = HashMap::new();
    rb_map.insert(0, skb);
    let mut lr_map = HashMap::new();
    let r_b = Receiver{
        sk: rb_map,
        e0: 0, e1: 0, r: 0,
        l: lr_map,
        sgk: sgkb,
        k: k_a,
        t: vec![]
    };

    let u_b = User{
        receiver: r_b,
        sender: s_b,
        name: "bob".to_string()
    };
    (u_a, u_b)

}


pub fn send(mut user: User, ad: Vec<u8>) -> (Vec<u8>, Vec<Vec<u8>>) {
    let (vfks, sgks) = generate();
    let mut seed = [0u8; 48];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut seed);
    let (pks, sks) = kukem::generate(seed);
    user.receiver.e1 += 1;
    user.receiver.sk.insert(user.receiver.e1, sks);

    let mut ks: Vec<u8> = vec![];
    //let (c1, mut c2) = kukem::encrypt(user.sender.pk.get(&start).unwrap());
    //ks = ks.append(&mut c1);

    let mut start = user.sender.e0;
    let end = user.sender.e1;
    loop {
        let (c1, mut c2) = kukem::encrypt(user.sender.pk.get(&start).unwrap());
        //ks = ks.append(&mut c1);
        c = c.append(&mut c2);
        start += 1;
        if start >= end {
            break;
        }
    };
    let mut message = ad.clone();
    message.append(&mut c);
    let mut sig = signature(&message, &user.receiver.sgk);
    c = c.append(&mut sig);
    //user.receiver.l.get(&user.receiver.e1) = ad.append(c);
    user.receiver.sgk = sgks;


    user.sender.t.append(c);
    //let (ko, Ks, coins) = oracle(user.sender.k, ks, user.sender.t);

    let pk = kukem::generate(coins).0;
    let i = 0;
    loop {
        user.sender.pk.remove(&i); // pk[i] = nil
        if i >= user.sender.e1 {
            break;
        }
    }
    user.sender.pk.insert(user.sender.e1,pk);
    user.sender.e0 = user.sender.e1;
    user.sender.s += 1;
    user.sender.k = Ks;
    user.sender.l.insert(user.sender.s, c);

    (ko, c)
}

pub fn receive(mut user: User, mut ad: Vec<u8>, mut c: Vec<Vec<u8>>) -> Vec<u8> {
    let mut c_clone = c.clone();
    let mut ts:Vec<u8> = vec![];
    ts.append(&mut ad);
    loop {
        if c_clone.is_empty() {
        break;
        }
        let mut c_s = c_clone.pop().unwrap();
        ad.append(&mut c_s);
    }
    user.sender.t.append(&mut ts);
    let sig = c.get(c.len() - 1).unwrap();
    c.pop();
    let mut clone_c = c.clone();
    let mut ts:Vec<u8> = vec![];
    ts.append(&mut ad);
    loop {
        if clone_c.is_empty() {
            break;
        }
        let mut c_s = clone_c.pop().unwrap();
        ad.append(&mut c_s);
    }
    if verify(&ts, &user.sender.vfk, sig) {
        /** reference*/
        /** Todo: unwind the ciphertext and delete old ciphertext */
        /*
        let r = c.get(0).unwrap();
        let pks = c.get(1).unwrap();
        let vfk = c.get(2).unwrap();
        c.remove(0);
        c.remove(1);
        c.remove(2);

        */
        user.sender.e1 += 1;
        user.sender.pk.insert(user.sender.e1, pks);
        user.sender.vfk = vfk;

        /** Todo: Check that received epoch is still active and delte old ciphertexts */



        let mut ks: Vec<u8> = vec![];
        let mut i = user.receiver.e0;
        loop {
            if i > e {
                break;
            }
            let c = c.get(0).unwrap();
            //let mut k = kukem::decrypt(user.receiver.sk.get(&i).unwrap(), c);
            ks.append(&mut k);
            i+=1;
        }

        user.receiver.t.append(&mut ts);
        //let (ko, Kr, coins) = oracle(user.receiver.k, ks, user.receiver.t);
        let sk = kukem::generate(coins).1;

        /** Todo: delete old secret keys and update*/
        user.receiver.sk.insert(e, sk);
        let mut i = e + 1;
        loop {
            if i > user.receiver.e1 {
                break;
            }
            //let s = kukem::update_secret_key(user.receiver.sk.get(&mut i).unwrap(), ts)
            user.receiver.sk.insert(i ,s );
            i += 1;
        }
        user.receiver.e0 = e;
        user.receiver.r += 1;
        user.receiver.k = kr;
        ko


    } else {
        None
    }
    vec![]
}
pub fn oracle(K_s: [u8; 16], ks:[u8; 16], ts: [u8;16]) -> ([u8;16], [u8;16], [u8;16]){
    let mut hash_array = [0u8;48];
    for i in 48 {
        if i < 16 {
            hash_array[i] = K_s[i];
        }
        if i >= 16 && i < 32 {
            hash_array[i] = ks[i-16];
        }
        if i >= 32 && i < 48 {
            hash_array[i] = ks[i - 32];
        }
    };
    let mut hasher = Sha512::new();
    let mut hash_output: [u8; 64] = [0; 64];
    hasher.input(&hash_array);
    hasher.result(&mut hash_output);
    let ko = [0u8;16];
    for i in 16 {
       ko[i] = hash_output[i]
    };
    let Ks = [0u8;16];
    for i in 16 {
        Ks[i] = hash_output[i+16]
    };
    let coins = [0u8;16];
    for i in 16{
        coins[i] = hash_output[i+32];
    };
    (ko, Ks, coins)
}