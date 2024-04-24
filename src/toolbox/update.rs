
use crate::datastructures::{domain::*, node::*, problem::{*}};
use crate::toolbox::{make_node, check_duplicates, make_partial_node, RelVars, Precondition, Called};

/// Adds the supertype of every object in the problem to a list on the object datastructure and returns a new Problem
pub fn update_objects( mut problem: Problem, domain: &Domain ) -> Problem {

	let mut new_object_list = Vec::<(String, String, Vec<String>)>::new();

	if domain.constants.is_some() {
		for constant in domain.constants.as_ref().unwrap() {
			problem.objects.push((constant.0.clone(), constant.1.clone(), vec![]));
		}
	}

	for object in &problem.objects {
		let mut final_subtype_list = Vec::<String>::new();
		let mut current_subtype_list = Vec::<String>::new();
		current_subtype_list.push(object.1.clone());
		final_subtype_list.push(object.1.clone());

		while !current_subtype_list.is_empty() {
			let current_sub_type = current_subtype_list.pop().unwrap();
			for sub_type in &domain.types {
				
				if sub_type.0 == current_sub_type && sub_type.1 != sub_type.0 && sub_type.1 != "" {
					current_subtype_list.push(sub_type.1.clone());
					final_subtype_list.push(sub_type.1.clone());
				}
			}
		}

		new_object_list.push((object.0.clone(), object.1.clone(), final_subtype_list));
	} 

	problem.objects = new_object_list;

	problem
} 

/// Updates relevant variables for the method that called the current task in order to trim the caller methods variables
pub fn update_vars_for_called_method( mut current_node: Node, method: &Method, relevant_variables: &RelVars) -> Node {

	let (calling_method, calling_relevant_vars, called_passing_precon) = current_node.called.1.pop().unwrap();

	let calling_method_subtask = calling_method.subtasks[current_node.called.2.last().unwrap() - 1].clone();

	let mut i = 0;

	let mut new_rel_vars: RelVars = RelVars::new();

	for task_arg in &method.task.1 {
		for var in relevant_variables {
			//println!("Checking loop: i - {}, method_task_arg - {:?}, var - {}", i, method.task.1, var.0);
			if &var.0 == task_arg {

				match &calling_method_subtask {
					(SubtaskTypes::Action(action), _actual_action_arg) => {
						new_rel_vars.push((action.parameters[i].name.clone(), var.1.clone(), var.2.clone()));
					},
					(SubtaskTypes::Task(task), _actual_task_args) => {
						new_rel_vars.push((task.parameters[i].name.clone(), var.1.clone(), var.2.clone()));
					},
					_ => {}
				}

				i = i + 1;
			} 
		}
	}

	let mut new_new_relevant_variables: RelVars = RelVars::new();

	for rel_var in &calling_relevant_vars {

		let mut found_var = false;

		for x in 0..new_rel_vars.len() {
			if new_rel_vars[x].0 == rel_var.0 {
				new_new_relevant_variables.push(new_rel_vars[x].clone());
				found_var = true;
				break;
			}
		}

		if !found_var {
			new_new_relevant_variables.push(rel_var.clone());
		}
	}

	// Push to subtask_q
	let mut new_sq = current_node.subtask_queue.clone();
	new_sq.push((SubtaskTypes::Method(calling_method.clone()), new_new_relevant_variables));

	let new_node = make_node(current_node.problem, new_sq, current_node.called, current_node.applied_functions, current_node.hash_table, called_passing_precon, current_node.goal_functions);

	new_node
}

pub fn update_vars_for_called_method_partial( current_node: PartialNode, method: &Method, relevant_variables: &RelVars, mut called: Called, _passing_preconditions: Vec<Precondition>, tried_count: usize) -> PartialNode {

	let (calling_method, calling_relevant_vars, called_passing_precon) = called.1.pop().unwrap();

	let calling_method_subtask = calling_method.subtasks[called.2.last().unwrap() - 1].clone();

	let mut i = 0;

	let mut new_rel_vars: RelVars = RelVars::new();

	for task_arg in &method.task.1 {
		for var in relevant_variables {
			//println!("Checking loop: i - {}, method_task_arg - {:?}, var - {}", i, method.task.1, var.0);
			if &var.0 == task_arg {

				match &calling_method_subtask {
					(SubtaskTypes::Action(action), _actual_action_arg) => {
						new_rel_vars.push((action.parameters[i].name.clone(), var.1.clone(), var.2.clone()));
					},
					(SubtaskTypes::Task(task), _actual_task_args) => {
						new_rel_vars.push((task.parameters[i].name.clone(), var.1.clone(), var.2.clone()));
					},
					_ => {}
				}

				i = i + 1;
			} 
		}
	}

	let mut new_new_relevant_variables: RelVars = RelVars::new();

	for rel_var in &calling_relevant_vars {

		let mut found_var = false;

		for x in 0..new_rel_vars.len() {
			if new_rel_vars[x].0 == rel_var.0 {
				new_new_relevant_variables.push(new_rel_vars[x].clone());
				found_var = true;
				break;
			}
		}

		if !found_var {
			new_new_relevant_variables.push(rel_var.clone());
		}
	}

	// Push to subtask_q
	let mut new_sq = current_node.subtask_queue.clone();
	new_sq[tried_count] = (SubtaskTypes::Method(calling_method.clone()), new_new_relevant_variables, called, called_passing_precon);

	let new_node = make_partial_node(current_node.problem, new_sq, current_node.applied_functions, current_node.hash_table, current_node.hash_counter, current_node.goal_functions);

	new_node
}


/// Updates relevant variables by combining the given parameters from task and finding the new parameters in objects in order to have all parameters in the signature
pub fn update_relevant_variables( problem: &Problem, method: &Method, old_relevant_variables: &RelVars) -> RelVars {

	let mut updated_relevant_parameters = RelVars::new();

	for param in &method.parameters {

		let mut found_in_task = false;
		let mut looking_count = 0;
			
		for task_param in &method.task.1 {
			
			// The parameter was provided by the task
			if &param.name == task_param {

				if old_relevant_variables[looking_count].1 != param.object_type{

					let mut new_var_value_list = Vec::<String>::new();

					for var in &old_relevant_variables[looking_count].2{
						for object in &problem.objects {
							if var == &object.0 && object.2.contains(&param.object_type) {
								new_var_value_list.push(var.clone());
							}
						}
					}

					updated_relevant_parameters.push((param.name.clone(), param.object_type.clone(), new_var_value_list));

				} else{
					updated_relevant_parameters.push((param.name.clone(), param.object_type.clone(), old_relevant_variables[looking_count].2.clone()));
				}						
				found_in_task = true;
			}
		
			looking_count = looking_count + 1;
		}
		
		// The parameter was not provided by the task and therefore the possible values of the parameter is every object matching the type
		if !found_in_task {
					
			let mut var_list = Vec::<String>::new();
	
			for object in &problem.objects {
				if object.2.contains(&param.object_type) {
					var_list.push(object.0.clone());
				}
			}
		
			updated_relevant_parameters.push((param.name.clone(), param.object_type.clone(), var_list.clone()));
		}		
	}

	updated_relevant_parameters = check_duplicates(&mut updated_relevant_parameters);

	updated_relevant_parameters
}