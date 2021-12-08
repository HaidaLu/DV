mod hash;
mod otae;
pub use crate::hash::*;
pub use crate::otae::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
