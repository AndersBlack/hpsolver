use std::{collections::HashSet, time::Instant, path::PathBuf};
use crate::algorithms::{*, self};
use crate::toolbox::{self};

// Relevant Variables datatype
type RelVars = Vec<(String, String, Vec<String>)>;

pub fn stoppable_depth_first(problem: &Problem, domain: &Domain, stopped: &Instant, path: &PathBuf) -> &'static str {

	let mut node_queue = Vec::<Node>::new();
	let mut htn_subtask_queue = Vec::<(SubtaskTypes, RelVars)>::new();
	let mut function_list = Vec::<String>::new();
	let mut new_problem: Problem = algorithms::update_objects(problem.clone(), domain);
  let applied_funtions = (("root".to_string(), Vec::<usize>::new()), Vec::<(SubtaskTypes, usize, Vec<usize>, RelVars)>::new());

	new_problem.htn.subtasks.reverse();

	for subtask in &new_problem.htn.subtasks {
		toolbox::prep_htn_subtasks(&mut htn_subtask_queue, subtask, &new_problem);
	}

	let new_domain = algorithms::reduce_domain(domain, &new_problem);

	if problem.goal.is_some() {
		function_list = toolbox::goal_oriented_finder(domain, problem.goal.clone().unwrap());
	}

	let called = (Vec::<bool>::new(), Vec::<(Method, RelVars, Vec<Precondition>)>::new(), Vec::<usize>::new());
	let new_node = algorithms::make_node(new_problem.clone(), htn_subtask_queue, called, applied_funtions, HashSet::<u64>::new(), Vec::<Precondition>::new(), function_list);
	
	node_queue.push(new_node);

	let return_string = run_df(&mut node_queue, &new_domain, stopped, path);

  return_string
}

fn run_df(node_queue: &mut Vec::<Node>, domain: &Domain, stopped: &Instant, path: &PathBuf) -> &'static str {

	let finished: bool = false;

	while !finished {

    if stopped.elapsed().as_secs() > 1800 { 
      return "stopped";
    }

		let current_node = node_queue.pop();

		// Handle subtasks
		match current_node {
			Some(mut current_node) => {

				if !toolbox::method_calls_method(&domain.methods) {
					let state_exists = toolbox::hash_state(&mut current_node);

					if state_exists {
						continue;
					}
				}

				let current_subtask = current_node.subtask_queue.pop(); 

				match current_subtask {

					Some((SubtaskTypes::HtnTask(htn_task), relevant_variables))=> {
						//println!("Htn_task: {:?}", htn_task.0);
						algorithms::perform_htn_task(node_queue, domain, current_node, htn_task, relevant_variables);
					},
					Some((SubtaskTypes::Task(task), relevant_variables)) => {
						//println!("Task: {:?}", task.name);
						algorithms::perform_task(node_queue, domain, current_node, task, relevant_variables);
					},
					Some((SubtaskTypes::Method(method), relevant_variables)) => {
						//println!("Method {:?}", method.name);					
						algorithms::perform_method(node_queue, domain, current_node, method, relevant_variables);
					},
					Some((SubtaskTypes::Action(action), relevant_variables)) => {
						//println!("Action: {:?}", action.name);
						algorithms::perform_action_cdcl(node_queue, current_node, action, relevant_variables);
					},
					None => { 

						let finished_state = toolbox::check_goal_condition( &current_node.problem.state, &current_node.problem.goal );

						if finished_state {
							toolbox::print_result(current_node.problem.name, current_node.applied_functions, path);
              return "success";
						}
					}
				}
			},
			None => { 
				return "Node queue found empty!";
			}
		}
	}

  return "error";
}