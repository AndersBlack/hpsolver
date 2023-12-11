use std::collections::HashSet;
use crate::{datastructures::{node::*, domain::{*, self}, problem::{*}}, toolbox::reduce_domain};
use crate::toolbox::{self};
pub mod iterative_df;
pub mod stoppable_df;

// Relevant Variables datatype
type RelVars = Vec<(String, String, Vec<String>)>;

pub fn depth_first(problem: Problem, domain: &Domain) {

	let mut node_queue = Vec::<Node>::new();
	let mut htn_subtask_queue = Vec::<(SubtaskTypes, RelVars)>::new();

	let mut new_problem: Problem = update_objects(problem.clone(), domain);

	println!("HTN: {:?}\n ", new_problem.htn);
	println!("Actions len: {}, Method len: {}\n", domain.actions.len(), domain.methods.len());

	new_problem.htn.subtasks.reverse();

	for subtask in &new_problem.htn.subtasks {

		let mut new_relevant_parameters = RelVars::new();

		for item in &subtask.2 {
			if item.contains("?") {
				
				for parameter in &new_problem.htn.parameters {
					if item == &parameter.0 {

						let mut var_vec = Vec::<String>::new();

						for object in &new_problem.objects {
							if object.1 == parameter.1 {
								var_vec.push(object.0.clone());
							} else if object.2.contains(&parameter.1) {
								var_vec.push(object.0.clone());
							}
						}

						new_relevant_parameters.push((parameter.0.clone(), parameter.1.clone(), var_vec));
					}
				}
			} else {
				for object in &new_problem.objects {
					if &object.0 == item {
						new_relevant_parameters.push(("no name".to_string(), object.1.clone(), vec![item.clone()]))
					}
				}
			}
		}

		//println!("REL_VAR: {:?}", new_relevant_parameters);

		htn_subtask_queue.push((SubtaskTypes::HtnTask(subtask.clone()), new_relevant_parameters));	
	}

	let new_domain = reduce_domain(domain, &new_problem);
	let called = (Vec::<bool>::new(), Vec::<(Method, RelVars)>::new(), Vec::<usize>::new());
	let new_node = make_node(new_problem.clone(), htn_subtask_queue, called, (Vec::<(String, i32, Vec<String>)>::new(),Vec::<(String, i32, Vec<String>)>::new()), HashSet::<u64>::new());
	
	node_queue.push(new_node);

	run_df(&mut node_queue, &new_domain);
}

fn run_df(node_queue: &mut Vec::<Node>, domain: &Domain) {

	let mut finished: bool = false;

	while !finished {

		let current_node = node_queue.pop();

		// Handle subtasks
		match current_node {
			Some(mut current_node) => {

				let state_exists = toolbox::hash_state(&mut current_node);

				if state_exists {
					continue;
				}

				let current_subtask = current_node.subtask_queue.pop(); 

				match current_subtask {

					Some((SubtaskTypes::HtnTask(htn_task), relevant_variables))=> {
						println!("Htn_task: {:?}, Rel_Vars: {:?}\n", htn_task.0, relevant_variables);

   					let mut line = String::new();
						let b1 = std::io::stdin().read_line(&mut line).unwrap();

						perform_htn_task(node_queue, domain, current_node, htn_task, relevant_variables);
					},
					Some((SubtaskTypes::Task(task), relevant_variables)) => {
						println!("Task: {:?}\n", task.name);

						let mut line = String::new();
						let b1 = std::io::stdin().read_line(&mut line).unwrap();

						perform_task(node_queue, domain, current_node, task, relevant_variables);
					},
					Some((SubtaskTypes::Method(method), relevant_variables)) => {
						println!("Method {:?}, RELVARS: {:?}\n", method.name, relevant_variables);
						
						let mut line = String::new();
						let b1 = std::io::stdin().read_line(&mut line).unwrap();

						perform_method(node_queue, domain, current_node, method, relevant_variables);
					},
					Some((SubtaskTypes::Action(action), relevant_variables)) => {
						println!("\n Action: {:?} Relevant_variables: {:?}\n", action.name, relevant_variables);

						let mut line = String::new();
						let b1 = std::io::stdin().read_line(&mut line).unwrap();

						perform_action(node_queue, current_node, action, relevant_variables);
					},
					None => { 

						let finished_state = toolbox::check_goal_condition( &current_node.problem.state, &current_node.problem.goal );

						if finished_state {
							finished = true;
							toolbox::print_result(current_node);
						} else {
							println!("Tested wrong goal state");
						}
					}
				}
			},
			None => { 
				panic!("Node queue found empty!")
			}
		}
	}	
}

fn update_relevant_variables(node: &Node, method: &Method, old_relevant_variables: &RelVars) -> RelVars {

	let mut updated_relevant_parameters = RelVars::new();

	//println!("\n OLD VARS: {:?}\n", old_relevant_variables);

	for param in &method.parameters {

		let mut found_in_task = false;
		let mut looking_count = 0;
			
		for task_param in &method.task.1 {
							
			if &param.name == task_param {
				updated_relevant_parameters.push((param.name.clone(), old_relevant_variables[looking_count].1.clone(), old_relevant_variables[looking_count].2.clone()));
							
				found_in_task = true;
			}
		
			looking_count = looking_count + 1;
		}
	
		if !found_in_task {
					
			let mut var_list = Vec::<String>::new();
	
			for object in &node.problem.objects {
				if object.2.contains(&param.object_type) {
					var_list.push(object.0.clone());
				}
			}
		
			updated_relevant_parameters.push((param.name.clone(), param.object_type.clone(), var_list.clone()));
		}
	}

	updated_relevant_parameters
}

// Takes the boolean prefix, the name, the list of lists of possible values and a ref to the state
fn check_precondition(precondition: &(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>), param_list: &RelVars, state: &Vec<(String, Vec<String>)>, problem: &Problem) -> bool {

	//println!("\nPRECON: {:?}\nPARAMS: {:?}\n", precondition, param_list);

	match precondition.0 {
		0 | 1 => {
			let mut precondition_value_list = Vec::<(String, Vec<String>)>::new();
			let mut param_counter = 0;
		
			//Find needed values
			for value in &precondition.2 {
				for param in param_list {
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
		
			// Find state parameter
			for value in state {
				let mut found_counter = 0;
		
				if value.0 == precondition.1 {
					// For every variable in state parameter
		
					for n in 0..value.1.len() {
						for param in &precondition_value_list[n].1 {
							if &value.1[n] == param {
								found_counter = found_counter + 1;
							} 
						}
					}
				}
		
				if found_counter == value.1.len() && precondition.0 == 0 {
					found_one = true;
					break;
				}
			}
		
			if (found_one == false && precondition.0 == 0) || (found_one == true && precondition.0 == 1) {
				return false;
			}
		
			return true
		},
		2 | 3 => { 
			
			let mut bool_to_return = false;

			//Find needed values
			for param in param_list {

				if &precondition.1 == &param.0 {
					for param_val in &param.2 {
						if precondition.2.contains(param_val) {
							bool_to_return = true;
						}
					}
				}
			}

			bool_to_return
		},
		4 => { 

			let forall = precondition.3.clone().unwrap();

			let mut overall_bool = true;

			let mut forall_param = (forall.0.0, Vec::<String>::new());

			// Get all for all objects
			for object in &problem.objects {
				if forall.0.1 == object.1 {
					forall_param.1.push(object.0.clone());
				}
			}

			let precondition_inner = forall.1[0].clone();

			'outer: for value in forall_param.1 {

				for state_var in state {

					if state_var.0 == precondition_inner.1 {

						let mut found_vars = 0;
						let mut var_index = 0;

						for val in &precondition_inner.2 {

							//println!("VAL: {}", val);

							if val == &forall_param.0 {

								// Found forall arguement in precondition inner
								//println!("Checking {} against {} forall param", value, state_var.1[var_index]);
								if value == state_var.1[var_index] {
									// Value var equal to found state var value
									found_vars = found_vars + 1;
								}

							} else {

								// val is a general parameter
								for general_param in param_list {

									if &general_param.0 == val {
										//println!("Checking {:?} against {} general param", general_param.2, state_var.1[var_index]);
										if general_param.2.contains(&state_var.1[var_index]) {
											// Value var equal to found state var value
											found_vars = found_vars + 1;
										}
									}
								}

							}

							var_index = var_index + 1;

						}

						if found_vars == state_var.1.len() {
							//println!("Found one! \n");
							if !precondition_inner.0 {
								overall_bool = false;
								break 'outer;
							}

						}
					}
				}
			}

			if overall_bool {
				println!("A precondition: {:?}, params: {:?} \n", precondition, param_list);
			}

			//let mut answer = String::new();
			//io::stdin().read_line(&mut answer).ok();

			return overall_bool
		},
		_ => { panic!{"preconditions integer that does not exist"} }
	}

}

fn permutation_tool( value_list: RelVars , precondition_list: Vec<(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)>, state: &Vec<(String, Vec<String>)>, problem: &Problem) ->   Vec::<Vec::<usize>> {

	let mut size_ref_list = Vec::<usize>::new();
	let mut permutation_holder = Vec::<usize>::new();
	let mut permutation_list_list = Vec::<Vec::<usize>>::new();

	for var_info in &value_list {
		size_ref_list.push(var_info.2.len());
		permutation_holder.push(0);
	}

	let mut n = 0;

	if precondition_list.len() == 0 {
		permutation_list_list.push(permutation_holder.clone());
		return permutation_list_list;
	}

	while n < size_ref_list.len() {

		n = 0;
		
		// Check precondition
		if precon_cleared(&permutation_holder, &value_list, &precondition_list, state, problem) {
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

fn precon_cleared (permutation: &Vec::<usize>, value_list: &RelVars, precondition_list: &Vec<(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)>, state: &Vec<(String, Vec<String>)>, problem: &Problem) -> bool {

	let mut clear = true;
	let mut new_value_list = RelVars::new();
	let mut perm_index = 0;

	//println!("{:?} - {:?}", permutation, value_list);

	for val in value_list {
		new_value_list.push((val.0.clone(), val.1.clone(), vec![val.2[permutation[perm_index]].clone()]));
		perm_index = perm_index + 1;
	}

	for precon in precondition_list {
		if !check_precondition(precon, &new_value_list, state, problem) {
			clear = false;
		}
	}

	clear
}

fn apply_effect( effect: &(bool,String,Vec<String>), problem: &mut Problem, param_list: RelVars ) {

	let _fg = ::flame::start_guard("apply effect");

	if effect.0 == false {
		// Remove found from state
		let optional_index = problem.state.iter().position(|x| x.0 == effect.1 && toolbox::compare_lists(x.1.clone(), effect.2.clone()));

		match optional_index {
				Some(index) => {
					problem.state.remove(index);
				},
				None => {
					// The variable was not set
				}
		}
	
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
		problem.state.push(new_state_param);
	}
} 

fn make_node( new_problem: Problem, sq: Vec::<(SubtaskTypes, RelVars)>, called: (Vec<bool>, Vec<(Method, RelVars)>, Vec<usize>), aal: (Vec<(String, i32, Vec<String>)>, Vec<(String, i32, Vec<String>)>), hs: HashSet<u64>) -> Node {

		let new_node = Node {
			problem: new_problem,
			subtask_queue: sq,
			called: called,
			applied_action_list: aal,
			hash_table: hs
		};

		new_node
}

fn perform_htn_task ( node_queue: &mut Vec::<Node>, domain: &Domain, mut current_node: Node, htn_task: (String, String, Vec<String>, bool), relevant_variables: RelVars) {

	//println!("htn_task: {} - VAR: {:?}", htn_task.0, relevant_variables);

	let mut method_list = Vec::<Method>::new(); 

	for method in &domain.methods {
		if method.task.0 == htn_task.0 { 
			method_list.push(method.clone());
		}
	}

	method_list.sort_by(|a,b| a.subtasks.len().cmp(&b.subtasks.len()));

	// Expand task and create a new node for every method that task expands to
	for method in method_list {
		let mut subtask_queue_clone = current_node.subtask_queue.clone();
		let updated_relevant_variables = update_relevant_variables(&current_node, &method, &relevant_variables);

		// Update relevant variables
		subtask_queue_clone.push((SubtaskTypes::Method(method.clone()),updated_relevant_variables));

		current_node.called.0.push(false);
		current_node.called.2.push(0);
		current_node.applied_action_list.1.push(("root".to_string(), method.id, Vec::<String>::new()));
		
		let new_node = make_node(current_node.problem.clone(), subtask_queue_clone, (current_node.called.0.clone(), current_node.called.1.clone(), current_node.called.2.clone()), current_node.applied_action_list.clone(), current_node.hash_table.clone());

		node_queue.push(new_node);
	}
}

fn perform_task ( node_queue: &mut Vec::<Node>, domain: &Domain, current_node: Node, task: Task, relevant_variables: RelVars ) {

	//println!("Task: {} - VAR: {:?}", task.name, relevant_variables);

	let mut method_list = Vec::<Method>::new(); 

	for method in &domain.methods {
		if method.task.0 == task.name { 
			method_list.push(method.clone());
		}
	}

	method_list.sort_by(|a,b| a.subtasks.len().cmp(&b.subtasks.len()));

	// Expand task and create a new node for every method that task expands to
	for method in method_list {

		let mut new_subtask_queue = current_node.clone().subtask_queue;
		let new_rel_vars = update_relevant_variables(&current_node, &method, &relevant_variables);
		let mut empty_rel_var = false;

		for rel_var in &new_rel_vars {
			if rel_var.2.is_empty() {
				empty_rel_var = true;
			}
		}

		if !empty_rel_var {

			new_subtask_queue.push((SubtaskTypes::Method(method.clone()), new_rel_vars));

			let mut new_called = current_node.called.clone();
			new_called.2.push(0);

			let new_node = make_node(current_node.problem.clone(), new_subtask_queue, new_called, current_node.applied_action_list.clone(), current_node.hash_table.clone());

			//println!("Pushing node");
			node_queue.push(new_node)
		}
	
	}
}

fn perform_method ( node_queue: &mut Vec::<Node>, domain: &Domain, mut current_node: Node, method: Method, mut relevant_variables: RelVars ) {

	// What is the index of this method in the subtask queue of the method that called it?
	let current_subtask_index = current_node.called.2.pop().unwrap();

	// Check preconditions
	if current_subtask_index == 0 {

		current_node.applied_action_list.1.push((method.name.clone(), method.id, Vec::<String>::new()));

		match &method.precondition {
			Some(precondition) => {

				let permutation_list = permutation_tool(relevant_variables.clone(), precondition.clone(), &current_node.problem.state, &current_node.problem);

				if permutation_list.len() == 0 {
					return
				}

				let perm_map = construct_perm_map(permutation_list);
				let mut new_relevant_variables = Vec::<(String, String, Vec<String>)>::new();
				let mut map_index = 0;

				for variable in &relevant_variables {
					let mut new_variable = (variable.0.clone(), variable.1.clone(), Vec::<String>::new());
					for index in &perm_map[map_index]{
						new_variable.2.push(variable.2[*index].clone()); 
					}
					map_index = map_index + 1;
					new_relevant_variables.push(new_variable); 
				}

				relevant_variables = new_relevant_variables;

			},
			None => {}
		}
		
		match &method.constraints {
			Some(constraint) => {
				let mut relevant_variables_list = check_constraints( &relevant_variables, constraint);

				relevant_variables = relevant_variables_list.pop().unwrap();

				// Push Node versions
				for relevant_variable in &relevant_variables_list{
							
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
					
					let mut new_called = current_node.called.clone();
					new_called.2.push(0);

					let new_node = make_node(current_node.problem.clone(), new_sq, new_called, current_node.applied_action_list.clone(), current_node.hash_table.clone());

					node_queue.push(new_node);
				}
			},
			None => {}
		}   
	
	}
	
	if method.subtasks.len() > 0 {

		// We have finished with this methods subtask 
		if current_subtask_index == method.subtasks.len() {

			// Is this not the first method?
			if current_node.called.0.pop().unwrap() {

				let new_node = update_vars_for_called_method(current_node, &method, &relevant_variables);

				node_queue.push(new_node);
			} else {
				node_queue.push(current_node.clone());
			}

		} else {

			let mut new_subtask_queue = current_node.subtask_queue.clone();
			let mut found_task = false;

			for task in &domain.tasks { 	
				if task.name == method.subtasks[current_subtask_index].0 {

					let mut updated_variables = RelVars::new();

					//println!("Found task: {:?}", method.subtasks[current_subtask_index]);

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

					new_subtask_queue.push((SubtaskTypes::Task(task.clone()), updated_variables));
					found_task = true;
					break;
				}
			}

			if !found_task {
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

						new_subtask_queue.push((SubtaskTypes::Action(action.clone()), updated_variables));
						break;
					}
				}
			}

			let mut new_called = current_node.called.clone();

			new_called.0.push(true);
			new_called.1.push((method.clone(), relevant_variables));
			new_called.2.push(current_subtask_index + 1);

			let new_node = make_node(current_node.problem.clone(), new_subtask_queue.clone(), new_called.clone(), current_node.applied_action_list.clone(), current_node.hash_table.clone());

			node_queue.push(new_node);
		}

	} else {

		if !current_node.called.0.pop().unwrap() {
			node_queue.push(current_node.clone());
		} else {				
			let new_node = update_vars_for_called_method(current_node, &method, &relevant_variables);

			node_queue.push(new_node);
		}

	}

	//next_node(node_queue, domain);
}

fn perform_action ( node_queue: &mut Vec::<Node>, mut current_node: Node, action: Action, relevant_variables: RelVars) {

	//println!("Reached action {}, Relvars: {:?}\n", action.name, relevant_variables);

	let permutation_list = permutation_tool(relevant_variables.clone(), action.precondition.unwrap(), &current_node.problem.state, &current_node.problem);

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
		//println!("Applied effect from {} for perm {:?}!\n", action.name, permutation);
		for effect in &action.effect.clone().unwrap() {
			apply_effect(&effect, &mut new_current_node.problem, new_relevant_variables.clone())
		}

		let mut new_applied_action = (action.name.clone(), action.id, Vec::<String>::new());

		for rel_var in &new_relevant_variables {
			new_applied_action.2.push(rel_var.2[0].clone());
		}

		new_current_node.applied_action_list.0.push(new_applied_action);

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

		let new_node = make_node(new_current_node.problem.clone(), new_current_node.subtask_queue.clone(), new_current_node.called.clone(), new_current_node.applied_action_list.clone(), current_node.hash_table.clone());

		node_queue.push(new_node);

	}

	//next_node(node_queue, domain)
}

fn construct_perm_map ( permutation_list: Vec<Vec<usize>>) -> Vec<Vec<usize>> {

	let mut perm_map = Vec::<Vec<usize>>::new();

	for _n in 0..permutation_list[0].len() {
		perm_map.push(Vec::<usize>::new());
	}

	let mut int_index = 0;
	for list_num in 0..perm_map.len() {
		
		for permutation in &permutation_list {
			if !perm_map[list_num].contains(&permutation[int_index]) {
				perm_map[list_num].push(permutation[int_index])
			}
		}

		int_index = int_index + 1;
	}

	perm_map
}

fn check_constraints( relevant_variables: &RelVars, constraints: &Vec<(bool, String, String)>) -> Vec<Vec<(String, String, Vec<String>)>> {

	let mut relevant_variables_list = Vec::<Vec<(String, String, Vec<String>)>>::new();
	let mut intermediate_var_list = Vec::<Vec<(String, String, Vec<String>)>>::new();

	//println!("In constraints: {} \n\nRelevant vars: {:?} \n", method, relevant_variables);

	intermediate_var_list.push(relevant_variables.clone());

	for constraint in constraints {

		while !intermediate_var_list.is_empty() {

			let current_rel_vars = intermediate_var_list.pop().unwrap();

			if constraint.0 {
				relevant_variables_list = constraint_equal(current_rel_vars, &constraint);
			} else {
				relevant_variables_list = constraint_unequal(current_rel_vars, &constraint);
			}

		}

		intermediate_var_list = relevant_variables_list.clone();
		relevant_variables_list = Vec::<Vec<(String, String, Vec<String>)>>::new();
	}

	intermediate_var_list
}

fn constraint_equal(current_rel_vars: Vec<(String, String, Vec<String>)>, constraint: &(bool, String, String)) -> Vec<Vec<(String, String, Vec<String>)>> {
	
	// De skal være ens
	let mut relevant_variables_list = Vec::<Vec<(String, String, Vec<String>)>>::new();

	let mut index_first = 0;
	let mut index_second = 0;
	let mut counting_int = 0;

	for param in &current_rel_vars {
		if param.0 == constraint.1 {
			index_first = counting_int;
		} else if param.0 == constraint.2 {
			index_second = counting_int;
		}

		counting_int = counting_int + 1;
	}

	for first_value in &current_rel_vars[index_first].2 {
		for second_value in &current_rel_vars[index_second].2 {
			if first_value == second_value {
				let mut rel_var_clone = current_rel_vars.clone();
				rel_var_clone[index_first].2 = vec![first_value.clone()];
				rel_var_clone[index_second].2 = vec![second_value.clone()];

				relevant_variables_list.push(rel_var_clone);
			}
		}
	}

	relevant_variables_list
}

fn constraint_unequal(mut current_rel_vars: Vec<(String, String, Vec<String>)>, constraint: &(bool, String, String)) -> Vec<Vec<(String, String, Vec<String>)>> {
	// De må ikke være ens

	let mut relevant_variables_list = Vec::<Vec<(String, String, Vec<String>)>>::new();

	let mut index_first = 0;
	let mut index_second = 0;
	let mut counting_int = 0;

	for param in &current_rel_vars {
		if param.0 == constraint.1 {
			index_first = counting_int;
		} else if param.0 == constraint.2 {
			index_second = counting_int;
		}

		counting_int = counting_int + 1;
	}

	// Check is values can even be removed
	if current_rel_vars[index_first].2.len() == 1 && current_rel_vars[index_second].2.len() == 1 {
		relevant_variables_list.push(current_rel_vars);
		return  relevant_variables_list;
	}

	let mut conflict_value_list = Vec::<String>::new();

	for val in &current_rel_vars[index_first].2 {
		if current_rel_vars[index_second].2.contains(val) {
			conflict_value_list.push(val.clone());
		}
	}

	if conflict_value_list.len() == 0 {
		relevant_variables_list.push(current_rel_vars);
		return relevant_variables_list
	}

	//Is one of the lists of length 1?
	if current_rel_vars[index_first].2.len() == 1 {

		let index = current_rel_vars[index_second].2.iter().position(|x| *x == conflict_value_list[0]).unwrap();
		current_rel_vars[index_second].2.remove(index);

		relevant_variables_list.push(current_rel_vars);

		return relevant_variables_list
	} else if current_rel_vars[index_second].2.len() == 1 {

		//println!("This one?");

		let index = current_rel_vars[index_second].2.iter().position(|x| *x == conflict_value_list[0]).unwrap();
		current_rel_vars[index_first].2.remove(index);

		relevant_variables_list.push(current_rel_vars);

		return relevant_variables_list
	}

	// Lav "safe value lister"

	for val in &conflict_value_list {

		let mut rel_clone_one = current_rel_vars.clone();

		// list 1 is the small one
		let mut list_one = Vec::<String>::new();

		for value in &current_rel_vars[index_first].2 {
			if !conflict_value_list.contains(&value) {
				list_one.push(value.clone());
			}
		}

		list_one.push(val.clone());

		let mut list_two = current_rel_vars[index_second].2.clone();

		let index = list_two.iter().position(|x| x == val).unwrap();
		list_two.remove(index);

		rel_clone_one[index_first].2 = list_one;
		rel_clone_one[index_second].2 = list_two;

		relevant_variables_list.push(rel_clone_one);
	}

	//println!("CONFLICT LIST: {:?}", conflict_value_list);

	relevant_variables_list
}

fn update_objects( mut problem: Problem, domain: &Domain ) -> Problem{

	let mut new_object_list = Vec::<(String, String, Vec<String>)>::new();

	match &domain.constants {
		Some(constants) => {
			for constant in constants {
				problem.objects.push((constant.0.clone(), constant.1.clone(), vec![]));
			}
		},
		None => {}
	}

	for object in &problem.objects {
		let mut final_subtype_list = Vec::<String>::new();
		let mut current_subtype_list = Vec::<String>::new();
		current_subtype_list.push(object.1.clone());
		final_subtype_list.push(object.1.clone());

		while !current_subtype_list.is_empty() {
			//println!("CSL: {:?}", current_subtype_list);
			let current_sub_type = current_subtype_list.pop().unwrap();
			for sub_type in &domain.types {
				
				if sub_type.0 == current_sub_type && sub_type.1 != sub_type.0 {
					current_subtype_list.push(sub_type.1.clone());
					final_subtype_list.push(sub_type.1.clone());
				}
			}
		}

		new_object_list.push((object.0.clone(), object.1.clone(), final_subtype_list));
	} 

	problem.objects = new_object_list;

	problem.clone()
} 

fn update_vars_for_called_method(mut current_node: Node, method: &Method, relevant_variables: &RelVars) -> Node {

	let (calling_method, calling_relevant_vars) = current_node.called.1.pop().unwrap();

	let calling_method_subtask = calling_method.subtasks.clone()[current_node.called.2.last().unwrap() - 1].clone();

	// let method_task = method.task.clone();
	let mut i = 0;

	let mut new_rel_vars: RelVars = RelVars::new();

	for var in relevant_variables {
		if i < method.task.1.len() && var.0 == method.task.1[i] {

			new_rel_vars.push((calling_method_subtask.2[i].clone(), var.1.clone(), var.2.clone()));
			i = i + 1;
			continue;
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

	let mut calling_meth = calling_method.clone();
	let mut subts = calling_meth.subtasks;
	subts[current_node.called.2.last().unwrap() - 1].3 = true;
	calling_meth.subtasks = subts;

	// Push to subtask_q
	let mut new_sq = current_node.subtask_queue.clone();
	new_sq.push((SubtaskTypes::Method(calling_meth.clone()), new_new_relevant_variables.clone()));

	let new_node = make_node(current_node.problem.clone(), new_sq, current_node.called.clone(), current_node.applied_action_list.clone(), current_node.hash_table.clone());

	new_node
}