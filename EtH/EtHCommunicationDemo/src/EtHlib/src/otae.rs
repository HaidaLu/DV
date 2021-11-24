/** This module provide three methods
    1. to derive a key pair
    2. to encrypt the plaintext using aes256
    3. to decrypt the ciphertext*/

use std::str;
use crypto::{symmetriccipher,buffer,aes,blockmodes};
use crypto::buffer::{ReadBuffer,WriteBuffer,BufferResult};
use crypto::aessafe::*;
use crypto::blockmodes::*;
use crypto::symmetriccipher::*;
use rand::{Rng,OsRng};
//mod KeyPair;
pub struct key_pair {
 sk: [u8; 32],
 rk: [u8; 32]
}
pub fn derive_key_pair() -> key_pair {
  let mut sender_key: [u8; 32] = [0; 32];
  let mut receive_key: [u8; 32] = [0; 32];

  let mut s_rng = OsRng::new().ok().unwrap();
  s_rng.fill_bytes(&mut sender_key);
  let mut r_rng = OsRng::new().ok().unwrap();
  r_rng.fill_bytes(&mut receive_key);

  let k_pair = key_pair {
   sk: sender_key,
   rk: receive_key,
  };

 k_pair

}
// Encrypt a buffer with the given key and iv(ad) using AES-256/CBC encryption
pub fn encrypt(sk: &[u8], ad: &[u8], pt: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
 let mut encryptor = aes::cbc_encryptor(
  aes::KeySize::KeySize256,
  sk,
  ad,
  blockmodes::PkcsPadding);

 let mut final_result = Vec::<u8>::new();
 let mut read_buffer = buffer::RefReadBuffer::new(data);
 let mut buffer = [0;4096];
 let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

 loop {
  let result=try!(encryptor.encrypt(&mut read_buffer,&mut write_buffer,true));

  final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

  match result {
   BufferResult::BufferUnderflow=>break,
   BufferResult::BufferOverflow=>{},
  }
 }
 Ok(final_result)
}


//Decrypts a buffer with the given key and ad using AES-256/CBC/Pkcs encryption.
pub fn decrypt(rk: &[u8], ad: &[u8], ct: &[u8]) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
 let mut decryptor = aes::cbc_decryptor(
  aes::KeySize::KeySize256,
  rk,
  ad,
  blockmodes::PkcsPadding);

 let mut final_result = Vec::<u8>::new();
 let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
 let mut buffer = [0; 4096];
 let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

 loop {
  let result = try!(decryptor.decrypt(&mut read_buffer, &mut write_buffer, true));
  final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
  match result {
   BufferResult::BufferUnderflow => break,
   BufferResult::BufferOverflow => { }
  }
 }

 Ok(final_result)

}
