
pub fn add_two(x: i32) -> i32 {
    x + 2
}


#[cfg(test)]
mod add_two_tests {

    use super::*;

    #[test]
    fn it_adds_two_to_positive() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_adds_two_to_negative() {
        assert_eq!(0, add_two(-2));
    }
}