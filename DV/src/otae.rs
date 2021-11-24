/** privide primitives for symmetric encryption/decryption
 */

use std::str;
use crypto::{symmetriccipher,buffer,aes,blockmodes};
use crypto::buffer::{ReadBuffer,WriteBuffer,BufferResult};
use crypto::aessafe::*;
use crypto::blockmodes::*;
use crypto::symmetriccipher::*;
use rand::{Rng,OsRng};



#[derive(Debug)]
pub struct KeyPair {
 pub sk: [u8; 32],
 pub rk: [u8; 32]
}


/**derive a secret key pair*/
pub fn derive_key_pair() -> KeyPair {
 let mut sender_key: [u8; 32] = [0; 32];
 let mut receive_key: [u8; 32] = [0; 32];

 let mut s_rng = OsRng::new().ok().unwrap();
 s_rng.fill_bytes(&mut sender_key);
 let mut r_rng = OsRng::new().ok().unwrap();
 r_rng.fill_bytes(&mut receive_key);

 let k_pair = KeyPair {
  sk: sender_key,
  rk: receive_key,
 };
 k_pair
}

/** Encrypt a buffer with the given key and iv(ad) using AES-256/CBC encryption*/
pub fn encrypt_aes_256_cbc(plaintext: &[u8], key: &[u8], ad: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
 let mut encryptor=aes::cbc_encryptor(
  aes::KeySize::KeySize256,
  key,
  ad,
  blockmodes::PkcsPadding);

 let mut final_result=Vec::<u8>::new();
 let mut read_buffer=buffer::RefReadBuffer::new(plaintext);
 let mut buffer=[0;4096];
 let mut write_buffer=buffer::RefWriteBuffer::new(&mut buffer);

 loop{
  let result= encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;

  final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

  match result {
   BufferResult::BufferUnderflow=>break,
   BufferResult::BufferOverflow=>{},
  }
 }

 Ok(final_result)
}


/** Decrypt the cipher with the given key and iv(ad) using AES-256/CBC encryption*/
pub fn decrypt_aes_256_cbc(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
 let mut decryptor = aes::cbc_decryptor(
  aes::KeySize::KeySize256,
  key,
  iv,
  blockmodes::PkcsPadding);
 let mut final_result = Vec::<u8>::new();
 let mut read_buffer = buffer::RefReadBuffer::new(ciphertext);
 let mut buffer = [0; 4096];
 let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

 loop {
  let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
  final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
  match result {
   BufferResult::BufferUnderflow => break,
   BufferResult::BufferOverflow => { }
  }
 }
 Ok(final_result)
}


/** Encrypt a buffer with the given key and iv(ad) using AES-256/CTR encryption*/
pub fn encrypt_aes_256_ctr(plaintext: &[u8], key: &[u8], ad: &[u8]) ->Result<Vec<u8>,symmetriccipher::SymmetricCipherError>{
 let mut final_result=Vec::<u8>::new();
 let mut read_buffer=buffer::RefReadBuffer::new(plaintext);
 let mut buffer=[0;4096];
 let mut write_buffer=buffer::RefWriteBuffer::new(&mut buffer);

 let mut encoder=CtrMode::new(AesSafe256Encryptor::new(key), ad.to_vec());
 encoder.encrypt(&mut read_buffer,&mut write_buffer,true)?;

 final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
 Ok(final_result)
}

/** Decrypt the cipher with the given key and iv(ad) using AES-256/CTR encryption*/
pub fn decrypt_aes_256_ctr(ciphertext: &[u8], key: &[u8], ad: &[u8]) ->Result<Vec<u8>,symmetriccipher::SymmetricCipherError>{
 let mut final_result = Vec::<u8>::new();
 let mut read_buffer = buffer::RefReadBuffer::new(ciphertext);
 let mut buffer = [0; 4096];
 let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

 let mut decoder=CtrMode::new(AesSafe256Encryptor::new(key), ad.to_vec());
 decoder.decrypt(&mut read_buffer,&mut write_buffer,true)?;

 final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
 Ok(final_result)
}
