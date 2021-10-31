use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let arg_case_insensitive = match args.next() {
            Some(arg) => arg.contains("case_insensitive"),
            None => false,
        };

        // is_err -> if variable is not set it returns true, otherwise false
        // CASE_INSENSITIVE=1 cargo run to poem.txt
        let case_sensitive = match arg_case_insensitive {
            true => false,
            false => env::var("CASE_INSENSITIVE").is_err(),
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// data returned by the search function will live
// as long as the data passed into the search function in the contents argument
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // the .lines metod returns an iterator
    // let mut results = Vec::new();
    // for line in contents.lines() {
    // if line.contains(query) {
    // results.push(line)
    // }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // the .lines metod returns an iterator
    let query = query.to_lowercase();
    // let mut results = Vec::new();
    // // Doesn't work with all unicode
    // for line in contents.lines() {
    // if line.to_lowercase().contains(&query) {
    // results.push(line)
    // }
    // }
    // results
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
// won't compile
// pub fn search(query: &str, contents: &str) -> Vec<&str> {
//     vec![]
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // backslash after the opening double quote tells Rust not to put
        // a newline character at the beginning of the contents of this string literal
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
