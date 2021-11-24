extern crate crypto;
mod bark;
mod uni;
mod pkc;
mod signcryption;
mod otae;
mod onion;
mod hash;
mod dss;
pub use crate::dss::*;
pub use crate::signcryption::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
