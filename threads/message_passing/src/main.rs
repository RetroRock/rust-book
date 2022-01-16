// Using Message Passing to Transfer Data Between Threads
// Docs: https://doc.rust-lang.org/stable/book/ch16-02-message-passing.html

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// mpsc = "multiple producer single consumer"
// tx = "transmitter"; rx = "receiver"
fn main() {
    // basic_message_passing();
    // basic_message_passing_ownership_rules_example();
    // send_mult_values();
    send_mult_values_from_multiple_producers();
}

// Basic example
fn basic_message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // Blocks main thread execution, .try_recv does not
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// Basic example with ownership rules to prevent runtime errors
fn basic_message_passing_ownership_rules_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // ! Does not work, because val is moved to new thread
        // println!("val is {}", val);
    });

    // Blocks main thread execution, .try_recv does not
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// Sending multiple values and seeing the receiver waiting
fn send_mult_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Don't have to call .recv ?? (Happens automatically? -> main thread is waiting)
    for received in rx {
        println!("Got: {}", received);
    }
}

// Sending multiple values from multiple producers
fn send_mult_values_from_multiple_producers() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Don't have to call .recv, treated as an iterator -> main thread is waiting
    for received in rx {
        println!("Got: {}", received);
    }
}
