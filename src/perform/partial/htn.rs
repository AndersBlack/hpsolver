use crate::toolbox::{self, make_partial_node, RelVars, Precondition, Called};
use crate::datastructures::{domain::*, node::*};

/// Perform a htn task
pub fn perform_htn_task( node_queue: &mut Vec::<PartialNode>, domain: &Domain, current_node: PartialNode, htn_task: (String, String, Vec<String>), relevant_variables: RelVars, called: &mut Called, subtask_queue_index: usize, passing_preconditions: Vec<Precondition>) -> bool {

	let method_list_option = domain.methods.get(&htn_task.0);

	if method_list_option.is_some() {

		let mut method_list = method_list_option.unwrap().to_vec();
		method_list.sort_by(|a,b| b.subtasks.len().cmp(&a.subtasks.len()));

		// Expand task and create a new node for every method that task expands to
		'outer: for mut method in method_list {

			let updated_relevant_variables = toolbox::update::update_relevant_variables(&current_node.problem, &method, &relevant_variables);

			for rel_var in &updated_relevant_variables {
				if rel_var.2.is_empty() {
					continue 'outer;
				}
			}

			let mut subtask_queue_clone = current_node.subtask_queue.clone();
			let mut applied_functions_clone = current_node.applied_functions.clone();

			//Applied function addition
			method.id = applied_functions_clone.1.len();
			applied_functions_clone.0.1.push(method.id);
			applied_functions_clone.1.push((SubtaskTypes::Method(method.clone()), method.id, Vec::<usize>::new(), relevant_variables.clone()));

			called.0.push(false);
			called.2.push(0);

			subtask_queue_clone[subtask_queue_index] = (SubtaskTypes::Method(method.clone()),updated_relevant_variables, called.clone(), passing_preconditions.clone());
			
			let new_node = make_partial_node(current_node.problem.clone(), subtask_queue_clone, applied_functions_clone, current_node.hash_table.clone(), current_node.hash_counter.clone(), current_node.goal_functions.clone());

			node_queue.push(new_node);
		}
	} else {

		for action in &domain.actions {

			if action.name == htn_task.0 {

				let mut new_node_init = current_node.clone();
				let mut updated_relevant_variables = RelVars::new();

				for n in 0..action.parameters.len() {
					for obj in &current_node.problem.objects{
						if obj.0 == htn_task.2[n] {
							updated_relevant_variables.push((action.parameters[n].name.clone(), obj.1.clone(), vec![obj.0.clone()]));
						}
					}
				}

				called.0.push(false);
				called.2.push(0);

	
				// Update relevant variables
				new_node_init.subtask_queue[subtask_queue_index] = (SubtaskTypes::Action(action.clone()), updated_relevant_variables, called.clone(), passing_preconditions.clone());


				//Applied function addition
				new_node_init.applied_functions.0.1.push(new_node_init.applied_functions.1.len().try_into().unwrap());
				
				let new_node = make_partial_node(current_node.problem.clone(), new_node_init.subtask_queue, new_node_init.applied_functions, current_node.hash_table.clone(), current_node.hash_counter.clone(), current_node.goal_functions.clone());

				node_queue.push(new_node);
			}
		}

	} 

	true
}