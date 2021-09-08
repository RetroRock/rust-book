// Docs: https://doc.rust-lang.org/stable/book/ch10-00-generics.html
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Alternative to combine both using generics look for traits
// Copy ParitalOrd trait and Copy trait
// if generic it can also be uknown size at compile time -> therefore only Copy trait
// Docs: https://doc.rust-lang.org/stable/book/ch10-02-traits.html
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    // only types whose values can be ordered (std::cmp::PartialOrd trait)
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// without Copy trait
fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    // only types whose values can be ordered (std::cmp::PartialOrd trait)
    for item in list {
        if item > largest {
            largest = &item;
        }
    }

    &largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// or oly implement methods on some types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointMult<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointMult<T, U> {
    // new point with x value from self PointMult of type T and y value from passed in PointMult of type W
    fn mixup<V, W>(self, other: PointMult<V, W>) -> PointMult<T, W> {
        PointMult {
            x: self.x,
            y: other.y,
        }
    }
}

// also generics
// enum Option<T> {
//     Some(T),
//     None,
// }

// also generics
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// ------------------
// Monomorphization -> reverse generics by looking where they are being used (at compile time)

// let integer = Some(5);
// let float = Some(5.0);

// becomes

// enum Option_i32 {
//     Some(i32),
//     None,
// }

// enum Option_f64 {
//     Some(f64),
//     None,
// }

// let integer = Option_i32::Some(5);
// let float = Option_f64::Some(5.0);

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_i32(&number_list);
    // println!("The largest number is {}", result);
    // assert_eq!(result, 100);

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);
    // println!("The largest char is {}", result);
    // assert_eq!(result, 'y');

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 };
    let integer_and_float = PointMult { x: 5, y: 3.0 };

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p1 = PointMult { x: 5, y: 10.4 };
    let p2 = PointMult { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
