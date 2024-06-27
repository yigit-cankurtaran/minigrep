use std::env;
use std::fs;
// filesystem module

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for '{query}'");
    println!("In file {file_path}");
    // gives an error if the program is run without any arguments
    // saves to the right variables

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    // read the file and save it to the contents variable
    // expect is a method that handles errors

    println!("With text:\n{}", contents);
}
