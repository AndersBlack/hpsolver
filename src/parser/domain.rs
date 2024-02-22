
use crate::datastructures::domain::*;
use crate::parser::{underscore_stringer, underscore_matcher, order_subtasks};
use nom::IResult;
use nom::bytes::complete::{tag, take_until};
use nom::branch::alt;
use nom::character::streaming::multispace0;
use nom::combinator::{opt, not};
use nom::character::complete::alphanumeric1;
use nom::sequence::{tuple, pair};
use nom::multi::{many1, many0};
use nom::error::context;
use std::collections::HashMap;

// ------------------------- DOMAIN PARSER ----------------------------------------

pub fn domain_parser( input: &str ) -> IResult<&str, Domain> {

  context("domain", 
    tuple((
      get_domain_name,
      get_domain_types,
      opt(get_domain_constants),
      get_domain_predicates,
      get_domain_tasks,
      get_domain_methods,
      get_domain_actions
    ))
  )(input)
  .map(|(next_input, res)| {
      let (domain_name, types, constants, predicates, tasks, methods, actions) = res;

      let mut method_hashmap = HashMap::new();
      
      for task in &tasks {

        let mut method_list = Vec::<Method>::new();

        for method in &methods {
          if method.task.0 == task.name {
            method_list.push(method.clone());
          }
        }

        method_hashmap.insert(task.name.clone(), method_list);
      }

      (
        next_input,
        Domain {
          name: domain_name,
          tasks: tasks,
          methods: method_hashmap,
          actions: actions,
          types: types,
          constants: constants,
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
  
  fn get_domain_types( input: &str ) -> IResult<&str, Vec<(String,String)>> { 
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
                alt((tag("_"),tag("-"))),
                alphanumeric1
              ))
            ),
            opt(tuple((
              tag(" - "),
              underscore_stringer
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
  
      let mut type_vec = Vec::<(String,String)>::new();
  
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
  
        type_vec.push((name, type_type.to_string()));
      }
  
      (
        next_input, type_vec
      )
    })
  }
  
fn get_domain_constants ( input: &str ) -> IResult<&str, Vec<(String, String)>> {

  context("domain constants", 
    tuple((
      tag("(:constants"),
      multispace0,
      many1(
        tuple((
          underscore_stringer,
          tag(" - "),
          underscore_stringer,
          multispace0,
        ))
      ),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tag, _ms, const_list, _tag1, _ms1) = res;

    let mut const_vec = Vec::<(String, String)>::new();

    for constant in const_list {
      const_vec.push((constant.0, constant.2))
    }

    (
      next_input, const_vec
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
            underscore_stringer,
            multispace0,
            many0(
              tuple((
                many1(tuple((
                  not(tag("-")),
                  underscore_stringer,
                  multispace0
                ))),
                tag("- "),
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
      let (_tag0, _ws0, predicates, _tag1, _ws1) = res;

      let mut predicates_vec = Vec::<Predicate>::new();
  
      for predicate in predicates {
        //println!("predicates:{:?}", predicate);
  
        let mut arg_vec = Vec::<Argument>::new();
        let predicate_name = predicate.1.to_string();
  
        for arg in predicate.3 {

            for arg_count in arg.0 {
              let new_argument = Argument {
                name: arg_count.1.to_string(),
                object_type: arg.2.to_string()
              };
      
              arg_vec.push(new_argument);
            }
        }
  
        let new_predicate = Predicate {
          name: predicate_name,
          args: arg_vec
        };
  
        predicates_vec.push(new_predicate);
      }
  
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
          underscore_stringer,
          multispace0,
          tag(":parameters"),
          multispace0,
          tag("("),
          many0(
            tuple((
              underscore_stringer,
              tag(" - "),
              underscore_stringer,
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
      let mut id_counter = 1000;
  
      for task in &task_list {
  
        // Task name
        let task_name = task.1.to_string();
  
        // Task arguments
        let mut arg_list = Vec::<Argument>::new();
  
        for task_args in &task.6 {
          let arg_name = task_args.0.to_string();
          let arg_type = task_args.2.to_string();

  
          let new_arg = Argument {
            name: arg_name,
            object_type: arg_type,
          };
  
          arg_list.push(new_arg);
        }
  
        let new_task = Task {
          name: task_name,
          parameters: arg_list,
          alias: "alias".to_string(),
          id: id_counter
        };
  
        task_vec.push(new_task);

        id_counter = id_counter + 1;
      }
  
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
          opt(tag(")")),
          multispace0
        ))
      )
    )(input)
    .map(|(next_input, res)| {
      let method_list = res;
  
      let mut method_vec = Vec::<Method>::new();
      let mut id_counter = 100;
  
      for method in method_list {
        //println!("{:?}\n", method);

        let ordered_subtasks = match (method.4, method.5) {
          (Some(inner0), Some(inner1)) => order_subtasks(inner0, Some(inner1)),
          (Some(inner0), None) => { inner0 },
          _ => Vec::<(String, String, Vec<String>, bool)>::new()
        };
  
        let new_method = Method {
          name: method.0,
          parameters: method.1, 
          task: method.2,
          precondition: method.3,
          subtasks: ordered_subtasks,
          constraints: method.6,
          id: id_counter
        };

        id_counter = id_counter + 1;
  
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
        many0(
          tuple((
            many1(tuple((
              not(tag("-")),
              underscore_stringer,
              multispace0
            ))),
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
      let (_tag0, param_list, _tag1, _ws0) = res;
      
      let mut final_param_list = Vec::<Argument>::new();
  
      for param in param_list {

        for arg in param.0 {

          let new_arg = Argument {
            name: arg.1.to_string(),
            object_type: param.2.clone()
          };

          final_param_list.push(new_arg);
        }
      }
  
      (
        next_input, final_param_list
      ) 
    })
  }
  
  fn get_method_task(input: &str) -> IResult<&str, (String, Vec<String>)> {
    //println!("Input for get_method_task: {}", input);
  
    context("method task",
      tuple((
        tag(":task "),
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
    )(input)
    .map(|(next_input, res)| {
      let (_tag0, _tag1, task_name, _ws0, arg_list, _tag2, _ws1) = res;
  
      let mut arg_vec = Vec::<String>::new();
  
      for arg in arg_list {
        arg_vec.push(arg.0.to_string());
      }
  
      (
        next_input, (task_name, arg_vec)
      ) 
    })
  }
  
  fn get_method_preconditions(input: &str) -> IResult<&str,  Vec<(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)>> {
    //println!("Input for get_method_preconditions : {}", input);
  
    context("domain method precondition",
      tuple((
        tag(":precondition"),
        opt(tag(" (and")),
        multispace0,
        many1(
          tuple((
            tag("("),
            not(tag(":")),
            opt(get_forall),
            opt(tag("not (")),
            opt(tag("= ")),
            opt(underscore_stringer),
            multispace0,
            many0(
              tuple((
                underscore_stringer,
                multispace0
              ))
            ),
            opt(tag(")")),
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
      let (_tag0, _, _ws0, precondition_list, _tag1, _ws1) = res;
  
      let mut precon_vec = Vec::<(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)>::new();
      
      for precon in precondition_list {
        //println!("{:?}", precon);
  
        let mut conditional_int = 0;
        let mut arg_vec = Vec::<String>::new();
  
        match (precon.3, precon.4, precon.2) {
          (_, _, Some(forall_item)) => { 
            conditional_int = 4;
            
            precon_vec.push((conditional_int, "forall".to_string(), arg_vec, Some(forall_item) ));
          },
          (None, Some(_equal), _) => { 

            conditional_int = 2;

            for arg in &precon.7 {
              arg_vec.push(arg.0.clone());
              precon_vec.push((conditional_int, precon.5.clone().unwrap().to_string(), arg_vec.clone(), None));
            }

          },
          (Some(_not), Some(_equal), None) => {
            conditional_int = 3;

            for arg in precon.7 {
              arg_vec.push(arg.0.to_string());
            }

            precon_vec.push((conditional_int, precon.5.unwrap(), arg_vec, None));
          },
          (Some(_not), None, None) => {
            conditional_int = 1;

            for arg in precon.7 {
              arg_vec.push(arg.0.to_string());
            }

            precon_vec.push((conditional_int, precon.5.unwrap(), arg_vec, None));
          },
          (None, None, None) => { 
            for arg in precon.7 {
              arg_vec.push(arg.0.to_string());
            }

            precon_vec.push((conditional_int, precon.5.unwrap(), arg_vec, None));
          }
        }
      }
  
      (
        next_input, precon_vec
      ) 
    })
  }
  
fn get_forall(input: &str) -> IResult<&str, ((String, String), Vec<(bool, String, Vec<String>)>)> {
  //println!("Forall input: {}", input);

  context("forall", 
    tuple((
      tag("forall"),
      multispace0,
      tag("("),
      underscore_stringer,
      tag(" - "),
      underscore_stringer,
      tag(")"),
      multispace0,
      many1(
        tuple((
          opt(tag("(not ")),
          tag("("),
          underscore_stringer,
          multispace0,
          many1(
            tuple((
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
    let (_tag0, _ms0, _tag1, forall_arg, _tag2, forall_arg_type, _tag3, _ms1, con_list, _tag4, _ms2) = res;

    let mut forall_constraint_list = Vec::<(bool, String, Vec<String>)>::new();

    for con in con_list {
        let mut condition_bool = true;

        if let Some(_condition_boolean) = con.0 {
          condition_bool = false;
        }

        let mut arg_vec = Vec::<String>::new();

        for arg in con.4 {
          arg_vec.push(arg.0.to_string())
        }

        forall_constraint_list.push((condition_bool, con.2, arg_vec));
    }

    (
      next_input, ((forall_arg.to_string(), forall_arg_type), forall_constraint_list)
    )
  })

} 

  fn get_method_subtasks(input: &str) -> IResult<&str, Vec<(String, String, Vec<String>, bool)>> {
    //println!("Input for get_method_subtasks: {}", input);
  
    context("domain method subtask",
      tuple((
        alt((tag(":subtasks"), tag(":ordered-subtasks"))),
        multispace0,
        tag("("),
        opt(tag("and")),
        multispace0,
        many0(
          tuple((
            multispace0,
            opt(tag("(")),
            underscore_stringer,
            multispace0,
            opt(pair(tag("("),
            underscore_stringer)),
            many0(
              tuple((
                multispace0,
                underscore_stringer
              ))
            ),
            opt(alt((tag("))"),tag(")")))),
            multispace0,
            opt(tag(")"))
          ))
        ),
        opt(tag(")")),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_tag0, _, _, _, _ws0, subtask_list, _, _ws1) = res;
  
      let mut subtask_vec = Vec::<(String, String, Vec<String>, bool)>::new();
  
      for subtask in subtask_list {
        let mut arg_vec = Vec::<String>::new();
        
        for arg in &subtask.5 {
          arg_vec.push(arg.1.to_string());
        }
  
        //println!("{:?}",subtask);
        match subtask.4 {
          Some(subtask3) => { subtask_vec.push((subtask3.1, subtask.2.to_string(), arg_vec, false)); },
          None => { subtask_vec.push(("No alias".to_string(), subtask.2.to_string(), arg_vec, false)); }
        }
  
        
      }
  
      //println!("subs: {:?}\n", subtask_vec);
  
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
  
        let mut boolean_val = true;
  
        match arg.1 {
          Some(_boolean) => {
            boolean_val = false;
          },
          None => { 
            //Nothing
          }
        }
  
        constraint_vec.push((boolean_val, arg.3[0].0.to_string(), arg.3[1].0.to_string()));
      }
  
      (
        next_input, constraint_vec
      ) 
    })
  }
  
  fn get_domain_actions( input: &str ) -> IResult<&str, Vec<Action>> { 
    //println!("Input for actions:\n{}", input);
  
    context("domain action",
      many1(
        tuple((
          get_action_name,
          get_action_parameters,
          opt(get_action_precondition),
          opt(get_action_effects),
          multispace0,
          opt(tag(")")),
          multispace0
        ))
      )
    )(input)
    .map(|(next_input, res)| {
      let action_list = res;
  
      let mut action_vec = Vec::<Action>::new();
      let mut id_counter = 0;
  
      for action in action_list {
        //println!("action: {:?}", action);
  
        let new_action = Action {
          name: action.0.to_string(),
          parameters: action.1,
          precondition: action.2,
          effect: action.3,
          id: id_counter
        };

        id_counter = id_counter + 1;
  
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
        many0(
          tuple((
            many1( tuple ((
              not(tag("-")),
              underscore_stringer,
              multispace0
            ))),
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
      let (_, parameter_list, _, _) = res;
  
      let mut arg_vec = Vec::<Argument>::new();
  
      for parameter in parameter_list {
        
        for arg in parameter.0 {
          let new_arg = Argument {
            name: arg.1.to_string(),
            object_type: parameter.2.to_string() 
          };

          arg_vec.push(new_arg);
        }
      }
  
      (
        next_input, arg_vec
      )
    })
  }
  
  fn get_action_precondition( input: &str ) -> IResult<&str, Vec<(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)>> {
    //println!("Input for action precondition: {}", input);
  
    context("action precondition", 
      tuple((
        tag(":precondition"),
        multispace0,
        tag("("),
        opt(tag("and")),
        multispace0,
        many0(
          tuple((
            tag("("),
            opt(tag("not (")),
            underscore_stringer,
            multispace0,
            many0(
              tuple((
                underscore_stringer,
                multispace0
              ))
            ),
            tag(")"),
            opt(tag(")")),
            multispace0
          ))
        ),
        opt(tag(")")),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_, _, _, _, _, precon_list, _, _) = res;
  
      let mut precon_vec = Vec::<(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)>::new();
  
      for precon in precon_list {
        //println!("precon: {:?}", precon);
        let mut arg_vec = Vec::<String>::new();
  
        for arg in precon.4 {
          arg_vec.push(arg.0);
        }

        if precon.1 != None {
          precon_vec.push((1, precon.2.to_string(), arg_vec, None));
        } else {
          precon_vec.push((0, precon.2.to_string(), arg_vec, None));
        }
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
        tag("("),
        opt(tag("and")),
        multispace0,
        many0( 
          tuple((
            opt(tag("(not ")),
            tag("("),
            underscore_stringer,
            multispace0,
            many0( 
              tuple ((
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
      let (_, _, _, _, _, effect_list, _, _) = res;
  
      let mut effect_vec = Vec::<(bool,String,Vec<String>)>::new();
  
      for effect in effect_list {
        //println!("{:?}", effect);
  
        let mut arg_vec = Vec::<String>::new();
  
        for arg in effect.4 {
          arg_vec.push(arg.0);
        };
  
        let boolean = match effect.0 {
          Some(_not) => false,
          None => true
        };
  
        effect_vec.push((boolean, effect.2, arg_vec));
      }
  
      (
        next_input, effect_vec
      )
    })
  
  }