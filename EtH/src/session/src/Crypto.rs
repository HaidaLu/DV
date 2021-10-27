use async_std::prelude::*;

use byteorder::{BigEndian, ByteOrder};
use crypto_box::aead::{Aead, Payload};
use crypto_box::{ChaChaBox, SecretKey};
use generic_array::GenericArray;
use parking_lot::Mutex;
use sha2::Digest;
use std::sync::Arc;
use typenum::*;
use x25519_dalek::PublicKey;
use async_std::net::{Shutdown, TcpStream};
use sha2::{Sha256, Sha512, Digest};
use secp256k1::SecretKey;

#[derive(Clone)]
pub struct Crypto{
    stream: TcpStream,
    send_secret: Arc<Mutex<EncryptionRatchet<ChaChaBox>>>,
    rev_secret: Arc<Mutex<EncryptionRatchet<ChaChaBox>>>,
}

impl Crypto {
    /// Creates a new crypto stream from a given Tcp Stream and with a given secret
    pub async fn new(
        inner:TcpStream,
        public_key: &PublicKey,
        secret_key: &SecretKey,
    ) -> Result<Self> {
        let send_box = EncryptionRatchet::new(ChaChaBox::new(public_key, secret_key));
        let rev_box = EncryptionRatchet::new(ChaChaBox::new(public_key, secret_key));

        Ok(Self {
            stream: inner,
            send_secret: Arc::new(Mutex::new(send_box)),
            rev_secret: Arc::new(Mutex::new(rev_box)),
        })
    }
    // Sends a new event encrypted
    pub fn send(&mut self){
        let ciphertext = self.send_secret.lock().encrypt()?;
        let mut length_raw = [0u8; 8];
        BigEndian::write_u64(&mut length_raw, ciphertext.len() as u64);

        self.stream.write(&length_raw).await?;
        self.stream.write(&ciphertext).await?;
        self.stream.flush().await?;

        Ok(())
    }

    // Reads the ciphertext  Blocks until data is received
    pub async fn read(&mut self){
        let mut length_raw = [0u8; 8];
        self.stream.read_exact(&mut length_raw).await?;

        let length = BigEndian::read_u64(&length_raw);
        let mut ciphertext = vec![0u8; length as usize];
        self.stream.read(&mut ciphertext).await?;

        let plaintext = self.rev_secret.lock().decrypt(&ciphertext)?;

        Ok()
    }

    // Updates the keys in the inner encryption box
    pub fn update_key(&self) {
        self.send_secret = generate_hash(SecretKey::to_hex(send_secret));
        self.rev_secret = generate_hash(SecretKey::to_hex(rev_secret));
        self.send_secret.lock().swap_box(send_box);
        self.rev_secret.lock().swap_box(rev_box);

    }


}

pub struct EncryptionRatchet<T>
    where
        T: Aead,
{
    inner: T,
    counter: u128,
}

impl<T> EncryptionRatchet<T> where
    T: Aead,
{
    /// Creates a new encryption ratchet with the given inner value
    pub fn new(inner: T) -> Self {
        Self { inner, counter: 0 }
    }

    /// Swaps the crypto ratchet for a new one
    pub fn swap_box(&mut self, new_box: T) {
        self.inner = new_box;
    }
}

impl EncryptionRatchet<ChaChaBox> {
    /// Encrypts the given data by using the inner ChaCha box and nonce
    pub fn encrypt(&mut self, data: &[u8]) -> VentedResult<Vec<u8>> {
        let nonce = generate_nonce(self.counter);

        let ciphertext = self.inner.encrypt(
            &nonce,
            Payload {
                aad: &[],
                msg: data,
            },
        )?;
        self.counter += 1;

        Ok(ciphertext)
    }

    /// Decrypts the data by using the inner ChaCha box and nonce
    pub fn decrypt(&mut self, data: &[u8]) -> VentedResult<Vec<u8>> {
        let nonce = generate_nonce(self.counter);

        let plaintext = self.inner.decrypt(
            &nonce,
            Payload {
                msg: data,
                aad: &[],
            },
        )?;
        self.counter += 1;

        Ok(plaintext)
    }
}

/// Generates a nonce by hashing the input number which is the message counter
fn generate_nonce(number: u128) -> GenericArray<u8, U24> {
    let mut number_raw = [0u8; 16];
    BigEndian::write_u128(&mut number_raw, number);
    let result = sha2::Sha256::digest(&number_raw).to_vec();
    let mut nonce = [0u8; 24];
    nonce.copy_from_slice(&result[0..24]);

    nonce.into()
}

// Generate a new secretkey by hash function
fn generate_hash(input: &[u8]) -> SecretKey {
    // create a Sha256 object
    let mut hasher = Sha256::new();

// write input message
    hasher.update(input);

// read hash digest and consume hasher
    let result = hasher.finalize();

    SecretKey::parse(result).expect("Fail to generate the key")

}