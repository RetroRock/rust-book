// Docs: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html

use std::slice;

fn main() {
    let mut num = 5;

    // Can't dereference raw pointers outside of an unsafe block
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // There might or might not be data at that adress
    let address = 0x012345usize;
    let r = address as *const i32;

    // Dereferencing raw pointers only works in unsafe blocks
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // split_at_mut uses unsafe rust
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    split_at_mut(r, 3);

    this_probably_crashes_if_slice_is_used();

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// Unsafe function example
unsafe fn dangerous() {}

// Creating a Safe Abstraction over Unsafe Code
// Implementing a simplified version of split_at_mut
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    // ptr -> *mut i32 raw pointer
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // Two mutable references at the same time
    // Rust doesn't know that we're borrowing different parts of the slice,
    // which is fundamentally ok, solution: unsafe
    // (&mut slice[..mid], &mut slice[mid..])

    unsafe {
        (
            // slice that starts from ptr and is mid items long
            // might or might not be a valid pointer -> unsafe block
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Might result in undefined behavior
fn this_probably_crashes_if_slice_is_used() {
    let address = 0x01234usize;
    let r = address as *mut i32;
    let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
}

// Using extern Funtions to Call External Code
// by utilizing FFI (Foreign Function Interface)
// "C" part defines which ABI (application binary inerface)
// the external function uses: the ABI defines how
// to call the function at the assembly level. The "C" ABI
// is the most common and follows the C programming language's ABI
extern "C" {
    // List names and signatures of external functions
    fn abs(input: i32) -> i32;
}

unsafe fn use_extern() {
    println!("Absolute value of -3 according to C: {}", abs(-3));
}

// Calling Rust Functions from Other Languages (does not require unsafe)
// Add extern keyword and specify the ABI to use just before the fn keyword
// Add $[no_mangle] annotation to tell the Rust compiler
// not to mangle the name of this function (Changing name of function to something
// that provides more information for the compilation process)
// in order to make it nameable by other languages
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Accessing or Modifying a Mutable Static Variable
// Can cause data races if two threads are accessing the same mutable global variable
// Static variable names are in the SCREAMING_SNAKE_CASE by convention
// Must have a static lifetime
// Always have a fixed address in memory, will always access the same data
// Constants however are allowed to duplicate their data whenever they're used (multiple pointers)
// Static variables can be mutable -> accessing and modifying mutable static variables is unsafe
static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

// Unsafe with multiple threads
// Use concurrency  tehniques and thread safe smart pointers instead
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Implementing an Unsafe Trait
// A trait is unsafe when at least one of its methods
// has some invariant that the compiler can't verify
// f we implement a type that contains a type
// that is not Send or Sync, such as raw pointers,
// and we want to mark that type as Send or Sync, we must use unsafe
// (Send and Sync: https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html#extensible-concurrency-with-the-sync-and-send-traits)
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

// Accesssing Fields of a Union
// Similar to a struct, but only one declared
// field is used in a particular instance at one time
// Docs: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#accessing-fields-of-a-union
