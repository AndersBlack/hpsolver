use core::panic;
use std::{collections::{HashMap, HashSet}, path::PathBuf};
use crate::{datastructures::{domain::*, node::*, problem::*}, toolbox::{self, constraints::*, hash_state, method_calls_method, passing_preconditions::*, precondition::{*}, reduce_domain}};
pub mod iterative_df;
pub mod stoppable_df;

/// Relevant Variables datatype: (Name, Type, List of values)
type RelVars = Vec<(String, String, Vec<String>)>;
type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);

/// Initiate depth first search
pub fn depth_first( problem: Problem, domain: &Domain, path: &PathBuf) {

	let mut node_queue = Vec::<Node>::new();
	let mut htn_subtask_queue = Vec::<(SubtaskTypes, RelVars)>::new();
	let mut new_problem: Problem = update_objects(problem.clone(), domain);
	let mut function_list: (HashMap<(String, Vec<String>), Vec<SubtaskTypes>>, Vec<String>) = (HashMap::<(String, Vec<String>), Vec<SubtaskTypes>>::new(), Vec::<String>::new());

	new_problem.htn.subtasks.reverse();

	let applied_funtions = (("root".to_string(), Vec::<usize>::new()), Vec::<(SubtaskTypes, usize, Vec<usize>, RelVars)>::new());

	for subtask in &new_problem.htn.subtasks {
		toolbox::prep_htn_subtasks(&mut htn_subtask_queue, subtask, &new_problem);
	}

	let new_domain = reduce_domain(domain, &new_problem);


	if problem.goal.is_some() {
		function_list = toolbox::goal_oriented_finder(&new_domain, problem.goal.unwrap());
	}


	println!("Function list");
	for (key, value) in function_list.0.drain() {
		println!("\nKey: {:?}", key);
		for val in value {
			match val {
					SubtaskTypes::Action(action) => {
						println!("action name: {:?}", action.name);
					},
					SubtaskTypes::Method(method) => {
						println!("method name: {:?}", method.name);
					},
					_ => {}
			}
		}
	}

	panic!("ARRRGH");

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
						println!("HTN_task: {:?} Values: {:?}", htn_task.0, htn_task.2);

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
						println!("Method: {:?}", method.name);

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
						println!("\n Action: {:?} Relevant_variables: {:?}\n", action.name, relevant_variables);
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

						if toolbox::check_goal_condition( &current_node.problem.state, &current_node.problem.goal ) {
							finished = true;
							toolbox::print_result(current_node, path);

						} else {
							println!("Hit finish without correct goal");

							let state_exists = hash_state(&mut current_node);

							if state_exists {
								panic!("Hit a dupe goal state");
							}
							
							toolbox::back_tracking::backtrack_from_goal(node_queue, &current_node.problem.state, &current_node.problem.goal.unwrap());
							
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

/// Generates a Node with the given arguments
fn make_node( new_problem: Problem, 
							sq: Vec::<(SubtaskTypes, RelVars)>, 
							called: (Vec<bool>, Vec<(Method, RelVars, Vec<Precondition>)>, Vec<usize>), 
							afl:((String, Vec<usize>), Vec<(SubtaskTypes, usize, Vec<usize>, RelVars)>) , 
							hs: HashSet<u64>, 
							passing_precondition: Vec<Precondition>, 
							goal_functions: (HashMap<(String, Vec<String>), Vec<SubtaskTypes>>, Vec<String>)) -> Node {

		let new_node = Node {
			problem: new_problem,
			subtask_queue: sq,
			called: called,
			applied_functions: afl,
			hash_table: hs,
			passing_preconditions: passing_precondition,
			goal_functions: goal_functions
		};

		new_node
}

/// Perform a htn task
fn perform_htn_task( node_queue: &mut Vec::<Node>, domain: &Domain, mut current_node: Node, htn_task: (String, String, Vec<String>, bool), relevant_variables: RelVars) {

	let method_list_option = domain.methods.get(&htn_task.0);

	if method_list_option.is_some() {

		let mut method_list = method_list_option.unwrap().to_vec();
		method_list.sort_by(|a,b| b.subtasks.len().cmp(&a.subtasks.len()));

		// Expand task and create a new node for every method that task expands to
		'outer: for mut method in method_list {
			let mut subtask_queue_clone = current_node.subtask_queue.clone();
			let updated_relevant_variables = update_relevant_variables(&current_node, &method, &relevant_variables);
			let mut applied_functions_clone = current_node.applied_functions.clone();

			//println!("htn relvar:\n method: {}\n{:?}\n", method.name, updated_relevant_variables);

			for rel_var in &updated_relevant_variables {
				if rel_var.2.is_empty() {
					continue 'outer;
				}
			}

			//Applied function addition
			method.id = applied_functions_clone.1.len();
			applied_functions_clone.0.1.push(method.id);
			applied_functions_clone.1.push((SubtaskTypes::Method(method.clone()), method.id, Vec::<usize>::new(), relevant_variables.clone()));

			// Update relevant variables
			subtask_queue_clone.push((SubtaskTypes::Method(method.clone()),updated_relevant_variables));
			
			current_node.called.0.push(false);
			current_node.called.2.push(0);
			
			let new_node = make_node(current_node.problem.clone(), subtask_queue_clone, (current_node.called.0.clone(), current_node.called.1.clone(), current_node.called.2.clone()), applied_functions_clone, current_node.hash_table.clone(), current_node.passing_preconditions.clone(), current_node.goal_functions.clone());

			node_queue.push(new_node);
		}
	} else {

		for action in &domain.actions {

			if action.name == htn_task.0 {

				let mut subtask_queue_clone = current_node.subtask_queue.clone();
				let mut updated_relevant_variables = RelVars::new();

				for n in 0..action.parameters.len() {

					for obj in &current_node.problem.objects{
						if obj.0 == htn_task.2[n] {
							updated_relevant_variables.push((action.parameters[n].name.clone(), obj.1.clone(), vec![obj.0.clone()]));
						}
					}
				}
	
				// Update relevant variables
				//println!("Updated stq with action: {}, Relvars: {:?}\n", action.name, updated_relevant_variables);
				subtask_queue_clone.push((SubtaskTypes::Action(action.clone()), updated_relevant_variables.clone()));

				current_node.called.0.push(false);
				current_node.called.2.push(0);

				//Applied function addition
				current_node.applied_functions.0.1.push(current_node.applied_functions.1.len().try_into().unwrap());
				
				let new_node = make_node(current_node.problem.clone(), subtask_queue_clone, (current_node.called.0.clone(), current_node.called.1.clone(), current_node.called.2.clone()), current_node.applied_functions.clone(), current_node.hash_table.clone(), current_node.passing_preconditions.clone(), current_node.goal_functions.clone());
	
				node_queue.push(new_node);
			}
		}

	} 

}

/// Perform a task (Make a new node for every possible method that solves the given task)
fn perform_task( node_queue: &mut Vec::<Node>, domain: &Domain, current_node: Node, task: Task, relevant_variables: RelVars ) {

	let method_list = toolbox::prioritize_methods(&domain, &current_node, task.name);

		// Expand task and create a new node for every method that task expands to
		for mut method in method_list {

			let mut cnaf = current_node.applied_functions.clone(); 

			method.id = cnaf.1.len();
			cnaf.1.push((SubtaskTypes::Method(method.clone()), method.id, Vec::<usize>::new(), relevant_variables.clone()));

			let mut new_subtask_queue = current_node.clone().subtask_queue;
			let new_rel_vars = update_relevant_variables(&current_node, &method, &relevant_variables);
			let mut empty_rel_var = false;

			for rel_var in &new_rel_vars {
				if rel_var.2.is_empty() {
					empty_rel_var = true;
				}
			}

			if !empty_rel_var {

				let new_passing_precon = update_passing_precondition(&current_node, &task.parameters);

				//println!("Updated stq with method: {}, Relvars: {:?}\n", method.name, new_rel_vars);
				new_subtask_queue.push((SubtaskTypes::Method(method.clone()), new_rel_vars));

				let mut new_called = current_node.called.clone();
				new_called.2.push(0);

				let new_node = make_node(current_node.problem.clone(), new_subtask_queue, new_called, cnaf.clone(), current_node.hash_table.clone(), new_passing_precon, current_node.goal_functions.clone());

				//println!("Pushing node");
				node_queue.push(new_node)
			}
		
		}
		
}

/// Perform a method (Check preconditions and constraints and attempt to perform every subtask)
fn perform_method( node_queue: &mut Vec::<Node>, domain: &Domain, mut current_node: Node, method: Method, mut relevant_variables: RelVars ) {

	// What is the index of this method in the subtask queue of the method that called it?
	let current_subtask_index = current_node.called.2.pop().unwrap();

	if current_subtask_index == 0 {

		// Check preconditions
		match &method.precondition {
			Some(precondition) => {

				let (new_relevant_variables, preconditions_cleared) = precon_trimmer(relevant_variables, precondition, &current_node.problem);

				if preconditions_cleared {
					relevant_variables = new_relevant_variables;
				} else {
					toolbox::back_tracking::backtrack_for_method_param_value(node_queue, &new_relevant_variables);
					return
				}

			},
			None => {}
		}
		
		// Check constraints
		if method.constraints.is_some() {
			let mut relevant_variables_list = check_constraints( &relevant_variables, &method.constraints.clone().unwrap() );

			//Return if no relevant_variables was returned from check_constraints
			if relevant_variables_list.is_empty() {
				return
			}

			relevant_variables = relevant_variables_list.pop().unwrap();

			// Push Node versions
			for relevant_variable in &relevant_variables_list {
						
				let new_method = Method {
					name: method.name.clone(),
					parameters: method.parameters.clone(),
					task: method.task.clone(),
					precondition: None,
					subtasks: method.subtasks.clone(),
					constraints: None,
					id: method.id
				};

				let mut new_sq = current_node.subtask_queue.clone();
				new_sq.push((SubtaskTypes::Method(new_method), relevant_variable.clone()));
				
				let mut new_called: (Vec<bool>, Vec<(Method, Vec<(String, String, Vec<String>)>, Vec<Precondition>)>, Vec<usize>) = current_node.called.clone();
				new_called.2.push(0);

				let new_node = make_node(current_node.problem.clone(), new_sq, new_called, current_node.applied_functions.clone(), current_node.hash_table.clone(), current_node.passing_preconditions.clone(), current_node.goal_functions.clone());

				node_queue.push(new_node);
			}
		}
	}

	if method.subtasks.len() > 0 {
		//println!("Passing precons: {:?}", new_passing_preconditions);

		// We have finished with this methods subtask 
		if current_subtask_index == method.subtasks.len() {

			let mut trimmed_task_rel_vars = RelVars::new();

			for task_val in &method.task.1 {
				for param_val in &relevant_variables {
					if task_val == &param_val.0 {
						trimmed_task_rel_vars.push(param_val.clone());
						break;
					}
				}
			}

			current_node.applied_functions.1[method.id].3 = trimmed_task_rel_vars.clone();

			// CHECK THAT EVERY VARIABLE HAS BEEN REDUCED TO ONE!
			let subtask_list = current_node.applied_functions.1[method.id].2.clone();

			for subtask in subtask_list {
				//For every subtask 
				let applied_method = &mut current_node.applied_functions.1[subtask];
				let mut param_counter = 0;
				let mut new_values = Vec::<String>::new();
				let mut found_one = false;

				for parameters in &applied_method.3.clone() {
					if parameters.2.len() > 1 {
						// println!("\n BIGGER THAN ONE! \n");
						// println!("OVERTASK PARAM: {:?}\n OVERMETHOD: {:?}\n", relevant_variables, method);
						// println!("SUBTASK PARAM: {:?}\n SUBMETHOD: {:?}", parameters, applied_method);

						//Parameter name
						let mut sub_task_task_name = String::new();

						// Get the called index for parameter
						match &applied_method.0 {
							SubtaskTypes::Method(subtask_method) => {
								sub_task_task_name = subtask_method.task.0.clone();
							},
							_ => {}
						}

						// Get name from overmethod
						for over_method_task in &method.subtasks {
							if over_method_task.0 == sub_task_task_name {
								let over_method_arg_name = over_method_task.2[param_counter].clone();
								for rel_var in &relevant_variables {
									if rel_var.0 == over_method_arg_name {
										new_values = rel_var.2.clone();
										found_one = true;
									}
								}
							}
						}
					}

					if found_one {
						applied_method.3[param_counter].2 = new_values.clone();
						found_one = false;
					}

					param_counter = param_counter + 1;
				}
			}

			// Is this not the first method?
			if current_node.called.0.pop().unwrap() {

				let new_node = update_vars_for_called_method(current_node, &method, &relevant_variables);

				node_queue.push(new_node);
			} else {
				node_queue.push(current_node.clone());
			}

		} else {
			let new_passing_preconditions = toolbox::passing_preconditions::decide_passing_preconditions( &mut current_node.passing_preconditions, &method, current_subtask_index);
			let mut new_subtask_queue = current_node.subtask_queue.clone();
			let mut found_task = false;

			// Look for the subtask among tasks in the domain
			for task in &domain.tasks { 	
				if task.name == method.subtasks[current_subtask_index].0 {

					let mut updated_variables = RelVars::new();

					for task_arg in method.subtasks[current_subtask_index].2.clone() {
						if task_arg.contains("?") {
							for var in &relevant_variables {
								if var.0 == task_arg {
									updated_variables.push(var.clone());
								}
							}
						} else {
							for obj in &current_node.problem.objects {
								if obj.0 == task_arg {
									updated_variables.push(("no name".to_string(), obj.1.clone(), vec![task_arg.clone()]));
								} 
							}
						}
					}

					let length = current_node.applied_functions.1.len();
					//println!("printing length {}", length);
					current_node.applied_functions.1[method.id].2.push(length);

					//println!("Updated stq with task: {}, Relvars: {:?}\n", method.name, updated_variables);
					new_subtask_queue.push((SubtaskTypes::Task(task.clone()), updated_variables));
					found_task = true;
					break;
				}
			}

			// Look for the subtask among actions in the domain
			if !found_task {
				// Look for the subtask among actions in the domain
				for action in domain.actions.iter().clone() {
					if action.name == method.subtasks[current_subtask_index].0 {

						let mut updated_variables = RelVars::new();
						
						for n in 0..method.subtasks[current_subtask_index].2.len() {

							if method.subtasks[current_subtask_index].2[n].contains("?"){
								for var in &relevant_variables {
									if var.0 == method.subtasks[current_subtask_index].2[n] {
										
										updated_variables.push((action.parameters[n].name.clone(), var.1.clone(), var.2.clone()));
									}
								}
							} else {

								// Found constant in action subtask
								for obj in &current_node.problem.objects{
									if obj.0 == method.subtasks[current_subtask_index].2[n] {
										updated_variables.push((action.parameters[n].name.clone(), obj.1.clone(), vec![obj.0.clone()]));
									}
								}
							}
						}

						let length = current_node.applied_functions.1.len();
						//println!("printing length {}", length);
						current_node.applied_functions.1[method.id].2.push(length);

						new_subtask_queue.push((SubtaskTypes::Action(action.clone()), updated_variables));
						break;
					}
				}
			}

			let mut new_called = current_node.called.clone();

			new_called.0.push(true);
			new_called.1.push((method, relevant_variables, current_node.passing_preconditions.clone()));
			new_called.2.push(current_subtask_index + 1);

			let new_node = make_node(current_node.problem.clone(), new_subtask_queue.clone(), new_called.clone(), current_node.applied_functions.clone(), current_node.hash_table.clone(), new_passing_preconditions.clone(), current_node.goal_functions.clone());

			node_queue.push(new_node);
		}

	} else {

		if !current_node.called.0.pop().unwrap() {
			node_queue.push(current_node.clone());
		} else {				

			let new_node = update_vars_for_called_method(current_node, &method, &relevant_variables);
			node_queue.push(new_node);
		}

	}

}

/// Improving the runtime of the perform action method using CDCL
fn perform_action_cdcl( node_queue: &mut Vec::<Node>, mut current_node: Node, action: Action, mut relevant_variables: RelVars ) {

	// Update passing preconditions	
	let new_passing_precon = toolbox::passing_preconditions::update_passing_precondition(&current_node, &action.parameters);

	// Add passing preconditions to actions own precondition list
	let mut precondition_list = action.precondition.clone().unwrap();
	precondition_list = [precondition_list, new_passing_precon.clone()].concat();

	let mut action_can_set_effects = true;
	let cleared_precon: bool;

	// Trim values based on locked values
	(relevant_variables, cleared_precon) = toolbox::precondition::precon_trimmer( relevant_variables, &precondition_list, &current_node.problem);

	for relvar in &relevant_variables {
		if relvar.2.len() == 0 || !cleared_precon {
			toolbox::back_tracking::backtrack_for_parameter_value(node_queue, &relevant_variables);
			return
		} 
	}

	if relevant_variables.len() == 0 && !cleared_precon {
		action_can_set_effects = false;
	}

	// Go through relevant variables to determine branches
	let mut relvar_index = 0;
	for relvar in &relevant_variables {

		//Relvar values list contain 1 value, move to the next
		if relvar.2.len() != 1 {
			action_can_set_effects = false;

			// Lock values and branch on action
			for value in &relvar.2 {

				let mut branch_relevant_variable = relevant_variables.clone();
				branch_relevant_variable[relvar_index].2 = vec![value.clone()];

				let mut new_sq = current_node.subtask_queue.clone();

				new_sq.push((SubtaskTypes::Action(action.clone()), branch_relevant_variable));

				let new_node = make_node(current_node.problem.clone(), new_sq.clone(), current_node.called.clone(), current_node.applied_functions.clone(), current_node.hash_table.clone(), new_passing_precon.clone(), current_node.goal_functions.clone());

				node_queue.push(new_node);
			}

			break;
		} 

		relvar_index += 1;
	}

	if action_can_set_effects {

		if current_node.called.1.len() != 0 {

			// Apply effects and return to calling method
			let (calling_method, calling_relevant_vars, called_passing_precon) = current_node.called.1.pop().unwrap();
			current_node.called.0.pop();

			if !action.effect.clone().unwrap().is_empty() && !action.precondition.clone().unwrap().is_empty() && toolbox::action_would_result_in_nothing(&relevant_variables, &action, &current_node.problem.state) {
				return;
			}

			// Apply effects!
			apply_effects_cdcl(&mut current_node, &relevant_variables, &action);

			for x in 0..relevant_variables.len() {
				let var_name = calling_method.subtasks.clone()[current_node.called.2.last().unwrap() - 1].2[x].clone();
				relevant_variables[x].0 = var_name;
			}

			let mut new_relevant_variables = RelVars::new();

			for rel_var in &calling_relevant_vars {

				let mut found_var = false;

				for new_var in &relevant_variables {
					if new_var.0 == rel_var.0 {
						new_relevant_variables.push(new_var.clone());
						found_var = true;
						break;
					}
				}

				if !found_var {
					new_relevant_variables.push(rel_var.clone());
				}
			}

			// SET METHOD BOOL TO TRUE
			let mut calling_meth = calling_method.clone();
			let mut subts = calling_meth.subtasks;
			subts[current_node.called.2.last().unwrap() - 1].3 = true;
			calling_meth.subtasks = subts;
		
			current_node.subtask_queue.push((SubtaskTypes::Method(calling_meth.clone()), new_relevant_variables.clone()));

			let new_node = make_node(current_node.problem.clone(), current_node.subtask_queue.clone(), current_node.called.clone(), current_node.applied_functions.clone(), current_node.hash_table.clone(), called_passing_precon.clone(), current_node.goal_functions.clone());

			node_queue.push(new_node);
		} else {

			apply_effects_cdcl(&mut current_node, &relevant_variables, &action);

			let new_node = make_node(current_node.problem.clone(), current_node.subtask_queue.clone(), current_node.called.clone(), current_node.applied_functions.clone(), current_node.hash_table.clone(), current_node.passing_preconditions.clone(), current_node.goal_functions.clone());

			node_queue.push(new_node);
		}
	}
}

/// Perform an action
fn perform_action( node_queue: &mut Vec::<Node>, mut current_node: Node, action: Action, relevant_variables: RelVars) {

	// Update passing preconditions	
	let new_passing_precon = toolbox::passing_preconditions::update_passing_precondition(&current_node, &action.parameters);

	// Add passing preconditions to actions own precondition list
	let mut precondition_list = action.precondition.clone().unwrap();
	precondition_list = [precondition_list, new_passing_precon].concat();

	let (mut permutation_list, edited_relevant_variables, cleared_precon) = toolbox::precondition::permutation_tool(relevant_variables.clone(), precondition_list, &current_node.problem.state, &current_node.problem);

	if action.parameters.len() == 0 && cleared_precon {
		permutation_list.push(Vec::<usize>::new());
	}

	if current_node.called.1.len() != 0 {

		let (calling_method, calling_relevant_vars, called_passing_precon) = current_node.called.1.pop().unwrap();
		current_node.called.0.pop();

		for permutation in permutation_list {

			if !action.effect.clone().unwrap().is_empty() && !action.precondition.clone().unwrap().is_empty() && toolbox::permutation_would_result_in_nothing(&permutation, &relevant_variables, &action, &current_node.problem.state) {
				continue;
			}

			let mut new_relevant_variables = RelVars::new();

			// Check if the permutation would add a duplicate state variable and ignore if so. (Very expensive)

			let mut new_current_node = clone_node_and_apply_effects(&current_node, &edited_relevant_variables, &permutation, &action, &mut new_relevant_variables);

			for x in 0..new_relevant_variables.len() {
				let var_name = calling_method.subtasks.clone()[new_current_node.called.2.last().unwrap() - 1].2[x].clone();
				new_relevant_variables[x].0 = var_name;
			}

			let mut new_new_relevant_variables = RelVars::new();

			for rel_var in &calling_relevant_vars {

				let mut found_var = false;

				for new_var in &new_relevant_variables {
					if new_var.0 == rel_var.0 {
						new_new_relevant_variables.push(new_var.clone());
						found_var = true;
						break;
					}
				}

				if !found_var {
					new_new_relevant_variables.push(rel_var.clone());
				}
			}

			// SET METHOD BOOL TO TRUE
			let mut calling_meth = calling_method.clone();
			let mut subts = calling_meth.subtasks;
			subts[current_node.called.2.last().unwrap() - 1].3 = true;
			calling_meth.subtasks = subts;
		
			new_current_node.subtask_queue.push((SubtaskTypes::Method(calling_meth.clone()), new_new_relevant_variables.clone()));

			let new_node = make_node(new_current_node.problem.clone(), new_current_node.subtask_queue.clone(), new_current_node.called.clone(), new_current_node.applied_functions.clone(), current_node.hash_table.clone(), called_passing_precon.clone(), current_node.goal_functions.clone());

			node_queue.push(new_node);
		}

	} else {

		// ACTION WAS CALLED DIRECTLY FROM HTN
		for permutation in permutation_list {

			let mut new_relevant_variables = RelVars::new();

			let new_current_node = clone_node_and_apply_effects(&current_node, &relevant_variables, &permutation, &action, &mut new_relevant_variables);

			let new_node = make_node(new_current_node.problem.clone(), new_current_node.subtask_queue.clone(), new_current_node.called.clone(), new_current_node.applied_functions.clone(), current_node.hash_table.clone(), Vec::<Precondition>::new(), current_node.goal_functions.clone());

			node_queue.push(new_node);
		}
	}
}

// Generates a new node with the effects applied based on the provided permutation
fn clone_node_and_apply_effects( current_node: &Node, relevant_variables: &RelVars, permutation: &Vec::<usize>, action: &Action, new_relevant_variables: &mut RelVars) -> Node {
	let mut new_current_node = current_node.clone();

	// Trim relevant_variables based on permutation list
	let mut index = 0;

	for variable_type in relevant_variables {
		new_relevant_variables.push((variable_type.0.clone(), variable_type.1.clone(), vec![variable_type.2[permutation[index]].clone()].clone()));
		index = index + 1;
	}



	// Apply effects for each of the possible permutation and append to node queue.
	for effect in &action.effect.clone().unwrap() {
		apply_effect(&effect, &mut new_current_node.problem, &new_relevant_variables)
	}

	new_current_node.applied_functions.1.push((SubtaskTypes::Action(action.clone()), new_current_node.applied_functions.1.len(), Vec::<usize>::new(), new_relevant_variables.clone()));

	new_current_node
}

fn apply_effects_cdcl( current_node: &mut Node, relevant_variables: &RelVars, action: &Action) {

	if action.effect.is_some() {
		for effect in &action.effect.clone().unwrap() {

			apply_effect(effect, &mut current_node.problem, relevant_variables);

		}
	}

	current_node.applied_functions.1.push((SubtaskTypes::Action(action.clone()), current_node.applied_functions.1.len(), Vec::<usize>::new(), relevant_variables.clone()));

}

/// Adds the supertype of every object in the problem to a list on the object datastructure and returns a new Problem
fn update_objects( mut problem: Problem, domain: &Domain ) -> Problem {

	let mut new_object_list = Vec::<(String, String, Vec<String>)>::new();

	if domain.constants.is_some() {
		for constant in domain.constants.clone().unwrap() {
			problem.objects.push((constant.0.clone(), constant.1.clone(), vec![]));
		}
	}

	for object in &problem.objects {
		let mut final_subtype_list = Vec::<String>::new();
		let mut current_subtype_list = Vec::<String>::new();
		current_subtype_list.push(object.1.clone());
		final_subtype_list.push(object.1.clone());

		while !current_subtype_list.is_empty() {
			let current_sub_type = current_subtype_list.pop().unwrap();
			for sub_type in &domain.types {
				
				if sub_type.0 == current_sub_type && sub_type.1 != sub_type.0 {
					current_subtype_list.push(sub_type.1.clone());
					final_subtype_list.push(sub_type.1.clone());
				}
			}
		}

		new_object_list.push((object.0.clone(), object.1.clone(), final_subtype_list));
	} 

	problem.objects = new_object_list;

	problem
} 

/// Updates relevant variables for the method that called the current task in order to trim the caller methods variables
fn update_vars_for_called_method( mut current_node: Node, method: &Method, relevant_variables: &RelVars) -> Node {

	let (mut calling_method, calling_relevant_vars, called_passing_precon) = current_node.called.1.pop().unwrap();

	let calling_method_subtask = calling_method.subtasks[current_node.called.2.last().unwrap() - 1].clone();

	let mut i = 0;

	let mut new_rel_vars: RelVars = RelVars::new();

	for task_arg in &method.task.1 {
		for var in relevant_variables {
			//println!("Checking loop: i - {}, method_task_arg - {:?}, var - {}", i, method.task.1, var.0);
			if &var.0 == task_arg {
				new_rel_vars.push((calling_method_subtask.2[i].clone(), var.1.clone(), var.2.clone()));
				i = i + 1;
			} 
		}
	}

	let mut new_new_relevant_variables: RelVars = RelVars::new();

	for rel_var in &calling_relevant_vars {

		let mut found_var = false;

		for x in 0..new_rel_vars.len() {
			if new_rel_vars[x].0 == rel_var.0 {
				new_new_relevant_variables.push(new_rel_vars[x].clone());
				found_var = true;
				break;
			}
		}

		if !found_var {
			new_new_relevant_variables.push(rel_var.clone());
		}
	}

	calling_method.subtasks[current_node.called.2.last().unwrap() - 1].3 = true;

	// Push to subtask_q
	let mut new_sq = current_node.subtask_queue.clone();
	new_sq.push((SubtaskTypes::Method(calling_method.clone()), new_new_relevant_variables.clone()));

	let new_node = make_node(current_node.problem.clone(), new_sq, current_node.called.clone(), current_node.applied_functions.clone(), current_node.hash_table.clone(), called_passing_precon, current_node.goal_functions);

	new_node
}

/// Updates relevant variables by combining the given parameters from task and finding the new parameters in objects in order to have all parameters in the signature
fn update_relevant_variables( node: &Node, method: &Method, old_relevant_variables: &RelVars) -> RelVars {

	let mut updated_relevant_parameters = RelVars::new();

	for param in &method.parameters {

		let mut found_in_task = false;
		let mut looking_count = 0;
			
		for task_param in &method.task.1 {
			
			// The parameter was provided by the task
			if &param.name == task_param {

				let param_name = param.name.clone();
				let param_object_type = param.object_type.clone();

				if old_relevant_variables[looking_count].1 != param.object_type{

					let mut new_var_value_list = Vec::<String>::new();

					for var in &old_relevant_variables[looking_count].2{
						for object in &node.problem.objects {
							if var == &object.0 && object.2.contains(&param.object_type) {
								new_var_value_list.push(var.clone());
							}
						}
					}

					updated_relevant_parameters.push((param_name, param_object_type, new_var_value_list));

				} else{
					updated_relevant_parameters.push((param_name, param_object_type, old_relevant_variables[looking_count].2.clone()));
				}						
				found_in_task = true;
			}
		
			looking_count = looking_count + 1;
		}
		
		// The parameter was not provided by the task and therefore the possible values of the parameter is every object matching the type
		if !found_in_task {
					
			let mut var_list = Vec::<String>::new();
	
			for object in &node.problem.objects {
				if object.2.contains(&param.object_type) {
					var_list.push(object.0.clone());
				}
			}
		
			updated_relevant_parameters.push((param.name.clone(), param.object_type.clone(), var_list.clone()));
		}
	}

	//println!("updated_rel_var: {:?}\n", updated_relevant_parameters);
	updated_relevant_parameters = check_duplicates(&mut updated_relevant_parameters);

	updated_relevant_parameters
}

fn check_duplicates(relvars: &mut RelVars) -> RelVars {
	let mut names = vec![];
	let mut new_relvars = RelVars::new();
	let mut relvar_index = 0;
	 
	for relvar in relvars{
		if names.contains(&relvar.0){
			let mut duplicate_index = 0;
			for duplicate in new_relvars.clone() {
				if relvar.0 == duplicate.0{
					let new_value_list = toolbox::intersection(vec![new_relvars[duplicate_index].2.clone(), relvar.2.clone()]);
					new_relvars[duplicate_index].2 = new_value_list;
					break;
				}
				duplicate_index = duplicate_index + 1;
			}
		} else {
			names.push(relvar.0.clone());
			new_relvars.push(relvar.clone());
		}
		relvar_index = relvar_index + 1;

	}
	new_relvars
}

/// Applies the effect of an action
fn apply_effect( effect: &(bool,String,Vec<String>), problem: &mut Problem, param_list: &RelVars ) {

	if effect.0 == false {

		let effect_values = generate_effect_param_list(effect, &param_list); 

		// Remove found from state
		let optional_index = problem.state.iter().position(|x| (x.0 == effect.1 && toolbox::compare_lists(&x.1, &effect_values)));

		if optional_index.is_some() {
			problem.state.remove(optional_index.unwrap());
		}
		
	} else {

		let effect_values = generate_effect_param_list(effect, &param_list); 

		let new_state_param = (effect.1.clone(), effect_values);
		problem.state.push(new_state_param);
	}

} 

/// Makes a list for every parameter relevant to the effect
fn generate_effect_param_list( effect: &(bool,String,Vec<String>), param_list: &RelVars) -> Vec<String> {

	let mut effect_values = Vec::<String>::new();

	for effect_var in &effect.2 {
		for value in param_list {
			if effect_var == &value.0 {
				effect_values.push(value.2[0].clone());
			}
		}
	}

	effect_values
}