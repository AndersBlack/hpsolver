
use crate::datastructures::domain::*;
use crate::datastructures::node::SubtaskTypes;
use crate::toolbox::update_method_subtasks;
use crate::parser::{underscore_stringer, order_subtasks, combine_precon_and_constraint};
use nom::bytes::streaming::tag;
use nom::IResult;
use nom::bytes::complete::take_until;
use nom::branch::alt;
use nom::character::streaming::multispace0;
use nom::combinator::{not, opt};
use nom::sequence::{pair, terminated, tuple};
use nom::multi::{many0, many1};
use nom::error::context;
use std::collections::HashMap;

// ------------------------- DOMAIN PARSER ----------------------------------------
pub fn domain_parser( input: &str ) -> IResult<&str, Domain> {

  context("domain", 
    tuple((
      get_domain_name,
      opt(get_domain_types),
      opt(get_domain_constants),
      get_domain_predicates,
      get_domain_tasks,
      get_domain_methods,
      get_domain_actions,
      tag(")")
    ))
  )(input)
  .map(|(next_input, res)| {
      let (domain_name, types, constants, predicates, tasks, methods, actions, _) = res;

      let mut method_hashmap = HashMap::new();
      let fixed_method_list = update_method_subtasks(methods, &actions, &tasks);

      for task in &tasks {

        let mut method_list = Vec::<Method>::new();

        for method in &fixed_method_list {
          if method.task.0 == task.name {
            method_list.push(method.clone());
          }
        }

        method_hashmap.insert(task.name.clone(), method_list);
      }

      let return_types;

      if types.clone().is_none() {
        return_types = Vec::<(String, String)>::new();
      } else {
        return_types = types.unwrap();
      }

      (
        next_input,
        Domain {
          name: domain_name,
          tasks: tasks,
          methods: method_hashmap,
          actions: actions,
          types: return_types,
          constants: constants,
          predicates: predicates
        }
      )
    })

}
  
fn get_domain_name( input: &str ) -> IResult<&str, String> { 

  context("domain name", 
    tuple((
      tag("(define"),
      multispace0,
      tag("(domain "),
      underscore_stringer,
      tag(")"),
      multispace0,
      take_until(")"),
      tag(")"),
      multispace0
    ))
  )(input)
  .map(|(next_input, res)| {
    let (_tag0, _ws0, _tag1, name, _tag2, _ws1, _take, _tag3, _ws2) = res;

    (
      next_input, name
    )
  })
}
  
fn get_domain_types( input: &str ) -> IResult<&str, Vec<(String,String)>> { 

    context("domain name", 
      tuple((
        tag("(:types"),
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
            opt(tag("-")),
            multispace0,
            opt(underscore_stringer),
            multispace0
          ))
        )
        ,
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_tag0, _ws0, types, _tag1, _ws1) = res;
  
      let mut type_vec = Vec::<(String,String)>::new();
  
      for type_vars in &types {

        for names in &type_vars.0 {
          if type_vars.3.is_some() {
            type_vec.push((names.1.clone(), type_vars.3.clone().unwrap()));
          } else {
            type_vec.push((names.1.clone(), String::new()))
          }
        }
          
      };
  
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
      many0(
        tuple((
          many1(tuple((
            not(tag("-")),
            underscore_stringer,
            multispace0
          ))),
          tag("-"),
          multispace0,
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
      for const_name in constant.0 {
        const_vec.push((const_name.1, constant.3.clone()))
      } 
    }
    
    (
      next_input, const_vec
    )
  })

 }

  fn get_domain_predicates( input: &str ) -> IResult<&str, Vec<Predicate>> { 

    context("predicates", 
      tuple((
        tag("(:predicates"),
        multispace0,
        many1(
          tuple((
            multispace0,
            tag("("),
            multispace0,
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
  
        let mut arg_vec = Vec::<Argument>::new();
        let predicate_name = predicate.3.to_string();
  
        for arg in predicate.5 {

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
              many0(
                tuple((
                  multispace0,
                  not(tag("-")),
                  underscore_stringer
                ))
              ),
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
          for arg_count in &task_args.0 {
            let arg_name = arg_count.2.to_string();
            let arg_type = task_args.2.to_string();

            let new_arg = Argument {
              name: arg_name,
              object_type: arg_type,
            };
    
            arg_list.push(new_arg);
          }
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
  
  fn get_domain_methods( input: &str ) -> IResult<&str, Vec<(Method, Vec<(String, String, Vec<String>)>)>> { 

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
  
      let mut method_vec = Vec::<(Method,  Vec<(String, String, Vec<String>)>)>::new();
      let mut id_counter = 100;
      let mut ordering = false;
  
      for mut method in method_list {

        let ordered_subtasks = match (method.4, method.5.clone()) {
          (Some(inner0), Some(inner1)) => {

            if method.5.is_some() || inner0.1 == ":ordered-subtasks" || inner0.1 == ":ordered-tasks" {
              ordering = true;
            }

            order_subtasks(inner0.0, &Some(inner1))
          },
          (Some(inner0), None) => {
            
            if method.5.is_some() || inner0.1 == ":ordered-subtasks" || inner0.1 == ":ordered-tasks" {
              ordering = true;
            }
            
             inner0.0 },
          _ => Vec::<(String, String, Vec<String>)>::new()
        };

        if method.6.is_some() {
          method.3 = combine_precon_and_constraint(method.6.unwrap(), method.3);
        }

        let new_method = Method {
          name: method.0,
          parameters: method.1, 
          task: method.2,
          precondition: method.3,
          subtasks: Vec::<(SubtaskTypes, Vec<Argument>)>::new(),
          ordering: ordering,
          id: id_counter 
        };

        id_counter = id_counter + 1;
  
        method_vec.push((new_method, ordered_subtasks));
      }
  
      (
        next_input, method_vec
      )   
    })
  }
  
  fn get_method_name(input: &str) -> IResult<&str, String> {
  
    context("domain method name",
      tuple((
        tag("("),
        multispace0,
        tag(":method"),
        multispace0,
        underscore_stringer,
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_tag, _, _, _, name, _ws) = res;
  
      (
        next_input, name
      ) 
    })
  }
  
  fn get_method_parameters(input: &str) -> IResult<&str, Vec<Argument>> {
  
    context("domain method parameters",
      tuple((
        tag(":parameters ("),
        many0(
          tuple((
            many1(tuple((
              not(tag("-")),
              multispace0,
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
            name: arg.2.to_string(),
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
  
    context("method task",
      tuple((
        tag(":task"),
        multispace0,
        tag("("),
        multispace0,
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
      let (_tag0, _, _tag1, _, task_name, _ws0, arg_list, _tag2, _ws1) = res;
  
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
  
    context("domain method precondition",
      tuple((
        tag(":precondition"),
        multispace0,
        opt(tag("(and")),
        multispace0,
        many1(
          tuple((
            tag("("),
            multispace0,
            not(tag(":")),
            opt(get_forall),
            opt(
              pair(tag("not"), pair(multispace0, tag("(")))
            ),
            multispace0,
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
      let (_tag0, _, _, _ws0, precondition_list, _tag1, _ws1) = res;
  
      let mut precon_vec = Vec::<(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)>::new();
      
      for precon in precondition_list {
  
        let mut conditional_int = 0;
        let mut arg_vec = Vec::<String>::new();
  
        match (precon.4, precon.6, precon.3) {
          (_, _, Some(forall_item)) => { 
            conditional_int = 4;
            
            precon_vec.push((conditional_int, "forall".to_string(), arg_vec, Some(forall_item) ));
          },
          (None, Some(_equal), _) => { 

            conditional_int = 2;

            for arg in &precon.9 {
              arg_vec.push(arg.0.clone());
            }

            arg_vec.push(precon.7.unwrap());

            precon_vec.push((conditional_int, "=".to_string(), arg_vec, None));

          },
          (Some(_not), Some(_equal), None) => {
            conditional_int = 3;

            for arg in precon.9 {
              arg_vec.push(arg.0.to_string());
            }

            arg_vec.push(precon.7.clone().unwrap());

            precon_vec.push((conditional_int, "!=".to_string(), arg_vec, None));
          },
          (Some(_not), None, None) => {
            conditional_int = 1;

            for arg in precon.9 {
              arg_vec.push(arg.0.to_string());
            }

            precon_vec.push((conditional_int, precon.7.unwrap(), arg_vec, None));
          },
          (None, None, None) => { 
            // Ignore

            if precon.7 == None {
              continue;
            }

            for arg in precon.9 {
              arg_vec.push(arg.0.to_string());
            }

            precon_vec.push((conditional_int, precon.7.unwrap(), arg_vec, None));
          }
        }
      }
  
      (
        next_input, precon_vec
      ) 
    })
  }
  
fn get_forall(input: &str) -> IResult<&str, ((String, String), Vec<(bool, String, Vec<String>)>)> {

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

  fn get_method_subtasks(input: &str) -> IResult<&str, (Vec<(String, String, Vec<String>)>, String)> {
  
    context("domain method subtask",
      tuple((
        alt((tag(":subtasks"), tag(":ordered-subtasks"), tag(":ordered-tasks"))),
        multispace0,
        tag("("),
        opt(tag("and")),
        multispace0,
        many0(
          tuple((
            multispace0,
            opt(tag("(")),
            multispace0,
            underscore_stringer,
            multispace0,
            opt(pair(tag("("),
            underscore_stringer)),
            many0(
              tuple((
                multispace0,
                underscore_stringer,
                multispace0
              ))
            ),
            multispace0,
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
      let (tag0, _, _, _, _ws0, subtask_list, _, _ws1) = res;
  
      let mut subtask_vec = Vec::<(String, String, Vec<String>)>::new();
  
      for subtask in subtask_list {

        let mut arg_vec = Vec::<String>::new();
        
        for arg in &subtask.6 {
          arg_vec.push(arg.1.to_string());
        }
  
        match subtask.5 {
          Some(subtask3) => { subtask_vec.push((subtask3.1, subtask.3.to_string(), arg_vec)); },
          None => { subtask_vec.push((subtask.3.to_string(), subtask.3.to_string(), arg_vec)); }
        }
      }
  
      (
        next_input, (subtask_vec, tag0.to_string())
      ) 
    })
  }
  
  fn get_method_ordering(input: &str) -> IResult<&str, Vec<(String, String, String)>> {
  
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
  
        let mut boolean_val = true;
  
        match arg.1 {
          Some(_boolean) => {
            boolean_val = false;
          },
          None => {}
        }
  
        constraint_vec.push((boolean_val, arg.3[0].0.to_string(), arg.3[1].0.to_string()));
      }
  
      (
        next_input, constraint_vec
      ) 
    })
  }
  
  fn get_domain_actions( input: &str ) -> IResult<&str, Vec<Action>> { 

    context("domain action",
      many1(
        tuple((
          tag("("),
          get_action_name,
          get_action_parameters,
          opt(alt((get_action_precondition, get_action_precondition_and))),
          opt(alt((get_action_effects_and, get_action_effects_no_and))),
          tag(")"),
          multispace0
        ))
      )
    )(input)
    .map(|(next_input, res)| {
      let action_list = res;
  
      let mut action_vec = Vec::<Action>::new();
      let mut id_counter = 0;
  
      for action in action_list {
  
        let new_action = Action {
          name: action.1.to_string(),
          parameters: action.2,
          precondition: action.3,
          effect: action.4,
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
  
    context("action name", 
      tuple((
        multispace0,
        tag(":action"),
        multispace0,
        underscore_stringer,
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_, _, _, name, _) = res;
  
      (
        next_input, name
      )
    })
  }
  
  fn get_action_parameters( input: &str ) -> IResult<&str, Vec<Argument>> {
  
    context("action parameters", 
      tuple((
        tag(":parameters ("),
        many0(
          tuple((
            many1( tuple ((
              multispace0,
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
            name: arg.2.to_string(),
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
  
  fn get_action_precondition_and ( input: &str ) -> IResult<&str, Vec<(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)>> {
  
    context("action precondition", 
      tuple((
        tag(":precondition"),
        multispace0,
        tag("("),
        multispace0,
        opt(tag("and")),
        multispace0,
        many0(
          alt((precon_equal, precon_not_equal, precon_false_pred, precon_true_pred, precon_forall))
        ),
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_, _, _, _, _, _, precon_list, _, _) = res;
  
      let mut precon_vec = Vec::<(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)>::new();
  
      for precon in precon_list {
        precon_vec.push(precon);
      }
  
      (
        next_input, precon_vec
      )
    })
  
  }

  fn get_action_precondition ( input: &str ) -> IResult<&str, Vec<(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)>> {
  
    context("action precondition", 
      tuple((
        tag(":precondition"),
        multispace0,
        many1(
          alt((precon_equal, precon_not_equal, precon_false_pred, precon_true_pred, precon_forall))
        ),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_, _, precon_list, _) = res;
  
      let mut precon_vec = Vec::<(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)>::new();
  
      for precon in precon_list {
        precon_vec.push(precon);
      }
  
      (
        next_input, precon_vec
      )
    })
  
  }


  fn precon_true_pred( input: &str ) -> IResult<&str, (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)> {

    context("action precondition true pred", 
      tuple((
        tag("("),
        multispace0,
        underscore_stringer,
        multispace0,
        many0(
          terminated(
            underscore_stringer, 
            multispace0
          )
        ),
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      
      let (_, _, pred_name, _, arg_list, _, _ ) = res;

      (
        next_input, (0, pred_name, arg_list, None)
      )
    })

  }

  fn precon_false_pred( input: &str ) -> IResult<&str, (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)> {

    context("action precondition false pred", 
    tuple((
        tag("("),
        multispace0,
        tag("not"),
        multispace0,
        tag("("),
        multispace0,
        underscore_stringer,
        multispace0,
        many0(
          terminated(
            underscore_stringer, 
            multispace0
          )
        ),
        tag(")"),
        multispace0,
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      
      let (_, _, _, _, _, _, pred_name, _, arg_list, _, _, _, _) = res;

      (
        next_input, (1, pred_name, arg_list, None)
      )
    })

  }

  fn precon_equal( input: &str ) -> IResult<&str, (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)> {
    
    context("action precondition equal", 
      tuple((
        tag("("),
        multispace0,
        tag("="),
        multispace0,
        many0(
          terminated(underscore_stringer, multispace0)
        ),
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      
      let (_, _, _, _, arg_list, _, _) = res;



      (
        next_input, (2, "no name".to_string(), arg_list, None)
      )
    })

  }

  fn precon_not_equal( input: &str ) -> IResult<&str, (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)> {
    
    context("action precondition equal", 
      tuple((
        tag("("),
        multispace0,
        tag("not"),
        multispace0,
        tag("("),
        multispace0,
        tag("="),
        multispace0,
        many0(
          terminated(underscore_stringer, multispace0)
        ),
        tag(")"),
        multispace0,
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      
      let (_, _, _, _, _, _, _, _, arg_list, _, _, _, _) = res;

      (
        next_input, (3, "no name".to_string(), arg_list, None)
      )

    })

  }
  fn precon_forall( input: &str ) -> IResult<&str, (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)> {

    context("action precondition forall", 
      tuple((
        tag("("),
        multispace0,
        tag("forall"),
        multispace0,
        tag("("),
        multispace0,
        underscore_stringer,
        tag(" - "),
        underscore_stringer,
        multispace0,
        tag(")"),
        multispace0,
        many0(
          alt((precon_equal, precon_not_equal, precon_false_pred, precon_true_pred))
        ),
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      
      let (_,_,_,_,_,_, forall_param_arg, _, forall_param_type, _, _, _, precon_list, _, _ ) = res;

      let mut inner_precon_list = Vec::<(bool, String, Vec<String>)>::new();

      for precon in precon_list {

        if precon.0 == 0 {
          inner_precon_list.push((true, precon.1, precon.2))
        } else {
          inner_precon_list.push((false, precon.1, precon.2))
        }

      }

      (
        next_input, (4, "forall".to_string(), vec![], Some(((forall_param_arg, forall_param_type), inner_precon_list)))
      )
    })

  }

  fn get_action_effects_and ( input: &str ) -> IResult<&str, Vec<(bool,String,Vec<String>)>> {
  
    context("action effect", 
      tuple((
        tag(":effect"),
        multispace0,
        tag("("),
        multispace0,
        opt(tag("and")),
        multispace0,
        many0( 
          alt((not_effect, effect))
        ),
        tag(")"),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_, _, _, _, _, _, effect_list, _, _) = res;
  
      (
        next_input, effect_list
      )
    })
  
  }

  fn get_action_effects_no_and ( input: &str ) -> IResult<&str, Vec<(bool,String,Vec<String>)>> {
  
    context("action effect", 
      tuple((
        tag(":effect"),
        multispace0,
        many0( 
          alt((not_effect, effect))
        ),
        multispace0
      ))
    )(input)
    .map(|(next_input, res)| {
      let (_, _, effect_list, _) = res;
  
      (
        next_input, effect_list
      )
    })
  
  }

  fn not_effect ( input: &str ) -> IResult<&str, (bool,String,Vec<String>)> {

    context("action effect list", 
      tuple((
            tag("("),
            multispace0,
            tag("not"),
            multispace0,
            tag("("),
            multispace0,
            underscore_stringer,
            multispace0,
            many0( 
              tuple ((
                underscore_stringer,
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

      let (_, _, _, _, _, _, name, _, arg_list, _, _, _, _) = res;

      let mut args = Vec::<String>::new();

      for arg in arg_list {
        args.push(arg.0);
      }

      let effect: (bool, String, Vec<String>) =  (false, name, args);

      (
        next_input, effect
      )
    })
  }

  fn effect ( input: &str ) -> IResult<&str, (bool,String,Vec<String>)> {
    
    context("action effect list", 
      tuple((
            tag("("),
            multispace0,
            underscore_stringer,
            multispace0,
            many0( 
              tuple ((
                underscore_stringer,
                multispace0
              ))
            ),
            tag(")"),
            multispace0
          ))  
    )(input)
    .map(|(next_input, res)| {

      let (_, _, name, _, arg_list, _, _) = res;

      let mut args = Vec::<String>::new();

      for arg in arg_list {
        args.push(arg.0);
      }

      let effect: (bool, String, Vec<String>) =  (true, name, args);

      (
        next_input, effect
      )
    })
  }