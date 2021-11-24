/** This module defines a State instance
    State (hk, sk, rk)
 */

pub struct state {
 pub hk: [u8; 32],
 pub sk: [u8; 32],
 pub rk: [u8; 32],
}


pub fn update_sk(mut st: state, new_sk: &[u8; 32]) -> state {
  st.sk = **&new_sk;
  st
}

 pub fn update_rk(mut st: state, new_rk: [u8; 32]) -> state {
  st.rk = *&new_rk;
  st
 }

