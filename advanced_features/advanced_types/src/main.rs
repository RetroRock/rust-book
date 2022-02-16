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

// The Never Type that Never Returns
// Rust has a special type named ! that's known in type theory lingo as the empty type because it has no values
// It stands in the place of the return type when a function will never return
// Functions that return never are called diverging functions

// fn bar() -> ! {
//   --snip--
// }

// Real world example of Never Type

// This doesn't work, guess can only have one type (string or integer)
// let guess = match guess.trim().parse() {
//  Ok(_) => 5,
//  Err(_) => "hello",
// };

// This works, because continue has a ! value
// Rust looks at both match arms, the former with a value of u32 and the latter with a ! value. Because ! can never have
// a value, Rust decides that the type of guess is u32, continue moves control back to the top of the loop, so in the Err
// case, we never assing a value to guess
// let guess: u32 = match guess.trim().parse() {
// Ok(num) => num,
// Err(_) => continue,
// };

// The formal way of describing this behavior is that expressions of type ! can be coerced into any other type
// The never type is useful with the panic! macro as well
// This code works because panic! doesn't produce a value
// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value!"),
//         }
//     }
// }

// Dynamically Sized Types and the Sized Trait (DST or unsized types)
// For example we can only know the size of str at compile time, the size of &str is known at compile time
// let s1: str = "Hello there!"; // doesn't work
// let s2: str = "How's it going?"; // doesn't work

// If Rust allowed us tto write this code, these two str values would need to take up the same amount of space.
// but they have different lengths: s1 needs 12 bytes of storage and s2 needs 15
// This is why it's not possible to create a variable holding a dynamically sized type

// Although &T is a single value that sotres the memory address of wher teh T is located, a &str is two values:
// the address of the str and it's length (twize the size of a usize -> we know size at compile time no matter how
// how long the string it refers to is)
// That is the general way in which dynamically sized types are used in Rust
// The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind
// For example Box<str> or Rc<str> work as well
// Every trait is a dynamically sized type we can refer to by using the name of the trait
// To use traits as trait objects, we must put them behind a pointer, such as &dyn Trait, Box<dyn Trait> or Rc<dyn Trait>
// Trait objects in chapter 17: https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types

// To work wit DSTs, Rust has a particular triat called the Sized trait to determine wheter or not a type's size is known at compile time.
// This trait is automatically implemented for everything whose size is known at compile time.
// In addition, Rust implicitly adds a bound on Sized to every generic function:
// fn generic<T>(t: T) {
// --snip--
// }
// becomes
fn generic1<T: Sized>(t: T) {
    // --snip--
}

// By default, generic functions wil work only on types that have a known size at compile time.
// We can use the following syntax to relax this restriction
fn generic2<T: ?Sized>(t: &T) {
    // --snip--
}
// A trait bound on Sized mean "T may or may not be Sized" and this notaiton overrides the default that generic type smust have
// a known size at compile time. The ?Trait syntax with this meaning is only available for Sized
// We switched the t parameter type from T to &T because the type might not be Sized -> we need to use it behind osme kind of pointer
// For example a reference

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    // Because Kilometers and i32 are the same type, we can add values of both types and we can pass
    // Kilometers values to functions that take i32 parameters
    println!("x + y = {}", x + y);

    // One final expression that has the type ! type is a loop
    // However, this wouldn't be true if we included a break,
    // because the loop would terminate when it got to the break
    print!("forever ");

    loop {
        print!("and ever");
    }
}
