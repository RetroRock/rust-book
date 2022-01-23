// Docs: https://doc.rust-lang.org/stable/book/ch18-01-all-the-places-for-patterns.html
//       https://doc.rust-lang.org/stable/book/ch18-02-refutability.html

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color")
        }
    } else {
        println!("Using blue as the background color");
    }

    // if let is not exhaustive -> not all cases have to be catched at compile time
    // -> have to use else, (Rust cannot prevent logic bugs)
    // match control flow is exhaustive -> catches all possibilities (at compile time)

    conditional_loops();

    for_loops();

    // also a pattern, let PATTERN =  EXPRESSION;
    // bind what matches here to the variale x
    let x = 5;

    // match a tuple against a pattern
    // Rust compares the value (1, 2, 3) against the patern (x, y, z)
    let (x, y, z) = (1, 2, 3);
    // let (x, y) = (1, 2, 3); // doesn't work, type doesn't match
    let (x, y, _) = (1, 2, 3); // works, ignore one value
    let (x, y, ..) = (1, 2, 3, 4, 5, 6); // works, ignore the rest of the values

    print_coordinates(&(2, 3));
}

fn conditional_loops() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loops() {
    let v = vec!['a', 'b', 'c'];

    // for x in y, x is the pattern
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

// Pattern matchin for functions or closures
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

// Refutability: Whether a Pattern Might Fail To Match

// Patterns that will match for any possible value passed are irrefutable
// Example: let x = 5; because x matches anything and therefore cannot fail to match

// if let Some(x) = a_value is refutable, because if the value in the a_value variable is None
// rather than Some, the Some(x) pattern will not match

// Function parameters, let statements, and for loops can only accept irrefutable patterns, because the program
// cannot do anything meaningful when values don't match

// Does not compile, let requires an irrefutable pattern, but an refutable pattern Some(x) is specified
// We cannot cover every valid value wit the pattern Some(x) -> compile time error
// let Some(x) = some_option_value

// Fix: Skip code in curly brackets if pattern doesn't match
// if let Some(x) = some_option_value {
//     println!("{}", x);
// }

// This will produce a warning, because irrefutable pattern will always match
// if let x = 5 {
//    println!("{}",x);
// }
