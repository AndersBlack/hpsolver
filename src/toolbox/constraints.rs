use crate::toolbox::RelVars;

/// Returns a list of relevant variables that fulfills the constraints
pub fn check_constraints( relevant_variables: &RelVars, constraints: &Vec<(bool, String, String)>) -> Vec<RelVars> {

	let mut relevant_variables_list;
	let mut intermediate_var_list = Vec::<RelVars>::new();
	let mut result = Vec::<RelVars>::new();

	intermediate_var_list.push(relevant_variables.clone());
	let mut i = 1;
	for constraint in constraints {
		while !intermediate_var_list.is_empty() {

			let current_rel_vars = intermediate_var_list.pop().unwrap();

			if constraint.0 {
				relevant_variables_list = constraint_equal(current_rel_vars, &constraint);
			} else {
				relevant_variables_list = constraint_unequal(current_rel_vars, &constraint);
			}
			
			for rel in &relevant_variables_list{
				result.push(rel.clone());
			}
			
		}

		intermediate_var_list = result.clone();

		if i < constraints.len(){
			result = Vec::<RelVars>::new();
		}

		i += 1;
	}
	
	result
}

/// Checks constraints where values are required to be equal
fn constraint_equal( current_rel_vars: RelVars, constraint: &(bool, String, String)) -> Vec<RelVars> {
	
	let mut relevant_variables_list = Vec::<RelVars>::new();

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

/// Checks constraints where values are required to be unequal
fn constraint_unequal( mut current_rel_vars: RelVars, constraint: &(bool, String, String)) -> Vec<RelVars> {
	
	let mut relevant_variables_list = Vec::<RelVars>::new();
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

	if current_rel_vars[index_first].2.len() == 1 && current_rel_vars[index_second].2.len() == 1 {

		if current_rel_vars[index_second].2[0] != current_rel_vars[index_first].2[0] {
			relevant_variables_list.push(current_rel_vars);
		}
		
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

	if current_rel_vars[index_first].2.len() == 1 {

		let index = current_rel_vars[index_second].2.iter().position(|x| *x == conflict_value_list[0]).unwrap();
		current_rel_vars[index_second].2.remove(index);

		relevant_variables_list.push(current_rel_vars);

		return relevant_variables_list
	} else if current_rel_vars[index_second].2.len() == 1 {

		let index = current_rel_vars[index_second].2.iter().position(|x| *x == conflict_value_list[0]).unwrap();
		current_rel_vars[index_first].2.remove(index);

		relevant_variables_list.push(current_rel_vars);

		return relevant_variables_list
	}

	for val in &conflict_value_list {

		let mut rel_clone_one = current_rel_vars.clone();
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

	relevant_variables_list
}
