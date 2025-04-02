use ::std::env;
use std::process;

use mini_grep::Config;

/// This is documentation comment
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    // no need to return an unwrap value
    // we use a simple if let ... instead of unwrap_or_else(...)
    if let Err(e) = mini_grep::run(config) {
        eprintln!("App error: {}", e);
        process::exit(1);
    }
}




