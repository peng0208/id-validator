#![allow(dead_code)]
mod check;
mod mock;
mod region;

pub use check::is_valid;
pub use mock::gen_code;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid() {
        let s = mock::gen_code();
        println!("{s}: {}", check::is_valid(&s))
    }

    #[test]
    fn gen_code() {
        println!("mock: {}", mock::gen_code())
    }
}
