// Docs: https://doc.rust-lang.org/stable/book/ch20-02-multithreaded.html#turning-our-single-threaded-server-into-a-multithreaded-server

// Improving Throughput with a Thread Pool

// A thread pool is a group of spawned threads that are waiting and ready to handle a task
// We'll have a fixed number of threads waiting in the pool to prevent incoming requests from creating potentionally unlimited threads.
// The pool will maintain a queue of incoming requests. Each of the
// threads in the pool will pop off a request from this queue, handle the request, and then ask the
// queue for another request. With this design, we can process N requests concurrently, where N is
// the number of threads

// This technique is just one of many ways to improve throughput of a web server. Other options are
// the fork/join model and the single-threaded async I/O model

use hello_multithreaded::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        // Make new threads without a limit
        // We dont't have to wait for /sleep requests to finish
        //thread::spawn(|| {
        //    handle_connection(stream);
        //});

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        // Simulating a slow request
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
