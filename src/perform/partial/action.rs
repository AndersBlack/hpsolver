use crate::datastructures::{domain::*, node::*};
use crate::toolbox::{self, effect, make_partial_node};

type RelVars = Vec<(String, String, Vec<String>)>;
type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);
type Called = (Vec<bool>, Vec<(Method, RelVars, Vec<Precondition>)>, Vec<usize>);

/// Improving the runtime of the perform action method using CDCL
pub fn perform_action_cdcl( node_queue: &mut Vec::<PartialNode>, mut current_node: PartialNode, action: Action, mut relevant_variables: RelVars, called: &mut Called, passing_precon: Vec<Precondition>, subtask_queue_index: usize ) -> bool {

	// Update passing preconditions	
	let new_passing_precon = toolbox::passing_preconditions::update_passing_precondition(&called, &passing_precon, &action.parameters); // Since the parameters is wrong, this must be wrong
	let precondition_list;

	// Add passing preconditions to actions own precondition list
	if action.precondition.is_some() {
		precondition_list = [action.precondition.clone().unwrap(), new_passing_precon.clone()].concat();
	} else {
		precondition_list = new_passing_precon.clone();
	}

	let mut action_can_set_effects = true;
	let cleared_precon: bool;

	// Trim values based on locked values
	(relevant_variables, cleared_precon) = toolbox::precondition::precon_trimmer( relevant_variables, &precondition_list, &current_node.problem);

	for relvar in &relevant_variables {
		if relvar.2.len() == 0 || !cleared_precon {
			return false
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

				let mut new_node_mod = current_node.clone();

				let mut branch_relevant_variable = relevant_variables.clone();
				branch_relevant_variable[relvar_index].2 = vec![value.clone()];

				new_node_mod.subtask_queue[subtask_queue_index] = (SubtaskTypes::Action(action.clone()), branch_relevant_variable, called.clone(), new_passing_precon.clone());

				let new_node = make_partial_node(new_node_mod.problem, new_node_mod.subtask_queue, new_node_mod.applied_functions, new_node_mod.hash_table, new_node_mod.hash_counter, new_node_mod.goal_functions);

				node_queue.push(new_node);
			}

			break;
		} 

		relvar_index += 1;
	}

	if action_can_set_effects {

		if called.1.len() != 0 {

			// Apply effects and return to calling method
			let (calling_method, calling_relevant_vars, called_passing_precon) = called.1.pop().unwrap();
			called.0.pop();

			//println!("Cleared action!\n");

			// Apply effects!
			effect::apply_effects_cdcl(&mut current_node.problem, &mut current_node.applied_functions, &relevant_variables, &action);

			for x in 0..relevant_variables.len() {

				match calling_method.subtasks[called.2.last().unwrap() - 1].clone() {
					(SubtaskTypes::Action(action), _) => {
						relevant_variables[x].0 = action.parameters[x].name.clone();
					},
					(SubtaskTypes::Task(task), _) => {
						relevant_variables[x].0 = task.parameters[x].name.clone();
					},
					_ => {
						// Do nothing
					}
				}
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
			
			current_node.subtask_queue[subtask_queue_index] = (SubtaskTypes::Method(calling_method), new_relevant_variables, called.clone(), called_passing_precon);

			let new_node = make_partial_node(current_node.problem, current_node.subtask_queue, current_node.applied_functions, current_node.hash_table, current_node.hash_counter, current_node.goal_functions);

			node_queue.push(new_node);

			return true
		} else {

			effect::apply_effects_cdcl(&mut current_node.problem, &mut current_node.applied_functions, &relevant_variables, &action);

			let new_node = make_partial_node(current_node.problem, current_node.subtask_queue, current_node.applied_functions, current_node.hash_table, current_node.hash_counter, current_node.goal_functions);

			node_queue.push(new_node);

			return true
		}
	}

	false
}
