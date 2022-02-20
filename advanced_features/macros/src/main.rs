// Docs: https://doc.rust-lang.org/stable/book/ch19-06-macros.html

// Fundamental, macros are a way of writing code that writes other code,
// which is known as metaprogramming.

// Declarative macros: macro_rules!
// Three types of procedural macros:
// - Custom: specify code added with the derive attribute used on structs and enums
// - Attribute-like: define custom attributes usable on any item
// - Function-like: look like function calls but operate on the tokens specified as their argument

// Macros can take a variable number of parameters (println!("{}",...)), functions don't,
// A macro can implement a trait on a given type, because the it is expnanded before the
// compiler intreprets the meaning of the code
// Functions can't, because it gets called at runtime and a trait needs to be implemented at compile time

// Macros must be defined or brought into scope before they are being called in a file,
// as opposed to functions that can be defined anywhere and called anywhere

use macros::HelloMacro;

// #[derive(HelloMacro)]
struct Pancakes;

// User needs to write implementation
impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}
fn main() {
    Pancakes::hello_macro();
}
