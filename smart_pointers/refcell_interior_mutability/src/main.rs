use std::{cell::RefCell, rc::Rc};
#[derive(Debug)]

// Docs: https://doc.rust-lang.org/stable/book/ch15-05-interior-mutability.html

// With references and Box<T>, the borrowing rules’ invariants are enforced at compile time.
// With RefCell<T>, these invariants are enforced at runtime.
// With references, if you break these rules, you’ll get a compiler error.
// With RefCell<T>, if you break these rules, your program will panic and exit.
// (single threaded -> multithreaded in chapter 16)

// Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
// Box<T> allows immutable or mutable borrows checked at compile time;
// Rc<T> allows only immutable borrows checked at compile time;
// RefCell<T> allows immutable or mutable borrows checked at runtime.
// Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value
// inside the RefCell<T> even when the RefCell<T> is immutable.

enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Docs: https://doc.rust-lang.org/stable/book/ch15-05-interior-mutability.html#having-multiple-owners-of-mutable-data-by-combining-rct-and-refcellt
    // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    // a, b and c have the modified value 15
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// Interior Mutability: A Mutable Borrow to an Immutable Value
fn does_not_compile() {
    let x = 5;
    // Cannot borrow immutable value as mutable
    // let y = &mut x;
}

// There are situations in which it would be useful for a value to mutate itself in its methods
// but appear immutable to other code.
