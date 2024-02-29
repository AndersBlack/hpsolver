use crate::toolbox::{self, make_node};
use crate::datastructures::{domain::*, node::*};

type RelVars = Vec<(String, String, Vec<String>)>;

/// Perform a task (Make a new node for every possible method that solves the given task)
pub fn perform_task( node_queue: &mut Vec::<Node>, domain: &Domain, current_node: Node, task: Task, relevant_variables: RelVars ) {

	let method_list = toolbox::prioritize_methods(&domain, &current_node, task.name);

		// Expand task and create a new node for every method that task expands to
		for mut method in method_list {

			let mut cnaf = current_node.applied_functions.clone(); 

			method.id = cnaf.1.len();
			cnaf.1.push((SubtaskTypes::Method(method.clone()), method.id, Vec::<usize>::new(), relevant_variables.clone()));

			let mut new_subtask_queue = current_node.clone().subtask_queue;
			let new_rel_vars = toolbox::update::update_relevant_variables(&current_node, &method, &relevant_variables);
			let mut empty_rel_var = false;

			for rel_var in &new_rel_vars {
				if rel_var.2.is_empty() {
					empty_rel_var = true;
				}
			}

			if !empty_rel_var {

				let new_passing_precon = toolbox::passing_preconditions::update_passing_precondition(&current_node, &task.parameters);

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