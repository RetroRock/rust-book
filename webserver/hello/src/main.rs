// Docs: https://doc.rust-lang.org/book/ch20-01-single-threaded.html

// TCP is the lower-level protocol taht describes the details of how information gets from one
// server to antoher but doesn't specify what that information is. HTTP builds on top of TCP by
// defining the contents of the requests and responses. It's technically possible to use HTTP with
// other procotocols, but in the vast majority of cases, HTTP sends its data over TCP. 
// We'll work with the raw bytes of TCP and HTTP requests and responses

// Listening to the TCP Connection

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // 7878 is "rust" typed on a telephone
    // Bind works like ::new and returns a new TcpListener instance (binding to a port)
    // bind method retuns a Result<T, E>, which indicates that binding might fail
    // For example listening on port 80 requires administrative privileges, 
    // nonadministrives can listen on ports higher than 1023
    // unwrap just stops program
    
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    // incoming method retuns an iterator that gives us a sequence of streams (of type TcpStream)
    // A single stream represents an open connection between the client and the server. 
    // A connection is the name for the full request and response process in which a client connects
    // to the server, the server generates a response, and the server closes the connection
    // TcpStream will read from itself to see what the client sent and then allow us to write our
    // response to the stream
    
    // Overall, this for loop will process each connection in turn and produce a series of streams
    // for us to handle
    // We might receive erros from the incoming method when a client connects to the sever,
    // because we're acutally iterating over connection attempts, not connections
    // Also, depending on the operating system, the number of simultanenous open connections 
    // can be limited to number
    
    // When opening 127.0.0.1:7878 in the browser, the browser shows an error message like
    // "Connection reset", because the server isn't currently sending back any data,
    // but the terminal should show "Connection established!"
    // Sometimes we see several messages, the reason might be that the browser is making a request
    // for the page as well as a request for other resources, like the favicon.ico
    // It could also be that the browser is trying to connect to the sever multiple times because the
    // server isn't responding with any data.
    // When stream goes out of scope and is dropped at the end of the loop, the connection is
    // closed as part of the drop implementation
    
    // Reading the Request
    // Separating the concerns of first getting a connection and then taking some action with the
    // connection by creating a new function for processing connections

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        handle_connection(stream);
    }

}

// Bring std::io::prelude into scope to get access to certain traits that let us read from and
// write to the stream.
// In the handle_connection function, the stream parameter is mutable, because the TcpStream
// instance keeps track of what data it reunts to us internally and it therefore needs to be mut
// because its internal state might change

// Next, we actually read from the stream. 

// First, we declare a buffer on the stack to hold the data that is read in. 
// The buffer size is 1024 butes, because it's big enough to hold the data of a basic request and
// sufficient for our purposes in this chapter. 
// Requests on arbitrary size would need complicated buffer management
// Then we pass the buffer to stream.read, which will read bytes from the TcpStream 
// and put them in the buffer

// Second, we convert the bytes in the buffer to a string and print that string.
// The String::from_utf8_lossy function takes a &[u8] and produces a String from it. The "lossy"
// part of the name indicates the behaviour of this function when it sees an invalid UTF-8
// sequence: it will replace the invalid sequence with �, the U+FFFD REPLACEMENT CHARACTER.
// We might see replacement characters for characters in the buffer that aren't filled by request
// data.

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// A closer Look at an HTTP Request

// HTTP is a text-based protocol, and a request takes this format:
//  Method Request-URI HTTP-Version CRLF
//  headers CRLF
//  message-body

// The first line is the request that holds information about what the client is requesting
// The method being used is GET

// Next part of the request line is /, which indicates the Unifrom Resource Identifier (URI)
// the client is requesting: URI almost, but not quite, the same as a Uniform Resource Locator
// (URL)

// The last part is the HTTP version the client uses, and then the request line ends in a CRLF
// sequence Carriage Return and Linfe Feed, which are terms from typewriter days)
// The CRLF can also be written as \r\n, \r being the carriage return and \n the line feed.
// The CRLF sequence separates the request line form the rest of the data.

// Writing a Response

// Responses have the following format:
//  HTTP-Version Status-Code Reason-Phrase CRLF
//  headers CRLF
//  message-body
// Example:
//  HTTP/1.1 200 OK\r\n\r\n

// as_bytes to convert string data to bytes
// write method on stream takes a &[u8] and sends those bytes directly down the connection
// because the write operation could fail, we use uunwrap on any error result as before.
// flush will wait and prevent the program from continuing until all the butes are written to the
// connection
// TcpStream contains an internal buffer to minimize calls to the underlying operating system
