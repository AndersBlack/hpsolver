use crate::problem::Problem;
use crate::problem::State;
use crate::problem::Htn;
use crate::problem::Object;
use std::collections::HashMap;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, is_a, take_while, take_until, take_till, is_not};
use nom::character::{is_alphabetic};
use nom::sequence::{terminated, delimited, pair, preceded};
use nom::multi::{separated_list0, many1, many0};
use parse_hyperlinks::take_until_unbalanced;

pub fn parse( input: &str ) -> IResult<&str, &str> {

  let (mut input, mut res) = delimited(tag("("), take_until_unbalanced('(', ')'), tag(")"))(input)?;

  let (mut input, vec_res) = fish_parentheses(res)?;

  //println!("Result: {:?}", vec_res);


  IResult::Ok((input, "test"))
}

pub fn fish_parentheses( input: &str ) -> IResult<&str, Problem> {

  let (remain, res) = many0( delimited(pair(take_until("("), tag("(")), take_until_unbalanced('(', ')'), tag(")")) )(input)?;

  // Initialize problem variables
  let mut initial_state: State = State::default();
  let mut problem_name = "";
  let mut objectives: Vec<Object> = Vec::new();
  let mut htn: Htn = Htn::default();
  let mut domain = "";

  for result in &res {
    if result.contains(":init") {
      let (input, state) = construct_init_state(result)?;
      println!("Init: {}", result);
    } else if result.contains("problem") {
      let (input, problem_name) = construct_problem(result)?;
      println!("prob: {}", input);
    } else if result.contains(":objects") {
      //let objectives: HashMap<String, String> = construct_objectives(result);
      println!("obj: {}", result);
    } else if result.contains(":htn") {
      //let htn: Htn = construct_htn(result);
      // TODO: REMOVE ORDERING AND ARRANGE SUBTASK PER ORDERING
      println!("htn: {}", result);
    } else if result.contains(":domain") {
      let (input, domain) = construct_domain(result)?;
      println!("dom: {}", input);
    }
  }

  //println!("RES: {}", input);

  let problem = Problem {
    name: problem_name.to_string(),
    domain: domain.to_string(),
    objects: objectives,
    htn: htn,
    state: initial_state,
  };

  IResult::Ok((input, problem))
}

fn construct_init_state( input: &str) -> IResult<&str, State> {

  let (input, res) = many0( delimited(pair(take_until("("), tag("(")), take_until_unbalanced('(', ')'), tag(")")) )(input)?;

  for result in res {
    //(input, res) = separated_list0(tag(' '), take_while(is_alphabetic))(result)?;
    println!("in init state: {:?}", result);
  }

  //let (input, res) = many0()(input)?;

  let mut map: HashMap<String, Vec<String>> = HashMap::new();

  todo!("construct initial state");
}

fn construct_objectives ( input: &str ) -> HashMap<String, String> {

  todo!("construct objectives");
} 

fn construct_htn ( input: &str ) -> Htn {
  todo!("construct htn");
} 

fn construct_domain ( input: &str ) -> IResult<&str, &str> {
  is_a(":domain ")(input)
}

fn construct_problem ( input: &str ) -> IResult<&str, &str> {
  is_a("problem ")(input)
}
