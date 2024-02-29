use crate::datastructures::{domain::*, node::*};
use crate::toolbox::{self, make_node, effect};

type RelVars = Vec<(String, String, Vec<String>)>;
type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);

/// Improving the runtime of the perform action method using CDCL
pub fn perform_action_cdcl( node_queue: &mut Vec::<Node>, mut current_node: Node, action: Action, mut relevant_variables: RelVars ) {

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
			//toolbox::back_tracking::backtrack_for_parameter_value(node_queue, &relevant_variables);
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
			effect::apply_effects_cdcl(&mut current_node, &relevant_variables, &action);

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

			effect::apply_effects_cdcl(&mut current_node, &relevant_variables, &action);

			let new_node = make_node(current_node.problem.clone(), current_node.subtask_queue.clone(), current_node.called.clone(), current_node.applied_functions.clone(), current_node.hash_table.clone(), current_node.passing_preconditions.clone(), current_node.goal_functions.clone());

			node_queue.push(new_node);
		}
	}
}

/// Perform an action
pub fn perform_action( node_queue: &mut Vec::<Node>, mut current_node: Node, action: Action, relevant_variables: RelVars) {

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

			let mut new_current_node = effect::clone_node_and_apply_effects(&mut current_node, &edited_relevant_variables, &permutation, &action, &mut new_relevant_variables);

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

			let new_current_node = effect::clone_node_and_apply_effects(&mut current_node, &relevant_variables, &permutation, &action, &mut new_relevant_variables);

			let new_node = make_node(new_current_node.problem.clone(), new_current_node.subtask_queue.clone(), new_current_node.called.clone(), new_current_node.applied_functions.clone(), current_node.hash_table.clone(), Vec::<Precondition>::new(), current_node.goal_functions.clone());

			node_queue.push(new_node);
		}
	}
}