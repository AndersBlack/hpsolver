use crate::toolbox::Node;
use crate::toolbox::SubtaskTypes;
use crate::toolbox::{*};

use super::Domain;
use super::Action;
use super::Method;

type RelVars = Vec<(String, String, Vec<String>)>;

// (1) Hit goal and find missing predicates
  // (2) Go through nodes and look for actions that can set the predicates (And methods that call the actions?) ( If only one node remains, pick that one )
    // (3) If action that can set predicate is found and can execute
      // (4) Pick that action
    // (5) Else if action that can set a predicate is found but cannot execute (Fails preconditions)
      // (6) Go back to (2) with the predicates from the failed preconditions ( With / Without values? )  
    // (7) Else continue

pub fn backtrack(node_queue: &mut Vec::<Node>, conflict: &String) {

  let next_node = node_queue.last();

  // println!("\nHit backtrack\n");

  // let mut line = String::new();
  // let b1 = std::io::stdin().read_line(&mut line).unwrap();

  if next_node.is_some() {
    match next_node.unwrap().subtask_queue.last() {
      Some((SubtaskTypes::HtnTask(_), _)) | Some((SubtaskTypes::Task(_), _)) => {
        backtrack_for_predicate(node_queue, conflict);
      },
      Some((SubtaskTypes::Action(_), _)) => {
        
      },
      _ => {
        println!("Was not action or HTN:");
        for st in &next_node.unwrap().subtask_queue {
          println!("Subtask: {:?}", st);
        }
      }
    }
  }

}

/// Look for the first instance in the node queue where the missing predicate is set to a different value
fn backtrack_for_predicate(node_queue: &mut Vec::<Node>, conflict: &String) {

  println!("\nBacktracking for pred\n");

  // let mut line = String::new();
  // let b1 = std::io::stdin().read_line(&mut line).unwrap();

  let mut found_node_of_interest = false;

  while node_queue.len() != 1 && !found_node_of_interest {

    let node = node_queue.pop().unwrap();

    let next_task = node.subtask_queue.last();

    match next_task {
      Some((SubtaskTypes::Action(action), _)) => {
        
        if action.effect.is_some() {
          for effect in action.effect.clone().unwrap() {
            if &effect.1 == conflict {
              found_node_of_interest = true;
            }
          }
        }

      },
      _ => {
        // Do nothing
      }
    }
  }

}

pub fn backtrack_from_goal( node_queue: &mut Vec::<Node>, current_node: &Node, domain: &Domain ){

  let mut pred_and_actions_list = Vec::<(String, Vec<Action>)>::new();
  let mut pred_list = Vec::<String>::new();

  //println!("\nMissing goals: {:?}\n", current_node.goal_functions.0.len());

  for missing_pred in current_node.goal_functions.0.keys() {
    let pred_actions = current_node.goal_functions.0.get(missing_pred).unwrap();

    if !pred_list.contains(&missing_pred.0) {
      pred_and_actions_list.push((missing_pred.0.clone(), pred_actions.clone()));
      pred_list.push(missing_pred.0.clone());
    }
  }

  // Manipulate node queue and break
  backtrack_for_goal_value(node_queue, &pred_and_actions_list, domain);
}

fn backtrack_for_goal_value(node_queue: &mut Vec::<Node>, missing_predicates: &Vec<(String, Vec<Action>)>, domain: &Domain) { 
  
  let mut mutable_missing_predicates = missing_predicates.clone();

  'while_loop: while node_queue.len() > 1 {

    let node = node_queue.pop().unwrap();
    let next_task_in_node = node.subtask_queue.last().unwrap();

    match &next_task_in_node {
      (SubtaskTypes::Action(action), relevant_variables) => {
        //find action in the missing predicates 
      
        //println!("Hit action: {}", action.name);

        if action_in_predicate(missing_predicates, action){
          if action_executable(action, &node, relevant_variables) {
            //println!("Jump to: {}", action.name);
            node_queue.push(node);
            break 'while_loop
          } else {
            println!("Updated mutable: {}", action.name);
            mutable_missing_predicates = update_missing_predicates(action, domain);
          }
        }
        
        println!("Skipped {}", action.name);
      },
      (SubtaskTypes::Method(method), _) => {
        if method_calls_interesting_action(method, &mutable_missing_predicates) || method_cant_be_excluded(method) {
          //println!("Jump to: {} with relvar: {:?}", method.name, relevant_variables);
          node_queue.push(node);
          break 'while_loop;
        }

        println!("Skipped {}", method.name);
      },
      _ => { 
        // Do nothing
      }
    }
  }
  
}

fn action_in_predicate(missing_predicates: &Vec<(String, Vec<Action>)>, action: &Action) -> bool {
  let mut found = false;
  'outer_loop: for predicate in missing_predicates{
    for predicate_action in &predicate.1{
      if predicate_action.name == action.name{
        found = true;
        break 'outer_loop
      }
    }
  }
  found
}

fn action_executable(action: &Action, node: &Node, relevant_variables: &RelVars) -> bool {

  if action.precondition.is_some() {
    for precon in action.precondition.clone().unwrap() {
      if !precondition::check_precondition(&precon, relevant_variables, &node.problem){
        return false
      }
    }
  }

  true
}

fn update_missing_predicates(action: &Action, domain: &Domain) -> Vec<(String, Vec<Action>)> {

  let mut new_predicates = Vec::<(String, Vec<Action>)>::new();

  for precondition in &action.precondition.clone().unwrap() {
    if precondition.0 == 0 {

      let pred_name = &precondition.1;
      let actions_with_correct_effect = find_actions_that_adds_predicate(pred_name.clone(), domain);
      new_predicates.push((pred_name.clone(), actions_with_correct_effect));
      
    }
  }

  new_predicates
}

fn find_actions_that_adds_predicate(pred_name: String, domain: &Domain) -> Vec<Action> {

  let mut action_list = Vec::<Action>::new();

  for action in &domain.actions {
    if action.effect.is_some() {

      for effect in action.effect.clone().unwrap() {
        if pred_name == effect.1 {
          action_list.push(action.clone());
        }
      }
    }
  }

  action_list
}

fn method_cant_be_excluded(method: &Method) -> bool {

  if method.precondition.is_some() {
    return true
  }

  if method.subtasks.len() > 1 {
    return true
  }

  false
}

fn method_calls_interesting_action(method: &Method, missing_predicates: &Vec<(String, Vec<Action>)>) -> bool {
  let mut method_calls_interesting_action = false;

  'outer: for subtask in &method.subtasks {
    for pred_data in missing_predicates {
      for interesting_action in &pred_data.1 {
        if interesting_action.name == subtask.0 {
          // Method was interesting!
          method_calls_interesting_action = true;
          break 'outer;
        }
      } 
    }
  }

  method_calls_interesting_action
}

// fn is_pred_vars_in_rel_vars(goal_pred: &(String, Vec<String>), relvars: &RelVars) -> bool {
  
//   println!("Pred: {:?} RelVars: {:?}", goal_pred, relvars);

//   let mut result = true;
//   for goal_value in &goal_pred.1 {
//     let mut goal_value_present = false;
//     for relvar in relvars {
//       if relvar.2.contains(goal_value){
//         goal_value_present = true;
//         break;
//       } 
//     }
    
//     if !goal_value_present{
//       result = false;
//       break;
//     }
//   }
  
//   result
// }


//pub fn backtrack_for_parameter_value(node_queue: &mut Vec::<Node>, conflict_relvars: &RelVars) {

  // println!("RELVARS in BACKTRACK: {:?}", conflict_relvars);

  // for node in node_queue {
  //   println!("Node: {:?}\n", node.subtask_queue.last().unwrap().0);
  // }

  // let mut line = String::new();
  // let b1 = std::io::stdin().read_line(&mut line).unwrap();

//}

// pub fn backtrack_for_method_param_value(node_queue: &mut Vec::<Node>, relvars: &RelVars) {

//   for relvar in relvars {
//     if relvar.2.len() == 0 {
//       panic!("Found one!");
//     }
//   }

// }