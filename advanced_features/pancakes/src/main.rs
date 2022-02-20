// Docs: https://doc.rust-lang.org/stable/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro

use macros::HelloMacro;
use macros_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
