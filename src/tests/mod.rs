use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use std::process::Command;

use crate::parser::parse_hddl;
use crate::algorithms::stoppable_df_partial::stoppable_depth_first_partial;

#[test]
fn test_satelite() {
  let result = run_problem_df_test("problems/competition_problems/partial-order/Satellite/problems/1obs-1sat-1mod.hddl", "problems/competition_problems/partial-order/Satellite/domains/domain.hddl");

  let verification = Command::new("additional_software/pandaPIparser/pandaPIparser").args(["-verify", 
                                                                                                          "problems/competition_problems/partial-order/Satellite/domains/domain.hddl",
                                                                                                          "problems/competition_problems/partial-order/Satellite/problems/1obs-1sat-1mod.hddl", 
                                                                                                          "problems/competition_problems/partial-order/Satellite/solutions/1obs-1sat-1mod.solution"]).output().unwrap();

  println!("v: {:?}", verification);

  assert_eq!(result, "success".to_string());
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
        println!("\nFinished parsing problem and domain!\n");
        res = stoppable_depth_first_partial(&problem, &domain, &now,&pb, 3600);
      },
      Err(e) => {
        println!("Failure parsing: {}", e);
        res = e;
      }
  }

  res.to_string()
}