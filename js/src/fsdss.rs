

use std::io::stderr;
//use num::{BigInt, BigUint};
use num_primes::{ Generator, BigUint};
use num_bigint::{ToBigInt, RandBigInt, BigUint as bigUint, Sign, BigInt};
use num_traits::ToPrimitive;
use sha2::{ Sha512, Digest};
use sha2::digest::Update;

const BELLARE_SECURITY: u64 = 512; // k: security parameter in bits
const BELLARE_NUM_POINTS : usize = 10; // l: number of points in the keys
const BELLARE_MAX_PERIOD: u32 = 1000; // t: maximum value of allowed key evolutions

//bellarePublicKey bundles the public key material.
pub struct BellarePublicKey {
     //n []byte
     //u [bellare_num_points][] byte
    n : Vec<u8>,
    u : Vec<Vec<u8>>
}

// BellarePrivateKey bundles the secret key material.
//SK{j-1} = (N, T, j-1, S{1,j-1}, ..., S{l, j-1}
pub struct BellarePrivateKey {
    //N []byte
    n : Vec<u8>,
    //S [bellareNumPoints][]byte
    s : Vec<Vec<u8>>,
    //S [][]byte
    j : i8, // J specifies the current period of this private key.
}

// bellareSignature bundles signature material.
pub struct BellareSignature {
    Y: Vec<u8>,
    Z: Vec<u8>,
    j: i8,
}

pub fn generate() -> (Vec<u8>, Vec<u8>) {

    //let mut rng = rand::thread_rng();
    //let a = rng.gen_bigint(1000);
    let mut p: BigUint;
    let mut q: BigUint;
    loop {

        p = Generator::new_prime((BELLARE_SECURITY / 2) as usize);
        q = Generator::new_prime((BELLARE_SECURITY / 2) as usize);


        let rp = p.clone() % BigUint::new(vec![4]);
        let rq = q.clone() % BigUint::new(vec![4]);
        if rp == BigUint::new(vec![3]) && rq == BigUint::new(vec![3]){
            break;
        }
    }


    let N = p * q;
    let mut S = Vec::new();
    let mut U = Vec::new();
    for i in BELLARE_NUM_POINTS {
        let mut s: BigUint;
        loop {
            s = Generator::new_uint(N.bits());
            //s =
            if s != BigUint::new(vec![0]) {
                break;
            }
        }
        S.push(s.clone().to_bytes_be());
        let e_a = bigUint::new( vec![2]);
        let e_b = bigUint::new(vec![BELLARE_MAX_PERIOD+1]);
        let e_c = e_a.pow(e_b.to_u32_digits().pop().unwrap()).to_bytes_be();
        let e = BigUint::from_bytes_be(&e_c);
        //let e = BigUint::new(vec![2]).modpow(&BigUint::new(vec![BELLARE_MAX_PERIOD+1]), &BigUint::new(vec![0]));
        let u = BigUint::new(s.modpow(&e, &N).to_u32_digits());
        U.push(u.to_bytes_be())
    }
    let bpk = BellarePublicKey {
        n: N.clone().to_bytes_be(),
        u: U
    };
    let bsk = BellarePrivateKey {
        n: N.clone().to_bytes_be(),
        s: S,
        j: 0,
    };
    let pk = bincode::serialize(&bpk).unwrap();
    let sk = bincode::serialize(&bsk).unwrap();
    (pk, sk)
}

// update evolives a private key into a new period
pub fn update(sk: Vec<u8>) -> Vec<u8> {
    let private: BellarePrivateKey = bincode::deserialize(&sk).unwrap();
    if private.j > BELLARE_MAX_PERIOD as i8 {
        //return 错误处理
    }
    let n = BigUint::from_bytes_be(&private.n);

    //新的S是一个S[l][j]二维数组
    let mut S : Vec<Vec<u8>> = vec![];
    for i in BELLARE_NUM_POINTS { // for i = 1 ... l

        //S{i,j} = S{i,j}^2 mod N
        let s = BigUint::from_bytes_be(&private.s.get(i));
        let new_s = BigUint::new(s.modpow(&BigUint::from(vec![2]), &n).to_u32_digits()).to_bytes_be();
        S.push(new_s);
    }
    // SKj = (N, T, j, S{1,j}...S[l,j})
    let new_private_key = BellarePrivateKey {
        n: private.n,
        s: S,
        j: private.j + 1
    };
    bincode::serialize(&new_private_key).unwrap()
}

//Sign creates a Bellare signature of a givevn message
pub fn sign(sk: Vec<u8>, msg: Vec<u8>) -> Vec<u8> {
    let private: BellarePrivateKey = bincode::deserialize(&sk).unwrap();
    let _n = BigUint::from_bytes_be(&private.n).bits();

    let r: BigUint;
    loop {
        r = Generator::new_uint(_n);
        if r.to_i64().unwrap() != 0 {       //  要求n小于64？？可能吗
            break;
        }
    }
    /*
    e := new(big.Int).Exp(big.NewInt(2), big.NewInt(bellareMaxPeriod+1-int64(private.J)), nil)
	Y := new(big.Int).Exp(R, e, n)
    */


    // Y = R^{2^(T+1-j)} mod N
    let e_a = bigUint::new( vec![2]);
    let pj =  private.j.to_i64().unwrap();
    let e_b = bigUint::new(vec![BELLARE_MAX_PERIOD + 1 - pj]);
    let e_c = e_a.pow(e_b.to_u32_digits().pop().unwrap()).to_bytes_be();
    let e = BigUint::from_bytes_be(&e_c);
    let y = BigUint::new(r.modpow(&r, &e).to_u32_digits());

    //digest := primitives.Digest(sha512.New(), []byte(strconv.Itoa(signature.J)), y.Bytes(), msg)
    let mut input : Vec<u8> = msg;
    input.push(private.j as u8);
    input.push(y.to_bytes_be().pop().unwrap());
    //hasher accept bytes
    let mut hasher = Sha512::new();
    hasher.update(&input);
    let digest = hasher.finalize().as_slice();

    let c = BigUint::from_bytes_be(digest);

    let mut p = BigUint::new(vec![1]);
    for i in BELLARE_NUM_POINTS {
        //e := new(big.Int).And(new(big.Int).Rsh(c, uint(i)), big.NewInt(1))
        let rsh = &c >> i;
        let e = rsh & BigUint::new(vec![1]);

        let s = BigUint::from_bytes_be(&private.s.get(i));
        // new(big.Int).Exp(s, e, nil)
        let s_a = s.to_bytes_be();
        let convert_s = bigUint::from_bytes_be(&s_a);
        let e_a = e.to_bytes_be();
        let convert_e = bigUint::from_bytes_be(&e_a);

        let mul = convert_s.pow(convert_e.to_u32_digits().pop().unwrap()).to_bytes_be();
        let _mul = BigUint::from_bytes_be(&mul);
        p = p * _mul;

    }

    p = r * p;
    let z = p * _n;

    let signature = BellareSignature {
        Y: y.to_bytes_be(),
        Z: z.to_bytes_be(),
        j: private.j,
    };
    bincode::serialize(&signature).unwrap()
}

//Verify checks the validity of a given signature
pub fn verify(pk: Vec<u8>, msg: Vec<u8>, sig: Vec<u8>) {
    let public: BellarePublicKey = bincode::deserialize(&pk).unwrap();
    let signature: BellareSignature = bincode::deserialize(&sig).unwrap();

    let y = BigUint::from_bytes_be(&signature.Y).to_bytes_be();
    let z = BigUint::from_bytes_be(&signature.Z).to_bytes_be();
    let n = BigUint::from_bytes_be(&public.n).to_bytes_be();

    //digest := primitives.Digest(sha512.New(), []byte(strconv.Itoa(signature.J)), y.Bytes(), msg)
    let mut input : Vec<u8> = msg;
    input.push(signature.j as u8);
    input.push(y.to_bytes_be().pop().unwrap());
    //hasher accept bytes
    let mut hasher = Sha512::new();
    hasher.update(&input);
    let digest = hasher.finalize().as_slice();


    let c = BigUint::from_bytes_be(&digest);
    //e := new(big.Int).Exp(big.NewInt(2), big.NewInt(bellareMaxPeriod+1-int64(signature.J)), nil)
    let e_a = bigUint::new( vec![2]);
    let pj =  signature.j as i64;
    let e_b = bigUint::new(vec![BELLARE_MAX_PERIOD+1 - pj]);
    let e_c = e_a.pow(e_b.to_u32_digits().pop().unwrap()).to_bytes_be();
    let e = BigUint::from_bytes_be(&e_c);
    //L := new(big.Int).Exp(z, e, n)
    let l = BigUint::new(z.modpow(&e, &n).to_u32_digits());

    let mut p = BigUint::new(vec![1]);
    for i in BELLARE_NUM_POINTS {
        //e := new(big.Int).And(new(big.Int).Rsh(c, uint(i)), big.NewInt(1))
        let rsh = &c >> i;
        let e = rsh & BigUint::new(vec![1]);
        let u = BigUint::from_bytes_be(&public.u.get(i));
        // new(big.Int).Exp(s, e, nil)
        let u_a = u.clone().to_bytes_be();
        let convert_u = bigUint::from_bytes_be(&u_a);
        let e_a = e.clone().to_bytes_be();
        let convert_e = bigUint::from_bytes_be(&e_a);

        let mul = convert_u.pow(convert_e.to_u32_digits().pop().unwrap()).to_bytes_be();
        let _mul = BigUint::from_bytes_be(&mul);
        p = p * _mul;
    }
    p = y * p;
    let r = p * BigUint::from_bytes_be(&n);

    if l.cmp(&r) != 0{
        //return err("unable to verify signature");
    }
}

