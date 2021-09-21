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

pub fn add_two(a: i32) -> i32 {
    a + 3
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}
pub struct Guess2 {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 || value > 100 {
        //     panic!("Guess value must be between 1 and 100, got {}.", value);
        // }

        Guess { value }
    }
}

impl Guess2 {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
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
    // assert! macro only indicates that it got the value false, but not what values it got
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // ! Adding Custom Failure messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting dis not contain name, value was '{}'",
            result
        );
    }

    // ! Checking for Panics with should_panic
    // To ensure that no guesses above 100 can be created
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // ! But what if the code paniced for a different reason than the one we expected?
    // expected = ... is a substring of the panic
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_2() {
        Guess2::new(200);
    }

    // ! Using Result<T, E> in Tests
    // Use ? operator in tests to catch Err variants
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equeal four"))
        }
    }
}

// ! Controlling How Tests Are Run
// cargo test --help -> for cargo test
// cargo test -- --help -> for binary
// cargo test -- --test-threads=1 -> limit number of threads, for example if they could interfer with each other
// cargo test -- --show-output -> Showing function output
// cargo test NAME_OF_THE_TEST_FUNCTION -> Run a single test
// cargo test PART_OF_THE_TESTS_FUNCTION_NAME -> Run multiple tests which match the given name partly (or completely)
// #[ignore] to ignore a test (above function)
// cargo test -- --ignored -> only run ignored tests
