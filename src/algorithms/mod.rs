use core::panic;
use std::{collections::HashSet, path::PathBuf};
use crate::datastructures::{domain::{*}, node::*, problem::*};
use crate::toolbox::{self, update::*, hash_state, method_calls_method, make_node, reduce_domain, RelVars, Precondition};
use crate::perform::{action::perform_action_cdcl, method::*, htn::perform_htn_task, task::*};
pub mod stoppable_df;
pub mod stoppable_df_partial;

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
						perform_htn_task(node_queue, domain, current_node, htn_task, relevant_variables);
					},
					Some((SubtaskTypes::Task(task), relevant_variables)) => {
						perform_task(node_queue, domain, current_node, task, relevant_variables);
					},
					Some((SubtaskTypes::Method(method), relevant_variables)) => {
						perform_method(node_queue, domain, current_node, method, relevant_variables);
					},
					Some((SubtaskTypes::Action(action), relevant_variables)) => {
						perform_action_cdcl(node_queue, current_node, action, relevant_variables);
					},
					None => { 
						if toolbox::check_goal_condition( &current_node.problem.state, &current_node.problem.goal )  {
							finished = true;
							toolbox::print_result(current_node.problem.name, current_node.applied_functions, path);
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