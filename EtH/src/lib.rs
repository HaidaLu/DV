mod sarcad;
mod state;

pub use crate::state::*;
pub use crate::sarcad::*;
pub use primitives::*;
use bincode;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
