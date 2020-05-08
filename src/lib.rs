#[macro_use]

pub mod board;
pub mod com;

#[cfg(test)]
mod tests {
    use crate::board::*;

    #[test]
    fn it_works() {
        let black: u64 = BIT_PATTERN::BLACK_INITIAL;
        let white: u64 = BIT_PATTERN::WHITE_INITIAL;
        assert_eq!((black | white).count_ones(), 4);
    }
}
