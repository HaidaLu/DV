use serde::{Serialize,Deserialize};
/** This module defines a State instance
   State (hk, sk, rk)
 */
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct State {
    pub hk: [u8; 32],
    pub sk: [u8; 32],
    pub rk: [u8; 32],
}

pub fn update_sk(mut st: State, new_sk: [u8; 32]) -> State {
    st.sk = new_sk;
    st
}

pub fn update_rk(mut st: State, new_rk: [u8; 32]) -> State {
    st.rk = new_rk;
    st
}

