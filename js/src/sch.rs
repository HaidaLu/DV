use crate::kudss;
use crate::kupke;


const hashingKeySize: usize = 16;
//User designates a participant in the protocol that can both send and receive messages.
//It has to be passed as an argument to both the send and receive routines
pub struct User {
    //vk and sk are the kuDSS public/private key pair
    vk : Vec<u8>,
    sk : Vec<u8>,
    ek : Vec<u8>, // ek is the kuPKE public key
    dk : Vec<u8> , // dk is an array of kuPKE private keys
    hk : [u8;16], //hk is the hashing key
    tau : Vec<u8>, // tau is the latest hash ciphertext
    t : Vec<Vec<u8>>, // t is the communication transcript
    //s, r and ack are the send, receive and acknowledge counters
    s : i8,
    r : i8,
    ack : i8,
}

// message bundles the ciphertext, the signature and auxiliary update data and is the object
// sent from one user to another.
pub struct Message {
    c : Vec<u8>, //c is the ciphertext
    sig : Vec<u8>, //sig is the message signature
    aux : Aux,
    l : Vec<u8>,
}

//Aux bundles signed auxiliary data that is sent alongside a ciphertext
pub struct Aux {
    vk : Vec<u8>,
    ek : Vec<u8>,
    ad : Vec<u8>,
    tau : Vec<u8>,
    t : Vec<u8>,
    s : i8,
    r : i8,
}

//Init creates and returns two User objects which can communicate with each other.
//Note that in case of an error during a send or receiver operation bth user states are
//considered corrupt thus requiring a fresh protocol initialization in order to resume communicating
pub fn init() -> (User, User) {
    let (vkb, ska) = kudss::generate();
    let (vka, skb) = kudss::generate();

    let (eka, dkb) = kupke::generate();
    let (ekb, dka) = kupke::generate();

    //初始化
    let hk: [u8; hashingKeySize] = [];
    //var hk [hashingKeySize]byte
    //if _, err := rand.Read(hk[:]); err != nil {
    //    return nil, nil, err
    //}

    let ua = User{
        vk: vka, sk: ska, ek: eka, dk:vec![dka], hk,
        //tau: nil,
        // t: [][]byte{nil},
        s: 0, r: 0, ack: 0,
    };

    let ub = User{
        vk: vkb, sk: skb, ek: ekb, dk: vec![dkb], hk,
        //tau: nil,
        // t: [][]byte{nil},
        s: 0, r: 0, ack: 0,
    };
    (ua, ub)
}

// Send encrypts and signs a given plaintext and associated data. It further advances
// the sender state one step forward (ratchet). The function returns a message object
// that contains the ciphertext, auxiliary data and a signature.
pub fn send(mut user: User, ad:Vec<u8>, pt: Vec<u8>) -> Vec<u8> {
    user.s += 1;

    let (vks, sks) = kudss::generate();
    let (eks, mut dks) = kupke::generate();

    user.dk.append(&mut dks);

    // Auxiliary data is both included in bth marshalled and unmarshalled form
    // in the message  sent such that the receiver only has to perform a single unmarshal operation
    let aux = Aux{
        vk: vks, ek: eks,
        ad, tau: user.tau, t: user.t.get(user.s - 1),
        s: user.s, r: user.r,
    };
    let l = bincode::serialize(&aux).unwrap();

    let uek = user.ek;

    //for i := user.ack + 1; i < user.s; i++ {
    //    uek, err = s.kuPKE.updatePublicKey(uek, user.t[i])
    //    if err != nil {
    //        return nil, errors.Wrap(err, "unable to update ku-pke public key")
    //    }
    //}

    let c = kupke::encrypt(uek,pt);


    let sig = kudss::sign(user.sk, c.append(l));
    let msg = bincode::serialize(&Message{
        c,
        sig,
        aux,
        l,
    }).unwrap();

    user.t.append(); //user.t = append(user.t, primitives.Digest(sha256.New(), user.hk, msg))
    user.sk = sks;

    msg
}

// Receive decrypts a given message consisting of the actual ciphertext, signed auxiliary
// data and a signature. A receive operation advances the receiver state of a user one
// step forward (ratchet). The function returns a decrypted plaintext.
pub fn receive(mut user: User, ad: Vec<u8>, ct: Vec<u8>) -> Vec<u8> {
    let msg: Message = bincode::deserialize(&ct).unwrap();

    /*

	if msg.Aux.S != user.r+1 {
		return nil, errors.New("sent/receive counters are out-of-sync")
	} else if !bytes.Equal(msg.Aux.Tau, user.t[msg.Aux.R]) || !bytes.Equal(msg.Aux.T, user.tau) {
		return nil, errors.New("sender/receiver transcripts are out-of-sync")
	} else if !bytes.Equal(msg.Aux.Ad, ad) {
		return nil, errors.New("local and received associated data does not match")
	}
    */

    let uvk = user.vk;
    /*
    for i := user.ack + 1; i <= msg.Aux.R; i++ {
		uvk, _ = s.kuDSS.updatePublicKey(uvk, user.t[i])
	}
	if err := s.kuDSS.verify(uvk, append(msg.C, msg.L...), msg.Sig); err != nil {
		return nil, errors.Wrap(err, "unable to verify signature")
	}
    */

    user.r += 1;
    user.ack = msg.aux.r;

    let pt = kupke::decrypt(user.dk.get(user.ack), msg.c);

    for i in user.ack {
        // user.t[i] = nil,
        // user.dk[i] = nil
    }

    //user.tau =
    //    user.tau = primitives.Digest(sha256.New(), user.hk, ct)

    sks = kudss::update_private_key(user.sk, user.tau);
    /*
    for i := user.ack; i <= user.s; i++ {
		user.dk[i], err = s.kuPKE.updatePrivateKey(user.dk[i], user.tau)
		if err != nil {
			return nil, errors.Wrap(err, "unable to udpate ku-pke private key")
		}
	}
    */

    user.sk = sks;
    user.vk = msg.aux.vk;
    user.ek = msg.aux.ek;

    pt
}



