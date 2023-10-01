use std::env;
use std::fs;
use crate::beginning::create_problem;
use crate::parser::parse_hddl;
use crate::algorithms::depth_first;

mod beginning;
mod problem;
mod parser;
mod domain;
mod algorithms;


fn main() {
  // Read the file path from command line
  let args: Vec<_> = env::args().collect();


  if args.len() == 3 {
    let problem_file_path: &String = &args[1];
    let problem_contents = fs::read_to_string(problem_file_path).expect("failed to read problem file");

    let domain_file_path: &String = &args[2];
    let domain_contents = fs::read_to_string(domain_file_path).expect("failed to read domain file");

    let (problem, domain) = parse_hddl( &problem_contents, &domain_contents);

    println!("\nFinished parsing problem and domain!\n");

    depth_first(problem,  domain);
  } else if args.len() == 1 {
    let (problem, domain) = create_problem();

    println!("Doing df");
    depth_first(problem, domain);
  } else {
    println!("Please provide a path for both the problem.hddl and the domain.hddl files. Or add nothing and try the test problem :)");
  }

}

