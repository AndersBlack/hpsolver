
#[cfg(test)]
mod test {

use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use std::process::Command;

use crate::parser::parse_hddl;
use crate::algorithms::stoppable_df_partial::stoppable_depth_first_partial;

#[test] 
fn test_satelite() {

  let problem_path = "problems/competition_problems/partial-order/Satellite/problems/1obs-1sat-1mod.hddl";
  let domain_path = "problems/competition_problems/partial-order/Satellite/domains/domain.hddl";
  let solution_path = "problems/competition_problems/partial-order/Satellite/solutions/1obs-1sat-1mod.solution";

  let result = run_problem_df_test(&problem_path, domain_path);
  let verification = Command::new("additional_software/pandaPIparser/pandaPIparser").args(["--verify", 
                                                                                                          domain_path,
                                                                                                          problem_path, 
                                                                                                          solution_path]).output().unwrap();

  let plan = String::from_utf8(verification.stdout);
  let mut plan_true = false;

  match plan {
   Ok(plan_string) => {
    if plan_string.contains("Plan verification result: \u{1b}[1;32mtrue") { plan_true = true; }
   },
   Err(_) => {}   
  }
  
  assert_eq!(result, "success".to_string());
  assert_eq!(plan_true, true);
}   

#[test] 
fn test_hiking() {

  let problem_path = "problems/competition_problems/total-order/group2/Hiking/problems/p01.hddl";
  let domain_path = "problems/competition_problems/total-order/group2/Hiking/domains/domain.hddl";
  let solution_path = "problems/competition_problems/total-order/group2/Hiking/solutions/p01.solution";

  let result = run_problem_df_test(&problem_path, domain_path);
  let verification = Command::new("additional_software/pandaPIparser/pandaPIparser").args(["--verify", 
                                                                                                          domain_path,
                                                                                                          problem_path, 
                                                                                                          solution_path]).output().unwrap();

  let plan = String::from_utf8(verification.stdout);
  let mut plan_true = false;

  match plan {
   Ok(plan_string) => {
    if plan_string.contains("Plan verification result: \u{1b}[1;32mtrue") { plan_true = true; }
   },
   Err(_) => {}   
  }
  
  assert_eq!(result, "success".to_string());
  assert_eq!(plan_true, true);
} 

#[test] 
fn test_snake() {

  let problem_path = "problems/competition_problems/total-order/group3/Snake/problems/pb01.snake.hddl";
  let domain_path = "problems/competition_problems/total-order/group3/Snake/domains/domain.hddl";
  let solution_path = "problems/competition_problems/total-order/group3/Snake/solutions/pb01.solution";

  let result = run_problem_df_test(&problem_path, domain_path);
  let verification = Command::new("additional_software/pandaPIparser/pandaPIparser").args(["--verify", 
                                                                                                          domain_path,
                                                                                                          problem_path, 
                                                                                                          solution_path]).output().unwrap();

  let plan = String::from_utf8(verification.stdout);
  let mut plan_true = false;

  match plan {
   Ok(plan_string) => {
    if plan_string.contains("Plan verification result: \u{1b}[1;32mtrue") { plan_true = true; }
   },
   Err(_) => {}   
  }
  
  assert_eq!(result, "success".to_string());
  assert_eq!(plan_true, true);
} 

fn run_problem_df_test(problem_path: &str, domain_path: &str) -> String {

  let now = Instant::now();

  let problem_contents = fs::read_to_string(problem_path.to_string()).expect("failed to read problem file");
  let domain_contents = fs::read_to_string(domain_path.to_string()).expect("failed to read domain file");

  let parse_result = parse_hddl( &problem_contents, &domain_contents);

  let mut pb = PathBuf::new();

  pb.push(problem_path);

  let res;

  match parse_result {
      Ok((problem,domain)) => {
        res = stoppable_depth_first_partial(&problem, &domain, &now,&pb, 1800);
      },
      Err(e) => {
        res = e;
      }
  }

  res.to_string()
}

}