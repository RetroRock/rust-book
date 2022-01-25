// Docs: https://doc.rust-lang.org/stable/book/ch18-03-pattern-syntax.html

fn main() {
    matching_literals();

    matching_named_variables();

    multiple_patterns();

    matching_ranges_of_values();

    destructuring_to_break_apart_values();

    destructuring_enums();

    destructuring_structs_and_tuples();

    ignore_first_value(5, 10);

    ignoring_parts_of_a_value_with_a_nested_();

    ignoring_unused_variables_with_();
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

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destructuring_enums() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

fn destructuring_structs_and_tuples() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: 10 });
    println!("Feet: {}, Inches: {}; x = {}, y = {}", feet, inches, x, y);
}

// Can be useful for implementing a trait when you need a certain type signature
// but the body of your implementation doesn't need one of the parameters
fn ignore_first_value(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn ignoring_parts_of_a_value_with_a_nested_() {
    let mut setting_value = Some(5);
    let mut new_setting_value = Some(10);

    // Only overwrites, if values are both None
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // Ignore multiple
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

// Can be used for prototyping
fn ignoring_unused_variables_with_() {
    let _x = 5; // does not cause compiler warning
    let y = 10; // causes compiler warning

    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    // println!("{:?}", s); // error: s has been moved into _s

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s); // works, because nothing can ever be moved into _
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

// .. must be ambigous
fn ignoring_remaining_parts_of_value_with_dotdot() {
    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // Does not work because it is ambigous
    // match numbers {
    // (.., second, ..) => {
    // println!("Some numbers: {}", second);
    // }
    // }
}
