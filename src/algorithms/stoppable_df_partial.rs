use std::collections::HashMap;
use std::{collections::HashSet, time::Instant, path::PathBuf};
use crate::algorithms::{*, self};
use crate::perform::partial::{action::perform_action_cdcl, htn::perform_htn_task, task::perform_task, method::perform_method};
use crate::toolbox::{self, make_partial_node, await_key};

// Relevant epic Variables datatype 
type RelVars = Vec<(String, String, Vec<String>)>;
type Called = (Vec<bool>, Vec<(Method, RelVars, Vec<Precondition>)>, Vec<usize>);

pub fn stoppable_depth_first_partial(problem: &Problem, domain: &Domain, stopped: &Instant, path: &PathBuf) -> &'static str {

	let mut node_queue = Vec::<PartialNode>::new();
	let mut htn_subtask_queue = Vec::<(SubtaskTypes, RelVars, Called, Vec<Precondition>)>::new();
	let mut function_list = Vec::<String>::new();
	let mut new_problem: Problem = algorithms::update_objects(problem.clone(), domain);
  let applied_functions = (("root".to_string(), Vec::<usize>::new()), Vec::<(SubtaskTypes, usize, Vec<usize>, RelVars)>::new());

	new_problem.htn.subtasks.reverse();

	for subtask in &new_problem.htn.subtasks {
		toolbox::prep_partial_htn_subtasks(&mut htn_subtask_queue, subtask, &new_problem);
	}

	let new_domain = algorithms::reduce_domain(domain, &new_problem);

	if problem.goal.is_some() {
		function_list = toolbox::goal_oriented_finder(domain, problem.goal.clone().unwrap());
	}

	let new_node = make_partial_node(new_problem, htn_subtask_queue, applied_functions, HashSet::new(), HashMap::<u64, usize>::new(), function_list);

	node_queue.push(new_node);

	let mut return_string;
	let mut hash_limit: usize = 0;
	let node_q_clone = node_queue.clone();

	loop {
		return_string = run_df(&mut node_queue, &new_domain, stopped, path, hash_limit);

		if return_string == "stopped" {
			break;
		}

		if return_string != "success" {
			println!("increased hash limit");
			hash_limit = hash_limit + 1;
			node_queue = node_q_clone.clone();
		} else {
			break; 
		}
	}

  return_string
}

fn run_df(node_queue: &mut Vec::<PartialNode>, domain: &Domain, stopped: &Instant, path: &PathBuf, hash_limit: usize) -> &'static str {

	let finished: bool = false;
	let mut tried_count = 0;

	while !finished {

    if stopped.elapsed().as_secs() > 1800 { 
      return "stopped";
    }

		let current_node_partial = node_queue.pop();

		// Handle subtasks
		match current_node_partial {
			Some(mut current_node) => {

				let state_exists = toolbox::partial_hash_state(&mut current_node, tried_count, hash_limit);

				/*
					The idea for partial order is to maintain the frontier in the subtask_queue.
					That means that any subtask in the subtask_queue can advance the node.
				*/

				let sq_size = current_node.clone().subtask_queue.len();

				if sq_size == 0 {
					let finished_state = toolbox::check_goal_condition( &current_node.problem.state, &current_node.problem.goal );

					println!("State: {:?} and {}", current_node.problem.state, finished_state);

					if finished_state {
						toolbox::print_result(current_node.problem.name, current_node.applied_functions, path);
						return "success";
					} else {
						continue;
					}
				}

				if tried_count > (sq_size - 1) {
					tried_count = tried_count - 1;
				}

				if state_exists {
					continue;
				}

				//await_key();

				let completed_subtask: bool = match current_node.subtask_queue[tried_count].clone() {

					(SubtaskTypes::HtnTask(htn_task), relevant_variables, mut called, passing_precon) => {
						//println!("Htn_task: {:?}, Rel_Vars: {:?}\n", htn_task.0, relevant_variables);
						let res = perform_htn_task(node_queue, domain, current_node.clone(), htn_task, relevant_variables, &mut called, tried_count, passing_precon);

						//await_key();

						res
					},
					(SubtaskTypes::Task(task), relevant_variables,  called, passing_precon) => {
						//println!("Task: {:?}\n", task.name);
						let res = perform_task(node_queue, domain, current_node.clone(), task, relevant_variables, called, passing_precon, tried_count);

						res
					},
					(SubtaskTypes::Method(method), relevant_variables,  called, passing_precon) => {
						//println!("Method {:?}, RELVARS: {:?}\n", method.name, relevant_variables);
						let res = perform_method(node_queue, domain, current_node.clone(), method, relevant_variables, called, tried_count, passing_precon);
						
						if !res {
							//println!("Failed {:?}", current_node.subtask_queue);
							tried_count += 1;
						}

						res
					},
					(SubtaskTypes::Action(action), relevant_variables,  mut called, passing_precon) => {
						//println!("\n Action: {:?} Relevant_variables: {:?}\n", action.name, relevant_variables);
						let res = perform_action_cdcl(node_queue, current_node.clone(), action, relevant_variables, &mut called, passing_precon, tried_count);

						if res {
							tried_count = 0;
						} else {
							//println!("Failed {:?}", current_node.subtask_queue.len());
							tried_count += 1;
						}

						//await_key();

						res
					}
				};

				//println!("TRIED COUNT: {}", tried_count);

				if tried_count < sq_size && !completed_subtask {
					node_queue.push(current_node.clone());
				} else if tried_count >= sq_size {
					tried_count = 0;
				}

			},
			None => { 
				return "Node queue found empty!";
			}
		}
	}

  return "error";
}