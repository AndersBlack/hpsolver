use crate::node::Node;
use crate::node::SubtaskTypes;
use crate::problem;
use crate::problem::*;
use crate::domain::*;

// Relevant Variables datatype
type RelVars = Vec<(String, String, Vec<String>)>;

pub fn depth_first(problem: &mut Problem, domain: &Domain) {

	let mut node_queue = Vec::<Node>::new();

	let mut htn_subtask_queue = Vec::<(SubtaskTypes, RelVars)>::new();

	for subtask in problem.htn.subtasks.clone().into_iter().rev() {

		let mut new_relevant_parameters = RelVars::new();

		for item in subtask.2.clone() {
			for object in &problem.objects {
				if object.object.0 == item {
					new_relevant_parameters.push(("no name".to_string(), object.object.1.clone(), vec![item.clone()].clone()))
				}
			}
		}

		htn_subtask_queue.push((SubtaskTypes::HtnTask(subtask), new_relevant_parameters));	
	}

	let called = (Vec::<bool>::new(), Vec::<(Method, RelVars)>::new(), Vec::<usize>::new());

	// Make node
	let new_node = make_node(problem.clone(), htn_subtask_queue, called, Vec::<(String, Vec<String>)>::new());
	
	node_queue.push(new_node);

	next_node(&mut node_queue, domain);
}

fn next_node(node_queue: &mut Vec::<Node>, domain: &Domain) {

	//println!("Node_queue length: {}", node_queue.len());
	let current_node = node_queue.pop();

	// Handle subtasks
	match current_node {
		Some(mut current_node) => {
			let current_subtask = current_node.subtask_queue.pop(); 
			//println!("State: {:?}", current_node.problem.state);

			match current_subtask {
				Some((SubtaskTypes::HtnTask(htn_task), relevant_variables))=> {
					println!("Htn_task: {:?}", htn_task.0);

					perform_htn_task(node_queue, domain, current_node, htn_task, relevant_variables);
					
				},
				Some((SubtaskTypes::Task(task), relevant_variables)) => {
					//println!("Task: {:?}", task.name);

					perform_task(node_queue, domain, current_node, task, relevant_variables);

				},
				Some((SubtaskTypes::Method(method), relevant_variables)) => {
					//println!("Method {:?} Rel_vars: {:?}", method.name, relevant_variables);

					perform_method(node_queue, domain, current_node, method, relevant_variables);

				},
				Some((SubtaskTypes::Action(action), relevant_variables)) => {
					//println!("\n Action: {:?} Relevant_variables: {:?}", action.name, relevant_variables);

					perform_action(node_queue, domain, current_node, action, relevant_variables);

				},
				None => { 

					println!("\nFINISHED PROBLEM!\n");
					for applied_action in current_node.applied_action_list{
						println!("Action name: {:?}, parameters: {:?}", applied_action.0, applied_action.1)
					}

				}
			}
		},
		None => { println!("Node queue found empty!") }
	}

}

fn update_relevant_variables(node: &Node, method: &Method, old_relevant_variables: RelVars) -> RelVars {

	let mut updated_relevant_parameters = RelVars::new();

	for param in &method.parameters {

		let mut found_in_task = false;
		let mut looking_count = 0;
			
		for task_param in &method.task.1 {
							
			if &param.name == task_param {
				//println!("Found in task param: {:?} {:?} {:?}", param.name.clone(), task_param.clone(), node.relevant_parameters.clone() );
				updated_relevant_parameters.push((param.name.clone(), old_relevant_variables[looking_count].1.clone(), old_relevant_variables[looking_count].2.clone()));
							
				found_in_task = true;
			}
		
			looking_count = looking_count + 1;
		}
	
		if !found_in_task {
					
			let mut var_list = Vec::<String>::new();
	
			for object in &node.problem.objects {
				if object.object.1 == param.object_type {
					var_list.push(object.object.0.clone());
				}
			}
		
			updated_relevant_parameters.push((param.name.clone(), param.object_type.clone(), var_list.clone()));
		}
	}

	//println!("{:?}", updated_relevant_parameters);

	updated_relevant_parameters.clone()

}

// Takes the boolean prefix, the name, the list of lists of possible values and a ref to the state
fn check_precondition(precondition: &(bool,String,Vec<String>), param_list: RelVars, state: &State) -> bool {

	let mut precondition_value_list = Vec::<(String, Vec<String>)>::new();
	let mut param_counter = 0;

	//Find needed values
	for value in &precondition.2 {
		for param in &param_list {
			if value == &param.0 {
				precondition_value_list.push((param.0.clone(), param.2.clone()));
				param_counter = param_counter + 1;
			}

			if param_counter == precondition.2.len() {
				break;
			}
		}
	};

	let mut found_one = false;
	//println!("Precondition: {:?} with precondition_value_list: {:?}\n", precondition, precondition_value_list);

	// Find state parameter
	for value in &state.state_variables {
		let mut found_counter = 0;

		if value.0 == precondition.1 {
			// For every variable in state parameter
			//println!("bob {:?} - {:?}",value, precondition);

			for n in 0..=(value.1.len()-1) {
				//println!("hmm {:?} {:?}", value.1, &precondition_value_list);
				for param in &precondition_value_list[n].1 {
					if &value.1[n] == param {
						found_counter = found_counter + 1;
					} 
				}
			}

		}

		if found_counter == value.1.len() && precondition.0 == true {
			//println!("Found match {:?} & {:?} in {}", value.1, precondition_value_list, precondition.1);
			found_one = true;
			break;
		}

	}

	if (found_one == false && precondition.0 == true) || (found_one == true && precondition.0 == false) {
		//panic!("Encountered unsatisfiable precondition: {:?} with {:?}", precondition, param_list);
		return false;
	}

	true
}

fn permutation_tool( value_list: RelVars , action: &Action, state: &State) ->   Vec::<Vec::<usize>> {

	let mut size_ref_list = Vec::<usize>::new();
	let mut permutation_holder = Vec::<usize>::new();
	let mut permutation_list_list = Vec::<Vec::<usize>>::new();

	for var_info in &value_list {
		size_ref_list.push(var_info.2.len());
		permutation_holder.push(0);
	}

	let mut n = 0;

	while n < size_ref_list.len() {

		n = 0;
		
		// Check precondition
		if precon_cleared(&permutation_holder, value_list.clone(), action, state) {
			permutation_list_list.push(permutation_holder.clone());		
		} 

		if permutation_holder[n] != (size_ref_list[n] - 1) {
			permutation_holder[n] = permutation_holder[n] + 1;
		} else {

			let mut found_expansion = false;

			while !found_expansion && n < size_ref_list.len() {

				permutation_holder[n] = 0;

				n = n + 1;

				if n < size_ref_list.len() && permutation_holder[n] != (size_ref_list[n] - 1) {
					permutation_holder[n] = permutation_holder[n] + 1;
					found_expansion = true;
				}
			}
		}
	}

	permutation_list_list

}

fn precon_cleared (permutation: &Vec::<usize>, value_list: RelVars, action: &Action, state: &State) -> bool {

	let mut clear = true;

	let mut new_value_list = RelVars::new();

	let mut perm_index = 0;

	for val in value_list {
		new_value_list.push((val.0.clone(), val.1.clone(), vec![val.2[permutation[perm_index]].clone()]));
		perm_index = perm_index + 1;
	}

	for precon in action.precondition.iter().flatten() {
		//println!("{:?}", new_value_list);
		if !check_precondition(precon, new_value_list.clone(), state) {
			clear = false;
		}
	}

	clear
}

fn apply_effect( effect: &(bool,String,Vec<String>), problem: &mut Problem, param_list: RelVars ) {

		if effect.0 == false {
	
			// Remove found from state
			let index = problem.state.state_variables.iter().position(|x| x.0 == effect.1).unwrap();
			problem.state.state_variables.remove(index);
		
		} else {

			// Add effect to state
			let mut args_list = Vec::<String>::new();
	
			for effect_var in &effect.2 {
				for params in &param_list {
					if effect_var == &params.0 {
						args_list.push(params.2[0].clone());
					}
				}
			}
	
			let new_state_param = (effect.1.clone(), args_list);
			problem.state.state_variables.push(new_state_param);
		}
	} 

fn make_node( new_problem: Problem, sq: Vec::<(SubtaskTypes, RelVars)>, called: (Vec<bool>, Vec<(Method, RelVars)>, Vec<usize>), aal: Vec<(String, Vec<String>)>) -> Node {

		let new_node = Node {
			problem: new_problem.clone(),
			subtask_queue: sq.clone(),
			called: called.clone(),
			applied_action_list: aal.clone()
		};

		new_node
}

fn perform_htn_task ( node_queue: &mut Vec::<Node>, domain: &Domain, mut current_node: Node, htn_task: (String, String, Vec<String>), relevant_variables: RelVars) {
	for method in domain.methods.clone() {
		if method.task.0 == htn_task.0 {
			let mut subtask_queue_clone = current_node.subtask_queue.clone();

			let updated_relevant_variables = update_relevant_variables(&current_node, &method, relevant_variables.clone());

			// Update relevant variables
			subtask_queue_clone.push((SubtaskTypes::Method(method),updated_relevant_variables));

			current_node.called.0.push(false);
			current_node.called.2.push(0);
			
			let new_node = Node {
				problem: current_node.problem.clone(),
				subtask_queue: subtask_queue_clone,
				called: (current_node.called.0.clone(), current_node.called.1.clone(), current_node.called.2.clone()),
				applied_action_list: current_node.applied_action_list.clone()
			};
			
			//println!("Pushing node");
			node_queue.push(new_node);
		}
	}

	next_node(node_queue, domain)
}

fn perform_task ( node_queue: &mut Vec::<Node>, domain: &Domain, current_node: Node, task: Task, relevant_variables: RelVars ) {
	//println!("Task: {}", task.name);

	// Expand task and create a new node for every method that task expands to
	for method in &domain.methods.clone() {
		if method.task.0 == task.name {

			let mut new_subtask_queue = current_node.clone().subtask_queue;
			new_subtask_queue.push((SubtaskTypes::Method(method.clone()),  update_relevant_variables(&current_node, &method, relevant_variables.clone())));

			let mut new_called = current_node.called.clone();
			new_called.2.push(0);

			let new_node = make_node(current_node.problem.clone(), new_subtask_queue, new_called, current_node.applied_action_list.clone());

			//println!("Pushing node");
			node_queue.push(new_node)
		}
	}

	next_node(node_queue, domain)

}

fn perform_method ( node_queue: &mut Vec::<Node>, domain: &Domain, mut current_node: Node, method: Method, mut relevant_variables: RelVars ) {
	// What is the index of this method in the subtask queue of the method that called it?
	let current_subtask_index = current_node.called.2.pop().unwrap();

	// Check preconditions (TODO: Trim variables here aswell)
	if current_subtask_index == 0 {
		
		for precon in method.precondition.iter().flatten() {
			let precon_clear = check_precondition(precon, relevant_variables.clone(), &current_node.problem.state);

			if !precon_clear {
				println!("Didnt clear precon");
				next_node(node_queue, domain)
			}
		}
	}

	match &method.subtasks {
		Some(subtask_list) => {

			// We have finished with this methods subtask 
			if current_subtask_index == subtask_list.len() {

				// Is this not the first method?
				if current_node.called.0.pop().unwrap() {

					let (calling_method, calling_relevant_vars) = current_node.called.1.pop().unwrap();

					let calling_method_subtask = calling_method.subtasks.clone().unwrap()[current_node.called.2.last().unwrap() - 1].clone();

					let method_task = method.task.clone();
					let mut i = 0;

					for variable in calling_method_subtask.2{
						if !(variable == method_task.1[i]){
							let mut j = 0;
							for meth_var in &relevant_variables.clone(){
								if meth_var.0 == method_task.1[i] {
									relevant_variables[j].0 = variable.clone();
								}
								j = j + 1;
							}
						}
						i = i + 1;
					}

					let mut new_new_relevant_variables: RelVars = RelVars::new();

					for rel_var in &calling_relevant_vars {

						let mut found_var = false;

						for new_var in &relevant_variables {
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

					// Push to subtask_q
					let mut new_sq = current_node.subtask_queue.clone();
					new_sq.push((SubtaskTypes::Method(calling_method.clone()), new_new_relevant_variables.clone()));

					let new_node = make_node(current_node.problem.clone(), new_sq, current_node.called.clone(), current_node.applied_action_list.clone());

					node_queue.push(new_node);
				} else {
					node_queue.push(current_node.clone());
				}

			} else {

				let mut new_subtask_queue = current_node.subtask_queue.clone();

				let mut found_task = false;

				for task in domain.tasks.clone() { 	
					if task.name == subtask_list[current_subtask_index].0 {
						//println!("task: {:?}", task.name);

						let mut updated_variables = RelVars::new();

						for task_arg in subtask_list[current_subtask_index].2.clone() {
							for var in &relevant_variables {
								if var.0 == task_arg {
									updated_variables.push(var.clone());
								}
							}
						}

						new_subtask_queue.push((SubtaskTypes::Task(task.clone()), updated_variables));
						found_task = true;
						break;
					}
				}

				if !found_task {
					for action in domain.actions.iter().clone() {
						//println!("\n{}", action);
						if action.name == subtask_list[current_subtask_index].0 {

							let mut updated_variables = RelVars::new();
							
							for n in 0..subtask_list[current_subtask_index].2.len() {
								for var in &relevant_variables {
									if var.0 == subtask_list[current_subtask_index].2[n] {
										
										updated_variables.push((action.parameters[n].name.clone(), var.1.clone(), var.2.clone()));
									}
								}
							}
							//println!("\n{}", action);
							new_subtask_queue.push((SubtaskTypes::Action(action.clone()), updated_variables));
							break;
						}
					}
				}

				let mut new_called = current_node.called.clone();
				new_called.0.push(true);
				new_called.1.push((method.clone(), relevant_variables));
				new_called.2.push(current_subtask_index + 1);

				let new_node = make_node(current_node.problem.clone(), new_subtask_queue.clone(), new_called.clone(), current_node.applied_action_list.clone());

				node_queue.push(new_node);
			}
		},
		None => {
			// Subtask list empty
			panic!("Hit empty subtask list, which is not implemented")
		}
	}

	next_node(node_queue, domain);
}

fn perform_action ( node_queue: &mut Vec::<Node>, domain: &Domain, mut current_node: Node, action: Action, relevant_variables: RelVars ) {
	//println!("Action {}", action.name);

	let permutation_list = permutation_tool(relevant_variables.clone(), &action, &current_node.problem.state);

	let (calling_method, calling_relevant_vars) = current_node.called.1.pop().unwrap();
	current_node.called.0.pop();

	//println!("Perm list: {:?}", permutation_list);

	for permutation in permutation_list {

		let mut new_relevant_variables = RelVars::new();

		// Trim relevant_variables based on permutation list
		let mut index = 0;

		for variable_type in &relevant_variables {
			//println!("{:?}", variable_type);
			new_relevant_variables.push((variable_type.0.clone(), variable_type.1.clone(), vec![variable_type.2[permutation[index]].clone()].clone()));
			
			index = index + 1;
		}

		let mut new_current_node = current_node.clone();

		// Apply effects for each of the possible permutations and append to node queue.
		for effect in &action.effect.clone().unwrap() {
			apply_effect(&effect, &mut new_current_node.problem, new_relevant_variables.clone())
		}

		let mut new_applied_action = (action.name.clone(), Vec::<String>::new());

		for rel_var in &new_relevant_variables {
			new_applied_action.1.push(rel_var.2[0].clone());
		}

		new_current_node.applied_action_list.push(new_applied_action);

		for x in 0..new_relevant_variables.len() {
			let var_name = calling_method.subtasks.clone().unwrap()[new_current_node.called.2.last().unwrap() - 1].2[x].clone();
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

		new_current_node.subtask_queue.push((SubtaskTypes::Method(calling_method.clone()), new_new_relevant_variables.clone()));

		let new_node = make_node(new_current_node.problem.clone(), new_current_node.subtask_queue.clone(), new_current_node.called.clone(), new_current_node.applied_action_list.clone());

		node_queue.push(new_node);

	}

	next_node(node_queue, domain)
}