use crate::datastructures::{domain::*, node::*, problem::*};
use crate::toolbox::{self};

type RelVars = Vec<(String, String, Vec<String>)>;

/// Applies the effect of an action
pub fn apply_effect( effect: &(bool,String,Vec<String>), problem: &mut Problem, param_list: &RelVars) {

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
pub fn generate_effect_param_list( effect: &(bool,String,Vec<String>), param_list: &RelVars) -> Vec<String> {

	let mut effect_values = Vec::<String>::new();

	for effect_var in &effect.2 {

		if effect_var.contains("?") {

			for value in param_list {
				if effect_var == &value.0 {
					effect_values.push(value.2[0].clone());
				}
			}

		} else {
			effect_values.push(effect_var.clone());
		}


	}

	effect_values
}

// Generates a new node with the effects applied based on the provided permutation
pub fn clone_node_and_apply_effects( current_node: &mut Node, relevant_variables: &RelVars, permutation: &Vec::<usize>, action: &Action, new_relevant_variables: &mut RelVars) -> Node {
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

pub fn apply_effects_cdcl( problem: &mut Problem, applied_functions: &mut ((String, Vec<usize>), Vec<(SubtaskTypes, usize, Vec<usize>, RelVars)>), relevant_variables: &RelVars, action: &Action) {

	if action.effect.is_some() {
		for effect in &action.effect.clone().unwrap() {
			apply_effect(effect, problem, relevant_variables);
		}
	}

	applied_functions.1.push((SubtaskTypes::Action(action.clone()), applied_functions.1.len(), Vec::<usize>::new(), relevant_variables.clone()));
}