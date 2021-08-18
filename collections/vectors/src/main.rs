// Book: https://doc.rust-lang.org/stable/book/ch08-01-vectors.html
// Documentation: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html

// The first collection type we’ll look at is Vec<T>, also known as a vector. 
// Vectors allow you to store more than one value in a single data structure 
// that puts all the values next to each other in memory. Vectors can only store values of the same type. 
// They are useful when you have a list of items, such as the lines of text in a file 
// or the prices of items in a shopping cart.
// Vectors are stored on the heap, because their size can vary

fn main() {
    // Vector without inital values (needs type annotation)
    let v: Vec<i32> = Vec::new();
    // Vector with initial values using vec! macro for convenience, Rust infers types
    let v = vec![1, 2, 3];
    
    // add mut in order to modify values
    let mut v = Vec::new();

    v.push(5); // rust infers type of v from data
    v.push(6);
    v.push(7);
    v.push(8);

    // Dropping a vector drops its elements (like any other)
    {
        let v = vec![1, 2, 3, 4];
    } // <- v goes out of scope and is freed here
    
    // Reading elements of vectors
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    match v.get(2) { // v.get(...) returns Option<T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    
    // let does_not_exist = &v[100]; // would crash
    let does_not_exist = v.get(100); // returns Option<T> -> would not crash

    // Ownership and Borrowing
    // The below wouldn't work because we cannot have mutable and immutable references at the same
    // time
    let mut v = vec![1, 2, 3, 4, 5];
    
    // let first = &v[0]; // would crash, because we already have a mutable reference

    v.push(6);

    // adding a new element onto the end of the vector might require allocating new memory and 
    // copying the old elements to the new space, if there isn’t enough room to put all the elements
    // next to each other where the vector currently is. In that case, the reference to the first element 
    // would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation

    // Iterating over the values in a vector 
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // Iterate over mutable references to make changes
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // have to use * (dereference operator) to change the value that the mutable reference refers to
    }

    // Using an Enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    // Rust needs to know what types will be in the vector at compile time so it knows exactly how
    // much memory on the heap needed to store each element.
    // If we don't know the exhaustive set of types, the enum technique won't work. instead use a
    // trait object, which is covered in chap 17
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
