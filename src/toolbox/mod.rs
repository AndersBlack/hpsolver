use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::path::PathBuf;
use crate::datastructures::{node::*, problem::{*}, domain::{*}};
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::collections::HashMap;

type RelVars = Vec<(String, String, Vec<String>)>;
type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);

pub mod passing_preconditions;
pub mod constraints;
pub mod precondition;
pub mod back_tracking;

/// Hashes the state and returns a boolean representing whether or not it is a duplicate state
pub fn hash_state(current_node: &mut Node) -> bool {

	//Hash and check if hashset contains
	let mut hasher: DefaultHasher = DefaultHasher::new();
	(current_node.problem.state.clone(), current_node.subtask_queue.clone()).hash(&mut hasher);
	let hash = hasher.finish();
	
	if current_node.hash_table.contains(&hash) {
		return true
	}
	
	current_node.hash_table.insert(hash);

	false
}

// Decide which actions has a positive impact on the goal state
pub fn goal_oriented_action_finder ( domain: &Domain, goal: Vec<(String, Vec<String>)>) -> Vec<String> {
	let mut goal_action_list = Vec::<String>::new();
	
	for predicate in goal{
		for action in &domain.actions{
			if !goal_action_list.contains(&action.name) && action.effect.is_some(){
				let effect_list = action.effect.clone().unwrap(); 
				for effect in effect_list{
					if effect.0 && effect.1 == predicate.0{
						goal_action_list.push(action.name.clone());
						break;
					}
				}
			}
		}
	}

	goal_action_list
}

pub fn goal_oriented_finder ( domain: &Domain, goal: Vec<(String, Vec<String>)>) -> (HashMap<(String, Vec<String>), Vec<SubtaskTypes>>, Vec<String> ) {
	let mut goal_list = (HashMap::<(String, Vec<String>), Vec<SubtaskTypes>>::new(),Vec::<String>::new());
	
	for predicate in goal {

		let mut function_list = Vec::<SubtaskTypes>::new();
		let mut precon_list = Vec::<Precondition>::new();
		let mut pushed_already = Vec::<String>::new();
		
		//  First loop
		for action in &domain.actions{
			if action.effect.is_some(){
				let effect_list = action.effect.clone().unwrap(); 
				for effect in effect_list{
					if effect.0 && effect.1 == predicate.0 {
						if !goal_list.1.contains(&action.name){
							pushed_already.push(action.name.clone());
							goal_list.1.push(action.name.clone());
						}
						
						function_list.push(SubtaskTypes::Action(action.clone()));

						if action.precondition.is_some() {
							for precondition in action.precondition.clone().unwrap() {
								precon_list.push(precondition);
							}
						}

						for method_list in domain.methods.clone().into_values(){
							for method in method_list {
								for subtask in &method.subtasks {
									if subtask.0 == action.name{
										pushed_already.push(method.name.clone());
										function_list.push(SubtaskTypes::Method(method.clone()));
										if method.precondition.is_some() {
											for precondition in method.precondition.clone().unwrap() {
												precon_list.push(precondition);
											}
										}
									}
								}
							}	
						}

						break;
					}
				}
			}
		}

		// Precondition while loop

		while !precon_list.is_empty() {

			let precondition = precon_list.pop().unwrap();

			match precondition.0 {
				0 => {

					// Look for actions & methods
					for action in &domain.actions {
						if action.effect.is_some() {
							for effect in action.effect.clone().unwrap() {
								if effect.1 == precondition.1 {

									if !pushed_already.contains(&action.name) {
										function_list.push(SubtaskTypes::Action(action.clone()));
										pushed_already.push(action.name.clone());
										goal_list.1.push(action.name.clone());
										
										if action.precondition.is_some() {
											for precondition in action.precondition.clone().unwrap() {
												if precondition.0 == 0 {
													precon_list.push(precondition.clone());
												}
											}
										}
									}

									for method_list in domain.methods.clone().into_values(){
										for method in &method_list {
											for subtask in &method.subtasks {
												if subtask.0 == action.name{
													if !pushed_already.contains(&method.name) {
														function_list.push(SubtaskTypes::Method(method.clone()));
														pushed_already.push(method.name.clone());

														if method.precondition.is_some() {
															for precondition in method.precondition.clone().unwrap() {
																if precondition.0 == 0 {
																	precon_list.push(precondition.clone());
																}
															}
														}
													}
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
					// I dont want nada
				}
			}
		}

		goal_list.0.insert(predicate, function_list);
	}
	



	goal_list
}

/// Removes actions, methods & tasks from the domain based on objects in problem
pub fn reduce_domain( domain: &Domain, problem: &Problem ) -> Domain {

	let mut new_domain = domain.clone();
	let mut new_types = Vec::<(String, String)>::new();

	// Can we remove objects of a certain type?
	'outer: for type_obj in &domain.types {

		for obj in &problem.objects {
			if obj.2.contains(&type_obj.0) {
				new_types.push(type_obj.clone());
				continue 'outer;
			}
		}

	}

	//println!("TYPES: Old length: {}, New length: {}", domain.types.len(), new_types.len());

	let mut new_actions = Vec::<Action>::new();

	// Can we remove any actions since we dont have the parameters types?
	'outer: for action in &domain.actions {

		for parameter in &action.parameters {
			if !type_contain_param(&new_types, &parameter.object_type) {
				continue 'outer;
			}
		}

		new_actions.push(action.clone())
	}

	//println!("ACTIONS: Old length: {}, New length: {}", domain.actions.len(), new_actions.len());

	let mut new_methods = HashMap::<String,Vec<Method>>::new();

	// Can we remove methods because we dont have the parameters types or because we dont have the actions in the subtask?
	for method_list in &domain.methods {
		let mut new_method_list = Vec::<Method>::new();

		'outer: for method in method_list.1 {

			for parameter in &method.parameters {
				if !type_contain_param(&new_types, &parameter.object_type) {
					continue 'outer;
				}
			}

			new_method_list.push(method.clone());
		}

		if !new_method_list.is_empty(){
			new_methods.insert(method_list.0.clone(),new_method_list);
		}
	}

	//println!("METHODS: Old length: {}, New length: {}", domain.methods.len(), new_methods.len());

	let mut new_tasks = Vec::<Task>::new();

	'outer: for task in &domain.tasks {

		for parameter in &task.parameters {
			if !type_contain_param(&new_types, &parameter.object_type) {
				continue 'outer;
			}
		}

		new_tasks.push(task.clone())
	}

	//println!("TASKS: Old length: {}, New length: {}", domain.tasks.len(), new_tasks.len());

	new_domain.methods = new_methods;
	new_domain.actions = new_actions;
	new_domain.types = new_types;
	new_domain.tasks = new_tasks;

	// if problem.goal.is_some() {
	// 	new_domain = effect_trim_domain(domain, &problem);
	// }

	new_domain
}

fn effect_trim_domain( domain: &Domain, problem: &Problem ) -> Domain {

	let mut new_domain = domain.clone();

	// Are there any actions with an effect that is not in the goal and not in a precondition of any method/action
	'action_loop: for action in &domain.actions {

		if action.effect.is_some() {
			for effect in action.effect.clone().unwrap() {
				// Check goal
				for goal_pred in problem.goal.clone().unwrap() {
					if goal_pred.0 == effect.1 {
						continue 'action_loop;
					}
				}

				// Check methods
				for methods_set in &domain.methods {
					for method in methods_set.1 {

						if method.precondition.is_some() {
							for precondition in method.precondition.clone().unwrap() {
								if precondition.0 == 1 || precondition.0 == 2 {
									if precondition.1 == effect.1 {

										continue 'action_loop;

									}
								}
							}
						}
					}
				}

				// Check actions
				for action in &domain.actions {
					if action.precondition.is_some() {
						for precondition in action.precondition.clone().unwrap() {
							if precondition.1 == effect.1 {
								continue 'action_loop;
							}
						}
					}
				}

				// No action/method uses the effect in precon and it is not part of the goal
				// For every method calling the action, is it the only action? And are there other methods that call the same task?
				let mut trim_method_list = Vec::<Method>::new();
				
				for methods_set in &domain.methods {
					let mut methods_calling_action = 0;
					for method in methods_set.1 {
						
						if method.subtasks.len() == 1 && method.subtasks[0].0 == action.name {

							if (methods_set.1.len() - methods_calling_action) <= 1 {
								continue 'action_loop;
							}

							methods_calling_action += 1;
							trim_method_list.push(method.clone());

						}  
					}	
				}

				if trim_method_list.is_empty() {
					continue 'action_loop;
				}

				// Actually trim action and all methods in method list
				// Trim Action
				let mut action_index = 0;
				let mut should_trim = false;
				for action_remove in &new_domain.actions {

					if action.name == action_remove.name {
						should_trim = true;
						break;
					}

					action_index += 1;
				}

				if should_trim {
					new_domain.actions.remove(action_index);
				}


				// Trim Method
				for trim_method in trim_method_list {
					
					let method_list = domain.methods.get(&trim_method.task.0).unwrap();
					let mut new_method_list = Vec::<Method>::new();

					for method in method_list {
						if !(trim_method.name == method.name) { 
							new_method_list.push(method.clone());
						}
					} 

					new_domain.methods.insert(trim_method.task.0, new_method_list);
				}

			} 
		}
	}
	

	new_domain
}

fn type_contain_param(types: &Vec<(String,String)>, check_type: &String) -> bool {

	for type_obj in types {
		if &type_obj.0 == check_type || &type_obj.1 == check_type {
			return true;
		}
	}

	false
}

/// Writes the solution to the solution files
pub fn print_result(current_node: Node, path: &PathBuf) {

	let path_buf = PathBuf::from(path);

	let path_stem = path_buf.file_stem().expect("wasn't able to stem file");
	let mut path_stem_pb = PathBuf::from(path_stem);
	let path_parent = path_buf.parent().unwrap().parent().unwrap();

	let mut new_path = PathBuf::from(path_parent);
	new_path.push("solutions");
	path_stem_pb.set_extension("solution");
	new_path.push(path_stem_pb);

	let _ = File::create(new_path.clone());

	//println!("Path stem: {:?}, path parent: {:?}, path: {:?}, new path: {:?}", path_stem, path_parent, path, new_path);

	let data_struct = OpenOptions::new()
	.append(true)
	.open(new_path);

	if data_struct.is_ok() {
		let mut data_file = data_struct.expect("cannot open file");

		data_file.set_len(0).ok();

		//println!("\nFINISHED PROBLEM!\n");

		// OUTPUT IN COMPETITION FORMAT
		let intro_string = "Solution for problem: {".to_string() + &current_node.problem.name + "} by Ajess19 & Andla19\n==>\n";
		data_file.write(intro_string.as_bytes()).expect("write failed");

		//PRINT
		for applied_function in &current_node.applied_functions.1 {
			// Actions
			match &applied_function {
				(SubtaskTypes::Action(action), id, _, relevant_vars) => {

					let mut string_to_print = id.to_string() + " " + &action.name + " ";

					for var in relevant_vars {
						string_to_print = string_to_print + &var.2[0] + " ";
					}

					string_to_print = string_to_print + "\n";

					data_file.write(string_to_print.as_bytes()).expect("write failed");
				},
				(_,_,_,_) => { 
					//Ignoring
				}
			}
		}

		// Root
		let mut string_to_print = current_node.applied_functions.0.0 + " ";

		for function_called in current_node.applied_functions.0.1 {
			string_to_print = string_to_print + &function_called.to_string() + " ";
		}

		string_to_print = string_to_print + "\n";

		data_file.write(string_to_print.as_bytes()).expect("write failed");
		

		for applied_function in &current_node.applied_functions.1 {
			// Methods
			match &applied_function {
				(SubtaskTypes::Method(method), id, call_list, relevant_vars) => {

					let mut string_to_print = id.to_string() + " " + &method.task.0 + " ";

					//println!("relvars: {:?} method: {:?}", relevant_vars, method.name);

					for var in relevant_vars {
						if var.2.len() > 1 {
							println!("panic relvars: {:?} in {:?} ID: {id}", var, method.name);
							//panic!("Values had not been reduced to 1")
						}
						string_to_print = string_to_print + &var.2[0] + " ";
					}

					string_to_print = string_to_print + "->" + " " + &method.name + " ";

					for called in call_list {
						string_to_print = string_to_print + &called.to_string() + " ";
					}

					string_to_print = string_to_print + "\n";

					data_file.write(string_to_print.as_bytes()).expect("write failed");
				},
				(_,_,_,_) => { 
					//Ignoring
				}
			}
		}

		data_file.write("<==".as_bytes()).expect("write failed");
	} else {

		let data_struct = OpenOptions::new()
		.append(true)
		.open("../../notes/solution.txt");

		if data_struct.is_ok() {
			let mut data_file = data_struct.expect("cannot open file");
			data_file.set_len(0).ok();

			//println!("\nFINISHED PROBLEM!\n");
		
			// OUTPUT IN COMPETITION FORMAT
			let intro_string = "Solution for problem: {".to_string() + &current_node.problem.name + "} by Ajess19 & Andla19\n==>\n";
			data_file.write(intro_string.as_bytes()).expect("write failed");
			
			//PRINT
		
			data_file.write("<==".as_bytes()).expect("write failed");
		}


	}

}

/// Checks if the domain contains methods that call itself
pub fn method_calls_method(method_list: &HashMap<String, Vec<Method>>) -> bool {

	for method_set in method_list {
		for method in method_set.1 {
			for sub in &method.subtasks {
				if method.task.0 == sub.0 {
					return true
				}
			}
		}
	}

	false
}

pub fn compare_lists(list1: &Vec<String>, list2: &Vec<String>) -> bool { 

	if list1.len() == list2.len() {
		for x in 0..list1.len() {
			if !(list1[x] == list2[x]) { return false; }
		}
	} else { return false; }

	//println!("return true from comp list");
	true
}

pub fn intersection(nums: Vec<Vec<String>>) -> Vec<String> {
	let mut intersect_result: Vec<String> = nums[0].clone();

	for temp_vec in nums {
			let unique_a: HashSet<String> = temp_vec.into_iter().collect();
			intersect_result = unique_a
					.intersection(&intersect_result.into_iter().collect())
					.map(|i| i.clone())
					.collect::<Vec<_>>();
	}
	intersect_result
}

pub fn prioritize_methods(domain: &Domain, current_node: &Node, task_name: String) -> Vec<Method> {

	if current_node.problem.goal.is_some() {
		
		if domain.methods.get(&task_name).is_some() {

			let mut method_list_interesting = Vec::<Method>::new(); 
			let mut method_list_rest = Vec::<Method>::new(); 

			for method in domain.methods.get(&task_name).unwrap() {

				let mut was_interesting = false;
					
				for subtask in &method.subtasks {
					if current_node.goal_functions.1.contains(&subtask.0) {
						method_list_interesting.push(method.clone());
						was_interesting = true;
						break;
					}
				}

				if !was_interesting {
					method_list_rest.push(method.clone());
				}
				
			}

			method_list_interesting.sort_by(|a,b| b.subtasks.len().cmp(&a.subtasks.len()));
			method_list_rest.sort_by(|a,b| b.subtasks.len().cmp(&a.subtasks.len()));

			method_list_interesting.append(&mut method_list_rest);

			return method_list_interesting

		} else {
			return vec![];
		}
	} else {

		let mut method_list = domain.methods.get(&task_name).unwrap().to_vec();

		method_list.sort_by(|a,b| b.subtasks.len().cmp(&a.subtasks.len()));

		return method_list
	}
}

/// Checks if the goal state reached is valid
pub fn check_goal_condition( state: &Vec<(String, Vec<String>)>, goal: &Option<Vec<(String, Vec<String>)>>) -> bool {

	let mut res = true;

	match goal {
		Some(goal) => {
			// Check if goal condition is satisfied
			for goal_req in goal {

				let mut sub_goal = false;

				for predicate in state {
					
					if goal_req.0 == predicate.0 {
						for i in 0..goal_req.1.len() {
							if goal_req.1[i] != predicate.1[i] {
								break;
							}

							if i == goal_req.1.len() -1 {
								sub_goal = true;
							}
						}
					}

					if sub_goal {
						break;
					}
				}

				if !sub_goal {
					res = false;
					break;
				}
			}

			res
		},
		None => {
			// No goal state is specified and therefore the condition is satisfied automatically
			res
		}
	}

}

pub fn action_would_result_in_nothing(relevant_variables: &RelVars, action: &Action, state: &Vec<(String, Vec<String>)>) -> bool {

	let mut action_would_result_in_nothing = true;

	if effect_is_in_precons(action) {
		return false;
	}

	for effect in action.effect.clone().unwrap() {
		let mut effect_would_add_something = true;

		if effect.0 {

			let mut found_pred = false;

			for pred in state {

				if effect.1 == pred.0 {

					found_pred = true;
					let mut pred_index = 0;
					
					'param: for param in &effect.2 {

						let mut rel_index = 0;

						for rel_var in relevant_variables{

							if param == &rel_var.0 {

								if !(pred.1[pred_index] == rel_var.2[0]) {
									break 'param;
								}

							}

							rel_index = rel_index + 1;
						}

						pred_index = pred_index + 1;

						if pred_index == effect.2.len()  {
							effect_would_add_something = false;
						}
					}
				}
			}

			if !found_pred {
				action_would_result_in_nothing = false;
				break;
			}
		} else {
			action_would_result_in_nothing = false;
		}

		if effect_would_add_something {
			action_would_result_in_nothing = false;
			break;
		}
	}


	action_would_result_in_nothing
}

pub fn permutation_would_result_in_nothing(permutation: &Vec<usize>, relevant_variables: &RelVars, action: &Action, state: &Vec<(String, Vec<String>)>) -> bool {

	let mut permutation_would_result_in_nothing = true;

	if effect_is_in_precons(action) {
		return false;
	}

	for effect in action.effect.clone().unwrap() {
		let mut effect_would_add_something = true;

		if effect.0 {

			let mut found_pred = false;

			for pred in state {

				if effect.1 == pred.0 {

					found_pred = true;
					let mut pred_index = 0;
					
					'param: for param in &effect.2 {

						let mut rel_index = 0;

						for rel_var in relevant_variables{

							if param == &rel_var.0 {

								//println!("Pred value: {:?}, Relvar value: {:?}", pred.1[pred_index], rel_var.2[permutation[rel_index]]);

								if !(pred.1[pred_index] == rel_var.2[permutation[rel_index]]) {
									break 'param;
								}
							}

							rel_index = rel_index + 1;
						}

						pred_index = pred_index + 1;

						if pred_index == effect.2.len()  {
							effect_would_add_something = false;
						}
					}
				}
			}

			if !found_pred {
				permutation_would_result_in_nothing = false;
				break;
			}
		} else {
			permutation_would_result_in_nothing = false;
		}

		if effect_would_add_something {
			permutation_would_result_in_nothing = false;
			break;
		}
	}

	permutation_would_result_in_nothing
}

pub fn effect_is_in_precons(action: &Action) -> bool {

	let mut precon_is_in_effect = false;

	if action.effect.is_some() && action.precondition.is_some() {

		for effect in &action.effect.clone().unwrap() {

			for precon in &action.precondition.clone().unwrap() {

				if effect.1 == precon.1 {

					let mut found_eq = 0;

					for i in 0..(effect.2.len()) {

						if effect.2[i] == precon.2[i] {
							found_eq += 1;
						}
					}

					if found_eq == effect.2.len() {
						precon_is_in_effect = true;
						break;
					}
				}
			}
		}
	}

	precon_is_in_effect
}

/// Prepare htn subtasks with relevant parameters
pub fn prep_htn_subtasks( htn_subtask_queue: &mut Vec::<(SubtaskTypes, RelVars)>, subtask: &(String, String, Vec::<String>, bool), new_problem: &Problem) {
	let mut new_relevant_parameters = RelVars::new();

	for item in &subtask.2 {
		if item.contains("?") {
			
			for parameter in &new_problem.htn.parameters {
				if item == &parameter.0 {

					let mut var_vec = Vec::<String>::new();

					for object in &new_problem.objects {
						if object.1 == parameter.1 {
							var_vec.push(object.0.clone());
						} else if object.2.contains(&parameter.1) {
							var_vec.push(object.0.clone());
						}
					}

					new_relevant_parameters.push((parameter.0.clone(), parameter.1.clone(), var_vec));
				}
			}
		} else {
			for object in &new_problem.objects {
				if &object.0 == item {
					new_relevant_parameters.push(("no name".to_string(), object.1.clone(), vec![item.clone()]))
				}
			}
		}
	}

	htn_subtask_queue.push((SubtaskTypes::HtnTask(subtask.clone()), new_relevant_parameters));	
}