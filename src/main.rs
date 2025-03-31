use ::std::env;
use std::process;

use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        //println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.file_path);

    //println!("Reading file: {}", config.file_path);

    // no need to return an unwrap value
    // we use a simple if let ... instead of unwrap_or_else(...)
    if let Err(e) = mini_grep::run(config) {
        //println!("App error: {}", e);
        process::exit(1);
    }
}




