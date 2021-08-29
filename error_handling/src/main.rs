use core::panic;
// Docs: https://doc.rust-lang.org/stable/book/ch09-01-unrecoverable-errors-with-panic.html
use std::error::Error;
use std::fs;

use std::io;
use std::{
    fs::File,
    io::{ErrorKind, Read},
};

fn main() {
    // panic_backtrace();
    // recoverable_errors_with_result();
    // shortcuts_for_panic_on_error();
    // read_username_from_file();
    // read_username_from_file_shortcut();
    // read_username_from_file_even_shorter();
    read_username_from_file_even_even_shorter();

    // ? can only be used in functions that return Result or Option
    // or another type that implements std::ops::Try, main returns ()
    // let f = File::open("hello.txt")?;
}
// can also return Result
// The Box<dyn Error> type is called a trait object,
// which we’ll talk about in the
// “Using Trait Objects that Allow for Values of Different Types” section in Chapter 17.
// For now, you can read Box<dyn Error> to mean “any kind of error.”
// Using ? in a main function with this return type is allowed.
// fn main() -> Result<(), Box<dyn Error>> {
//     let f = File::open("hello.txt")?;

//     Ok(())
// }

fn panic_backtrace() {
    // panic!("crash and burn");
    // Using a panic! Backtrace
    // run "RUST_BACKTRACE=1 cargo run" to see all
    // the functions that have been called to get to this point
    let v = vec![1, 2, 3];
    v[99]; // panic occurs here :)
}

// What Result looks like
// enum Result<T, E> {
//     Ok(T), // T = Return type if succeeds
//     Err(E), // E = Return type if fails
// }

fn recoverable_errors_with_result() {
    // Returns result, because it could fail
    // T = std::fs::File, E = std::io::Error
    let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,                                              // Result::Ok
    //     Err(error) => panic!("Problem opening the file: {:?}", error), // Result::Err
    // };

    // Match error kind
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the  file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Better version without match -> Chapter 13
    //   let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
}

fn shortcuts_for_panic_on_error() {
    // if Result is Err variant, unwrap will call the pnaic! macro
    // let f = File::open("hello.txt").unwrap();
    // expect let's us specify the error message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// Move error to calling code (can be a lot shorter)
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // reads contents to s
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    // ? -> if value is Ok value will be returned to variable, if it is an Err,
    // the err will be returned from the whole function
    // ? implements the from function of the From trait (std library), which converts the error
    // received into the return type of the current function
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    // chaining both together
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_even_even_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Docs: https://doc.rust-lang.org/stable/book/ch09-03-to-panic-or-not-to-panic.html
pub struct Guess {
    value: i32, // value is private, therefore getter (value)
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
