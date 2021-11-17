extern crate crypto;
extern crate sha1;
mod state;
mod sarcad;
mod hash;
mod otae;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
