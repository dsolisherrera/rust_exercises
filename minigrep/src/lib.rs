use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // Ignore first value, it is the program name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments, Specify a query "),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments, Specify a filename "),
        };

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}


/// Searches the query string in the text of contents.
/// 
/// # Examples
/// ```
/// let query = "dup";
/// let contents = "This is some tests that contains duplicates
/// more importantly its duplicates are on several lines";
/// assert_eq!(vec!["This is some tests that contains duplicates", "more importantly its duplicates are on several lines"],
///           minigrep::search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
Save, fast, productive.
Pick three.";

        assert_eq!(vec!["Save, fast, productive."], search(query, contents));
    }
}
