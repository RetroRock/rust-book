// Docs: https://doc.rust-lang.org/stable/book/ch19-05-advanced-functions-and-closures.html

// Function Pointers
// You can pass closures as well as functions to functions. The latter is useful when you want
// to pass a function you've already defined rather that defing a new closure
// Functions coerce to the type fn, not to be confused with the Fn closure trait
// The fn type is called a function pointer

fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Unlike closures, fn is a type rather than a trait, so we speicify fn as the pararmter type directly rather than
// declaring a generic type pararameter with one of the Fn traits as a trait bound

// Function pointers implement all three of the closure traits (Fn, FnMut and FnOnce),
// so you can always pass a function pointer as an argument for a function that expects a closure
// It's best to write functions usuing a generic type and one of the clousre traits so your functions can accept
// either functions or closures

// An example for only accepting fn and not closures is when interfacing with e xternal code that doens't have closures:
// C doesnt have closures

enum Status {
    Value(u32),
    Stop,
}

// Returning Closures
// Closures are represented by traits, which means we can't return closures directly
// In most cases we might want to return the concrete type that implements the trait instead of returning the trait
// This doesn't work with closures, because they don't have a concrete type that is returnable
// You're not allewd to use the function pointer fn as a return type, for example
// fn returns_closure() -> dyn Fn(i32) -> i32 {
// Return type cannot have an unboxed trait object
// |x| x + 1
// }
// The error references the Sized trait again! Rust doesn't know how much space it will need to stroe the closure.
// We can use a trait object as a solution (Chapter 17: https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types)
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    // Example of where we can use either a closure defined inline or a named function, is map
    let list_of_numbers = vec![1, 2, 3];

    // Closure example
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // function instead of closure
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    // We must use the fully qualified syntax that we talked about earlier in the "Advanced Traits" section
    // because there are multiple functions available named to_string. Here, we're using the to_string function defined
    // in the ToString trait, which the standard library has implemented for any type that implements Display

    // Another useful patter that exploits an implementation detail of tuple structs and tuple-struct enum variants. These types
    // use () as initializer syntax, which looks like a function call
    // The initializers are implemented as functions returning an instance that's constructed from their arguments. We can use
    // these initializer functions as function pointers that implement the closure traits, which means we can specify the initializer functions
    // as arguments for methods that take closures:
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    // Here we crate Status::Value instances using each u32 value in the range that ma is called on by using the initializer function
    // of Status::Value. Closures and functions both compile to the same code
}
