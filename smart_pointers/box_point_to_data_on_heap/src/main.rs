// Docs: https://doc.rust-lang.org/stable/book/ch15-01-box.html

// Boxes allow you to store data on the heap rather than the stack.
// What remains on the stack is the pointer to the heap data.

// Each item in a cons list contains two elements:
// The value of the current item and the next item.
// The last item in the list contains only a value called Nil without a next item.
// The cons list is produced by recursively calling the cons function.
// The canonical name to denote the base case of the recursion is Nil

// Boxes provide only the indirection and heap allocation
// They don’t have any other special capabilities, like other smart pointer types.

// The Box<T> type is a smart pointer because it implements the Deref trait,
// which allows Box<T> values to be treated like references. When a Box<T> value goes out of scope,
// the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation.

// Error: Recurse type has infinite size
enum List_Infinite_Size {
    Cons(i32, List),
    Nil,
}

/* Usage:
fn use_cons() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
} */

// Using Box<T> to Get A Recursive Type with a Known Size
// Fix: Do not store value directly, but pointer to that value

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
