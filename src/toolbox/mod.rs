use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::path::PathBuf;
use crate::datastructures::{node::*, problem::{*}, domain::{*}};
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::collections::HashMap;
use itertools::Itertools;

type RelVars = Vec<(String, String, Vec<String>)>;
type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);
type Called = (Vec<bool>, Vec<(Method, RelVars, Vec<Precondition>)>, Vec<usize>);

pub mod passing_preconditions;
pub mod constraints;
pub mod precondition;
pub mod back_tracking;
pub mod effect;
pub mod update;

/// Hashes the state and returns a boolean representing whether or not it is a duplicate state
pub fn hash_state(current_node: &mut Node) -> bool {

	//Hash and check if hashset contains
	let mut hasher: DefaultHasher = DefaultHasher::new();
	(&current_node.problem.state, &current_node.subtask_queue, &current_node.called.2).hash(&mut hasher);
	let hash = hasher.finish();

	if current_node.hash_table.contains(&hash) {
		return true
	}
	
	current_node.hash_table.insert(hash);

	false
}

pub fn partial_hash_state(current_node: &mut PartialNode, subtask_queue_index: usize, hash_limit: usize) -> bool {

	//Hash and check if hashset contains
	let mut hasher: DefaultHasher = DefaultHasher::new();
	let mut subtask_vector = Vec::<(String, RelVars, &usize)>::new();

	for st in &current_node.subtask_queue {
		match &st.0 {
			SubtaskTypes::Method(method) => {
				subtask_vector.push((method.name.clone(), st.1.clone(), st.2.2.last().unwrap()));
			},
			SubtaskTypes::Action(action) => {
				subtask_vector.push((action.name.clone(), st.1.clone(), st.2.2.last().unwrap()));
			}
			_ => {
				return false
			}
		}
	}

	(&current_node.problem.state, &subtask_vector, subtask_queue_index).hash(&mut hasher);
	let hash = hasher.finish();

	if current_node.hash_table.contains(&hash) {

		if hash_limit == 0 {
			return true
		} else {

			match current_node.hash_counter.get(&hash) {
				Some(counter) => {
					if &hash_limit >= counter {
						current_node.hash_counter.insert(hash, counter + 1);
					} else {
						return  true;
					}
				},
				None => {}
			}

		}
	} else {
		current_node.hash_table.insert(hash);
		current_node.hash_counter.insert(hash, 1);
	}

	
	false
}

pub fn await_key() {

	let mut line = String::new();
	let _b1 = std::io::stdin().read_line(&mut line).unwrap();

}

pub fn generate_method_subtask_perm(current_subtask_list: &Vec<(SubtaskTypes, Vec<Argument>)>) -> Vec<Vec<(SubtaskTypes, Vec<Argument>)>> {

	let mut subtask_permutation_list = Vec::<Vec<(SubtaskTypes, Vec<Argument>)>>::new();

	let index_vec: Vec<usize> = (0..current_subtask_list.len()).collect();	

	for perm in index_vec.iter().permutations(index_vec.len()).unique() {

		let mut new_st_list = Vec::<(SubtaskTypes, Vec<Argument>)>::new();

		for index in perm {
			new_st_list.push(current_subtask_list[*index].clone());
		}

		subtask_permutation_list.push(new_st_list);
	}

	subtask_permutation_list
}

/// Update the subtask list so that it holds subtasktypes
pub fn update_method_subtasks(methods: Vec<(Method, Vec<(String, String, Vec<String>)>)>, actions: &Vec<Action>, tasks: &Vec<Task>) -> Vec<Method> {

	let mut update_method = Vec::<Method>::new();

	for mut method in methods {

		let mut subtask_list = Vec::<(SubtaskTypes, Vec<Argument>)>::new();

		for subtask in &method.1 {

			let mut found_function = false;

			// Is it a task?
			for task in tasks {
				if subtask.0 == task.name {

					// Change arg names for task
					let mut task_with_updated_arguments = task.clone();

					for x in 0..task.parameters.len() {
						task_with_updated_arguments.parameters[x].name = subtask.2[x].clone();
					}

					subtask_list.push((SubtaskTypes::Task(task_with_updated_arguments), task.parameters.clone()));
					found_function = true;
					break;
				}
			}

			if found_function {
				continue;
			}

			// Is it an action
			for action in actions {

				if subtask.0 == action.name {

					// Change arg names for action
					let mut action_with_updated_arguments = action.clone();

					for x in 0..action.parameters.len() {
						action_with_updated_arguments.parameters[x].name = subtask.2[x].clone();
					}

					subtask_list.push((SubtaskTypes::Action(action_with_updated_arguments), action.parameters.clone()));
					break;
				}
			}
		}


		method.0.subtasks = subtask_list;
		update_method.push(method.0);
	}

	update_method
}

/// In the case that 2 parameter arguments has the same name, get the intersection of the values
pub fn check_duplicates(relvars: &mut RelVars) -> RelVars {
	let mut names = vec![];
	let mut new_relvars = RelVars::new();
	let mut relvar_index = 0;
	 
	for relvar in relvars{
		if names.contains(&relvar.0){
			let mut duplicate_index = 0;
			for duplicate in &new_relvars {
				if relvar.0 == duplicate.0{
					let new_value_list = intersection(vec![new_relvars[duplicate_index].2.clone(), relvar.2.clone()]);
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

// Decide which actions has a positive impact on the goal state
pub fn goal_oriented_action_finder( domain: &Domain, goal: Vec<(String, Vec<String>)>) -> Vec<String> {
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

pub fn goal_oriented_finder ( domain: &Domain, goal: Vec<(String, Vec<String>)>) -> Vec<String> {

	let mut action_goal_list = Vec::<String>::new();
	let mut method_goal_list = Vec::<String>::new();
	
	for predicate in goal {
		
		//  First loop
		for action in &domain.actions{
			if action.effect.is_some(){
				let effect_list = action.effect.clone().unwrap(); 
				for effect in effect_list{
					if effect.0 && effect.1 == predicate.0 {
						if !action_goal_list.contains(&action.name){
							action_goal_list.push(action.name.clone());
						}

						break;
					}
				}
			}
		}
	}

	for method_lists in &domain.methods {
		for method in method_lists.1 {
			for subtask in &method.subtasks {
				match subtask {
					(SubtaskTypes::Action(action), _) => {
						if action_goal_list.contains(&action.name) {
							method_goal_list.push(method.name.clone());
							break;
						}
					},
					_ => {}

				}
			}
		}
	}
	
	method_goal_list
}

/// Generates a Node with the given arguments
pub fn make_node( new_problem: Problem, 
	sq: Vec::<(SubtaskTypes, RelVars)>, 
	called: (Vec<bool>, Vec<(Method, RelVars, Vec<Precondition>)>, Vec<usize>), 
	afl:((String, Vec<usize>), Vec<(SubtaskTypes, usize, Vec<usize>, RelVars)>) , 
	hs: HashSet<u64>, 
	passing_preconditions: Vec<Precondition>, 
	goal_functions: Vec<String>) -> Node {

	let new_node = Node {
		problem: new_problem,
		subtask_queue: sq,
		called,
		applied_functions: afl,
		hash_table: hs,
		passing_preconditions,
		goal_functions
	};

	new_node
}

pub fn make_partial_node( new_problem: Problem, 
	sq: Vec::<(SubtaskTypes, RelVars, Called, Vec<Precondition>)>, 
	afl:((String, Vec<usize>), Vec<(SubtaskTypes, usize, Vec<usize>, RelVars)>) , 
	hs: HashSet<u64>,
	hc: HashMap<u64, usize>,
	goal_functions: Vec<String>) -> PartialNode {

	let new_node = PartialNode {
		problem: new_problem,
		subtask_queue: sq,
		applied_functions: afl,
		hash_table: hs,
		hash_counter: hc,
		goal_functions
	};

	new_node
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

	let mut new_tasks = Vec::<Task>::new();

	'outer: for task in &domain.tasks {

		for parameter in &task.parameters {
			if !type_contain_param(&new_types, &parameter.object_type) {
				continue 'outer;
			}
		}

		new_tasks.push(task.clone())
	}

	new_domain.methods = new_methods;
	new_domain.actions = new_actions;
	new_domain.types = new_types;
	new_domain.tasks = new_tasks;

	if problem.goal.is_some() {
		new_domain = effect_trim_domain(domain, &problem);
	}

	new_domain
}

fn effect_trim_domain( domain: &Domain, problem: &Problem ) -> Domain {

	let mut new_domain = domain.clone();

	// Are there any actions with an effect that is not in the goal and not in a precondition of any method/action
	'action_loop: for action in &domain.actions {

		if action.effect.is_some() {
			for effect in action.effect.as_ref().unwrap() {
				// Check goal
				for goal_pred in problem.goal.as_ref().unwrap() {
					if goal_pred.0 == effect.1 {
						continue 'action_loop;
					}
				}

				// Check methods
				for methods_set in &domain.methods {
					for method in methods_set.1 {

						let method_precons = &method.precondition;

						if method_precons.is_some() {
							let method_precons = method_precons.as_ref().unwrap();
							for precondition in method_precons {
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
						for precondition in action.precondition.as_ref().unwrap() {
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
						
						if method.subtasks.len() > 0 {
							match method.subtasks[0].clone() {
								(SubtaskTypes::Action(sub_action), _) => {
									if method.subtasks.len() == 1 && sub_action.name == action.name {

										if (methods_set.1.len() - methods_calling_action) <= 1 {
											continue 'action_loop;
										}
			
										methods_calling_action += 1;
										trim_method_list.push(method.clone());
									}
								},
								_ => {}
							}
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
pub fn print_result(problem_name: String, applied_functions: ((String, Vec<usize>), Vec<(SubtaskTypes, usize, Vec<usize>, RelVars)>), path: &PathBuf) {

	let path_buf = PathBuf::from(path);

	let path_stem = path_buf.file_stem().expect("wasn't able to stem file");
	let mut path_stem_pb = PathBuf::from(path_stem);
	let path_parent = path_buf.parent().unwrap().parent().unwrap();

	let mut new_path = PathBuf::from(path_parent);
	new_path.push("solutions");
	path_stem_pb.set_extension("solution");
	new_path.push(path_stem_pb);

	//println!("Path: {:?}\n", new_path);

	let mut _file = File::create(&new_path);

	match &_file {
		Err(e) => {
			path_stem_pb = PathBuf::from(path_stem);
			let error_path_parent = path_buf.parent().unwrap();

			new_path = PathBuf::from(error_path_parent);
			new_path.push("solutions");
			path_stem_pb.set_extension("solution");
			new_path.push(path_stem_pb);
			_file = File::create(&new_path);
		},
		_ => {}
	}

	//print!("file: {:?}\n", _file);
	//println!("Path stem: {:?}, path parent: {:?}, path: {:?}, new path: {:?}", path_stem, path_parent, path, new_path);

	let data_struct = OpenOptions::new()
	.append(true)
	.open(new_path);

	if data_struct.is_ok() {
		let mut data_file = data_struct.expect("cannot open file");

		data_file.set_len(0).ok();

		//println!("\nFINISHED PROBLEM!\n");

		// OUTPUT IN COMPETITION FORMAT
		let intro_string = "Solution for problem: {".to_string() + &problem_name + "} by Ajess19 & Andla19\n==>\n";
		data_file.write(intro_string.as_bytes()).expect("write failed");

		//PRINT
		for applied_function in &applied_functions.1 {
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
		let mut string_to_print = applied_functions.0.0 + " ";
		for function_called in applied_functions.0.1 {
			string_to_print = string_to_print + &function_called.to_string() + " ";
		}

		string_to_print = string_to_print + "\n";

		data_file.write(string_to_print.as_bytes()).expect("write failed");
		

		for applied_function in &applied_functions.1 {
			// Methods
			match &applied_function {
				(SubtaskTypes::Method(method), id, call_list, relevant_vars) => {

					let mut string_to_print = id.to_string() + " " + &method.task.0 + " ";

					//println!("relvars: {:?} method: {:?}", relevant_vars, method.name);

					for var in relevant_vars {
						if var.2.len() > 1 {
							println!("panic relvars: {:?} in {:?} ID: {id}", var, method.name);
							panic!("Values had not been reduced to 1")
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
			let intro_string = "Solution for problem: {".to_string() + &problem_name + "} by Ajess19 & Andla19\n==>\n";
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
				match sub {
					(SubtaskTypes::Task(task), _) => {
						if method.task.0 == task.name {
							return true
						}
					},
					_ => {}
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

pub fn prioritize_methods(domain: &Domain, problem: &Problem, goal_functions: &Vec<String>, task_name: String) -> Vec<Method> {

	if problem.goal.is_some() {
		
		if domain.methods.get(&task_name).is_some() {

			let mut method_list_interesting = Vec::<Method>::new(); 
			let mut method_list_rest = Vec::<Method>::new(); 

			for method in domain.methods.get(&task_name).unwrap() {
				if goal_functions.contains(&method.name) {
					method_list_interesting.push(method.clone());
				} else {
					method_list_rest.push(method.clone());
				}
			}

			method_list_interesting.sort_by(|a,b| b.subtasks.len().cmp(&a.subtasks.len()));
			method_list_rest.sort_by(|a,b| b.subtasks.len().cmp(&a.subtasks.len()));

			method_list_rest.append(&mut method_list_interesting);

			method_list_rest

		} else {
			vec![]
		}
	} else {

		let mut method_list = domain.methods.get(&task_name).unwrap().to_vec();

		method_list.sort_by(|a,b| b.subtasks.len().cmp(&a.subtasks.len()));

		method_list
	}
}

// pub fn check_for_sibling_subtask(subtask_queue: &Vec<(SubtaskTypes, RelVars, Called, Vec<Precondition>)>, id: usize, new_rel_vars: RelVars) -> bool {

// 	for subtask in subtask_queue {
// 		for called_sub in &subtask.2.1 {
// 			if called_sub.0.id == id {

// 				// 

// 				return true
// 			}	
// 		}
// 	} 

// 	return false
// }

/// Checks if the goal state reached is valid
pub fn check_goal_condition( state: &Vec<(String, Vec<String>)>, goal: &Option<Vec<(String, Vec<String>)>>) -> bool {

	let mut res = true;

	match goal {
		Some(goal) => {
			// Check if goal condition is satisfied
			for goal_req in goal {

				let mut sub_goal = false;

				for predicate in state {
					
					if goal_req.0 == predicate.0 && goal_req.1.len() > 0 {
						for i in 0..goal_req.1.len() {
							if goal_req.1[i] != predicate.1[i] {
								break;
							}

							if i == goal_req.1.len() - 1 {
								sub_goal = true;
							}
						}
					} else if goal_req.0 == predicate.0 && goal_req.1.len() == 0 {
						sub_goal = true;
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

	for effect in action.effect.as_ref().unwrap() {
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

	for effect in action.effect.as_ref().unwrap() {
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

		for effect in action.effect.as_ref().unwrap() {

			for precon in action.precondition.as_ref().unwrap() {

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
pub fn prep_htn_subtasks( htn_subtask_queue: &mut Vec::<(SubtaskTypes, RelVars)>, subtask: &(String, String, Vec::<String>), new_problem: &Problem) {
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

pub fn prep_partial_htn_subtasks( htn_subtask_queue: &mut Vec::<(SubtaskTypes, RelVars, Called, Vec<Precondition>)>, subtask: &(String, String, Vec::<String>), new_problem: &Problem) {
	let mut new_relevant_parameters = RelVars::new();
	let called = (Vec::<bool>::new(), Vec::<(Method, RelVars, Vec<Precondition>)>::new(), Vec::<usize>::new());

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

	htn_subtask_queue.push((SubtaskTypes::HtnTask(subtask.clone()), new_relevant_parameters, called.clone(), Vec::<Precondition>::new()));	
}