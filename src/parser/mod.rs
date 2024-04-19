mod problem;
mod domain;

use problem::problem_parser;
use domain::domain_parser;

use crate::datastructures::{domain::*, problem::*};

use nom::IResult;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::alphanumeric1;
use nom::multi::many1;
use nom::bytes::complete::{is_not, take_until};
use nom::combinator::eof;
use nom::multi::many_till;
use nom::error::context;

type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);

/// Parses 2 strings in form of a problem.hddl and a domain.hddl and returns a tuple of the datastructures for each
pub fn parse_hddl( input_problem: &str, input_domain: &str ) -> Result<(Problem, Domain), &'static str> {

  let no_comment_problem = if let Ok((_resprob, prob)) = clean_for_comments(input_problem) {
    prob
  } else {
    println!("Unable to remove comment error");
    return Err("error, didnt remove comment problem")
  };

  let no_comment_domain = if let Ok((_resdom, dom)) = clean_for_comments(input_domain) {
    dom
  } else {
    println!("Unable to remove comment error");
    return Err("error, didnt remove comment domain")
  };

  let problem_res = problem_parser(&no_comment_problem);
  let domain_res = domain_parser(&no_comment_domain);

  match (problem_res, domain_res) {
    (Ok((_res_problem, problem)), Ok((_res_domian, domain))) => {
      return Ok((problem, domain));
    },
    (Err(problem_err), Ok((_res_domian, _domain))) => {
      print!("problem didnt parse: {}\n", problem_err);
      return Err("error, didnt parse domain")
    },
    (Ok((_res_domian, _problem)), Err(domain_err)) => {
      print!("Domain didnt parse: {}\n", domain_err);
      return Err("error, didnt parse domain")
    },
    (Err(problem_err), Err(domain_err)) => {
      print!("Domain didnt parse: {} and problem didnt parse: {}\n", domain_err, problem_err);
      return Err("error, didnt parse domain and problem")
    }
  }
}

fn combine_precon_and_constraint(constraints: Vec<(bool, String, String)>, preconditions: Option<Vec<Precondition>>) -> Option<Vec<Precondition>>  {

  let mut precondition_list;

  if preconditions.is_some() {
    precondition_list = preconditions.unwrap();
  } else {
    precondition_list = Vec::<Precondition>::new();
  }

  for constraint in constraints {

    if constraint.0 {
      precondition_list.push((2, "no pred".to_string(), vec![constraint.1, constraint.2], None));
    } else {
      precondition_list.push((3, "no pred".to_string(), vec![constraint.1, constraint.2], None));
    }

  }
  
  Some(precondition_list)
}

// ------------------------- TOOL FUNCTIONS ----------------------------------------

fn underscore_stringer( input: &str ) -> IResult<&str, String> {
  context("underscore stringer",
    many1(
      alt((
        alphanumeric1,
        tag("_"),
        tag("-"),
        tag("?")
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

fn clean_for_comments (input: &str) -> IResult<&str, String> {

  context("remove comment",
      many_till(
          alt((
              is_not(";"),
              take_until("\n")
            )), 
        eof)
    )(input)
    .map(|(next_input, res)| {
      
      let mut return_string: String = String::new();
      let mut x = 0;

      while x < res.0.len() {

        if !res.0[x].contains(";") {
          return_string = return_string + res.0[x];
        }

        x = x + 1;
      }

      (
        next_input, return_string
      )
    })
}

fn order_subtasks(subtasks: Vec<(String, String, Vec<String>)>, ordering: &Option<Vec<(String, String, String)>>) -> Vec<(String, String, Vec<String>)> {

  match ordering {
    Some(ordering) => {

      if ordering.len() == 0 { return subtasks }

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

      let mut task_we_are_looking_for = "empty".to_string();
      let st_length = subtasks.len();

      while sorted_subs.len() != st_length {

        for node in &degree_list {

          if task_we_are_looking_for == "empty" {

            if node.0 == 0 {

              for sub in &subtasks {
                if sub.1 == node.1 {
                  sorted_subs.push((sub.0.clone(), sub.1.clone(), sub.2.clone()));
                  task_we_are_looking_for = node.2[0].clone();
                }
              }

              break;
            }

          } else if task_we_are_looking_for == node.1 {

            for sub in &subtasks {
              if sub.1 == node.1 {
                sorted_subs.push((sub.0.clone(), sub.1.clone(), sub.2.clone()));

                if !node.2.is_empty() {
                  task_we_are_looking_for = node.2[0].clone();
                }
              }
            }

          }

        }

      }

      sorted_subs
    },
    None => {
      subtasks
    }    
  }

  
}