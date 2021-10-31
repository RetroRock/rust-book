// Docs: https://doc.rust-lang.org/stable/book/ch12-01-accepting-command-line-arguments.html
// Does not support unicode (See docs for more info)
use minigrep::Config;
use std::env;
// use std::error::Error;
// use std::fs;
use std::process;

fn main() {
    // ! Reading the Argument Values
    // std::env::args() returns an Iterator of command line arguments
    // .collect() turns that into a collection
    // Have to specify type of args, because that affects what kind of collection .collect() returns
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // ! Saving the Argument Values in Variables
    // let query = &args[1];
    // let filename = &args[2];
    // let (query, filename) = parse_config(&args);
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     // Print Errors to Standard Error (Error Stream)
    //     // Command line programs are expected to send error messages to the standard error stream
    //     // so we can still see error messages on the screen
    //     // even if we redirect the standard output stream to a file.
    //     eprintln!("Problem parsing arguments: {}", err);
    //     // Signal to the process that called our program, that the program exited with an error state
    //     // 0 == ok
    //     // similar to panic! but without all the extra output
    //     process::exit(1);
    // });

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", query);
    println!("Searching for \"{}\"", config.query);
    // println!("In file {}", filename);
    println!("in file \"{}\":\n", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

// Box<dyn Error>: Type that implements the Error trait, here dyn -> dynamic any kind of error
// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     // Read Specified File, ? will return the Error value from the current fun function
//     let contents = fs::read_to_string(config.filename)?;
//     println!("With text:\n{}", contents);

//     Ok(())
// }

// struct Config {
//     query: String,
//     filename: String,
// }

// Result for Config in successful and Err for error case
// impl Config {
//     fn new(args: &[String]) -> Result<Config, &str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Ok(Config { query, filename })
//     }
// }

// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             panic!("not enough arguments");
//         }

//         let query = args[1].clone();
//         let filename = args[2].clone();

//         Config { query, filename }
//     }
// }

// fn parse_config(args: &[String]) -> Config {
//     // Clone is a bit slow -> more on that in Chapter 13
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config { query, filename }
// }
