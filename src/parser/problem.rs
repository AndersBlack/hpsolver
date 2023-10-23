use crate::problem::*;

use crate::parser::{underscore_stringer, underscore_matcher, order_subtasks};

use nom::IResult;
//use nom::branch::alt;
use nom::bytes::complete::{tag};
use nom::branch::{alt};
use nom::combinator::{opt};
use nom::character::complete::{alphanumeric1, multispace0};
use nom::sequence::{tuple};
use nom::multi::{many1, many0};
use nom::error::{context};

// ------------------------- PROBLEM PARSER ----------------------------------------

pub fn problem_parser( input: &str ) -> IResult<&str, Problem> {
  
    context("problem", 
      tuple((
        get_name,
        get_domain,
        get_objects,
        get_htn,
        get_init,
      ))
    )(input)
    .map(|(next_input, res)| {
        let (name, domain, objects, htn, state) = res;

        let problem = Problem {
          name,
          domain,
          objects,
          htn,
          state,
        };

        (
          next_input,
          problem
        )
      })
  
  }
  
  fn get_name( input: &str ) -> IResult<&str, String> {
    //println!("name input:\n{}", input);
  
    context("name", 
      tuple((
        tag("(define"),
        multispace0,
        tag("(problem"),
        multispace0,
        underscore_stringer,
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_define, _newline1, _problem, name, _, _tag, _newline2) = res;
      (
        next_input, name.to_string()
      )
    })
  }
  
  fn get_domain( input: &str ) -> IResult<&str, String> {
    //println!("Domain input: {}", input);
  
    context("domain", 
      tuple((
        tag("(:domain"),
        multispace0,
        underscore_stringer,
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_domain_keyword, _, domain, _tag, _newline) = res;
  
      (
        next_input, domain.to_string()
      )
    })
  }
  
  fn get_objects( input: &str ) -> IResult<&str, Vec<Object>> {
    //println!("object input:\n{}", input);
  
    context("objects",
      tuple((
        tag("(:objects"),
        many1(
          tuple((
            multispace0,
            underscore_stringer, 
            tag(" - "),
            underscore_stringer,
            multispace0
          ))
        ),
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
  
      let (_object_tag, object_list, _tag1, _newline2) = res;
  
      let mut obj_vec = Vec::<Object>::new();
  
      for result in object_list {
  
        let (_, name, _filler, obj_type, _newline) = result;
  
        let obj = Object {
          object: (name, obj_type)
        };
  
        obj_vec.push(obj);
      }
  
      //println!("{:?}", obj_vec);      
  
      (
        next_input, obj_vec
      )
    })
  }
  
  fn get_htn( input: &str ) -> IResult<&str, Htn> {
    //println!("htn input:\n{}", input);
  
    context("name", 
      tuple((
        multispace0,
        tag("(:htn"),
        multispace0,
        get_htn_parameters,
        get_htn_subtasks,
        opt(get_htn_ordering),
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_, _header, _newline, parameters, subtasks, ordering, _tag, _ws) = res;
  
      let sorted_subtasks = order_subtasks(subtasks, ordering);
  
      //println!("{:?}", ordering);
  
      let htn = Htn {
        parameters: parameters,
        subtasks: sorted_subtasks
      };
  
      (
        next_input, htn 
      )
    })
  
  }
  
  fn get_htn_parameters( input: &str) -> IResult<&str, Vec<String>> {
    //println!("htn_parameters input:\n{}", input);
  
    context("parameters", 
      tuple((
        multispace0,
        tag(":parameters "),
        tag("("),
        opt(many0(tuple
          ((alphanumeric1, tag(" - "), alphanumeric1))
        )),
        tag(")"),
        multispace0,
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_, _header, _tag1, parameters, _tag2, _) = res;
  
      let mut parameters_vec = Vec::<String>::new();
  
      // FILL VECTOR FROM 'parameters' variable
      match parameters {
        Some(parameters) => {
          for param in parameters {
            let (_arg, _dash, type_name) = param;
            parameters_vec.push(type_name.to_string());
          }
        },
        None =>  { println!("Found no parameters"); }
      }
  
      (
        next_input, parameters_vec 
      )
    })
  }
  
  fn get_htn_subtasks( input: &str) -> IResult<&str, Vec<(String, String, Vec<String>)>> {
    //println!("htn_subtasks input:\n{}", input);
  
    context("subtasks",
      tuple((
        multispace0,
        tag(":subtasks (and"),
        multispace0,
        many0(
          tuple((
            tag("("),
            underscore_stringer,
            tag(" ("),
            underscore_stringer,
            multispace0,
            many1(tuple((
              underscore_stringer,
              multispace0
            ))),
            tag("))"),
            multispace0
          ))
        ),
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let(_ws1, _tag1, _, tuple, _tag0, _ws2) = res;
  
      let mut subtask_vec: Vec<(String, String, Vec<String>)> = Vec::<(String, String, Vec<String>)>::new();
  
      // TODO: MOVE 
      for task in tuple {
  
        let mut obj_vec = Vec::<String>::new();
  
        //Construct obj vector
        for obj in task.5 {
          obj_vec.push(obj.0);
        }
  
        subtask_vec.push((task.3, task.1, obj_vec));
      }
  
      (next_input, subtask_vec)
    })
  }
  
  fn get_htn_ordering( input: &str) -> IResult<&str, Vec<(String, String, String)>> {
    //println!("htn_ordering input:\n{}", input);
  
    context("ordering", 
      tuple((
        tag(":ordering (and"),
        multispace0,
        many1(
          alt((
            tuple((
              tag("("),
              tag("<"),
              tag(" "),
              alphanumeric1,
              tag(" "),
              alphanumeric1,
              tag(")"),
              multispace0
            )),
            tuple((
              tag("("),
              tag(">"),
              tag(" "),
              alphanumeric1,
              tag(" "),
              alphanumeric1,
              tag(")"),
              multispace0
            ))
          ))
        ),
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_tag0, _ws0, list, _tag1, _ws2) = res;
  
      let mut order_vec = Vec::<(String, String, String)>::new();
  
      for orders in list {
        order_vec.push((orders.1.to_string(), orders.3.to_string(), orders.5.to_string()));
      }
  
      (
        next_input, order_vec 
      )
    })
  }
  
  fn get_init( input: &str ) -> IResult<&str, State> {
    //println!("htn_init input:\n{}", input);
  
    context("init", 
      tuple((
        tag("(:init"),
        multispace0,
        many1(
          tuple((
            tag("("),
            many1(
              tuple((
                alphanumeric1,
                many0(
                  tuple((
                    tag("_"),
                    alphanumeric1
                  ))
                ),
                opt(tag(" "))
              ))
            ),
            tag(")"),
            multispace0
        ))
        ),
        tag(")"),
        multispace0,
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_tag0, _ws0, list, _tag1, _ws1, _tag2, _ws2) = res;
  
      let mut state_vector = Vec::<(String, Vec<String>)>::new();
  
      for state_var in list {
        let mut main_name_found = false;
        let mut main_name = String::new();  
        let mut var_list = Vec::<String>::new();
        
        for state_details in &state_var.1 {
  
          // Make the name
          let mut state_var_name = state_details.0.to_string();
          if state_var.1.len() != 0 {
            for name_extension in &state_details.1 {
              state_var_name = underscore_matcher(state_var_name, name_extension.1);
            }
          }
  
          if main_name_found == false {
            main_name = state_var_name;
            main_name_found = true;
          } else {
            var_list.push(state_var_name);
          }
        }
  
        state_vector.push((main_name, var_list));
      }
  
      //println!("State_vector: {:?}", state_vector);
  
      let init_state = State {
        state_variables: state_vector
      };
  
      (
        next_input, init_state
      )
    })
  }