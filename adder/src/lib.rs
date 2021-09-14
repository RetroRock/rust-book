// Docs: https://doc.rust-lang.org/stable/book/ch11-01-writing-tests.html
// Projected created via 'cargo new adder --lib'

struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    // Visibility rules of mods -> bring outer code into inner scope
    use super::*;

    // tells Rust, that this is a test function
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let smaller = Rectangle {
            width: 8,
            height: 7,
        };

        let larger = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!larger.can_hold(&smaller));
    }

    // ! Testing Equality with the assert_eq! and assert_ne! Macros
}
