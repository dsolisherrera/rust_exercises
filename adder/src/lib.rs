
mod shapes{

    #[derive(Debug)]
    pub struct Rectangle {
        pub length: u32,
        pub width: u32,
    }

    impl Rectangle {
        pub fn new(width_in: u32, length_in: u32) -> Self{
            Self {width: width_in,
                length: length_in
            }
        }

        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
    }
}

#[cfg(test)]
mod tests {
    use super::shapes::Rectangle;

    #[test]
    fn bigger_rectagle_contains_smaller() {
        let bigger = Rectangle::new(200,300);
        let smaller = Rectangle::new(20,30);
        assert!(bigger.can_hold(&smaller),
        "bigger ({:?}) should contain smaller ({:?})", bigger, smaller);
    }

    #[test]
    fn smaller_rectagle_cannot_contain_bigger() {
        let bigger = Rectangle::new(200,300);
        let smaller = Rectangle::new(20,30);
        assert!(!smaller.can_hold(&bigger),
        "Smaller ({:?}) should not contain bigger ({:?})", smaller, bigger);
    }
}
