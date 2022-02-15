// Docs: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

// Associated types connect a type place holder with a trait such taht the trait method definitions
// can use these placeholder types in their signatures. The implementor of a trait will specify the
// concrete type to be used in this types's place for the particular implementation.
// The difference to generics is that with generics for each implementation types must be annotated
// -> multiple implementations are possible
// With associated types we don't need to annotate types because we can't implement a trait multiple times

use std::{fmt, ops::Add};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // Set default type to Point
    // Can be changed
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Default Add Type
// Rhs -> "right hand side" parameter, defaults to Self
//trait Add<Rhs = Self> {
//    type Output;
//
//    fn add(self, rhs: Rhs) -> Self::Output;
//}

// Newtype pattern -> Thin wrapping of an existing type in another known
// https://doc.rust-lang.org/book/ch19-04-advanced-types.html#using-the-newtype-pattern-for-type-safety-and-abstraction
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// You’ll use default type parameters in two main ways:
// - To extend a type without breaking existing code
// - To allow customization in specific cases most users won’t need

// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    // 1st method called fly
    fn fly(&self) {
        println!("This is your captains speaking");
    }
}

impl Wizard for Human {
    // 2nd method called fly
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    // Compiler defaults to this method, because it is directly implemented on the type
    fn fly(&self) {
        println!("*waving arms furiosly*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Using Supertraits to Require One Trait's Functionality Within Another Trait
// OutlinePrint requires Display -> we can use to_string function that is automatically
// implemented for any type that implements Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // Wouldn't work without Display
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Error: Point doesn't implement Display
impl OutlinePrint for Point {}

// Fix error by implementing Display
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Using the Newtype Pattern to Implement External Traits on External Types
// In Chapter 10 in "Implementing a Trait on a Type" section, we mentioned the orphan rule
// that states we're allowed to implement a trait on a type as long as either the trait or the type are
// local to our crate

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Index 0 of tupe struct Wrapper
        write!(f, "[{}]", self.0.join(", "))
    }
}

// Downside is that Wrapper is a new type and doesn't have the methods of the value it's holding
// We would have to implement all the methods of Vec<T> directly on Wrapper such taht the methods
// delegate to self.0, which would allow us to treat Wrapper Exactly like a Vec<T>
// If we wanted the new type to have every method the inner type has, implementing the Deref trait
// (chapter 15: https://doc.rust-lang.org/book/ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-the-deref-trait)
//

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;

    // To call the fly method from either the Pilot trati or the Wizard trait, we need to use more explicit
    // syntax to specify which fly method we means
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly(); // equivalent to Human::fly(&person)

    // Won't compile, because Animal::baby_name is an associated funtion rather than a method, and therefore
    // doesn't have a self parameter, Rust can't figure out which implementation of Animal::baby_name we want.
    // println!("A baby dog is called a {}", Animal::baby_name());

    // To disambiguate an tell Rust that we want to use the implementation of Animal for Dog, we need
    // to use the fully qualified syntax
    // treat dog as animal
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // In general:
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    // You only need to use this more verbose syntax in cases where there are multiple implementations
    // that use the same name and Rust needs help to identify which implementation you want to call.
    // If we want Wrapper to just have some of the methods of the inner type, we would have to implement just the
    // methods we do want manually

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
