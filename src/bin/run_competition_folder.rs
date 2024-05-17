use core::f64;
use std::fs;
use std::fs::ReadDir;
use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::io::Write;
use std::env;
use std::path::PathBuf;

extern crate hp_solver;

use hp_solver::algorithms::stoppable_df_partial::stoppable_depth_first_partial;
use hp_solver::parser::parse_hddl;
use hp_solver::algorithms::stoppable_df::stoppable_depth_first;


fn main() {

  let args: Vec<_> = env::args().collect();

  if args.len() < 3 {
    println!("The binary takes 2 arguments: Implementation (total/partial) & Path to problem folder.\n An example could be: total path/to/problem/folder/ ");
    return
  }

  // Run total order track 
  let path = String::from(args[2].clone());
  let implementation = String::from(args[1].clone());
  let mut category_folder = PathBuf::new();
  category_folder.push(path);

  println!("\n Category: {:?}\n", category_folder);
  
  let mut problem_folder_path = category_folder.clone();

  problem_folder_path.push("problems/");

  let mut domain_path = category_folder.clone();

  domain_path.push("domains/");

  // Does the folder contain a domains and problems folder?
    // Yes -> Apply the same domain to every problem
    // No -> Look for a seperate domain for each problem
  match (fs::read_dir(problem_folder_path), fs::read_dir(domain_path)) {
    (Ok(problem_file_paths), Ok(domain_file_path)) => {
      single_domain(problem_file_paths, domain_file_path, implementation);
    }
    _ => {
      multiple_domain(category_folder, implementation);
    }
  }

}

fn multiple_domain (category_folder: PathBuf, implementation: String) {

  let file_paths = fs::read_dir(category_folder.clone());

  let mut collective_score: f64 = 0.0;
  let mut problem_count: f64 = 0.0;

  match file_paths {
    Ok(files) => {

      let mut paths: Vec<_> = files.map(|r| r.unwrap()).collect();
      paths.sort_by_key(|dir| dir.path());

      for file in paths {
        
        let file_path = file.path();

        if !file_path.clone().into_os_string().into_string().unwrap().contains("-domain.hddl") && !file_path.clone().into_os_string().into_string().unwrap().contains(".md") && !file_path.clone().into_os_string().into_string().unwrap().contains("solutions") {

          let domain_path = look_for_domain_file(fs::read_dir(category_folder.clone()).unwrap(), file_path.clone());
          let problem_path = file_path;

          problem_count = problem_count.add(1.0);
          let now = Instant::now();

          let domain_contents = fs::read_to_string(domain_path).expect("failed to read domain file");
          let problem_contents = fs::read_to_string(problem_path.clone()).expect("failed to read problem file");


          let parse_result = parse_hddl( &problem_contents, &domain_contents);

          print!("Running: {} ", problem_path.display());
          std::io::stdout().flush().unwrap();

          let time_allowed: u64 = 1800;

          match parse_result {
              Ok((problem, domain)) => {

                let imp_clone = implementation.clone();

                let handle = thread::spawn(move || {

                  let mut result = "No result";

                  if imp_clone == "partial".to_string() {
                    result = stoppable_depth_first_partial(&problem, &domain, &now, &problem_path, time_allowed);
                  } else if imp_clone == "total".to_string() {
                    result = stoppable_depth_first(&problem, &domain, &now, &problem_path, time_allowed);
                  } else {
                    println!("\nImplementation doesnt exist, try 'total' or 'partial'\n");
                    return (result, now.elapsed().as_secs())
                  }

                  (result, now.elapsed().as_secs())
                });

                while now.elapsed().as_secs() < time_allowed {
                  
                  thread::sleep(Duration::from_millis(50));

                  if handle.is_finished() {
                    break;
                  }
                }

                let (message, time) = handle.join().unwrap();
                print!("Result: {}, Time: {} seconds\n", message, time);

                let score = compute_score(time);

                collective_score = collective_score.add(score);
              },
              Err(e) => {
                println!("Failure: {}", e);
              }
          }
        }
      }

    },
    _ => {
      panic!("Unknown folder");
    }
  }

  let domain_score = collective_score.div(problem_count);

  println!(" --------------- Domain score: {} --------------- \n", domain_score);

}

fn single_domain (problem_file_paths: ReadDir, mut domain_file_path: ReadDir, implementation: String) {

  let domain_path = domain_file_path.nth(0).unwrap();
  let domain_contents = fs::read_to_string(domain_path.unwrap().path()).expect("failed to read domain file");

  let mut collective_score: f64 = 0.0;
  let mut problem_count: f64 = 0.0;

  let mut problem_paths: Vec<_> = problem_file_paths.map(|r| r.unwrap()).collect();

  problem_paths.sort_by_key(|dir| dir.path());

  for problem_file_path in problem_paths {

    problem_count = problem_count.add(1.0);

    let now = Instant::now();

    let path_clone = problem_file_path.path().clone();

    let problem_contents = fs::read_to_string(path_clone.clone()).expect("failed to read problem file");

    let parse_result = parse_hddl( &problem_contents, &domain_contents);

    print!("Parsing: {} ", path_clone.display());
    std::io::stdout().flush().unwrap();

    let time_allowed: u64 = 1800;

    let imp_clone = implementation.clone();

    match parse_result {
        Ok((problem, domain)) => {

          let handle = thread::spawn(move || {

            let mut result = "No result";

            if imp_clone == "partial".to_string() {
              result = stoppable_depth_first_partial(&problem, &domain, &now, &path_clone, time_allowed);
            } else if imp_clone == "total".to_string() {
              result = stoppable_depth_first(&problem, &domain, &now, &path_clone, time_allowed);
            } else {
              println!("\nImplementation doesnt exist, try 'total' or 'partial'\n");
              return (result, now.elapsed().as_secs())
            }

            (result, now.elapsed().as_secs())
          });

          while now.elapsed().as_secs() < time_allowed {
            
            thread::sleep(Duration::from_millis(50));

            if handle.is_finished() {
              break;
            }
          }

          let (message, time) = handle.join().unwrap();
          print!("Result: {}, Time: {} seconds\n", message, time);

          let score = compute_score(time);

          collective_score = collective_score.add(score);
        },
        Err(e) => {
          println!("Failure: {}", e);
        }
    }
  }

  let domain_score = collective_score.div(problem_count);

  println!(" --------------- Domain score: {} --------------- \n", domain_score);
}

fn look_for_domain_file(files: ReadDir, problem_file: PathBuf) -> PathBuf {

  match problem_file.file_stem() {
    Some(file_no_ending) => {
      for file in files {

        let file_entry = file.unwrap();
        let file_name = file_entry.file_name();
        let domain_file_name = file_no_ending.to_os_string().into_string().unwrap().to_owned() + &"-domain.hddl".to_string();

        if file_name.into_string().unwrap().contains(&domain_file_name) {
          return file_entry.path()
        }
      }

      panic!("Didnt find domain file for: {:?}", file_no_ending);
    },
    None => {
      panic!("No ending!") 
    }
  }
}

fn compute_score(time: u64) -> f64 {

  let ftime = time as f64;
  let log_time = ftime.ln() as f64;
  let full_log_time = 1800_f64.ln();

  let f_res = 1_f64.sub(log_time.div(full_log_time));

  let onef = 1.0 as f64;

  return f64::min(onef, f_res);
}