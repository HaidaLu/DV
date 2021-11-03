extern crate crypto;
mod bark;
mod onion;
mod asymmetric;
mod signature;
mod signcryption;



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
