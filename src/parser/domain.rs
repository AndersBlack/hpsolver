use crate::domain::*;

use crate::parser::{underscore_stringer, underscore_matcher, order_subtasks};

use nom::IResult;
//use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::branch::{alt};
use nom::combinator::{opt};
use nom::character::complete::{alphanumeric1, multispace0};
use nom::sequence::{tuple};
use nom::multi::{many1, many0};
use nom::error::{context};

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
        //println!("{:?}\n", method);

        let ordered_subtasks = match (method.4, method.5) {
          (Some(inner0), Some(inner1)) => Some(order_subtasks(inner0, inner1)),
          (Some(inner0), None) => { Some(inner0) },
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

        //println!("{}", new_method);
  
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
            opt(tag("not (")),
            underscore_stringer,
            multispace0,
            many1(
              tuple((
                tag("?"),
                underscore_stringer,
                multispace0
              ))
            ),
            opt(tag(")")),
            opt(tag(")")),
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
          Some(_inner) => { conditional_bool = false }
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
    //println!("Input for get_method_subtasks: {}", input);
  
    context("domain method subtask",
      tuple((
        tag(":subtasks ("),
        opt(tag("and")),
        multispace0,
        many0(
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
      let (_tag0, _, _ws0, subtask_list, _tag1, _ws1) = res;
  
      let mut subtask_vec = Vec::<(String, String, Vec<String>)>::new();
  
      for subtask in subtask_list {
        let mut arg_vec = Vec::<String>::new();
        
        for arg in &subtask.4 {
          arg_vec.push(format!("{}{}","?".to_string(), arg.2));
        }
  
        //println!("{:?}",subtask);
  
        subtask_vec.push((subtask.3, subtask.1.to_string(), arg_vec));
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
          Some(_boolean) => {
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
    //println!("Input for actions:\n{}", input);
  
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
        tag("("),
        opt(tag("and")),
        multispace0,
        many0(
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
      let (_, _, _, _, _, precon_list, _, _) = res;
  
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