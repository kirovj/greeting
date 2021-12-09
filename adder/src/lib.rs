#![allow(unused)]

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_hold_test() {
        let a = Rectangle {
            width: 8,
            height: 7,
        };

        let b = Rectangle {
            width: 5,
            height: 6,
        };

        assert!(a.can_hold(&b));
        assert!(!b.can_hold(&a))
    }
}
