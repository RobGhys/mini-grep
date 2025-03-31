use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("Not enough arguments were provided. Expected 2 at least.");
        }

        // copy the data
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // 'search' returns a vector of str.
    // iterate in this vector
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

// lifetime of 'contents' is connected to lifetime of return str
// -> data returned by fn search will live as long as the data passed
// ... in the function's 'contents' &str
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "like";
        let contents = "\
Rust:
loud, liker, love.
Pick three.";

        assert_eq!(vec!["loud, liker, love."], search(query, contents));
    }
}