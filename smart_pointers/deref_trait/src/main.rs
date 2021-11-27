// Docs: https://doc.rust-lang.org/stable/book/ch15-02-deref.html

fn main() {
    println!("Hello, world!");
}

fn dereference_operator() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // Use * to compare values and not value with reference to value
}

fn using_box_like_a_reference() {
    let x = 5;
    let y = Box::new(x);
    // Difference: y is pointing to a copied value of x instead of to x directly

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Defining Our Own Samrt Pointer
// Box is defined as tuple struct with one element

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T; // Associated type covered in chapter 19
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn using_mybox_like_a_reference() {
    let x = 5;
    let y = MyBox::new(x);
    // Difference: y is pointing to a copied value of x instead of to x directly

    assert_eq!(5, x);
    assert_eq!(5, *y); // Error: Deref trait missing (without deref impl)
                       // *y translates to *(y.deref()) behind the scenes
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// Deref coercion is a convenience that Rust performs on arguments to functions and methods.
// Deref coercion works only on types that implement the Deref trait.
// Deref coercion converts such a type into a reference to another type.
// For example, deref coercion can convert &String to &str because String implements the Deref trait such that
// it returns &str. Deref coercion happens automatically when we pass a reference to a particular type’s value
// as an argument to a function or method that doesn’t match the parameter type in the function or method definition.
// A sequence of calls to the deref method converts the type we provided into the type the parameter needs.

fn deref_coercion() {
    let m = MyBox(String::from("Rust"));
    hello(&m)
}

// The (*m) dereferences the MyBox<String> into a String. Then the & and [..]
// take a string slice of the String that is equal to the whole string to match the signature of hello.

// When the Deref trait is defined for the types involved,
// Rust will analyze the types and use Deref::deref as many times as necessary to get a reference
// to match the parameter’s type. The number of times that Deref::deref needs to be inserted is resolved at compile time,
// so there is no runtime penalty for taking advantage of deref coercion!

// How Deref Coercion Interacts with Mutability
// -> From &T to &U when T: Deref<Target=U>
// -> From &mut T to &mut U when T: DerefMut<Target=U>
// -> From &mut T to &U when T: Deref<Target=U> (Not the other way around -> only one mutible reference)

fn without_deref_coercion() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
