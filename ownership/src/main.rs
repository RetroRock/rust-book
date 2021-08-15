fn main() {
    {                    // s is not valid here, hasn't been declared yet
        let s = "hello"; // s is valid from this point forward
        // do something with s
    }                    // thsi scope is over now, and s is no longer valid, Rust calls 'drop' here to free memory
    
    // :: is name spacing -> chap 5
    let mut s = String::from("hello"); // stored in heap instead of stack -> therefore can be mutable unlike 'let s = "hello"'

    s.push_str(", world!"); // appends a literalt ot a String

    println!("{}", s); // hello, world!

    // string literals 'lets = "hello"' -> hard coded in executable -> fast, but immutable, String isn't

    let x = 5;
    let y = x; // copy of x, bot x and y on the stack

    let s1 = String::from("hello");
    let s2 = s1; // not the data on the heap is being copied, but only the pointer to the heap that is sitting on the stack
    // this is being called a 'move' -> shallow copy and invalidating first variable

    // s1 is no longer valid
    // println!("{}, world", s1); // this would throw an error, because s1 is out of scope now

    // Rust never performes a deep copy automatically (only pointer gets copied, not the data) ->
    // very fast and inexpensive
    
    // How to actually perform a deep copy of the heap data of the String, not just stack data
    
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy of s1, s1 is still valid

    println!("s1 = {}, s2 = {}", s1, s2);

    // Since the size of integers is known at compile time, the data is stored on the stack and
    // copies are fast to make
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Scalar types can implement the 'Copy' trait, which means after an older variable is still
    // usable after assignment -> Doesn't work with the 'Drop' trait
    
    // OWNERSHIP AND FUNCTIONS
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s);            // s's value moves into the function
                                   // ... and so is no longer valid here

    let x = 5;                     // x comes into scope

    makes_copy(x);                 // x would move into the function,
                                   // but i32 is Copy, so it's okay to still 
                                   // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of the scope and 'drop' is called. The backing
  // memory is freeed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of the scope. Nothing special happens.

fn return_values_and_scope() {
    let s1 = gives_ownership();         // gives_ownership -> moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope an is dropped. s2 dgoes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_wnership will move its 
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and 
                                             // moves out of the calling function
}

// takes_and_gives_back will take a String and return one

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string // a_string is returned and moves out to the calling function
}
