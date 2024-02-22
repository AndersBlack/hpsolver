use core::panic;

use crate::toolbox::Node;
use crate::toolbox::SubtaskTypes;

type RelVars = Vec<(String, String, Vec<String>)>;


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

  let mut line = String::new();
  let b1 = std::io::stdin().read_line(&mut line).unwrap();

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

pub fn backtrack_from_goal( node_queue: &mut Vec::<Node>, state: &Vec<(String, Vec<String>)>, goal: &Vec<(String, Vec<String>)>){

  for goal_req in goal {

    let mut found_goal_pred = false;

    for state_pred in state {

      let mut found_counter = 0;

      if state_pred.0 == goal_req.0 {
        for i in 0..goal_req.1.len() {
          if state_pred.1[i] == goal_req.1[i] {
            found_counter += 1;
          }
        }
      }

      if found_counter == state_pred.1.len() {
        found_goal_pred == true;
      };
    }

    if !found_goal_pred {
      // Manipulate node queue and break
      backtrack_for_goal_value(node_queue, goal_req, goal, state);
      break;
    }

  }

}

fn backtrack_for_goal_value(node_queue: &mut Vec::<Node>, goal_pred: &(String, Vec<String>), goal_state: &Vec<(String, Vec<String>)>, state: &Vec<(String, Vec<String>)>) {

  let mut goal_pred_mutable = goal_pred; 

  // Find problem predicate recursively
  'while_loop: while node_queue.len() > 1 {

    let node = node_queue.pop().unwrap();
    let next_task_in_node = node.subtask_queue.last().unwrap();

    match next_task_in_node {
      (SubtaskTypes::Method(method), relevant_variables) => {
        node_queue.push(node);
        break 'while_loop;
      },
      (SubtaskTypes::Action(action), relevant_variables) => {

        if action.effect.is_some() {
          for effect in action.effect.clone().unwrap() {
            if effect.0 && effect.1 == goal_pred.0 {
              if action.precondition.is_some() {
                for precon in action.precondition.clone().unwrap() {

                  if precon.0 == 0 {



                    for state_pred in state {
                      if state_pred.0 == precon.1 {

                        println!("Relvars: {:?}", relevant_variables);

                        let mut line = String::new();
						            let b1 = std::io::stdin().read_line(&mut line).unwrap();
                      } 
                    } 
                    
                  }

                }
              }
            }
          }
        }
        

      },
      _ => {
        // Next task is task or htn and therefore not usable to us
        panic!("TASK OR HTN")
      }
    }

  }

}

/// Look for the first instance in the node queue where the conflicting parameter is set to a different value
pub fn backtrack_for_parameter_value(node_queue: &mut Vec::<Node>, conflict_relvars: &RelVars) {

  // println!("RELVARS in BACKTRACK: {:?}", conflict_relvars);

  // for node in node_queue {
  //   println!("Node: {:?}\n", node.subtask_queue.last().unwrap().0);
  // }

  // let mut line = String::new();
  // let b1 = std::io::stdin().read_line(&mut line).unwrap();

}

pub fn backtrack_for_method_param_value(node_queue: &mut Vec::<Node>, relvars: &RelVars) {

  for relvar in relvars {
    if relvar.2.len() == 0 {
      panic!("Found one!");
    }
  }

}