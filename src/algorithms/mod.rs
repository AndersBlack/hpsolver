use core::panic;
use std::{collections::HashSet, path::PathBuf};
use crate::datastructures::{domain::{*}, node::*, problem::*};
use crate::toolbox::{self, update::*, hash_state, method_calls_method, make_node, reduce_domain};
use crate::perform::{action::perform_action_cdcl, method::*, htn::perform_htn_task, task::*};
pub mod iterative_df;
pub mod stoppable_df;
pub mod stoppable_df_partial;

/// Relevant Variables datatype: (Name, Type, List of values)
type RelVars = Vec<(String, String, Vec<String>)>;
type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);

/// Initiate depth first search
pub fn depth_first( problem: Problem, domain: &Domain, path: &PathBuf) {

	let mut node_queue = Vec::<Node>::new();
	let mut htn_subtask_queue = Vec::<(SubtaskTypes, RelVars)>::new();
	let mut new_problem: Problem = toolbox::update::update_objects(problem.clone(), domain);
	let mut function_list = Vec::<String>::new();
	let applied_funtions = (("root".to_string(), Vec::<usize>::new()), Vec::<(SubtaskTypes, usize, Vec<usize>, RelVars)>::new());
	let called = (Vec::<bool>::new(), Vec::<(Method, RelVars, Vec<Precondition>)>::new(), Vec::<usize>::new());

	new_problem.htn.subtasks.reverse();

	println!("Domain: {}\n Problem: {:?}", domain, new_problem);

	for subtask in &new_problem.htn.subtasks {
		toolbox::prep_htn_subtasks(&mut htn_subtask_queue, subtask, &new_problem);
	}

	let new_domain = reduce_domain(domain, &new_problem);

	if problem.goal.is_some() {
		function_list = toolbox::goal_oriented_finder(&new_domain, problem.goal.unwrap());
	}

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
						println!("State existed");
						continue;
					}
				}

				let current_subtask = current_node.subtask_queue.pop();

				match current_subtask {

					Some((SubtaskTypes::HtnTask(htn_task), relevant_variables))=> {
						//println!("Htn_task: {:?}, Rel_Vars: {:?}\n", htn_task.0, relevant_variables);
						//println!("\nHTN_task: {:?} Values: {:?}\n", htn_task.0, htn_task.2);

						// println!("\n");
						// for pred in &current_node.problem.state {
						// 	println!("{:?}", pred);
						// }
						// println!("\n");

						//await_key();

						perform_htn_task(node_queue, domain, current_node, htn_task, relevant_variables);
					},
					Some((SubtaskTypes::Task(task), relevant_variables)) => {
						println!("Task: {:?}\nrelvars: {:?}", task.name, relevant_variables);

						let mut line = String::new();
						let _b1 = std::io::stdin().read_line(&mut line).unwrap();

						perform_task(node_queue, domain, current_node, task, relevant_variables);
					},
					Some((SubtaskTypes::Method(method), relevant_variables)) => {
						//println!("Method: {:?}\nRelvars: {:?}\n", method.name, relevant_variables);
						println!("Method: {:?}\n", method.name);

						// println!("\n");
						// for pred in &current_node.problem.state {
						// 	println!("{:?}", pred);
						// }
						// println!("\n");

						//await_key();

						perform_method(node_queue, domain, current_node, method, relevant_variables);
					},
					Some((SubtaskTypes::Action(action), relevant_variables)) => {
						//println!("Action: {:?} Relevant_variables: {:?}\n", action.name, relevant_variables);
						//println!("Action: {}\n", action);

						// println!("\n");
						// for pred in &current_node.problem.state {
						// 	println!("{:?}", pred);
						// }
						// println!("\n");

						//await_key();

						perform_action_cdcl(node_queue, current_node, action, relevant_variables);

					},
					None => { 

						//if current_node.goal_functions.0.is_empty() { 
						if toolbox::check_goal_condition( &current_node.problem.state, &current_node.problem.goal )  {
							finished = true;
							toolbox::print_result(current_node.problem.name, current_node.applied_functions, path);

						} else {
							println!("Hit finish without correct goal");

							println!("State: {:?}", current_node.problem.state);
							
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


