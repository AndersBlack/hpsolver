use core::panic;
use std::{collections::{HashMap, HashSet}, path::PathBuf};
use crate::datastructures::{domain::*, node::*, problem::*};
use crate::toolbox::{self, update::*, hash_state, method_calls_method, make_node, reduce_domain};
use crate::perform::{action::perform_action_cdcl, method::*, htn::perform_htn_task, task::*};
pub mod iterative_df;
pub mod stoppable_df;

/// Relevant Variables datatype: (Name, Type, List of values)
type RelVars = Vec<(String, String, Vec<String>)>;
type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);

/// Initiate depth first search
pub fn depth_first( problem: Problem, domain: &Domain, path: &PathBuf) {

	let mut node_queue = Vec::<Node>::new();
	let mut htn_subtask_queue = Vec::<(SubtaskTypes, RelVars)>::new();
	let mut new_problem: Problem = toolbox::update::update_objects(problem.clone(), domain);
	let mut function_list: (HashMap<(String, Vec<String>), Vec<Action>>, Vec<String>) = (HashMap::<(String, Vec<String>), Vec<Action>>::new(), Vec::<String>::new());

	new_problem.htn.subtasks.reverse();

	let applied_funtions = (("root".to_string(), Vec::<usize>::new()), Vec::<(SubtaskTypes, usize, Vec<usize>, RelVars)>::new());

	for subtask in &new_problem.htn.subtasks {
		toolbox::prep_htn_subtasks(&mut htn_subtask_queue, subtask, &new_problem);
	}

	let new_domain = reduce_domain(domain, &new_problem);

	if problem.goal.is_some() {
		function_list = toolbox::goal_oriented_finder(&new_domain, problem.goal.unwrap());
	}

	// println!("Function list");
	// for (key, value) in function_list.0.drain() {
	// 	println!("\nKey: {:?}", key);
	// 	for val in value {
	// 		match val {
	// 				SubtaskTypes::Action(action) => {
	// 					println!("action name: {:?}", action.name);
	// 				},
	// 				SubtaskTypes::Method(method) => {
	// 					println!("method name: {:?}", method.name);
	// 				},
	// 				_ => {}
	// 		}
	// 	}
	// }

	//panic!("ARRRGH");

	let called = (Vec::<bool>::new(), Vec::<(Method, RelVars, Vec<Precondition>)>::new(), Vec::<usize>::new());
	let new_node = make_node(new_problem.clone(), htn_subtask_queue, called, applied_funtions, HashSet::<u64>::new(), Vec::<Precondition>::new(), function_list);

	node_queue.push(new_node);

	run_df(&mut node_queue, &new_domain, path);
}

/// Loop through the queue and perform actions accordingly
fn run_df( node_queue: &mut Vec::<Node>, domain: &Domain, path: &PathBuf) {

	let mut finished: bool = false;

	while !finished {

		let current_node = node_queue.pop();

		// Handle subtasks
		match current_node {
			Some(mut current_node) => {

				if !method_calls_method(&domain.methods) {
					let state_exists = hash_state(&mut current_node);

					if state_exists {
						continue;
					}
				}

				let current_subtask = current_node.subtask_queue.pop();

				match current_subtask {

					Some((SubtaskTypes::HtnTask(htn_task), relevant_variables))=> {
						//println!("Htn_task: {:?}, Rel_Vars: {:?}\n", htn_task.0, relevant_variables);
						// println!("\nHTN_task: {:?} Values: {:?}\n", htn_task.0, htn_task.2);

						// println!("\n");
						// for pred in &current_node.problem.state {
						// 	println!("{:?}", pred);
						// }
						// println!("\n");

   					// let mut line = String::new();
						// let b1 = std::io::stdin().read_line(&mut line).unwrap();

						perform_htn_task(node_queue, domain, current_node, htn_task, relevant_variables);
					},
					Some((SubtaskTypes::Task(task), relevant_variables)) => {
						// println!("Task: {:?}", task.name);

						// let mut line = String::new();
						// let b1 = std::io::stdin().read_line(&mut line).unwrap();

						perform_task(node_queue, domain, current_node, task, relevant_variables);
					},
					Some((SubtaskTypes::Method(method), relevant_variables)) => {
						//println!("Method {:?}Relvars: {:?} \n", method.name, relevant_variables);
						//println!("Method: {:?}", method.name);

						// let mut line = String::new();
						// let b1 = std::io::stdin().read_line(&mut line).unwrap();

						// println!("\n");
						// for pred in &current_node.problem.state {
						// 	println!("{:?}", pred);
						// }
						// println!("\n");

						perform_method(node_queue, domain, current_node, method, relevant_variables);
					},
					Some((SubtaskTypes::Action(action), relevant_variables)) => {
						//println!("\n Action: {:?} Relevant_variables: {:?}\n", action.name, relevant_variables);
						//println!("Action: {:?}", action.name);

						// println!("\n");
						// for pred in &current_node.problem.state {
						// 	println!("{:?}", pred);
						// }
						// println!("\n");

						// let mut line = String::new();
						// let b1 = std::io::stdin().read_line(&mut line).unwrap();

						perform_action_cdcl(node_queue, current_node, action, relevant_variables);

					},
					None => { 

						if current_node.goal_functions.0.is_empty() { //toolbox::check_goal_condition( &current_node.problem.state, &current_node.problem.goal ) 
							finished = true;
							toolbox::print_result(current_node, path);

						} else {
							println!("Hit finish without correct goal");

							// let state_exists = hash_state(&mut current_node);	

							// if state_exists {
							// 	panic!("Hit a dupe goal state");
							// }

							// println!("Node function prio:");
							// for goal_f in current_node.goal_functions.0.values() {
							// 	println!("Goal f: {:?}\n", goal_f);
							// }
							// println!("Node queue:");
							// for node in node_queue {
							// 	println!("Task in queue: {:?}\n", &node.subtask_queue.last())
							// }
							//panic!("ARRRGH");
							
							if current_node.goal_functions.0.len() > 0 {
								toolbox::back_tracking::backtrack_from_goal(node_queue, &current_node, &domain);
							}
							
						}

					}
				}
			},
			None => { 
				panic!("Node queue found empty!")
			}
		}
	}	
}


