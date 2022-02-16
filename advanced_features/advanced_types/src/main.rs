// Docs: https://doc.rust-lang.org/stable/book/ch19-04-advanced-types.html

// Newtypes, type aliases, the ! type and dynamically sized types

// Newtypes can also hide implementation details
// For example, we could provide a People type to wrap a HashMap<i32, String> that stores a person's ID
// associated with their name

// Creating Type Synonyms with Type Aliases
type Kilometers = i32;

// However, using this method, we don't get the type checking benefits that we get from the newtype pattern
// discussed earlier

// The main use case for type synonyms is to reduce repetition.
// Box<dyn Fn() + Send + 'static>

// let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

// fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
// --snip--
//}

//fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
// --snip--
//}

// Using Type Alias
// thunk is a word for code to be evaluated at a later time, so it's an appropriate
// name for a closure that gets stored
type Thunk = Box<dyn Fn() + Send + 'static>;

// let f: Thunk = Box::new(|| println!("hi"));

// fn takes_long_type(f: Thunk) {
// --snip--
//}

// fn returns_long_type() -> Thunk {
// --snip--
//}

// Good use of type aliases for Result type
// use std::fmt;
// use std::io::Error;
// pub trait Write {
// fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
// fn flush(&mut self) -> Result<(), Error>;
//
// fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
// fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
// }

// std::io has this type alias declaration
type Result<T> = std::result::Result<T, std::io::Error>;

// pub trait Write {
// fn write(&mut self, buf: &[u8]) -> Result<usize>;
// fn flush(&mut self) -> Result<()>;
//
// fn write_all(&mut self, buf: &[u8]) -> Result<()>;
// fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
// }

// The type alias makes code easier to write and it gives us a consistent interface across std::io
// Because it's an alias, we can use any methods that work on Result<T, E> on it, as well as special
// syntax like the ? operator

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    // Because Kilometers and i32 are the same type, we can add values of both types and we can pass
    // Kilometers values to functions that take i32 parameters
    println!("x + y = {}", x + y);
}
