use crate::datastructures::problem::*;
use crate::parser::{underscore_stringer, order_subtasks};

use nom::IResult;
//use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::branch::alt;
use nom::combinator::{opt, not};
use nom::character::complete::{alphanumeric1, multispace0};
use nom::sequence::{tuple, pair};
use nom::multi::{many1, many0};
use nom::error::context;

// ------------------------- PROBLEM PARSER ----------------------------------------

pub fn problem_parser( input: &str ) -> IResult<&str, Problem> {

  context("problem", 
  tuple((
    take_until("(define"),
    get_name,
    get_domain,
    opt(get_objects),
    get_htn,
    get_init,
    opt(get_goal)
  ))
)(input)
.map(|(next_input, res)| {
    let (_, name, domain, objects, htn, state, goal) = res;
    
    let return_objects;

    if objects.clone().is_none() {
      return_objects = Vec::<(String, String, Vec<String>)>::new();
    } else {
      return_objects = objects.unwrap();
    }

    let problem = Problem {
      name,
      domain,
      objects: return_objects,
      htn,
      state,
      goal
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
      multispace0,
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_define, _newline1, _problem, _, name, _, _tag, _newline2) = res;
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
      multispace0,
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_domain_keyword, _, domain, _tag, _, _newline) = res;

    (
      next_input, domain.to_string()
    )
  })
}

fn get_objects( input: &str ) -> IResult<&str, Vec<(String, String, Vec<String>)>> {
  //println!("object input:\n{}", input);

  context("objects",
    tuple((
      tag("(:objects"),
      multispace0,
      many1(
        tuple((
          many1(
            tuple((
              not(tag("-")),
              underscore_stringer,
              multispace0
          ))
          ),
          tag("- "),
          underscore_stringer,
          multispace0
        ))
      ),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {

    let (_object_tag, _, object_list, _tag1, _newline2) = res;

    let mut obj_vec = Vec::<(String, String, Vec<String>)>::new();

    for result in object_list {
      for value in result.0 {
        let obj = (value.1, result.2.clone(), Vec::<String>::new());
        obj_vec.push(obj);
      }
    };
    
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
      opt(get_htn_parameters),
      get_htn_subtasks,
      opt(get_htn_ordering),
      opt(tag(":constraints ( )")),
      multispace0,
      opt(tag(")")),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_, _header, _newline, parameters, subtasks, ordering, _, _, _tag, _ws) = res;

    let sorted_subtasks = order_subtasks(subtasks, &ordering);

    let mut parms = Vec::<(String, String)>::new();

    match parameters {
      Some(params) => { parms = params },
      None => {}
    }

    let htn = Htn {
      parameters: parms,
      subtasks: sorted_subtasks
    };

    (
      next_input, htn 
    )
  })

}

fn get_htn_parameters( input: &str) -> IResult<&str, Vec<(String,String)>> {
  //println!("htn_parameters input:\n{}", input);

  context("parameters", 
    tuple((
      multispace0,
      tag(":parameters "),
      tag("("),
      multispace0,
      opt(many0(tuple
        ((tag("?"), underscore_stringer, tag(" - "), underscore_stringer, multispace0))
      )),
      tag(")"),
      multispace0,
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_, _header, _tag1, _, parameters, _tag2, _) = res;

    let mut parameters_vec = Vec::<(String, String)>::new();

    // FILL VECTOR FROM 'parameters' variable
    match parameters {
      Some(parameters) => {
        for param in parameters {
          let (_qm, arg, _dash, type_name, _ms) = param;
          parameters_vec.push(("?".to_string() + &arg, type_name.to_string()));
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
      alt((tag(":subtasks"), tag(":tasks"),tag(":ordered-subtasks"), tag(":ordered-tasks"))),
      multispace0,
      opt(tag("(and")),
      multispace0,
      many0(
        tuple((
          tag("("),
          underscore_stringer,
          multispace0,
          opt(pair(tag("("), underscore_stringer)),
          multispace0,
          many0(tuple((
            opt(tag("?")),
            underscore_stringer,
            multispace0
          ))),
          tag(")"),
          multispace0,
          opt(tag(")")),
          multispace0
        ))
      ),
      opt(tag(")")),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let(_ws1, _tag1, _, _, _, tuple, _tag0, _ws2) = res;

    let mut subtask_vec: Vec<(String, String, Vec<String>)> = Vec::<(String, String, Vec<String>)>::new();

    // TODO: MOVE 
    for task in tuple {

      let mut obj_vec = Vec::<String>::new();

      //Construct obj vector
      for obj in task.5 {
        match obj.0 {
          Some(_task0) => { obj_vec.push("?".to_string() + &obj.1) },
          None => { obj_vec.push(obj.1); }
        }
      }

      match task.3 {
        Some(task3) => { subtask_vec.push((task3.1.to_string(), task.1, obj_vec)); },
        None => { subtask_vec.push((task.1.to_string(), "No alias".to_string(), obj_vec)); }
      }
    }

    //println!("{:?}", subtask_vec);

    (next_input, subtask_vec)
  })
}

fn get_htn_ordering( input: &str) -> IResult<&str, Vec<(String, String, String)>> {
  //println!("htn_ordering input:\n{}", input);

  context("ordering", 
    tuple((
      alt((tag(":ordering (and"), tag(":ordering ( )"))),
      multispace0,
      many0(
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
      opt(tag(")")),
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

fn get_init( input: &str ) -> IResult<&str, Vec<(String, Vec<String>)>> {
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
              underscore_stringer,
              opt(tag(" "))
            ))
          ),
          tag(")"),
          multispace0
        ))
      ),
      opt(tag(")")),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tag0, _ws0, list, _tag1, _ws1) = res;

    let mut state_vector = Vec::<(String, Vec<String>)>::new();

    for state_var in list {
      let mut name = String::new();
      let mut found_name = false;
      let mut arg_vec = Vec::<String>::new();

      for state_attribute in state_var.1 {

        if found_name {
          arg_vec.push(state_attribute.0);
        } else {
          name = state_attribute.0;
          found_name = true;
        }
      }

      state_vector.push((name, arg_vec));
    }

    //println!("State_vector: {:?}", state_vector);

    (
      next_input, state_vector
    )
  })
}

fn get_goal( input: &str ) -> IResult<&str, Vec<(String, Vec<String>)>> {
  //println!("goal input:\n{}", input);

  context("name", 
    tuple((
      tag("(:goal"),
      multispace0,
      opt(tag("(and")),
      multispace0,
      many1(
        tuple((
          tag("("),
          underscore_stringer,
          multispace0,
          many0(
            tuple((
              underscore_stringer,
              multispace0
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
    let (_, _, _, _, goal_list, _, _, _, _) = res;

    let mut final_goal = Vec::<(String, Vec<String>)>::new(); 

    for goal in goal_list {
        let mut goal_params = Vec::<String>::new();

        for goal_param in goal.3 {
          goal_params.push(goal_param.0);
        }

        final_goal.push((goal.1, goal_params));
    }

    (
      next_input, final_goal
    )
  })

}