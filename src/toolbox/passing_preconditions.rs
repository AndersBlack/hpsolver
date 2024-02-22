use crate::datastructures::{node::*, domain::{*}};

type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);

/// Update passing preconditions
pub fn update_passing_precondition(current_node: &Node, parameters: &Vec<Argument>) -> Vec<Precondition> {
	let mut new_passing_precon = Vec::<Precondition>::new();

	if !current_node.passing_preconditions.is_empty() {
		let caller_method = &current_node.called.1.last().unwrap().0;

		let subtask = &caller_method.subtasks[current_node.called.2.last().unwrap().clone() - 1];
		
		for precon in &current_node.passing_preconditions.clone() {
			let mut new_precon = (precon.0, precon.1.clone(), Vec::<String>::new(), precon.3.clone());

			for precon_parameter in &precon.2 {
				let mut j = 0;
				for subtask_parameter in &subtask.2 {
					if subtask_parameter == precon_parameter {
						
						new_precon.2.push(parameters[j].name.clone());
					}

					j += 1;
				}
			}

			new_passing_precon.push(new_precon);
		}	
	}

	new_passing_precon
}

/// Go through all preconditions and checks whether they should be passed to the next method/action
pub fn decide_passing_preconditions( passing_preconditions: &mut Vec<Precondition>, method: &Method, index: usize) -> Vec<Precondition> {

	let mut new_passing_preconditions = Vec::<Precondition>::new();

	//println!("Old passing precon: {:?}", passing_preconditions);

	// Add relevant preconditions for the coming subtask
	if method.precondition.is_some() {
		let precons = method.precondition.clone().unwrap();
		for precon in precons {
			if precon.0 < 2 {
				if precondition_should_be_passed(&precon, &method.subtasks[index]) {
					new_passing_preconditions.push(precon);
				}
			}
		}
	}

	// Which passing preconditions should continue?
	for passing_precon in passing_preconditions {
		if precondition_should_be_passed(&passing_precon, &method.subtasks[index]) {
			new_passing_preconditions.push(passing_precon.clone());
		}
	}

	//println!("New passing precon: {:?}", new_passing_preconditions);
	new_passing_preconditions
}

// Check if a precon should be forwarded to a given subtask based on relevant variables
fn precondition_should_be_passed(precondition: &Precondition, subtask: &(String, String, Vec<String>, bool)) -> bool {

	let mut precondition_passed = false;

	if precondition.2.len() == 1 {
		return false;
	}

	// Is precondition relevant for this subtask?
	let mut param_count = 0;
	for precon_param in &precondition.2 {
		for subtask_param in &subtask.2 {
			if precon_param == subtask_param { 
				param_count = param_count + 1;
			}
		}
	}

	if param_count == precondition.2.len() {
		//println!("precon was relevant! {precondition:?}");
		precondition_passed = true; 
	}

	precondition_passed
} 