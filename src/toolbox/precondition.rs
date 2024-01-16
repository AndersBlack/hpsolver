use crate::datastructures::{node::*, problem::{*}, domain::{*}};

type RelVars = Vec<(String, String, Vec<String>)>;
type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);

/// Checks a given precondition. Takes the boolean prefix, the name, the list of lists of possible values and a ref to the state
fn check_precondition( precondition: &Precondition, param_list: &RelVars, state: &Vec<(String, Vec<String>)>, problem: &Problem) -> bool {

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
					//println!("Equal {} and {}", value.0, precondition.1);
					// For every variable in state parameter
		
					for n in 0..value.1.len() {
						for param in &precondition_value_list[n].1 {
							if &value.1[n] == param {
								found_counter = found_counter + 1;
							} 
						}
					}

					if found_counter == value.1.len() {
						//println!("Found {:?}", value);
						found_one = true;
						break;
					}
				}
			}
		
			if (found_one == false && precondition.0 == 0) || (found_one == true && precondition.0 == 1) {
				//println!("Failed precon: {:?}", precondition);
				return false;
			}
			//println!("Succeded precon: {:?}", precondition);
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

							if val == &forall_param.0 {

								// Found forall arguement in precondition inner
								if value == state_var.1[var_index] {
									// Value var equal to found state var value
									found_vars = found_vars + 1;
								}

							} else {

								// val is a general parameter
								for general_param in param_list {
									if &general_param.0 == val {
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
							if !precondition_inner.0 {
								overall_bool = false;
								break 'outer;
							}
						}
					}
				}
			}

			return overall_bool
		},
		_ => { panic!{"preconditions integer that does not exist"} }
	}

}

/// Generates a list of lists of indexes representing a valid permutations of the available variables values
pub fn permutation_tool( value_list: RelVars , precondition_list: Vec<Precondition>, state: &Vec<(String, Vec<String>)>, problem: &Problem) -> (Vec<Vec<usize>>, bool) {

	let mut size_ref_list = Vec::<usize>::new();
	let mut permutation_holder = Vec::<usize>::new();
	let mut permutation_list_list = Vec::<Vec::<usize>>::new();

	// If there are no relevant variables, we still need to check precondition
	if value_list.len() == 0 {
		if precon_cleared(&permutation_holder, &value_list, &precondition_list, state, problem) {
			return (permutation_list_list, true)
		} else {
			return (permutation_list_list, false)
		}
	}

	for var_info in &value_list {
		size_ref_list.push(var_info.2.len());
		permutation_holder.push(0);
	}

	let mut n = 0;

	if precondition_list.len() == 0 {
		permutation_list_list.push(permutation_holder.clone());
		return (permutation_list_list, true);
	}

	while n < size_ref_list.len() {
		n = 0;
		
		// Check precondition
		if precon_cleared(&permutation_holder, &value_list, &precondition_list, state, problem) {
			//println!("WHILE PRECON");
			permutation_list_list.push(permutation_holder.clone());		
		} 

		if permutation_holder[n] != (size_ref_list[n] - 1) {
			//println!("WHILE PERMHOLDER NOT SIZE REF");
			permutation_holder[n] = permutation_holder[n] + 1;
		} else {

			let mut found_expansion = false;

			while !found_expansion && n < size_ref_list.len() {
				//println!("INNER WHILE");
				permutation_holder[n] = 0;

				n = n + 1;

				if n < size_ref_list.len() && permutation_holder[n] != (size_ref_list[n] - 1) {
					//println!("FOUND EXPANSION");
					permutation_holder[n] = permutation_holder[n] + 1;
					found_expansion = true;
				}
			}
		}
	}

	(permutation_list_list, true)

}

/// Loop preconditions and determine whether or not all preconditions was cleared
fn precon_cleared( permutation: &Vec::<usize>, value_list: &RelVars, precondition_list: &Vec<Precondition>, state: &Vec<(String, Vec<String>)>, problem: &Problem) -> bool {

	let mut clear = true;
	let mut new_value_list = RelVars::new();
	let mut perm_index = 0;

	// Push the relevant values from each relevant variables based on the given permutation to new_value_list
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