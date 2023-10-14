use crate::node::Node;
use crate::node::SubtaskTypes;
use crate::problem::*;
use crate::domain::*;

pub fn depth_first(problem: &mut Problem, domain: Domain) {

	let mut node_queue = Vec::<Node>::new();

	let mut htn_subtask_queue = Vec::<SubtaskTypes>::new();

	for subtask in problem.htn.subtasks.clone().into_iter().rev() {
		htn_subtask_queue.push(SubtaskTypes::HtnTask(subtask));
	}

	let subtask = htn_subtask_queue.pop();

	match subtask {
		Some(SubtaskTypes::HtnTask(htn_task))=> {
			for method in &domain.methods {
				if method.task.0 == htn_task.0 {
					let mut subtask_queue_clone = htn_subtask_queue.clone();
					subtask_queue_clone.push(SubtaskTypes::Method(method));

					let mut new_relevant_parameters = Vec::<(String, String, Vec<String>)>::new();

					for item in htn_task.2.clone() {
						for object in &problem.objects {
							if object.object.0 == item {
								new_relevant_parameters.push(("no name".to_string(), object.object.1.clone(), vec![item.clone()]))
							}
						}
					}

					let new_node = Node {
						problem: problem.clone(),
						subtask_queue: subtask_queue_clone,
						relevant_parameters: new_relevant_parameters 
					};

					node_queue.push(new_node);
				}
			}
		},
		Task => {},
		Method => {},
		Action => {},
		None => {}
	}

	next_node(&mut node_queue, &domain)
}

fn next_node(node_queue: &mut Vec::<Node>, domain: &Domain) {

	let current_node = node_queue.pop();

	// Handle subtasks
	match current_node {
		Some(current_node) => {
			let current_subtask = current_node.clone().subtask_queue.pop(); 

			match current_subtask {
				Some(SubtaskTypes::HtnTask(htn_task))=> {
					println!("htn")
				},
				Some(SubtaskTypes::Task(task)) => {
					//println!("task")

					// Expand task and create a new node for every method that task expands to
					for method in domain.methods.clone() {
						if method.task.0 == task.name {
							//println!("task: {:?}", task.name)
						}
					}


				},
				Some(SubtaskTypes::Method(method)) => {
					// Update relevant parameters
					let updated_relevant_variables = update_relevant_variables(&current_node, method);
					//println!("{:?}", updated_relevant_variables);

					// Check preconditions
					for precon in method.precondition.iter().flatten() {
						let precon_clear = check_precondition(precon, &updated_relevant_variables, &current_node.problem.state);

						// Not finished!
						if !precon_clear {
							println!("Precon didnt clear, this node is not it!");
						}
					}

					// Create node 
					// Check tasks and actions in order to add them to subtask queue
					let mut new_subtask_queue = current_node.subtask_queue.clone(); 
					
					for subtask in method.subtasks.iter().flatten().rev() {

						for task in domain.tasks.clone() {
							if task.name == subtask.0 {
								println!("task: {:?}", task.name);
								new_subtask_queue.push(SubtaskTypes::Task(task.clone()));
							}
						}

						for action in domain.actions.iter().clone() {
							//println!("{:?}", action);
							if action.name == subtask.0 {
								println!("task: {:?}", subtask.0);
								new_subtask_queue.push(SubtaskTypes::Action(action.clone()));
							}
						}
					}

					// --------- OBS! Der er noget galt med relevant variables. Vi laver kun en node for 3 items på en subtask kø. Derved er relevant variables umulig at oprethold. overvej flyt til tuple i subtask kø

					let new_node = Node {
						problem: current_node.problem.clone(),
						subtask_queue: new_subtask_queue,
						relevant_parameters: updated_relevant_variables 
					};

					node_queue.push(new_node);

					next_node(&mut node_queue.clone(), domain);
				},
				Some(SubtaskTypes::Action(action)) => {
					println!("action")
				},
				None => {}
			}


		},
		None => {panic!("Node queue found empty!")}
	}

}

fn update_relevant_variables(node: &Node, method: &Method) -> Vec<(String, String, Vec<String>)> {

	let mut updated_relevant_parameters = Vec::<(String, String, Vec<String>)>::new();

	for param in &method.parameters {

		let mut found_in_task = false;
		let mut looking_count = 0;
			
		for task_param in &method.task.1 {
							
			if &param.name == task_param {
				//println!("Found in task param: {:?} {:?} {:?}", param.name.clone(), task_param.clone(), node.relevant_parameters.clone() );
				updated_relevant_parameters.push((param.name.clone(), node.relevant_parameters[looking_count].1.clone(), node.relevant_parameters[looking_count].2.clone()));
							
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

	updated_relevant_parameters

}

// Takes the boolean prefix, the name, the list of lists of possible values and a ref to the state
fn check_precondition(precondition: &(bool,String,Vec<String>), param_list: &Vec::<(String, String, Vec<String>)>, state: &State) -> bool {

	let mut precondition_value_list = Vec::<(String, Vec<String>)>::new();
	let mut param_counter = 0;

	//Find needed values (Explain to Andreas)
	'outer: for value in &precondition.2 {
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
	let mut found_count = 0;
	//println!("Precondition: {:?} with precondition_value_list: {:?}\n", precondition, precondition_value_list);
	//println!("param list {:?}", precondition_value_list);

	// Find state parameter
	for value in &state.state_variables {
		let mut foundCounter = 0;

		if value.0 == precondition.1 {
			// For every variable in state parameter
			//println!("bob {:?} - {:?}",value, precondition);

			for n in 0..=(value.1.len()-1) {
				//println!("hmm {:?} {:?}", value.1, &precondition_value_list);
				for param in &precondition_value_list[n].1 {
					if &value.1[n] == param {
						foundCounter = foundCounter + 1;
					} 
				}
			}

		}

		if foundCounter == value.1.len() && precondition.0 == true {
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

// ------------------------------------------------- With Node ----------------------------------


// pub fn depth_first(problem: &mut Problem, domain: Domain) {

// 	let mut node_queue = Vec::<Node>::new();


// 	//println!("{:?}", domain);
// 	let mut shared_action_list = Vec::<Action>::new();

// 	// Loop every task in the htn
// 	for task in problem.htn.subtasks.clone() {
// 			println!("------------------------------------- {} ------------------------------------- \n", task.1);

// 			let mut variable_list = Vec::<Vec<String>>::new();

// 			for variable in &task.2 {
// 				let mut var_group = Vec::<String>::new();
// 				var_group.push(variable.to_string());
// 				variable_list.push(var_group);
// 			}

// 			go_through_method( &task, problem, &domain, &mut variable_list , &mut shared_action_list)
// 	}
// }

// fn go_through_method(task: &(String, String, Vec<String>),  problem: &mut Problem, domain: &Domain, variable_list: &mut Vec::<Vec<String>>, shared_action_list: &mut Vec::<Action>) {
	
// 	//Find methods with matching task
// 	let mut method_list = Vec::<&Method>::new();
// 	for method in &domain.methods {
// 		if method.task.0 == task.0 {
// 			method_list.push(method);
// 		}
// 	}

// 	for method in method_list {
// 		// Generate list of relevant variables
// 		let mut relevant_vars = Vec::<(String, String, Vec<String>)>::new();

// 		//println!("{:?}", variable_list);

// 		// for param in method
// 		for param in &method.parameters {

// 			let mut found_in_task = false;
// 			let mut looking_count = 0;

// 			for task_param in &method.task.1 {
				
// 				if &param.name == task_param {
// 					//println!("Found in task param: {:?} {:?} {:?}", param.name.clone(), task_param.clone(), variable_list[looking_count].clone() );
// 					relevant_vars.push((param.name.clone(), param.object_type.clone(), variable_list[looking_count].clone()));
					
// 					found_in_task = true;
// 				}

// 				looking_count = looking_count + 1;
// 			}

// 			if !found_in_task {
				
// 				let mut var_list = Vec::<String>::new();

// 				for object in &problem.objects {
// 					if object.object.1 == param.object_type {
// 						var_list.push(object.object.0.clone());
// 					}
// 				}

// 				relevant_vars.push((param.name.clone(), param.object_type.clone(), var_list.clone()));
// 			}
// 		}

// 		let mut preconditions_clear = true;

// 		// Check preconditions
// 		for precon in method.precondition.iter().flatten() {

// 			let (this_precon_clear, valid_count) = check_precondition(&precon, &relevant_vars, &problem.state);
// 			if !this_precon_clear { 
// 				preconditions_clear = false;
// 				break; 
// 			}
// 		}

// 		// Needs to change at some point!
// 		if !preconditions_clear {
// 			panic!("preconditions didnt clear");
// 		}

// 		// Calling go_through_method again!
// 		for subtask in method.subtasks.iter().flatten() {
// 			//Make reduced relevant var list
// 			let mut updated_relevant_var_list = Vec::<Vec<String>>::new();
// 			for arg in &subtask.2{
// 				for var in &relevant_vars {
// 					if &var.0 == arg {
// 						updated_relevant_var_list.push(var.2.clone());
// 					}
// 				}
// 			}

// 			go_through_method(subtask, problem, domain, &mut updated_relevant_var_list, shared_action_list);
// 		}
// 	}

// 	//Find action with matching task
// 	let mut action_list = Vec::<&Action>::new();
// 	for action in &domain.actions {
// 		if action.name == task.0 {
// 			action_list.push(action);
// 		}
// 	}

// 	for action in action_list {

// 		// Generate list of relevant variables
// 		let mut relevant_vars = Vec::<(String, String, Vec<String>)>::new();

// 		println!("Var list: {:?} for action: {}\n", variable_list, action.name);
// 		let mut param_counter = 0;

// 		// for param in action
// 		for param in &action.parameters {
// 			//println!("Found in task param: {:?} {:?} {:?}", param.name.clone(), task_param.clone(), variable_list[looking_count].clone() );
// 			relevant_vars.push((param.name.clone(), param.object_type.clone(), variable_list[param_counter].clone()));
// 			param_counter = param_counter + 1;
// 		}
		
// 		// Check preconditions for action
// 		for precon in action.precondition.iter().flatten() {

// 			let (this_precon_clear, valid_count) = check_precondition(&precon, &relevant_vars, &problem.state);
// 			if !this_precon_clear { 
// 				panic!("unable to satisfy precondition: {:?} count: {}", precon, valid_count);
// 			}

// 		}

// 		for effect in &action.effect {
// 			// TODO: how many ways can the effect become valid?
// 			let valid_count = 1;

// 			// for args in effect.2 {
// 			// 	for var in relevant_vars {
// 			// 		if var.0 == args {
						
// 			// 		}
// 			// 	}
// 			// }


// 			for shared_action in shared_action_list.clone() {
// 				for effect in &shared_action.effect {
// 					println!("Effect of shared action: {:?}", effect);
// 				}
// 			}

// 			if valid_count > 1 {
// 				// Add action to shared list
// 				shared_action_list.push(action.clone());
// 			} else if valid_count == 1 {
// 				// TODO: Apply effect of action!
// 				for effect in &action.effect {
// 					apply_effect(&effect, problem, &relevant_vars);
// 				}
// 			} else {
				
// 			}
// 		}

// 	} 

// 	//panic!("test!");
// } 

// // Takes the boolean prefix, the name, the list of lists of possible values and a ref to the state
// fn check_precondition(precondition: &(bool,String,Vec<String>), param_list: &Vec::<(String, String, Vec<String>)>, state: &State) -> (bool, usize) {

// 	let mut precondition_value_list = Vec::<(String, Vec<String>)>::new();
// 	let mut param_counter = 0;

// 	//Find needed values (Explain to Andreas)
// 	'outer: for value in &precondition.2 {
// 		for param in param_list {
// 			if value == &param.0 {
// 				precondition_value_list.push((param.0.clone(), param.2.clone()));
// 				param_counter = param_counter + 1;
// 			}

// 			if param_counter == precondition.2.len() {
// 				break;
// 			}
// 		}
// 	};

// 	let mut found_one = false;
// 	let mut found_count = 0;
// 	println!("Precondition: {:?} with precondition_value_list: {:?}\n", precondition, precondition_value_list);
// 	//println!("param list {:?}", precondition_value_list);

// 	// Find state parameter
// 	for value in &state.state_variables {
// 		let mut foundCounter = 0;

// 		if value.0 == precondition.1 {
// 			// For every variable in state parameter
// 			//println!("bob {:?} - {:?}",value, precondition);

// 			for n in 0..=(value.1.len()-1) {
// 				//println!("hmm {:?} {:?}", value.1, &precondition_value_list);
// 				for param in &precondition_value_list[n].1 {
// 					if &value.1[n] == param {
// 						foundCounter = foundCounter + 1;
// 					} 
// 				}
// 			}

// 		}

// 		if foundCounter == value.1.len() && precondition.0 == true {
// 			//println!("Found match {:?} & {:?} in {}", value.1, precondition_value_list, precondition.1);
// 			found_one = true;
// 			break;
// 		}

// 	}

// 	if (found_one == false && precondition.0 == true) || (found_one == true && precondition.0 == false) {
// 		//panic!("Encountered unsatisfiable precondition: {:?} with {:?}", precondition, param_list);
// 		return (false, 0);
// 	}

// 	// // Calculate combinations
// 	// let mut combinations = 1;
// 	// for val in precondition_value_list.1 {
// 	// 	combinations = combinations * val.len();
// 	// }

// 	(true, 1)
// }

// fn apply_effect( effect: &(bool,String,Vec<String>), problem: &mut Problem, param_list: &Vec::<(String, String, Vec<String>)> ) {

// 	//println!("Effect: {:?} - {:?} - {:?}\n", effect, param_list, problem.state.state_variables);

// 	let mut found_in_state = false;

// 	if effect.0 == false {

// 		// Remove found from state
// 		let index = problem.state.state_variables.iter().position(|x| x.0 == effect.1).unwrap();
// 		problem.state.state_variables.remove(index);
	
// 	} else {
// 		let mut args_list = Vec::<String>::new();

// 		for effect_var in &effect.2 {
// 			for params in param_list {
// 				if effect_var == &params.0 {
// 					// Need change (just selects the first one now)
// 					args_list.push(params.2[0].clone());
// 				}
// 			}
// 		}

// 		let new_state_param = (effect.1.clone(), args_list);
// 		problem.state.state_variables.push(new_state_param);
// 	}
// } 


// ----------------------------------------------- OLD FUNCTIONS -------------------------------------------------



// fn execute_subtasks(task: &(String, String, Vec<String>),  problem: &mut Problem, domain: &Domain, param_list: &Option<Vec<(String, Vec<String>, String)>>) {
// 	// Loop through methods and locate task from htn
// 		//println!("looping method: {} against {}",method.task.0 , task.0);

// 		match param_list {
// 			Some(inner) => {
// 				for method in &domain.methods{
// 					if task.0 == method.task.0 {

// 						// Translate parameters
// 						for sending_param in param_list {
// 							for receiving_param in &task.2{
// 								//println!("{:?} <-> {:?}", sending_param, receiving_param);
// 							}
// 						}

// 						if method.subtasks != None {
// 							//println!("\n METHOD IN EXECUTE {} \n", method);
// 							prep_method(method, problem, domain, &task.2, param_list);
// 						} 
// 					}
// 				}
				
// 				for action in &domain.actions{
// 					if task.0 == action.name{

// 						let mut new_param_list = Vec::<(String, Vec<String>, String)>::new();

// 						// Translate parameters
// 						for receiving_param in &task.2{
// 							for sending_param in param_list.iter().flatten() {
// 								if &sending_param.0 == receiving_param {
// 									//println!("sending param {:?}", receiving_param);
// 									new_param_list.push(sending_param.clone());
// 								}
// 							}
// 						}

// 						prep_action(&Some(new_param_list), problem, domain, action)
// 					}
// 				}
// 			},
// 			None => {
// 				for method in &domain.methods {	
// 					if method.task.0 == task.0 && method.subtasks != None {
// 						// Found task from htn
// 						prep_method(method, problem, domain, &task.2, &None); 
// 					}		
// 				}
// 			}
// 		}
// }


// fn prep_method( method: &Method, problem: &mut Problem, domain: &Domain, objects: &Vec<String>, param_list: &Option<Vec<(String, Vec<String>, String)>>) {
// 	//println!("Found the method!");
// 	let mut set_parameters = Vec::<(bool, Argument)>::new();

// 	//println!("\n METHOD IN PREP {:?}", method);

// 	// Loop through method parameter and check if the htn task provided it
// 	for method_parameter in &method.parameters{
// 		let task_parameters: Vec<String> = method.task.1.clone();
// 		let mut i = 0;
// 		let mut found: bool = false;

// 		while i < task_parameters.len() && !found {
// 			//println!("i: {} < param_len: {}",i ,task_parameters.len());

// 			// The htn task provided this parameter
// 			if task_parameters[i] == method_parameter.name{
// 				set_parameters.push(( true, method_parameter.clone()));
// 				found = true; 
// 			}
			
// 			i = i + 1;
// 		}

// 		if !found {
// 			// The htn task did not provide this parameter
// 			set_parameters.push(( false, method_parameter.clone()));
// 		}
// 	}


// 	let mut new_param_list = Vec::<(String, Vec<String>, String)>::new(); // all possible combinations of parameters for the methods
// 	let mut int = 0;
// 	// Look for unprovided parameters
// 	for parameter in set_parameters {
// 		//println!("Task: {:?}, Task args: {:?}, Set param:{:?}", method.task.1, objects, parameter);

// 		if parameter.0 == false || !param_list.is_none() {
// 			let mut unset_param_object_list = Vec::<String>::new();
// 			let param_name = parameter.1.name.clone();

// 			for object in &problem.objects {
// 					if object.object.1 == parameter.1.object_type {
// 							unset_param_object_list.push(object.object.0.clone());
// 					}
// 			}

// 			new_param_list.push((param_name, unset_param_object_list, parameter.1.object_type));
// 		} else {
// 			let mut set_param_object_list = Vec::<String>::new();
			
// 			set_param_object_list.push(objects[int].clone());

// 			for object in &problem.objects {
// 				if objects[int] == object.object.0 {
// 					new_param_list.push((method.task.1[int].clone(), set_param_object_list.clone(), object.object.1.clone()));
// 				}
// 			} 
			
// 			int = int + 1;
// 		}
// 	}

// 	//println!("\n PARAM FROM PREP {:?}", new_param_list);

// 	resolve_method(&Some(new_param_list), problem, domain, method);
// } 

// // Needs return type
// fn resolve_method(param_list: &Option<Vec<(String, Vec<String>, String)>>, problem: &mut Problem, domain: &Domain, method: &Method){
  
// 	//println!("{:?}\n", param_list);
// 	//println!("{}\n", method);

// 	// Check preconditions
// 	for precondition in method.precondition.iter().flatten() {
// 		//check_precondition(precondition, param_list, &problem.state);
// 	}

// 	// Call subtasks
// 	match &method.subtasks {
// 		Some(subtasks) => {
// 			for subtask in subtasks {
// 				//println!("GIMME UBS:{:?}",subtask);
// 				execute_subtasks(subtask, problem, domain, param_list);
// 			}
// 		},
// 		None => {
// 			//Check preconditions
// 		}
// 	}


// }

// fn prep_action(param_list: &Option<Vec<(String, Vec<String>, String)>>, problem: &mut Problem, domain: &Domain, action: &Action) {
// 	//println!("Action: {:?}\n", action);
// 	let mut index_counter = 0;

// 	let mut new_param_list = Vec::<(String, Vec<String>, String)>::new();

// 	for param in param_list.iter().flatten() {
// 		new_param_list.push((action.parameters[index_counter].name.clone(), param.1.clone(), param.2.clone()));
// 		index_counter = index_counter + 1;
// 	}

// 	//panic!("Stopped at action prep");

// 	resolve_action(&Some(new_param_list), problem, domain, action);
// } 

// fn resolve_action(param_list: &Option<Vec<(String, Vec<String>, String)>>, problem: &mut Problem, domain: &Domain, action: &Action){

// 	//println!("PARAM LIST - Action: {}\nParams: {:?} \n", action.name, param_list);

// 	println!("{}\n", action);

// 	let mut preconditions_clear = true;

// 	// Check preconditions
// 	for precondition in action.precondition.iter().flatten() {
// 		let this_precon_clear = check_precondition(precondition, param_list, &problem.state);
// 		if !this_precon_clear { 
// 			preconditions_clear = false;
// 			break; 
// 		}
// 	}

// 	if preconditions_clear {
// 		for effect in &action.effect {
// 			apply_effect(effect, problem, param_list);
// 		} 
// 	}
// }


// fn apply_effect( effect: &(bool,String,Vec<String>), problem: &mut Problem, param_list: &Option<Vec<(String, Vec<String>, String)>> ) {

// 	println!("\nEffect: {:?} - {:?}", effect, param_list);

// 	let mut found_in_state = false;

// 	if effect.0 == false {

// 		// Remove found from state
// 		let index = problem.state.state_variables.iter().position(|x| x.0 == effect.1).unwrap();
// 		problem.state.state_variables.remove(index);
	
// 	} else {
// 		let mut args_list = Vec::<String>::new();

// 		for effect_var in &effect.2 {
// 			for params in param_list.iter().flatten() {
// 				if effect_var == &params.0 {
// 					// Need change (just selects the first one now)
// 					args_list.push(params.1[0].clone());
// 				}
// 			}
// 		}

// 		let new_state_param = (effect.1.clone(), args_list);
// 		problem.state.state_variables.push(new_state_param);
// 	}

// 	//todo!("Implement apply effect!") 

// } 

// // Takes the boolean prefix, the name, the list of lists of possible values and a ref to the state
// // fn check_precondition(precondition: &(bool,String,Vec<String>), param_list: &Option<Vec<(String, Vec<String>, String)>>, state: &State) -> bool {

// // 	let mut precondition_value_list = Vec::<Vec<String>>::new();
// // 	let mut param_counter = 0;

// // 	//Find needed values (Explain to Andreas)
// // 	'outer: for value in &precondition.2 {
// // 		for param in param_list.iter().flatten() {
// // 			if value == &param.0 {
// // 				precondition_value_list.push(param.1.clone());
// // 				param_counter = param_counter + 1;
// // 			}

// // 			if param_counter == precondition.2.len() {
// // 				break;
// // 			}
// // 		}
// // 	};

// // 	let mut found_one = false;
// // 	//println!("Precondition: {:?} with param_list: {:?}", precondition, param_list);

// // 	//println!("param list {:?}", precondition_value_list);

// // 	// Find state parameter
// // 	for value in &state.state_variables {
// // 		let mut foundCounter = 0;

// // 		if value.0 == precondition.1 {
// // 			// For every variable in state parameter
// // 			//println!("bob {:?} - {:?}",value, precondition);
// // 			for n in 0..=(value.1.len()-1) {
// // 				//println!("hmm {:?} {:?}", value.1, &precondition_value_list);
// // 				for param in &precondition_value_list[n] {
// // 					if &value.1[n] == param {
// // 						foundCounter = foundCounter + 1;
// // 					} 
// // 				}
// // 			}
// // 		}

// // 		if foundCounter == value.1.len() && precondition.0 == true {
// // 			println!("Found match {:?} & {:?} in {}", value.1, precondition_value_list, precondition.1);
// // 			found_one = true;
// // 			break;
// // 		}

// // 	}

// // 	if (found_one == false && precondition.0 == true) || (found_one == true && precondition.0 == false) {
// // 		//panic!("Encountered unsatisfiable precondition: {:?} with {:?}", precondition, param_list);
// // 		return false;
// // 	} 

// // 	true
// // }


