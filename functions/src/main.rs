// is a statement -> does not return a value
fn main() {
    println!("Hello, world!");

    another_function(5, 6);
    let x = five();
    let y = plus_one(5);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// also a statement
fn another_function(x: i32, y: i32) {
    println!("The value of x is {}, the value of y is {}.", x, y);
    // let x = (let y = 6); // Doesn't work, because let is a statement, does not return anything
    // but in C and C++ that would return a value, e.g. x = y = 6 would work therei
    // 6 is an expression, calling a function, macro, or {} scope body (of function) is an
    // expression

    let x = 5;
    
    // {...} is an expression -> returns something, not ; at the end of "x + 1", 
    // adding it, would convert it into an statement
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
// if you want to return before the last element you can just write "return"
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1; // This would throw an error because of the ;
}
