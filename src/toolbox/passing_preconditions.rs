use crate::datastructures::{domain::*, node::*};

use super::{precondition::check_precondition, Problem, RelVars, Called, Precondition};

/// Update passing preconditions
pub fn update_passing_precondition(called: &Called, passing_preconditions: &Vec<Precondition>, parameters: &Vec<Argument>) -> Vec<Precondition> {
	let mut new_passing_precon = Vec::<Precondition>::new();

	if !passing_preconditions.is_empty() {
		let caller_method = &called.1.last().unwrap().0;

		let subtask = &caller_method.subtasks[called.2.last().unwrap().clone() - 1];
		
		for precon in &passing_preconditions.clone() {
			let mut new_precon = (precon.0, precon.1.clone(), Vec::<String>::new(), precon.3.clone());

			for precon_parameter in &precon.2 {
				let mut j = 0;
				match subtask {
					(SubtaskTypes::Action(action), _) => {
						for subtask_parameter in &action.parameters {
							if &subtask_parameter.name == precon_parameter {
								
								new_precon.2.push(parameters[j].name.clone());
							}
		
							j += 1;
						}
					},
					(SubtaskTypes::Task(task), _) => {
						for subtask_parameter in &task.parameters {
							if &subtask_parameter.name == precon_parameter {
								
								new_precon.2.push(parameters[j].name.clone());
							}
		
							j += 1;
						}
					},
					_ => {}
				}
			}

			new_passing_precon.push(new_precon);
		}	
	}

	new_passing_precon
}

/// Go through all preconditions and checks whether they should be passed to the next method/action
pub fn decide_passing_preconditions( passing_preconditions: &mut Vec<Precondition>, method: &Method, index: usize, relevant_variables: &RelVars, problem: &Problem) -> Vec<Precondition> {

	let mut new_passing_preconditions = Vec::<Precondition>::new();

	// Add relevant preconditions for the coming subtask
	if method.precondition.is_some() {
		let precons = method.precondition.clone().unwrap();
		for precon in precons {
			if precon.0 < 2 {

				if precondition_should_be_passed(&precon, &method.subtasks[index], &relevant_variables) {
					if index > 0 && check_precondition(&precon, relevant_variables, problem) {
						new_passing_preconditions.push(precon);
					} else if index == 0 {
						new_passing_preconditions.push(precon);
					}
				}

			}
		}
	}

	// Which passing preconditions should continue?
	for passing_precon in passing_preconditions {
		if precondition_should_be_passed(&passing_precon, &method.subtasks[index], &relevant_variables) {
			if index > 0 && check_precondition(&passing_precon, relevant_variables, problem) {
				new_passing_preconditions.push(passing_precon.clone());
			} else if index == 0 {
				new_passing_preconditions.push(passing_precon.clone());
			}
		}
	}

	//println!("New passing precon: {:?}", new_passing_preconditions);
	new_passing_preconditions
}

// Check if a precon should be forwarded to a given subtask based on relevant variables
fn precondition_should_be_passed(precondition: &Precondition, subtask: &(SubtaskTypes, Vec<Argument>), relevant_variables: &RelVars) -> bool {

	let mut precondition_passed = false;

	if precondition.2.len() == 1 {
		return false;
	}

	// Is precondition relevant for this subtask?
	let mut param_count = 0;
	for precon_param in &precondition.2 {

		match subtask {
			(SubtaskTypes::Action(action), _) => {
				for subtask_param in &action.parameters {
					if precon_param == &subtask_param.name && !value_length_is_one(precon_param, relevant_variables) {
						param_count = param_count + 1;
					}
				}
			},
			(SubtaskTypes::Task(task), _) => {
				for subtask_param in &task.parameters {
					if precon_param == &subtask_param.name && !value_length_is_one(precon_param, relevant_variables) { 
						param_count = param_count + 1;
					}
				}
			},
			_ => {}
		}
	}

	if param_count == precondition.2.len() {
		precondition_passed = true; 
	}

	precondition_passed
} 

fn value_length_is_one(relvar_name: &String, relvars: &RelVars) -> bool {

	for relvar in relvars {

		if &relvar.0 == relvar_name && relvar.2.len() == 1 {
			return true
		} else if &relvar.0 == relvar_name {
			return false
		}

	}

	return false

} 