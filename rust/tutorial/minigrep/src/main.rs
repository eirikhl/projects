use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // args() gets the command line arguments, collect() turns them into a vector of words
    // Keep in mind that the first element is the name of the program, as in C
     
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // eprintln! is a function for printing error message to stdout, regardless of normal output
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
