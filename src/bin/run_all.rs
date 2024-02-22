use std::fs;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::io::Write;

extern crate hp_solver;

use hp_solver::parser::parse_hddl;
use hp_solver::algorithms::stoppable_df::stoppable_depth_first;

fn main() {

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

      let now = Instant::now();

      let path_clone = problem_file_path.unwrap().path().clone();

      let problem_contents = fs::read_to_string(path_clone.clone()).expect("failed to read problem file");

      let parse_result = parse_hddl( &problem_contents, &domain_contents);

      print!("Parsing: {} ", path_clone.display());
      std::io::stdout().flush().unwrap();

      match parse_result {
          Ok((problem, domain)) => {

            let handle = thread::spawn(move || {
              let result = stoppable_depth_first(problem, &domain, &now, &path_clone);

              (result, now.elapsed().as_millis())
            });

            //while now.elapsed().as_secs() < 10
            loop {
              
              thread::sleep(Duration::from_millis(50));

              if handle.is_finished() {
                break;
              }
            }

            let (message, time) = handle.join().unwrap();

            print!("Result: {}, Time: {} milliseconds\n", message, time);
          },
          Err(_e) => {
            println!("Failure");
          }
      }
    }

  }

}