use std::env;
use std::error::Error;
use std::fs;
// filesystem module

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        // |err| because we're passing the error to the closure
        // unwrap_or_else is a method that handles errors
        // if the result is an error, it will run the closure
        // if the result is Ok, it will return the value
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file {}", config.file_path);
    // gives an error if the program is run without any arguments
    // saves to the right variables

    if let Err(e) = run(config) {
        // if the run function returns an error
        println!("Application error: {}", e);
        std::process::exit(1);
        // exit the program with an error code
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // struct for the config
    // this is a method for the struct
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        // we're cloning the values because we want to own them
        // we don't want to take ownership of the values

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Box is a trait object
    // means the function will return a type that implements the Error trait
    // dyn = dynamic
    let contents = fs::read_to_string(config.file_path)?;
    // read the file and save it to the contents variable
    // ? returns the error value from the current function for the caller to handle

    println!("With text:\n{}", contents);
    // print the contents of the file
    Ok(())
    // return an Ok value
    // indicates we're not returning a value
    // we're just running for the side effects
}
