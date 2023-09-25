use crate::problem::Problem;
use crate::problem::State;
use crate::problem::Htn;
use crate::problem::Object;
use crate::domain::Task;

use crate::domain::*;

use std::collections::HashMap;

use nom::IResult;
//use nom::branch::alt;
use nom::bytes::complete::{tag, is_a, take_while, take_until, take_till, is_not};
use nom::branch::{permutation, alt};
use nom::combinator::{opt};
use nom::character::{is_alphabetic, is_digit, is_alphanumeric};
use nom::character::complete::{newline, tab, alphanumeric1, anychar, multispace0};
use nom::sequence::{terminated, delimited, pair, preceded, tuple};
use nom::multi::{separated_list0, many1, many0, count};
use nom::error::{context};
use parse_hyperlinks::take_until_unbalanced;

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
      (
        next_input,
        Problem {
          name,
          domain,
          objects,
          htn,
          state,
        },
      )
    })

}

fn get_name( input: &str ) -> IResult<&str, String> {
  //println!("name input:\n{}", input);

  context("name", 
    tuple((
      tag("(define"),
      newline,
      tab,
      tag("(problem "),
      take_until(")"),
      tag(")"),
      newline
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_define, _newline1, _tab, _problem, name, _tag, _newline2) = res;
    (
      next_input, name.to_string()
    )
  })
}

fn get_domain( input: &str ) -> IResult<&str, String> {
  //println!("Domain input: {}", input);

  context("domain", 
    tuple((
      tab,
      tag("(:domain "),
      take_until(")"),
      tag(")"),
      newline
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tab, _domain_keyword, domain, _tag, _newline) = res;
    (
      next_input, domain.to_string()
    )
  })
}

fn get_objects( input: &str ) -> IResult<&str, Vec<Object>> {
  //println!("object input:\n{}", input);

  context("objects",
    tuple((
      tab,
      tag("(:objects"),
      newline,
      many1(
        tuple((
          tab,
          tab,
          alphanumeric1, opt(tag("_")), opt(alphanumeric1), 
          tag(" - "),
          alphanumeric1, opt(tag("_")), opt(alphanumeric1),
          newline
        ))
      ),
      tab,
      tag(")"),
      newline
    ))
  )(input)
  .map(|(next_input, res)| {

    let (_tab1, _object_tag, _newline1, obj_list, _tab2, _tag1, _newline2) = res;

    let mut obj_vec = Vec::<Object>::new();

    for result in obj_list {

      let (_tab1, _tab2, name, underscore0, name_ending, _filler, obj_type, underscore1, obj_type_ending, _newline) = result;

      let obj = Object {
        //object: (name.to_string(), obj_type.to_string())
        object: (option_underscore_matcher(name, name_ending), option_underscore_matcher(obj_type, obj_type_ending))
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
      tab,
      tag("(:htn"),
      newline,
      get_htn_parameters,
      get_htn_subtasks,
      opt(get_htn_ordering),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tab, _header, _newline, parameters, subtasks, ordering, _tag, _ws) = res;

    let mut sorted_subtasks = Vec::<(String, String, Vec<String>)>::new();

    //println!("{:?}", ordering);

    match ordering {
      Some(ordering) => {
        sorted_subtasks = order_subtasks(subtasks, ordering);
      },
      None => sorted_subtasks = subtasks
    }

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
      tab,
      tab,
      tag(":parameters "),
      tag("("),
      opt(many0(tuple
        ((alphanumeric1, tag(" - "), alphanumeric1))
      )),
      tag(")"),
      newline,
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tab0, _tab1, _header, _tag1, parameters, _tag2, _newline) = res;

    let mut parameters_vec = Vec::<String>::new();

    // FILL VECTOR FROM 'parameters' variable
    match parameters {
      Some(parameters) => {
        for param in parameters {
          let (arg, dash, type_name) = param;
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
      newline,
      many1(
        tuple((
          tab,
          tab,
          tag(" ("),
          alphanumeric1,
          tag(" ("),
          many1(tuple((
            alphanumeric1,
            many0(tuple((
              tag("_"),
              alphanumeric1
            )))
          ))),
          many1(tuple((
            tag(" "),
            alphanumeric1,
            many0(tuple((
              tag("_"),
              alphanumeric1
            )))
          ))),
          tag("))"),
          newline
        ))
      ),
      tab,
      tab,
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let(_ws1, _newline1, _tag1, tuple, _tab0, _tab1, _tag0, _ws2) = res;

    let mut subtask_vec: Vec<(String, String, Vec<String>)> = Vec::<(String, String, Vec<String>)>::new();
    
    //println!("{:?}", tuple);

    // TODO: MOVE 
    for task in tuple {

      let mut obj_vec = Vec::<String>::new();
      let mut task_name = String::new();

      //Construct task_name
      for obj in task.5 {

        task_name = obj.0.to_string();
        if obj.1.len() != 0 {
          for name_extension in obj.1 {
            task_name = underscore_matcher(task_name, name_extension.1);
          }
        }
      }

      //Construct obj vector
      for obj in task.6 {
        let mut obj_name = obj.1.to_string();
        if obj.2.len() != 0 {
          for name_extension in obj.2 {
            obj_name = underscore_matcher(obj_name, name_extension.1);
          }
        }
        obj_vec.push(obj_name);
      }

      subtask_vec.push((task_name, task.3.to_string(), obj_vec));
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

fn order_subtasks(subtasks: Vec<(String, String, Vec<String>)>, ordering: Vec<(String, String, String)>) -> Vec<(String, String, Vec<String>)> {  
  //println!("SUBS: {:?} \n ORDERING: {:?}", subtasks, ordering);

  let mut sorted_subs = Vec::<(String, String, Vec<String>)>::new();

  let mut degree_list = Vec::<(i32, String, Vec<String>)>::new();

  for sub in &subtasks {
    let mut point_vec = Vec::<String>::new();
    degree_list.push((0, sub.1.to_string(), point_vec));
  }

  // Building the graph 
  for mut order in ordering {
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

// ------------------------- DOMAIN PARSER ----------------------------------------

pub fn domain_parser( input: &str ) -> IResult<&str, Domain> {

  context("domain", 
    tuple((
      get_domain_name,
      get_domain_types,
      get_domain_predicates,
      get_domain_tasks,
      get_domain_methods,
      get_domain_actions
    ))
  )(input)
  .map(|(next_input, res)| {
      let (domain_name, types, predicates, tasks, methods, actions) = res;

      (
        next_input,
        Domain {
          name: domain_name,
          tasks: tasks,
          methods: methods,
          actions: actions,
          types: types,
          predicates: predicates
        }
      )
    })

}

fn get_domain_name( input: &str ) -> IResult<&str, String> { 
  //println!("Input for domain name:{}", input);

  context("domain name", 
    tuple((
      tag("(define"),
      multispace0,
      tag("(domain "),
      alphanumeric1,
      many0(tuple((
        tag("_"),
        alphanumeric1
      ))),
      tag(")"),
      multispace0,
      take_until(")"),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tag0, _ws0, _tag1, start_of_name, end_of_name, _tag2, _ws1, _take, _tag3, _ws2) = res;

    //println!("NAME: {} {:?}", start_of_name, end_of_name);

    let mut name = start_of_name.to_string();

    if end_of_name.len() != 0 {
      for ending in end_of_name {
         name = underscore_matcher(name, ending.1);
      }
    }

    (
      next_input, name
    )
  })
}

fn get_domain_types( input: &str ) -> IResult<&str, Vec<Type>> { 
  //println!("Input for domain types:{}", input);

  context("domain name", 
    tuple((
      tag("(:types"),
      multispace0,
      many1(
        tuple((
          alphanumeric1,
          many0(
            tuple((
              tag("_"),
              alphanumeric1
            ))
          ),
          opt(tuple((
            tag(" - "),
            alphanumeric1
          ))),
          multispace0
        ))
      ),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tag0, _ws0, types, _tag1, _ws1) = res;

    let mut type_vec = Vec::<Type>::new();

    for type_vars in types {
      let mut name = type_vars.0.to_string();
      

      for name_extension in type_vars.1 {
        name = underscore_matcher(name, name_extension.1);
      };

      let mut type_type = name.clone();

      match type_vars.2 {
        Some(t) => {
          type_type = t.1.to_string();
        },
        None => {
          // Nothing
        }
      };

      let new_type = Type {
        object_type: (name, type_type.to_string())
      };

      type_vec.push(new_type);
    }

    (
      next_input, type_vec
    )
  })
}

fn get_domain_predicates( input: &str ) -> IResult<&str, Vec<Predicate>> { 
  //println!("Input for domain predicates: {}", input);

  context("predicates", 
    tuple((
      tag("(:predicates"),
      multispace0,
      many1(
        tuple((
          tag("("),
          alphanumeric1,
          many0(
            tuple((
              tag("_"),
              alphanumeric1
            ))
          ),
          tag(" "),
          many1(
            tuple((
              tag("?"),
              alphanumeric1,
              tag(" - "),
              alphanumeric1,
              many0(
                tuple((
                  tag("_"),
                  alphanumeric1
                ))
              ),
              multispace0
            ))
          ),
          tag(")"),
          multispace0
        ))
      ),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tag0, _ws0, predicates, _tag1, _ws1) = res;

    let mut predicates_vec = Vec::<Predicate>::new();

    for predicate in predicates {
      //println!("predicates:{:?}", predicate);

      let mut arg_vec = Vec::<Argument>::new();
      let mut predicate_name = predicate.1.to_string();

      for pred_name_extension in predicate.2 {
        predicate_name = underscore_matcher(predicate_name, pred_name_extension.1);
      }

      for arg in predicate.4 {

        //println!("{:?}", arg);

        let mut object_type_name = arg.3.to_string();

        for obj_name_extension in arg.4 {
          object_type_name = underscore_matcher(object_type_name, obj_name_extension.1);
        }

        let new_argument = Argument {
          name: arg.1.to_string(),
          object_type: object_type_name
        };

        arg_vec.push(new_argument);
      }

      let new_predicate = Predicate {
        name: predicate_name,
        args: arg_vec
      };

      predicates_vec.push(new_predicate);
    }

    //println!("predicates:{:?}", predicates_vec);

    (
      next_input, predicates_vec
    )
  })
}

fn get_domain_tasks( input: &str ) -> IResult<&str, Vec<Task>> { 
  //println!("Input for tasks: {}", input);

  context("tasks", 
    many1(
      tuple((
        tag("(:task "),
        alphanumeric1,
        many0(
          tuple((
            tag("_"),
            alphanumeric1
          ))
        ),
        multispace0,
        tag(":parameters"),
        multispace0,
        tag("("),
        many1(
          tuple((
            tag("?"),
            alphanumeric1,
            many0(
              tuple((
                tag("_"),
                alphanumeric1
              ))
            ),
            tag(" - "),
            alphanumeric1,
            many0(
              tuple((
                tag("_"),
                alphanumeric1
              ))
            ),
            multispace0
          ))
        ),
        tag(")"),
        multispace0,
        tag(")"),
        multispace0
      ))
    ),

  )(input)
  .map(|(next_input, res)| {
    let task_list = res;

    let mut task_vec = Vec::<Task>::new();

    for task in &task_list {

      // Task name
      let mut task_name = task.1.to_string();
      for task_name_extension in &task.2 {
        task_name = underscore_matcher(task_name, task_name_extension.1);
      }

      // Task arguments
      let mut arg_list = Vec::<Argument>::new();

      for task_args in &task.7 {
        let mut arg_name = "?".to_string() + task_args.1;
        let mut arg_type = task_args.4.to_string();

        for arg_name_extension in &task_args.2 {
          arg_name = underscore_matcher(arg_name, arg_name_extension.1);
        }

        for arg_type_extension in &task_args.5 {
          arg_type = underscore_matcher(arg_type, arg_type_extension.1);
        }

        let new_arg = Argument {
          name: arg_name,
          object_type: arg_type,
        };

        arg_list.push(new_arg);
      }

      let new_task = Task {
        name: task_name,
        parameters: arg_list,
        alias: "alias".to_string()
      };

      task_vec.push(new_task);
    }

    //println!("{:?}", task_vec);

    (
      next_input, task_vec
    )
  })

}

fn get_domain_methods( input: &str ) -> IResult<&str, Vec<Method>> { 
  //println!("Input for methods: {}", input);

  context("domain methods",
    many1(
      tuple((
        get_method_name,
        get_method_parameters,
        get_method_task,
        opt(get_method_preconditions),
        opt(get_method_subtasks),
        opt(get_method_ordering),
        opt(get_method_constraint),
        tag(")"),
        multispace0
      ))
    )
  )(input)
  .map(|(next_input, res)| {
    let method_list = res;

    let mut method_vec = Vec::<Method>::new();

    for method in method_list {
      //println!("{:?}", method);

      let ordered_subtasks = match (method.4, method.5) {
        (Some(inner0), Some(inner1)) => Some(order_subtasks(inner0, inner1)),
        _ => None
      };

      let new_method = Method {
        name: method.0,
        parameters: method.1, 
        task: method.2,
        precondition: method.3,
        subtasks: ordered_subtasks,
        contraints: method.6
      };

      method_vec.push(new_method);
    }

    (
      next_input, method_vec
    )   
  })
}

fn get_method_name(input: &str) -> IResult<&str, String> {
  //println!("Input for get_method_name:{}", input);

  context("domain method name",
    tuple((
      tag("(:method "),
      underscore_stringer,
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tag, name, _ws) = res;

    (
      next_input, name
    ) 
  })
}

fn get_method_parameters(input: &str) -> IResult<&str, Vec<Argument>> {
  //println!("Input for get_method_parameters:{}", input);

  context("domain method parameters",
    tuple((
      tag(":parameters ("),
      many1(
        tuple((
          tag("?"),
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
    let (_tag0, param_list, _tag1, _ws0) = res;
    
    let mut final_param_list = Vec::<Argument>::new();

    for param in param_list {
      let new_arg = Argument {
        name: format!("{}{}","?".to_string(), param.1),
        object_type: param.3
      };
      final_param_list.push(new_arg);
    }

    (
      next_input, final_param_list
    ) 
  })
}

fn get_method_task(input: &str) -> IResult<&str, (String, Vec<String>)> {
  //println!("Input for get_method_task : {}", input);

  context("method task",
    tuple((
      tag(":task "),
      tag("("),
      underscore_stringer,
      multispace0,
      many1(
        tuple((
          tag("?"),
          underscore_stringer,
          multispace0
        ))
      ),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tag0, _tag1, task_name, _ws0, arg_list, _tag2, _ws1) = res;

    let mut arg_vec = Vec::<String>::new();

    for arg in arg_list {
      arg_vec.push(format!("{}{}","?".to_string(), arg.1));
    }

    (
      next_input, (task_name, arg_vec)
    ) 
  })
}

fn get_method_preconditions(input: &str) -> IResult<&str,  Vec<(bool,String,Vec<String>)>> {
  //println!("Input for get_method_preconditions : {}", input);

  context("domain method precondition",
    tuple((
      tag(":precondition (and"),
      multispace0,
      many1(
        tuple((
          tag("("),
          opt(tag("(not)")),
          underscore_stringer,
          multispace0,
          many1(
            tuple((
              tag("?"),
              underscore_stringer,
              multispace0
            ))
          ),
          tag(")"),
          multispace0
        ))
      ),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tag0, _ws0, precondition_list, _tag1, _ws1) = res;

    let mut precon_vec = Vec::<(bool,String,Vec<String>)>::new();

    for precon in precondition_list {
      //println!("{:?}", precon);

      let mut conditional_bool = true;

      match precon.1 {
        Some(inner) => { conditional_bool = false }
        None => { 
          // Nothing 
        } 
      }

      let mut arg_vec = Vec::<String>::new();

      for arg in precon.4 {
        arg_vec.push(format!("{}{}","?".to_string(), arg.1));
      }

      precon_vec.push((conditional_bool, precon.2, arg_vec));
    }

    (
      next_input, precon_vec
    ) 
  })
}

fn get_method_subtasks(input: &str) -> IResult<&str, Vec<(String, String, Vec<String>)>> {
  //println!("Input for get_method_subtasks : {}", input);

  context("domain method subtask",
    tuple((
      tag(":subtasks (and"),
      multispace0,
      many1(
        tuple((
          tag("("),
          underscore_stringer,
          tag(" ("),
          underscore_stringer,
          many1(
            tuple((
              multispace0,
              tag("?"),
              underscore_stringer
            ))
          ),
          tag("))"),
          multispace0
        ))
      ),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tag0, _ws0, subtask_list, _tag1, _ws1) = res;

    let mut subtask_vec = Vec::<(String, String, Vec<String>)>::new();

    for subtask in subtask_list {
      let mut arg_vec = Vec::<String>::new();
      
      for arg in &subtask.4 {
        arg_vec.push(format!("{}{}","?".to_string(), arg.2));
      }

      //println!("{:?}",subtask);

      subtask_vec.push((subtask.3, subtask.1.to_string(), arg_vec));
    }

    //println!("subs: {:?}", subtask_vec);

    (
      next_input, subtask_vec
    ) 
  })
}

fn get_method_ordering(input: &str) -> IResult<&str, Vec<(String, String, String)>> {
  //println!("Input for get_method_ordering : {}", input);

  context("domain method ordering",
    tuple((
      tag(":ordering (and"),
      multispace0,
      many1( 
        tuple ((
          tag("("),
          alt((tag(">"), tag("<"))),
          multispace0,
          underscore_stringer,
          multispace0,
          underscore_stringer,
          tag(")"),
          multispace0
        ))
      ),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tag0, _ws0, ordering_list, _tag1, _ws1) = res;

    let mut ordering_vec = Vec::<(String, String, String)>::new();

    for ordering in ordering_list {
      ordering_vec.push((ordering.1.to_string(), ordering.3, ordering.5));
    }

    (
      next_input, ordering_vec
    ) 
  })
}

fn get_method_constraint(input: &str) -> IResult<&str, Vec<(bool, String, String)>> {
  //println!("Input for get_method_constraint : {}", input);

  context("domain method constraint",
    tuple((
      tag(":constraints (and"),
      multispace0,
      many1( 
        tuple((
          tag("("),
          opt(tag("not ")),
          tag("(= "),
          many1(
            tuple((
              tag("?"),
              underscore_stringer,
              multispace0
            ))
          ),
          tag("))"),
          multispace0,
        ))
      ),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tag0, _ws0, arg_list, _tag3, _ws1) = res;

    let mut constraint_vec = Vec::<(bool, String, String)>::new();

    for arg in arg_list {
      //println!("{:?}",arg);

      let mut boolean_val = false;

      match arg.1 {
        Some(boolean) => {
          boolean_val = true;
        },
        None => { 
          //Nothing
        }
      }

      constraint_vec.push((boolean_val, format!("{}{}","?".to_string(), arg.3[0].1), format!("{}{}","?".to_string(), arg.3[1].1)));
    }

    (
      next_input, constraint_vec
    ) 
  })
}

fn get_domain_actions( input: &str ) -> IResult<&str, Vec<Action>> { 
  //println!("Input for actions: {}", input);

  context("domain action",
    many1(
      tuple((
        get_action_name,
        get_action_parameters,
        opt(get_action_precondition),
        get_action_effects,
        multispace0,
        tag(")"),
        multispace0
      ))
    )
  )(input)
  .map(|(next_input, res)| {
    let action_list = res;

    let mut action_vec = Vec::<Action>::new();

    for action in action_list {
      //println!("action: {:?}", action);

      let new_action = Action {
        name: action.0.to_string(),
        parameters: action.1,
        precondition: action.2,
        effect: action.3
      };

      action_vec.push(new_action);
    }

    (
      next_input, action_vec
    ) 
  })
}

fn get_action_name( input: &str ) -> IResult<&str, String> {
  //println!("Input for action name: {}", input);

  context("action name", 
    tuple((
      tag("(:action"),
      multispace0,
      underscore_stringer,
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_, _, name, _) = res;

    (
      next_input, name
    )
  })
}

fn get_action_parameters( input: &str ) -> IResult<&str, Vec<Argument>> {
  //println!("Input for action parameters: {}", input);

  context("action parameters", 
    tuple((
      tag(":parameters ("),
      many1(
        tuple((
          tag("?"),
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
    let (_, parameter_list, _, _) = res;

    let mut arg_vec = Vec::<Argument>::new();

    for parameter in parameter_list {

      let new_arg = Argument {
        name: format!("{}{}","?".to_string(), parameter.1),
        object_type: parameter.3.to_string() 
      };

      arg_vec.push(new_arg);
    }

    (
      next_input, arg_vec
    )
  })
}

fn get_action_precondition( input: &str ) -> IResult<&str, Vec<(bool,String,Vec<String>)>> {
  //println!("Input for action precondition: {}", input);

  context("action precondition", 
    tuple((
      tag(":precondition"),
      multispace0,
      tag("(and"),
      multispace0,
      many1(
        tuple((
          tag("("),
          underscore_stringer,
          multispace0,
          many1(
            tuple((
              tag("?"),
              underscore_stringer,
              multispace0
            ))
          ),
          tag(")"),
          multispace0
        ))
      ),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_, _, _, _, precon_list, _, _) = res;

    let mut precon_vec = Vec::<(bool,String,Vec<String>)>::new();

    for precon in precon_list {
      //println!("precon: {:?}", precon);

      let mut arg_vec = Vec::<String>::new();

      for arg in precon.3 {
        arg_vec.push(format!("{}{}","?".to_string(), arg.1));
      }

      precon_vec.push((true, precon.1.to_string(), arg_vec));
    }

    (
      next_input, precon_vec
    )
  })

}

fn get_action_effects( input: &str ) -> IResult<&str, Vec<(bool,String,Vec<String>)>> {
  //println!("Input for action effects: {}", input);

  context("action effect", 
    tuple((
      tag(":effect"),
      multispace0,
      tag("(and"),
      multispace0,
      many1( 
        tuple((
          opt(tag("(not ")),
          tag("("),
          underscore_stringer,
          multispace0,
          many1( 
            tuple ((
              tag("?"),
              underscore_stringer,
              multispace0
            ))
          ),
          tag(")"),
          opt(tag(")")),
          multispace0
        ))  
      ),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_, _, _, _, effect_list, _, _) = res;

    let mut effect_vec = Vec::<(bool,String,Vec<String>)>::new();

    for effect in effect_list {
      //println!("{:?}", effect);

      let mut arg_vec = Vec::<String>::new();

      for arg in effect.4 {
        arg_vec.push(format!("{}{}","?".to_string(), arg.1));
      };

      let boolean = match effect.0 {
        Some(not) => false,
        None => true
      };

      effect_vec.push((boolean, effect.2, arg_vec));
    }

    (
      next_input, effect_vec
    )
  })

}


// ------------------------- TOOL FUNCTIONS ----------------------------------------

fn option_underscore_matcher(x: &str, y: Option<&str>) -> String {
  match y {
    Some(y) => {
      format!("{}_{}", x, y)
    },
    None => x.to_string()
  }
}

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

/// Parses 2 strings in form of a problem.hddl and a domain.hddl and returns a tuple of the datastructures for each
pub fn parse_hddl( input_problem: &str, input_domain: &str ) -> (Problem, Domain) {
  let (res_problem, problem) = if let Ok((res_problem, problem)) = problem_parser(input_problem) {
    (res_problem, problem)
  } else if let Err(error) = problem_parser(input_domain) {
    println!("PROBLEM DIDNT PARSE: {}", error);
    panic!("error, didnt parse problem")
  } else {
    println!("Unable to produce error");
    panic!("error, didnt parse problem")
  };

  let (res_problem, domain) = if let Ok((res_problem, domain)) = domain_parser(input_domain) {
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