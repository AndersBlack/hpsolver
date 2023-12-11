use std::env;
use std::fs;

extern crate hp_solver;

use hp_solver::parser::parse_hddl;

fn main() {

  let args: Vec<_> = env::args().collect();

  if &args[1] == "all" {

    let data_folder_path = "additional_problems/";

    let category_folders = fs::read_dir(data_folder_path).unwrap();

    for category_folder in category_folders {

      println!("\n Category: {}\n", category_folder.as_ref().unwrap().path().display());
      
      let mut problem_folder_path = category_folder.as_ref().unwrap().path();

      problem_folder_path.push("problems/");

      let problem_file_paths = fs::read_dir(problem_folder_path).unwrap();
  
      let mut domain_path = category_folder.unwrap().path();

      domain_path.push("domains/");

      let mut domain_file_path = fs::read_dir(domain_path).unwrap();

      let domain_path = domain_file_path.nth(0).unwrap();

      let domain_contents = fs::read_to_string(domain_path.unwrap().path()).expect("failed to read domain file");
  
      for problem_file_path in problem_file_paths {
  
        let path_clone = problem_file_path.unwrap().path().clone();
  
        let problem_contents = fs::read_to_string(path_clone.clone()).expect("failed to read problem file");
  
        let parse_result = parse_hddl( &problem_contents, &domain_contents);
  
        print!("Parsing: {} Result: ", path_clone.display());
        match parse_result {
            Ok((_problem,_domain)) => {
              println!("Success");
            },
            Err(_e) => {
              println!("Failure");
            }
        }
      }

    }

  } else {

    let problem_folder_path: &String = &args[1];

    let problem_file_paths = fs::read_dir(problem_folder_path).unwrap();

    let domain_file_path: &String = &args[2];
    let domain_contents = fs::read_to_string(domain_file_path).expect("failed to read domain file");

    for problem_file_path in problem_file_paths {

      let path_clone = problem_file_path.unwrap().path().clone();

      let problem_contents = fs::read_to_string(path_clone.clone()).expect("failed to read problem file");

      let parse_result = parse_hddl( &problem_contents, &domain_contents);

      print!("Parsing: {} Result: ", path_clone.display());
      match parse_result {
          Ok((_problem,_domain)) => {
            println!("Success");
          },
          Err(_e) => {
            println!("Failure");
          }
      }
    }

  }

}