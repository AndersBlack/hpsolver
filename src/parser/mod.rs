mod problem;
mod domain;

use problem::problem_parser;
use domain::domain_parser;

use crate::problem::*;
use crate::domain::*;

use nom::IResult;
use nom::bytes::complete::{tag};
use nom::branch::{alt};
use nom::character::complete::{alphanumeric1};
use nom::multi::{many1};
use nom::error::{context};

/// Parses 2 strings in form of a problem.hddl and a domain.hddl and returns a tuple of the datastructures for each
pub fn parse_hddl( input_problem: &str, input_domain: &str ) -> (Problem, Domain) {

  let (_res_problem, problem) = if let Ok((res_problem, problem)) = problem_parser(input_problem) {
    (res_problem, problem)
  } else if let Err(error) = problem_parser(input_problem) {
    println!("PROBLEM DIDNT PARSE: {} \n", error);
    panic!("error, didnt parse problem")
  } else {
    println!("Unable to produce error");
    panic!("error, didnt parse problem")
  };

  let (_res_domain, domain) = if let Ok((res_problem, domain)) = domain_parser(input_domain) {
    (res_problem, domain)
  } else if let Err(error) = domain_parser(input_domain) {
    println!("DOMAIN DIDNT PARSE: {}", error);
    panic!("error, didnt parse domain")
  } else {
    println!("Unable to produce error");
    panic!("error, didnt parse domain")
  };

  (problem, domain)
}

// ------------------------- TOOL FUNCTIONS ----------------------------------------

fn underscore_matcher(x: String, y: &str) -> String {
  format!("{}_{}", x, y)
}

fn underscore_stringer( input: &str ) -> IResult<&str, String> {
  context("underscore stringer",
    many1(
      alt((
        alphanumeric1,
        tag("_")
      ))
    )
  )(input)
  .map(|(next_input, res)| {
    let string_list = res;

    let mut final_string = String::new();

    for part in string_list {
      final_string = format!("{}{}", final_string, part);
    }

    (
      next_input, final_string
    ) 
  })
}

fn order_subtasks(subtasks: Vec<(String, String, Vec<String>)>, ordering: Option<Vec<(String, String, String)>>) -> Vec<(String, String, Vec<String>)> {  
  //println!("SUBS: {:?} \n ORDERING: {:?}", subtasks, ordering);

  match ordering {
    Some(ordering) => {
      let mut sorted_subs = Vec::<(String, String, Vec<String>)>::new();

      let mut degree_list = Vec::<(i32, String, Vec<String>)>::new();
    
      for sub in &subtasks {
        let point_vec = Vec::<String>::new();
        degree_list.push((0, sub.1.to_string(), point_vec));
      }
    
      // Building the graph 
      for order in ordering {
        for mut node in &mut degree_list {
    
          if order.0 == "<".to_string() {
            if node.1 == order.1 {
              node.2.push(order.2.clone());
            } else if node.1 == order.2 {
              node.0 = node.0 + 1;
            }
          } else {
            if node.1 == order.1 {
              node.0 = node.0 + 1;
            } else if node.1 == order.2 {
              node.2.push(order.2.clone());
            }
          }
    
        }
      }
    
      let mut degree_counter = 0;
      let mut push_counter = 0;
      while push_counter < degree_list.len() {
        for node in &mut degree_list {
          if node.0 == degree_counter {
            for sub in &subtasks {
              if node.1 == sub.1 {
                push_counter = push_counter + 1;
                sorted_subs.push((sub.0.clone(), sub.1.clone(), sub.2.clone()));
              }
            }
          }
        }
        degree_counter = degree_counter + 1;
      }
    
      sorted_subs
    },
    None => {
      subtasks
    }    
  }

  
}