use crate::toolbox::{self, make_partial_node, RelVars, Precondition, Called};
use crate::datastructures::{domain::*, node::*};
use crate::toolbox::precondition::{*};

/// Perform a method (Check preconditions and constraints and attempt to perform every subtask)
pub fn perform_method( node_queue: &mut Vec::<PartialNode>, _domain: &Domain, mut current_node: PartialNode, method: Method, mut relevant_variables: RelVars, mut called: Called, subtask_queue_index: usize, mut passing_preconditions: Vec<Precondition> ) -> bool  {

	// What is the index of next function in the subtask queue of this method?
	let current_subtask_index = called.2.pop().unwrap();

	if current_subtask_index == 0 {
		// Check preconditions
		match &method.precondition {
			Some(precondition) => {
				let (new_relevant_variables, preconditions_cleared) = precon_trimmer(relevant_variables, precondition, &current_node.problem, &current_node.problem.state);

				if preconditions_cleared {
					relevant_variables = new_relevant_variables;
				} else {
					return false
				}

			},
			None => {}
		}
	}

	if method.subtasks.len() > 0 {

		// We have finished with this methods subtask 
		if current_subtask_index == method.subtasks.len() {

			let mut trimmed_task_rel_vars = RelVars::new();

			for task_val in &method.task.1 {
				for param_val in &relevant_variables {
					if task_val == &param_val.0 {
						trimmed_task_rel_vars.push(param_val.clone());
						break;
					}
				}
			}

			current_node.applied_functions.1[method.id].3 = trimmed_task_rel_vars;

			let subtask_list = current_node.applied_functions.1[method.id].2.clone();

			for subtask in subtask_list {
				//For every subtask 
				let applied_method = &mut current_node.applied_functions.1[subtask];
				let mut param_counter = 0;
				let mut new_values = Vec::<String>::new();
				let mut found_one = false;

				for parameters in &applied_method.3.clone() {
					if parameters.2.len() > 1 {
						//Parameter name
						let mut sub_task_task_name = String::new();

						// Get the called index for parameter
						match &applied_method.0 {
							SubtaskTypes::Method(subtask_method) => {
								sub_task_task_name = subtask_method.task.0.clone();
							},
							_ => {}
						}

						// Get name from overmethod
						for over_function in &method.subtasks {
							match over_function {
									(SubtaskTypes::Task(over_task), _actual_args_task) => {
										if over_task.name == sub_task_task_name {
											
											let over_task_arg_name = over_task.parameters[param_counter].name.clone();

											for rel_var in &relevant_variables {
												if rel_var.0 == over_task_arg_name {
													new_values = rel_var.2.clone();
													found_one = true;
												}
											}
										}
									},
									_ => {}
							}
							
						}
					}

					if found_one {
						applied_method.3[param_counter].2 = new_values.clone();
						found_one = false;
					}

					param_counter = param_counter + 1;
				}
			}

			// Is this not the first method?
			if called.0.pop().unwrap() {
				let new_node = toolbox::update::update_vars_for_called_method_partial(current_node, &method, &relevant_variables, called, passing_preconditions, subtask_queue_index);
				node_queue.push(new_node);
			} else {
				current_node.subtask_queue.remove(subtask_queue_index);
				node_queue.push(current_node.clone());
			}

			true

		} else {

			if method.precondition.is_some() {
				let method_precons = method.clone().precondition.unwrap();
				passing_preconditions.extend(method_precons);
				
				let (new_relevant_variables, preconditions_cleared) = toolbox::precondition::precon_trimmer(relevant_variables, &passing_preconditions, &current_node.problem, &current_node.original_state);
				
				if preconditions_cleared {
					relevant_variables = new_relevant_variables;
				} else {
					return false
				}
			} else {
				let (new_relevant_variables, preconditions_cleared) = toolbox::precondition::precon_trimmer(relevant_variables, &passing_preconditions, &current_node.problem, &current_node.original_state);
				
				if preconditions_cleared {
					relevant_variables = new_relevant_variables;
				} else {
					return false
				}
			}



			let new_passing_preconditions = toolbox::passing_preconditions::decide_passing_preconditions( &mut passing_preconditions, &method, current_subtask_index, &relevant_variables, &current_node.problem);
			let mut new_subtask_queue = current_node.subtask_queue.clone();

			match method.subtasks[current_subtask_index].clone() {
				(SubtaskTypes::Task(task), actual_task_args) => {

					let mut updated_variables = RelVars::new();

					for x in 0..task.parameters.len() {
						if task.parameters[x].name.contains("?") {
							for var in &relevant_variables {
								if var.0 == task.parameters[x].name {
									updated_variables.push((actual_task_args[x].name.clone(), var.1.clone(), var.2.clone()));
								}
							}
						} else {
							for obj in &current_node.problem.objects {
								if obj.0 == task.parameters[x].name {
									updated_variables.push(("no name".to_string(), obj.1.clone(), vec![obj.0.clone()]));
								} 
							}
						}
					}

					let length = current_node.applied_functions.1.len();
					current_node.applied_functions.1[method.id].2.push(length);

					called.0.push(true);
					called.1.push((method, relevant_variables, passing_preconditions, current_node.original_state));
					called.2.push(current_subtask_index + 1);

					new_subtask_queue[subtask_queue_index] = (SubtaskTypes::Task(task), updated_variables, called, new_passing_preconditions);
				},
				(SubtaskTypes::Action(mut action), actual_action_args) => {

					let mut updated_variables = RelVars::new();
						
					'outer: for n in 0..action.parameters.len() {

						if action.parameters[n].name.contains("?"){
							for var in &relevant_variables {
								if var.0 == action.parameters[n].name {
									updated_variables.push((actual_action_args[n].name.clone(), var.1.clone(), var.2.clone()));
									action.parameters[n].name = actual_action_args[n].name.clone();
									continue 'outer;
								}
							}
						} else {

							// Found constant in action subtask
							for obj in &current_node.problem.objects{
								if obj.0 == action.parameters[n].name {
									updated_variables.push((actual_action_args[n].name.clone(), obj.1.clone(), vec![obj.0.clone()]));
								}
							}
						}
					}

					let length = current_node.applied_functions.1.len();
					current_node.applied_functions.1[method.id].2.push(length);

					called.0.push(true);
					called.1.push((method, relevant_variables, passing_preconditions, current_node.original_state));
					called.2.push(current_subtask_index + 1);

					new_subtask_queue[subtask_queue_index] = (SubtaskTypes::Action(action), updated_variables, called, new_passing_preconditions);

				},
				_ => {}
			}

			let new_node = make_partial_node(current_node.problem.clone(), new_subtask_queue, current_node.applied_functions, current_node.hash_table, current_node.hash_counter, current_node.goal_functions, &current_node.problem.state);

			node_queue.push(new_node);

			true
		}

	} else {
		if !called.0.pop().unwrap() {

			current_node.subtask_queue.remove(subtask_queue_index);
			node_queue.push(current_node);

		} else {

			let new_node = toolbox::update::update_vars_for_called_method_partial(current_node, &method, &relevant_variables, called, passing_preconditions, subtask_queue_index);
			node_queue.push(new_node);
		}

		true
	}
}