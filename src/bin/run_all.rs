use std::fs;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::io::Write;

extern crate hp_solver;

use hp_solver::algorithms::stoppable_df_partial::stoppable_depth_first_partial;
use hp_solver::parser::parse_hddl;

fn main() {

  let data_folder_path = "problems/additional_problems/";

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

    let mut paths: Vec<_> = problem_file_paths.map(|r| r.unwrap()).collect();
    paths.sort_by_key(|dir| dir.path());

    for problem_file_path in paths {

      let now = Instant::now();

      let path_clone = problem_file_path.path().clone();

      let problem_contents = fs::read_to_string(path_clone.clone()).expect("failed to read problem file");

      let parse_result = parse_hddl( &problem_contents, &domain_contents);

      print!("Parsing: {} ", path_clone.display());
      std::io::stdout().flush().unwrap();

      let time_allowed: u64 = 1000;

      match parse_result {
          Ok((problem, domain)) => {

            let handle = thread::spawn(move || {
              let result = stoppable_depth_first_partial(&problem, &domain, &now, &path_clone, time_allowed);

              (result, now.elapsed().as_millis())
            });

            while now.elapsed().as_secs() < time_allowed {
              
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