use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::datastructures::{node::*, problem::{*, self}, domain::{*, self}};
use std::fs::OpenOptions;
use std::io::Write;

pub fn hash_state(current_node: &mut Node) -> bool {

	let _fg = ::flame::start_guard("hash state");

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

	let new_domain = domain.clone();

	let new_types = Vec::<(String, String)>::new();

	// Can we remove objects of a certain type?
	println!("{:?}", problem.objects);

	// Can we remove any actions since we dont have the parameters types?

	// Can we remove methods because we dont have the parameters types or because we dont have the actions in the subtask? 

	todo!("Finish reduce_domain")
}

pub fn print_result(current_node: Node) {

	let _fg = ::flame::start_guard("print result");

	let data_struct = OpenOptions::new()
	.append(true)
	.open("../../notes/solution.txt");

	if data_struct.is_ok() {
		let mut data_file = data_struct.expect("cannot open file");

		data_file.set_len(0).ok();

		println!("\nFINISHED PROBLEM!\n");

		// OUTPUT IN COMPETITION FORMAT
		let intro_string = "Solution for problem: {".to_string() + &current_node.problem.name + "} by Ajess19 & Andla19\n==>\n";
		data_file.write(intro_string.as_bytes()).expect("write failed");

		for applied_action in current_node.applied_action_list.0 {

			let mut string_collect = applied_action.1.to_string() + " " + &applied_action.0 + " ";

			for param in &applied_action.2 {
				string_collect = string_collect + param + " "
			}

			string_collect = string_collect + "\n";

			data_file.write(string_collect.as_str().as_bytes()).expect("write failed");
		}

		data_file.write("<==".as_bytes()).expect("write failed");
	} else {

		let data_struct = OpenOptions::new()
		.append(true)
		.open("notes/solution.txt");

		if data_struct.is_ok() {
			let mut data_file = data_struct.expect("cannot open file");
			data_file.set_len(0).ok();

			println!("\nFINISHED PROBLEM!\n");
		
			// OUTPUT IN COMPETITION FORMAT
			let intro_string = "Solution for problem: {".to_string() + &current_node.problem.name + "} by Ajess19 & Andla19\n==>\n";
			data_file.write(intro_string.as_bytes()).expect("write failed");
		
			for applied_action in current_node.applied_action_list.0 {
		
				let mut string_collect = applied_action.1.to_string() + " " + &applied_action.0 + " ";
		
				for param in &applied_action.2 {
					string_collect = string_collect + param + " "
				}
		
				string_collect = string_collect + "\n";
		
				data_file.write(string_collect.as_str().as_bytes()).expect("write failed");
			}
		
		
			data_file.write("<==".as_bytes()).expect("write failed");
		}


	}


}

pub fn compare_lists(list1: Vec<String>, list2: Vec<String>) -> bool {

	if list1.len() == list2.len() {
		for x in 0..list1.len() {
			if !(list1[x] == list2[x]) { return false; }
		}
	} else { return false; }

	true
}

pub fn check_goal_condition( state: &Vec<(String, Vec<String>)>, goal: &Option<Vec<(String, Vec<String>)>>) -> bool {

	let _fg = ::flame::start_guard("check goal condition");

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