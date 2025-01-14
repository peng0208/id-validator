#![allow(dead_code)]
mod check;
mod mock;
mod region;

pub use check::get_info;
pub use check::is_valid;
pub use mock::gen_code;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid() {
        let s = gen_code();
        assert!(check::is_valid(&s));
    }

    #[test]
    fn is_valid2() {
        assert_eq!(check::is_valid("320706199909015782"), false);
    }

    #[test]
    fn get_info() {
        println!("{:?}", check::get_info("320706199909015782"));
    }
}
