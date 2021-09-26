// Docs: https://doc.rust-lang.org/stable/book/ch12-01-accepting-command-line-arguments.html
// Does not support unicode (See docs for more info)
use std::env;

fn main() {
    // ! Reading the Argument Values
    // std::env::args() returns an Iterator of command line arguments
    // .collect() turns that into a collection
    // Have to specify type of args, because that affects what kind of collection .collect() returns
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // ! Saving the Argument Values in Variables
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
