use core::f64;
use std::fs;
use std::path::PathBuf;
use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::io::Write;
use std::fs::ReadDir;

extern crate hp_solver;

use hp_solver::algorithms::stoppable_df_partial::stoppable_depth_first_partial;
use hp_solver::parser::parse_hddl;

fn main() {

  let category_folders_partial = fs::read_dir("problems/competition_problems/partial-order/").unwrap();
  let category_folders_total = fs::read_dir("problems/competition_problems/total-order/").unwrap();
  let joined_folders = category_folders_partial.chain(category_folders_total);

  for category_folder in joined_folders {

    println!("\n Category: {:?}\n", category_folder.as_ref().unwrap().path());

    let path_category_folder = category_folder.unwrap().path();
  
    let mut problem_folder_path = path_category_folder.clone();

    problem_folder_path.push("problems/");

    let mut domain_path = path_category_folder.clone();

    domain_path.push("domains/");
    
    match (fs::read_dir(problem_folder_path), fs::read_dir(domain_path)) {
      (Ok(problem_file_paths), Ok(domain_file_path)) => {

        single_domain(problem_file_paths, domain_file_path);

      }
      _ => {

        multiple_domain(path_category_folder);

      }
    }
  }
}

fn multiple_domain (category_folder: PathBuf) {

  let file_paths = fs::read_dir(category_folder.clone());

  let mut collective_score: f64 = 0.0;
  let mut problem_count: f64 = 0.0;

  match file_paths {
    Ok(files) => {

      for file in files {
        
        match file {
            Ok(dir_entry) => {

              let file_path = dir_entry.path();

              if !file_path.clone().into_os_string().into_string().unwrap().contains("-domain.hddl") && !file_path.clone().into_os_string().into_string().unwrap().contains(".md") && !file_path.clone().into_os_string().into_string().unwrap().contains("-domain.hddl") && !file_path.clone().into_os_string().into_string().unwrap().contains("solutions") {

                let domain_path = look_for_domain_file(fs::read_dir(category_folder.clone()).unwrap(), file_path.clone());
                let problem_path = file_path;

                problem_count = problem_count.add(1.0);
                let now = Instant::now();

                let domain_contents = fs::read_to_string(domain_path).expect("failed to read domain file");
                let problem_contents = fs::read_to_string(problem_path.clone()).expect("failed to read problem file");

                let parse_result = parse_hddl( &problem_contents, &domain_contents);

                std::io::stdout().flush().unwrap();

                let time_allowed: u64 = 1800;

                match parse_result {
                    Ok((problem, domain)) => {

                      let handle = thread::spawn(move || {
                        let result = stoppable_depth_first_partial(&problem, &domain, &now, &problem_path, time_allowed);

                        (result, now.elapsed().as_secs())
                      });

                      while now.elapsed().as_secs() < time_allowed {
                        
                        thread::sleep(Duration::from_millis(50));

                        if handle.is_finished() {
                          break;
                        }
                      }

                      let (_message, time) = handle.join().unwrap();
                      let score = compute_score(time);

                      collective_score = collective_score.add(score);
                    },
                    Err(e) => {
                      println!("Failure: {}", e);
                    }
                }
              }

            },
            Err(error) => {
              panic!("path failed: {}", error);
            }
        }
      }

    },
    _ => {
      panic!("Unknown folder");
    }
  }

  let domain_score = collective_score.div(problem_count);

  println!("\n --------------- Domain score: {} --------------- \n", domain_score);

}

fn single_domain (problem_file_paths: ReadDir, mut domain_file_path: ReadDir) {

  let domain_path = domain_file_path.nth(0).unwrap();

  let domain_contents = fs::read_to_string(domain_path.unwrap().path()).expect("failed to read domain file");

  let mut collective_score: f64 = 0.0;
  let mut problem_count: f64 = 0.0;

  for problem_file_path in problem_file_paths {

    problem_count = problem_count.add(1.0);

    let now = Instant::now();
    let path_clone = problem_file_path.unwrap().path().clone();
    let problem_contents = fs::read_to_string(path_clone.clone()).expect("failed to read problem file");
    let parse_result = parse_hddl( &problem_contents, &domain_contents);

    std::io::stdout().flush().unwrap();

    let time_allowed: u64 = 1800;

    match parse_result {
        Ok((problem, domain)) => {

          let handle = thread::spawn(move || {
            let result = stoppable_depth_first_partial(&problem, &domain, &now, &path_clone, time_allowed);

            (result, now.elapsed().as_secs())
          });

          while now.elapsed().as_secs() < time_allowed {
            
            thread::sleep(Duration::from_millis(50));

            if handle.is_finished() {
              break;
            }
          }

          let (_message, time) = handle.join().unwrap();

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