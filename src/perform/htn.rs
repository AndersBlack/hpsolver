use crate::toolbox::{self, make_node};
use crate::datastructures::{domain::*, node::*};

type RelVars = Vec<(String, String, Vec<String>)>;

/// Perform a htn task
pub fn perform_htn_task( node_queue: &mut Vec::<Node>, domain: &Domain, mut current_node: Node, htn_task: (String, String, Vec<String>, bool), relevant_variables: RelVars) {

	let method_list_option = domain.methods.get(&htn_task.0);

	if method_list_option.is_some() {

		let mut method_list = method_list_option.unwrap().to_vec();
		method_list.sort_by(|a,b| b.subtasks.len().cmp(&a.subtasks.len()));

		// Expand task and create a new node for every method that task expands to
		'outer: for mut method in method_list {
			let mut subtask_queue_clone = current_node.subtask_queue.clone();
			let updated_relevant_variables = toolbox::update::update_relevant_variables(&current_node, &method, &relevant_variables);
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