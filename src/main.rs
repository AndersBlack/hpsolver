use std::env;
use std::fs;
use crate::beginning::create_problem;

mod beginning;
mod problem;
mod parser;
mod domain;


fn main() {
//     // Read the file path from command line
//     let args: Vec<_> = env::args().collect();
//     let file_path: &String = &args[1];

//     let contents = fs::read_to_string(file_path).expect("failed to read problem file");
    
//     let res_string = if let Ok((_, _string)) = parser::parse(&contents) { _string } else { "something failed" };

//     println!("Finished: {}", res_string);

  let problem1 = create_problem();

  println!("It worked!");

}

