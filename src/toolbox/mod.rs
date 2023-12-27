use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::string;
use crate::datastructures::{node::*, problem::{*}, domain::{*}};
use std::fs::OpenOptions;
use std::io::Write;

pub fn hash_state(current_node: &mut Node) -> bool {

	//Hash and check if hashset contains
	let mut hasher: DefaultHasher = DefaultHasher::new();
	(current_node.problem.state.clone(), current_node.subtask_queue.clone()).hash(&mut hasher);
	let hash = hasher.finish();
	

	if current_node.hash_table.contains(&hash) {
		return true
	} else {
		current_node.hash_table.insert(hash);
	}

	return false
}

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

	let mut new_methods = Vec::<Method>::new();

	// Can we remove methods because we dont have the parameters types or because we dont have the actions in the subtask?
	'outer: for method in &domain.methods {

		for parameter in &method.parameters {
			if !type_contain_param(&new_types, &parameter.object_type) {
				continue 'outer;
			}
		}

		new_methods.push(method.clone())
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

pub fn print_result(current_node: Node) {

	let data_struct = OpenOptions::new()
	.append(true)
	.open("notes/solution.txt");

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
					let mut string_to_print = String::new();

					string_to_print = id.to_string() + " " + &action.name + " ";

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
					let mut string_to_print = String::new();

					string_to_print = id.to_string() + " " + &method.task.0 + " ";

					for var in relevant_vars {
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

pub fn method_calls_method(method_list: &Vec<Method>) -> bool {

	for method in method_list {
		for sub in &method.subtasks {
			if method.task.0 == sub.0 {
				return true
			}
		}
	}

	false
}

pub fn compare_lists(list1: Vec<String>, list2: Vec<String>) -> bool { 

	if list1.len() == list2.len() {
		for x in 0..list1.len() {
			if !(list1[x] == list2[x]) { return false; }
		}
	} else { return false; }

	//println!("return true from comp list");
	true
}

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

			return res
		},
		None => {
			// No goal state is specified and therefore the condition is satisfied automatically
			return res
		}
	}

}