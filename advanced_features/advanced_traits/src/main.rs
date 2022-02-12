// Docs: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

// Associated types connect a type place holder with a trait such taht the trait method definitions
// can use these placeholder types in their signatures. The implementor of a trait will specify the
// concrete type to be used in this types's place for the particular implementation.
// The difference to generics is that with generics for each implementation types must be annotated
// -> multiple implementations are possible
// With associated types we don't need to annotate types because we can't implement a trait multiple times

use std::ops::Add;

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

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
