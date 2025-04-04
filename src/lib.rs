use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // 'args' implements an Iterator that returns Strings
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query string!")
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file path!")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {query, file_path, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// lifetime of 'contents' is connected to lifetime of return str
// -> data returned by fn search will live as long as the data passed
// ... in the function's 'contents' &str
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str, contents: &'a str
) -> Vec<&'a str> {
    // new String object
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
rust-LOud, liker, love.
Pick three.";

        assert_eq!
            (vec!["Rust:", "rust-LOud, liker, love."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "like";
        let contents = "\
Rust:
loud, liker, love.
Pick three.";

        assert_eq!(vec!["loud, liker, love."], search(query, contents));
    }
}