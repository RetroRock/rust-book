// Docs: https://doc.rust-lang.org/stable/book/ch15-04-rc.html

// The Rc<T> type keeps track of the number of references to a value
// to determine whether or not the value is still in use.
// If there are zero references to a value, the value can be cleaned up
// without any references becoming invalid.
// (only sinle-threaded, immutable references (yet))

enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn wont_compile() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let a = Cons(5, Box::new(a)); // Breaks because of ownership
}

fn compiles() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Rc::clone(&a) instead of a.clone(), because Rc::clone just increases reference count (fast),
    // .clone() makes deep copy -> takes time
    let b = Cons(3, Rc::clone(&a));
    let a = Cons(5, Rc::clone(&a));
}
fn print_reference_count() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Strong because there also is a week_count -> chapter 15-6
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!(
        "counter after c goes out of scope = {}",
        Rc::strong_count(&a)
    );
}

fn main() {
    wont_compile();
    compiles();
    print_reference_count();
}
