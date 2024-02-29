use crate::toolbox::{self, make_node};
use crate::datastructures::{domain::*, node::*};
use crate::toolbox::{constraints::*, precondition::{*}};

type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);
type RelVars = Vec<(String, String, Vec<String>)>;

/// Perform a method (Check preconditions and constraints and attempt to perform every subtask)
pub fn perform_method( node_queue: &mut Vec::<Node>, domain: &Domain, mut current_node: Node, method: Method, mut relevant_variables: RelVars ) {

	// What is the index of this method in the subtask queue of the method that called it?
	let current_subtask_index = current_node.called.2.pop().unwrap();

	if current_subtask_index == 0 {

		// Check preconditions
		match &method.precondition {
			Some(precondition) => {

				let (new_relevant_variables, preconditions_cleared) = precon_trimmer(relevant_variables, precondition, &current_node.problem);

				if preconditions_cleared {
					relevant_variables = new_relevant_variables;
				} else {
					//toolbox::back_tracking::backtrack_for_method_param_value(node_queue, &new_relevant_variables);
					return
				}

			},
			None => {}
		}
		
		// Check constraints
		if method.constraints.is_some() {
			let mut relevant_variables_list = check_constraints( &relevant_variables, &method.constraints.clone().unwrap() );

			//Return if no relevant_variables was returned from check_constraints
			if relevant_variables_list.is_empty() {
				return
			}

			relevant_variables = relevant_variables_list.pop().unwrap();

			// Push Node versions
			for relevant_variable in &relevant_variables_list {
						
				let new_method = Method {
					name: method.name.clone(),
					parameters: method.parameters.clone(),
					task: method.task.clone(),
					precondition: None,
					subtasks: method.subtasks.clone(),
					constraints: None,
					id: method.id
				};

				let mut new_sq = current_node.subtask_queue.clone();
				new_sq.push((SubtaskTypes::Method(new_method), relevant_variable.clone()));
				
				let mut new_called: (Vec<bool>, Vec<(Method, Vec<(String, String, Vec<String>)>, Vec<Precondition>)>, Vec<usize>) = current_node.called.clone();
				new_called.2.push(0);

				let new_node = make_node(current_node.problem.clone(), new_sq, new_called, current_node.applied_functions.clone(), current_node.hash_table.clone(), current_node.passing_preconditions.clone(), current_node.goal_functions.clone());

				node_queue.push(new_node);
			}
		}
	}

	if method.subtasks.len() > 0 {
		//println!("Passing precons: {:?}", new_passing_preconditions);

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

			current_node.applied_functions.1[method.id].3 = trimmed_task_rel_vars.clone();

			// CHECK THAT EVERY VARIABLE HAS BEEN REDUCED TO ONE!
			let subtask_list = current_node.applied_functions.1[method.id].2.clone();

			for subtask in subtask_list {
				//For every subtask 
				let applied_method = &mut current_node.applied_functions.1[subtask];
				let mut param_counter = 0;
				let mut new_values = Vec::<String>::new();
				let mut found_one = false;

				for parameters in &applied_method.3.clone() {
					if parameters.2.len() > 1 {
						// println!("\n BIGGER THAN ONE! \n");
						// println!("OVERTASK PARAM: {:?}\n OVERMETHOD: {:?}\n", relevant_variables, method);
						// println!("SUBTASK PARAM: {:?}\n SUBMETHOD: {:?}", parameters, applied_method);

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
						for over_method_task in &method.subtasks {
							if over_method_task.0 == sub_task_task_name {
								let over_method_arg_name = over_method_task.2[param_counter].clone();
								for rel_var in &relevant_variables {
									if rel_var.0 == over_method_arg_name {
										new_values = rel_var.2.clone();
										found_one = true;
									}
								}
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
			if current_node.called.0.pop().unwrap() {

				let new_node = toolbox::update::update_vars_for_called_method(current_node, &method, &relevant_variables);

				node_queue.push(new_node);
			} else {
				node_queue.push(current_node.clone());
			}

		} else {
			let new_passing_preconditions = toolbox::passing_preconditions::decide_passing_preconditions( &mut current_node.passing_preconditions, &method, current_subtask_index);
			let mut new_subtask_queue = current_node.subtask_queue.clone();
			let mut found_task = false;

			// Look for the subtask among tasks in the domain
			for task in &domain.tasks { 	
				if task.name == method.subtasks[current_subtask_index].0 {

					let mut updated_variables = RelVars::new();

					for task_arg in method.subtasks[current_subtask_index].2.clone() {
						if task_arg.contains("?") {
							for var in &relevant_variables {
								if var.0 == task_arg {
									updated_variables.push(var.clone());
								}
							}
						} else {
							for obj in &current_node.problem.objects {
								if obj.0 == task_arg {
									updated_variables.push(("no name".to_string(), obj.1.clone(), vec![task_arg.clone()]));
								} 
							}
						}
					}

					let length = current_node.applied_functions.1.len();
					//println!("printing length {}", length);
					current_node.applied_functions.1[method.id].2.push(length);

					//println!("Updated stq with task: {}, Relvars: {:?}\n", method.name, updated_variables);
					new_subtask_queue.push((SubtaskTypes::Task(task.clone()), updated_variables));
					found_task = true;
					break;
				}
			}

			// Look for the subtask among actions in the domain
			if !found_task {
				// Look for the subtask among actions in the domain
				for action in domain.actions.iter().clone() {
					if action.name == method.subtasks[current_subtask_index].0 {

						let mut updated_variables = RelVars::new();
						
						for n in 0..method.subtasks[current_subtask_index].2.len() {

							if method.subtasks[current_subtask_index].2[n].contains("?"){
								for var in &relevant_variables {
									if var.0 == method.subtasks[current_subtask_index].2[n] {
										
										updated_variables.push((action.parameters[n].name.clone(), var.1.clone(), var.2.clone()));
									}
								}
							} else {

								// Found constant in action subtask
								for obj in &current_node.problem.objects{
									if obj.0 == method.subtasks[current_subtask_index].2[n] {
										updated_variables.push((action.parameters[n].name.clone(), obj.1.clone(), vec![obj.0.clone()]));
									}
								}
							}
						}

						let length = current_node.applied_functions.1.len();
						//println!("printing length {}", length);
						current_node.applied_functions.1[method.id].2.push(length);

						new_subtask_queue.push((SubtaskTypes::Action(action.clone()), updated_variables));
						break;
					}
				}
			}

			let mut new_called = current_node.called.clone();

			new_called.0.push(true);
			new_called.1.push((method, relevant_variables, current_node.passing_preconditions.clone()));
			new_called.2.push(current_subtask_index + 1);

			let new_node = make_node(current_node.problem.clone(), new_subtask_queue.clone(), new_called.clone(), current_node.applied_functions.clone(), current_node.hash_table.clone(), new_passing_preconditions.clone(), current_node.goal_functions.clone());

			node_queue.push(new_node);
		}

	} else {

		if !current_node.called.0.pop().unwrap() {
			node_queue.push(current_node.clone());
		} else {				

			let new_node = toolbox::update::update_vars_for_called_method(current_node, &method, &relevant_variables);
			node_queue.push(new_node);
		}

	}

}