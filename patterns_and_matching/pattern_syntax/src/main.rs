// Docs: https://doc.rust-lang.org/stable/book/ch18-03-pattern-syntax.html

fn main() {
    matching_literals();

    matching_named_variables();

    multiple_patterns();

    matching_ranges_of_values();

    destructuring_to_break_apart_values();
}

fn matching_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // will match any value inside a Some value (new y variable)
        _ => println!("Default case, x = {:?}", x), // x is still the outer x, because no new x was introduced
    } // scope of y ends -> y from outer scope again

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn matching_ranges_of_values() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"), // match if x is 1, 2, 3, 4, 5; only for numeric or char values
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"), // match
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destructuring_to_break_apart_values() {
    // Destructuring Structs
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p; // shorthand => let Point { x: x, y: y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // Destructure with literal values
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x), // matches if y = 0
        Point { x: 0, y } => println!("On the y axis at {}", y), // matches if x = 0
        Point { x, y } => println!("On neither axis: ({}, {})", x, y), // does not specify any literals, matches any other point
    }
}
