// Using Mutexes to Allow Access to Data from One Thread at a Time
// Docs: https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html#shared-state-concurrency

use std::rc::Rc;
use std::{
    sync::{Arc, Mutex},
    thread, vec,
};

fn main() {
    basic_mutex();
    share_mutex_between_mult_threads_broken();
    share_mutex_between_mult_threads_rc_broken();
    share_mutex_between_mult_threads_arc();
}

fn basic_mutex() {
    let m = Mutex::new(5); // Smart pointer

    {
        // Acquire lock, would fail if another hread holding the lock panicked (Blocks current thread, waits for lock)
        // Use .unwrap to make this thread panic if lock fails (because of the above comment)
        // .lock returns smart pointer wrapped into LockResult (handled with .unwrap)
        // MutexGuard implements Deref trait to point at inner data, also has Drop trait to release lock automatically
        // when MutexGuard goes out of scope
        let mut num = m.lock().unwrap();
        *num = 6; // Due to Deref trait
    }

    println!("m = {:?}", m);
}

// Sharing a Mutex<T> between multiple threads
fn share_mutex_between_mult_threads_broken() {
    // let counter = Mutex::new(0);
    // let mut handles = vec![];

    // for _ in 0..10 {
    // let handle = thread::spawn(move || {
    // counter moved into first thread
    // let mut num = counter.lock().unwrap();

    // *num += 10;
    // });
    // handles.push(handle);
    // }

    // for handle in handles {
    // handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());
}

// Sharing a Mutex<T> between multiple threads (broken with referenced counted pointer)
fn share_mutex_between_mult_threads_rc_broken() {
    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];
    //
    // for _ in 0..10 {
    // let counter = Rc::clone(&counter);
    //RC<T> is not safe to share across threads
    // let handle = thread::spawn(move || {
    // let mut num = counter.lock().unwrap();
    //
    // *num += 10;
    // });
    // handles.push(handle);
    // }
    //
    // for handle in handles {
    // handle.join().unwrap();
    // }
    //
    // println!("Result: {}", *counter.lock().unwrap());
}

// Sharing a Mutex<T> between multiple threads using Atomic Reference Counting with Arc<T>
// Arc has a performance penalty
fn share_mutex_between_mult_threads_arc() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // RC<T> is not safe to share across threads
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Mutexes can create deadlocks when two threads have to acquire two resources and both have
// acquired one of them -> wait for it infinitely
