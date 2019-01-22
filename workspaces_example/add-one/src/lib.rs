
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod add_one_tests {

    use super::*;

    #[test]
    fn it_adds_one_to_positive() {
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn it_adds_one_to_negative() {
        assert_eq!(-3, add_one(-4));
    }
}
