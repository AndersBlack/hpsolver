use std::env;
use std::fs;
use std::time::Instant;
use flame;
use std::fs::File;

use crate::beginning::create_problem;
use crate::parser::parse_hddl;
use crate::algorithms::depth_first;
use crate::algorithms::iterative_df::*;

mod beginning;
mod datastructures;
mod toolbox;
mod parser;
mod algorithms;

fn main() {
  let _fg = ::flame::start_guard("main");
  // Read the file path from command line
  let args: Vec<_> = env::args().collect();

  if args.len() == 3 {

    let now = Instant::now();

    let problem_file_path: &String = &args[1];
    let problem_contents = fs::read_to_string(problem_file_path).expect("failed to read problem file");

    let domain_file_path: &String = &args[2];
    let domain_contents = fs::read_to_string(domain_file_path).expect("failed to read domain file");

    let (problem, domain) = parse_hddl( &problem_contents, &domain_contents);

    println!("\nFinished parsing problem and domain!\n");

    depth_first(problem,  &domain);

    let elapsed_time = now.elapsed();
    println!("\nRunning i_d_f took {} milli seconds.\n", elapsed_time.as_millis());

  } else if args.len() == 1 {
    let (problem, domain) = create_problem();

    println!("Doing df");
    depth_first(problem, &domain);
  } else if args[1] == "benchmark" {

    // let paths = fs::read_dir("./data/").unwrap();

    // for path in paths {
    //   let problem_paths = fs::read_dir( path.unwrap().path().join("problems")).unwrap();

    //   problem_paths.last().unwrap();

    //   for problem_path in problem_paths {
    //     println!("Name: {}", problem_path.unwrap().path().display())
    //   }

    //   for domain_path in problem_paths {
    //     println!("Name: {}", problem_path.unwrap().path().display())
    //   }
    // }

  } else {
    println!("Please provide a path for both the problem.hddl and the domain.hddl files. Or add nothing and try the test problem :) arg length: {}", args[1]);
  }

  flame::dump_html(File::create("flamegraph.html").unwrap()).unwrap();
}

