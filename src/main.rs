use std::env;
use std::fs;

mod problem;
mod parser;

fn main() {
    // Read the file path from command line
    let args: Vec<_> = env::args().collect();
    let file_path: &String = &args[1];

    let contents = fs::read_to_string(file_path).expect("failed to read problem file");
    
    let res_string = if let Ok((_, _string)) = parser::parse(&contents) { _string } else { "something failed" };


    println!("Finished: {}", res_string);
}

