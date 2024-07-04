use minigrep::Config;
// go style, dependency injection using structs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        // |err| because we're passing the error to the closure
        // unwrap_or_else is a method that handles errors
        // if the result is an error, it will run the closure
        // if the result is Ok, it will return the value
        eprintln!("Problem parsing arguments: {}", err);
        // standard library that prints to the standard error stream
        std::process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file {}", config.file_path);
    // gives an error if the program is run without any arguments
    // saves to the right variables

    if let Err(e) = minigrep::run(config) {
        // if the run function returns an error
        eprintln!("Application error: {}", e);
        // eprintln for error again
        // to print to the standard error stream
        std::process::exit(1);
        // exit the program with an error code
    }
}
