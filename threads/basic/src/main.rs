// Using Threads to Run Code Simultaneously
// Docs: https://doc.rust-lang.org/stable/book/ch16-01-threads.html

use std::thread;
use std::time::Duration;

fn main() {
    basic();
    basic_with_join();
    with_values();
    with_values_and_move();
}

fn basic() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // New thread will be stopped when main thread ends (does not reach 5 - 10)
}

fn basic_with_join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Will wait for handle thread to finish (blocks thread)
    handle.join().unwrap();
}

fn with_values() {
    // let v = vec![1, 2, 3];

    // Error: Closure may outlive current function (with_values and v variable)
    // let handle = thread::spawn(|| {
    // println!("Here's a vector: {:?}", v);
    // });
    // Error: When thread starts v might no longer be valid
    // drop(v);

    // handle.join().unwrap();
}

fn with_values_and_move() {
    let v = vec![1, 2, 3];

    // Move ownership of v into new thread -> will be valid (instead of only borrowing)
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
