use std::error::Error;
use std::fs;

pub struct Config {
    // pub means the struct is public
    // means it's accessible from outside the module
    pub query: String,
    pub file_path: String,
    // the fields are public
}

impl Config {
    // struct for the config
    // this is a method for the struct
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // public method
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 'a is the explicit lifetime
    // data returned by search will live
    // as long as the data passed by contents
    let mut results = Vec::new();
    for line in contents.lines() {
        // lines is built in
        // method that handles line-by-line iter of strings

        if line.contains(query) {
            // contains is built in
            // checks if a string contains something

            results.push(line);
        }
    }

    results
}

// in rust we add tests underneath the file we want to test
#[cfg(test)]
mod tests {
    use super::*;
    // bring the parent module into scope
    // so we can access the functions and structs

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        // the contents of the file
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        // assert_eq! is a macro that compares the two values
        // if they're not equal, it will panic
    }
}
