use core::panic;
use crate::datastructures::problem::{*};

type RelVars = Vec<(String, String, Vec<String>)>;
type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);

/// Checks a given precondition. Takes the boolean prefix, the name, the list of lists of possible values and a ref to the state
pub fn check_precondition( precondition: &Precondition, relevant_variables: &RelVars, problem: &Problem) -> bool {

	match precondition.0 {
		0 | 1 => {
			let mut precondition_value_list = Vec::<(String, Vec<String>)>::new();
			let mut param_counter = 0;

			//Find needed values
			for value in &precondition.2 {
				for param in relevant_variables {
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
			for value in &problem.state {
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
			for param in relevant_variables {

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

				for state_var in &problem.state {

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
								for general_param in relevant_variables {
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
pub fn permutation_tool( relevant_variables: RelVars , precondition_list: Vec<Precondition>, state: &Vec<(String, Vec<String>)>, problem: &Problem) -> (Vec<Vec<usize>>, RelVars, bool) {

  let mut size_ref_list = Vec::<usize>::new();
	let mut permutation_holder = Vec::<usize>::new();
	let mut permutation_list_list = Vec::<Vec::<usize>>::new();

  let new_relevant_variables = pre_permutation_cleanup(&precondition_list, &relevant_variables, state);

  for new_rel_var in &new_relevant_variables {
    if new_rel_var.2.len() == 0 {
      return (permutation_list_list, new_relevant_variables, false)
    }
  }

	// If there are no relevant variables, we still need to check precondition
	if new_relevant_variables.len() == 0 {
		if precon_cleared(&permutation_holder, &new_relevant_variables, &precondition_list, problem) {
			return (permutation_list_list, new_relevant_variables, true)
		} else {
			return (permutation_list_list, new_relevant_variables, false)
		}
	}

	for var_info in &new_relevant_variables {
		size_ref_list.push(var_info.2.len());
		permutation_holder.push(0);
	}

	let mut n = 0;

	if precondition_list.len() == 0 {
		permutation_list_list.push(permutation_holder.clone());
		return (permutation_list_list, new_relevant_variables, true);
	}

	while n < size_ref_list.len() {
		n = 0;
		
		// Check precondition
		if precon_cleared(&permutation_holder, &new_relevant_variables, &precondition_list, problem) {
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

	(permutation_list_list, new_relevant_variables, true)
}

/// Counts permutations and returns the count
pub fn perm_count( size_ref: &Vec::<Vec<usize>>) -> usize {
  let mut count = 1;

  for list in size_ref {
    count *= list.len();
  }

  count
}

/// Remove values from relevant variables that is unable to clear any precondition in any permutation
pub fn precon_trimmer( relevant_variables: RelVars , precondition_list: &Vec<Precondition>, problem: &Problem ) -> (RelVars, bool) {

  let mut new_relvars = relevant_variables.clone();
  let mut cleared_precons = true;
  let mut check_precon_again= true;
  let mut trimmed_something;

  while check_precon_again {
    check_precon_again = false;
    for precondition in precondition_list {

      if relevant_variables.len() == 0 {
        let mut found_it_bool = false;
        for state_var in &problem.state {

          if state_var.0 == precondition.1 && precondition.0 == 0 {
            found_it_bool = true;
          } else if state_var.0 == precondition.1 && precondition.0 == 1 {
            cleared_precons = false;
            break;
          }

        }

        if !found_it_bool && precondition.0 == 0 {
          cleared_precons = false;
          break;
        }
      }

        match precondition.0 {
          0 => { (new_relvars, trimmed_something) = precon_trim_zero( &new_relvars, &precondition, &problem.state) },
          1 => { (new_relvars, trimmed_something) = precon_trim_one( &new_relvars, &precondition, &problem.state) },
          2 => { (new_relvars, trimmed_something) = precon_trim_two( &new_relvars, &precondition) },
          3 => { (new_relvars, trimmed_something) = precon_trim_three( &new_relvars, &precondition) },
          _ => { (new_relvars, trimmed_something, cleared_precons) = precon_trim_forall( &new_relvars, &precondition, &problem) },
        }

        if trimmed_something {
          check_precon_again = true;
        }
    }
  }


  for new_vars in &new_relvars {
    if new_vars.2.len() == 0 {
      //println!("Didnt clear precon");
      cleared_precons = false;
    }
  }

  (new_relvars, cleared_precons)
} 

pub fn precon_trim_zero( relevant_variables: &RelVars , precondition: &Precondition, state: &Vec<(String, Vec<String>)> ) -> (RelVars, bool) {

  let mut new_relvars = relevant_variables.clone();
  let relvar_indexes = setup_relvar_indexes(&precondition.2, relevant_variables);
  let mut trimmed_something = false;

	// Make size-ref
  let size_ref = setup_size_ref(&relvar_indexes, relevant_variables);

  // Permutation holder list
  let mut perm_holder = Vec::<usize>::new();

  // Make new value list
  let mut new_value_list = Vec::<Vec<String>>::new();
  for _i in 0..relvar_indexes.len() {
    new_value_list.push(Vec::<String>::new());
    perm_holder.push(0);
  }

  // Test permutations of size ref
  let mut n = 0;
  let perm_count = perm_count(&size_ref);
  while n < perm_count {

    // Check precondition
    for state_predicate in state {
      if state_predicate.0 == precondition.1 {

        let mut found_count = 0;
        for perm_index in &perm_holder {
          // Does values match?
          if state_predicate.1[found_count] == relevant_variables[ relvar_indexes[found_count] ].2[ size_ref[found_count][*perm_index] ] {
            found_count += 1;
          } else {
            break;
          }
        }

        if found_count == precondition.2.len() {
          // Push values to new_value_list (Precon cleared)
          let mut counter = 0;
          for index in &perm_holder {
            if !new_value_list[counter].contains(&relevant_variables[relvar_indexes[counter]].2[*index]) {
              new_value_list[counter].push(relevant_variables[relvar_indexes[counter]].2[*index].clone());
            }
            counter = counter + 1;
          }

          break;
        }
      }
    }

    // Increase perm_holder based on list sizes
		let mut i = 0;
		for val in &perm_holder.clone() {
			if val < &(size_ref[i].len() - 1) {
				perm_holder[i] += 1; 
				break;
			} else {
				perm_holder[i] = 0;
			}
			i += 1;
		}
    
    
		n += 1;
  }

  // Update new_relvars
  let mut counter = 0;
  for new_values in new_value_list {
    if new_relvars[relvar_indexes[counter]].2.len() != new_values.len() {
      trimmed_something = true;
    }
    
    new_relvars[relvar_indexes[counter]].2 = new_values;
    counter = counter + 1;
  }

  return (new_relvars, trimmed_something)
}

pub fn precon_trim_one( relevant_variables: &RelVars , precondition: &Precondition, state: &Vec<(String, Vec<String>)> ) -> (RelVars, bool) {

  let mut new_relvars = relevant_variables.clone();
  let relvar_indexes = setup_relvar_indexes(&precondition.2, relevant_variables);
  let mut trimmed_something = false;

  if relevant_variables.len() == 0 {
    for state_var in state {
      if state_var.0 == precondition.1 {
        return (new_relvars, false)
      }
    }
  }

	// Make size-ref
  let size_ref = setup_size_ref(&relvar_indexes, relevant_variables);

  // Permutation holder list
  let mut perm_holder = Vec::<usize>::new();

  // Make new value list
  let mut new_value_list = Vec::<Vec<String>>::new();
  for _i in 0..relvar_indexes.len() {
    new_value_list.push(Vec::<String>::new());
    perm_holder.push(0);
  }

  // Test permutations of size ref
  let mut n = 0;
	let perm_count = perm_count(&size_ref);
  while n < perm_count {

    // Check precondition
    let mut cleared_precondition = true;
    for state_predicate in state {
      if state_predicate.0 == precondition.1 {

        let mut found_count = 0;
        for perm_index in &perm_holder {
          // Does values match?
          if state_predicate.1[found_count] == relevant_variables[ relvar_indexes[found_count] ].2[ size_ref[found_count][*perm_index] ] {
            found_count = found_count + 1;
          } else {
            break;
          }
        }

        if found_count == precondition.2.len() {
          // Push values to new_value_list (Precon cleared)
          cleared_precondition = false;
          break;
        }
      }
    }

    if cleared_precondition {

      let mut counter = 0;
      for index in &perm_holder {
        if !new_value_list[counter].contains(&relevant_variables[relvar_indexes[counter]].2[*index]) {
          new_value_list[counter].push(relevant_variables[relvar_indexes[counter]].2[*index].clone());
        }
        counter = counter + 1;
      }

    } 

    // Increase perm_holder based on list sizes
    let mut i = 0;
    for val in &perm_holder.clone() {
      if val < &(size_ref[i].len() - 1) {
        perm_holder[i] += 1; 
        break;
      } else {
        perm_holder[i] = 0;
      }
      i += 1;
    }
  
    
		n += 1;
  }

  // Update new_relvars
  let mut counter = 0;
  for new_values in new_value_list {

    if new_relvars[relvar_indexes[counter]].2.len() != new_values.len() {
      trimmed_something = true;
    }

    new_relvars[relvar_indexes[counter]].2 = new_values;
    counter = counter + 1;
  }

  return (new_relvars, trimmed_something);
}

pub fn precon_trim_two( relevant_variables: &RelVars , precondition: &Precondition) -> (RelVars, bool) {

  let mut new_relvars = relevant_variables.clone();
  let mut trimmed_something = false;
  let mut found_rel_var = false;
  let mut rel_var_index = 0;

  if precondition.0 == 2 {
    
    for relvar in &new_relvars {
      if relvar.0 == precondition.1 && new_relvars[rel_var_index].2.contains(&precondition.2[0]) {
        found_rel_var = true;
        break; 
      }

      rel_var_index = rel_var_index + 1;
    }

    if found_rel_var {
      if new_relvars[rel_var_index].2 != precondition.2 {
        trimmed_something = true;
      }

      new_relvars[rel_var_index].2 = precondition.2.clone();
    } 
    else {
      new_relvars[0].2 = vec![] // Make the relvar invalid
    }

  }

  return (new_relvars, trimmed_something)
}

pub fn precon_trim_three( relevant_variables: &RelVars , precondition: &Precondition ) -> (RelVars, bool) {

  let mut new_relvars = relevant_variables.clone();
  let mut value_index = 0;
  let mut rel_var_index = 0;
  let mut parameter_contained = false;


  for relvar in &new_relvars {
    if relvar.0 == precondition.1 && new_relvars[rel_var_index].2.contains(&precondition.2[0]) {
      parameter_contained = true;
      for value in &relvar.2  {

        if value == &precondition.2[0] {
          break;
        }

        value_index = value_index + 1;
      }

      break; 
    }
    rel_var_index = rel_var_index + 1;
  }

  if parameter_contained {
    new_relvars[rel_var_index].2.remove(value_index);
  }

  return (new_relvars, parameter_contained)
}

pub fn precon_trim_forall( relevant_variables: &RelVars , precondition: &Precondition, problem: &Problem ) -> (RelVars, bool, bool) {
  
  let mut new_rel_var = relevant_variables.clone();
  let forall = precondition.3.clone().unwrap();
  let mut forall_param = (forall.0.0, Vec::<String>::new());
  let mut _forall_param_index = 0;
  let mut relvar_indexes = Vec::<usize>::new();
  let mut precon_relvar_index = Vec::<usize>::new();
  let mut value_string_list = Vec::<Vec<String>>::new();
  let mut trimmed_something = false;
  let mut cleared_precondition = true;

  // Get all for all objects
  for object in &problem.objects {
    if forall.0.1 == object.1 {
      forall_param.1.push(object.0.clone());
    }
  }

  for precon_inner in forall.1 {

    // Find index of none forall param in relevant variables
    let mut index_counter = 0;
    for precon_arg in &precon_inner.2 {

      if precon_arg == &forall_param.0 {
        _forall_param_index = index_counter;
        value_string_list.push(forall_param.1.clone());
      } else {
        let mut relvar_index = 0;
        for relvar in relevant_variables {
          if &relvar.0 == precon_arg {
            relvar_indexes.push(relvar_index);
            precon_relvar_index.push(index_counter);
            value_string_list.push(relevant_variables[relvar_index].2.clone());
          }
          relvar_index += 1;
        }
      }

      index_counter += 1;
    }

    // Make size-ref
    let mut size_ref = Vec::<Vec<usize>>::new();

    for index in 0..value_string_list.len() {

      let mut inner_size_ref = Vec::<usize>::new();

      for i in 0..value_string_list[index].len() {
        inner_size_ref.push(i);
      }
      size_ref.push(inner_size_ref);
    }

    // Permutation holder list
    let mut perm_holder = Vec::<usize>::new();

    // Make new value list
    let mut new_value_list = Vec::<Vec<String>>::new();
    for _i in 0..value_string_list.len() {
      new_value_list.push(Vec::<String>::new());
      perm_holder.push(0);
    }

    // Test permutations of size ref
    let mut n = 0;
    let perm_count = perm_count(&size_ref);
    while n < perm_count {

      // Check precondition
      for state_predicate in &problem.state {
        if state_predicate.0 == precon_inner.1 {

          let mut found_count = 0;
          let mut rel_var_reached = 0;
          for perm_index in &perm_holder {
            // Does values match?
            if precon_relvar_index.contains(&found_count) {
              // The value is from a relevant variable
              if state_predicate.1[found_count] == relevant_variables[ relvar_indexes[rel_var_reached] ].2[ size_ref[found_count][*perm_index] ] {
                found_count = found_count + 1;
                rel_var_reached += 1;
              } else {
                break;
              }
            } else {
              // The value is from the forall param
              if state_predicate.1[found_count] == forall_param.1[*perm_index] {

                found_count = found_count + 1;
              } else {
                break;
              }
            }
          }

          if found_count == precon_inner.2.len() {
            // Push values to new_value_list (Precon cleared)
            cleared_precondition = false;
            break;
          }
        }
      }

      if cleared_precondition {
        let mut counter = 0;
        let mut rel_var_precons_found = 0;
        for index in &perm_holder {
          if precon_relvar_index.contains(&counter) {
            if !new_value_list[counter].contains(&relevant_variables[relvar_indexes[rel_var_precons_found]].2[*index]) {
              new_value_list[counter].push(relevant_variables[relvar_indexes[rel_var_precons_found]].2[*index].clone());
            }

            rel_var_precons_found += 1;
          }

          counter = counter + 1;
        }
      } 

      // Increase perm_holder based on list sizes
      let mut i = 0;
      for val in &perm_holder.clone() {
        if val < &(size_ref[i].len() - 1) {
          perm_holder[i] += 1; 
          break;
        } else {
          perm_holder[i] = 0;
        }
        i += 1;
      }
    
      n += 1;
    }

    // Update new_relvars
    let mut counter = 0;
    for new_values in new_value_list {
      
      if precon_relvar_index.contains(&counter) {

        if new_rel_var[relvar_indexes[counter]].2.len() != new_values.len() {
          trimmed_something = true;
        }

        

        if cleared_precondition {
          new_rel_var[relvar_indexes[counter]].2 = new_values;
        } else {
          new_rel_var[relvar_indexes[counter]].2 = vec![];
        }

        counter = counter + 1;
      }
      
    }
  }

  //println!("CLEARED PRECONDITION: {cleared_precondition}");
  (new_rel_var, trimmed_something, cleared_precondition)
}

/// Loop preconditions and determine whether or not all preconditions was cleared
fn precon_cleared( permutation: &Vec::<usize>, relevant_variables: &RelVars, precondition_list: &Vec<Precondition>, problem: &Problem) -> bool {

	let mut clear = true;
	let mut new_value_list = RelVars::new();
	let mut perm_index = 0;

	// Push the relevant values from each relevant variables based on the given permutation to new_value_list
	for val in relevant_variables {
		new_value_list.push((val.0.clone(), val.1.clone(), vec![val.2[permutation[perm_index]].clone()]));
		perm_index = perm_index + 1;
	}

	for precon in precondition_list {
		if !check_precondition(precon, &new_value_list, problem)  {
			clear = false;
		}
	}

	clear
}

fn setup_relvar_indexes(preconditions: &Vec<String>, relevant_variables: &RelVars) -> Vec<usize> {

  let mut relvar_indexes = Vec::<usize>::new();

  for precon_arg in preconditions {
		let mut relvar_index = 0;
    for relvar in relevant_variables {
      if precon_arg == &relvar.0 {
        relvar_indexes.push(relvar_index);
      }
      relvar_index = relvar_index + 1;
    }
  }

  relvar_indexes
}

fn setup_size_ref(relvar_indexes: &Vec<usize>, relevant_variables: &RelVars ) ->  Vec<Vec<usize>> {

  let mut size_ref = Vec::<Vec<usize>>::new();

  for relvar in relvar_indexes {
    let mut inner_size_ref = Vec::<usize>::new();
    for i in 0..relevant_variables[*relvar].2.len() {
      inner_size_ref.push(i);
    }
    size_ref.push(inner_size_ref);
  }

  size_ref
}

pub fn pre_permutation_cleanup ( precondition_list: &Vec<Precondition>, relevant_variables: &RelVars, state: &Vec<(String, Vec<String>)> ) -> RelVars {

  let mut mutable_relvar = relevant_variables.clone();

  //println!("STATE: {:?}", state);

  for precondition in precondition_list {

    match precondition {
      (0|1, _ ,_ , _) => {

        if precondition.2.len() == 1 {
          let mut relevant_variables_index = 0;
          for relevant_variable in relevant_variables {

            if precondition.2[0] == relevant_variable.0 {
              let mut index_data = Vec::<usize>::new();

              for state_value in state {

                if precondition.1 == state_value.0 {
                  let mut rel_var_index = 0;
                  for relvar_value in &mutable_relvar[relevant_variables_index].2 {

                    // If the value exist in state and should exist in state
                    if &state_value.1[0] == relvar_value {
                      index_data.push(rel_var_index);
                    } 

                    rel_var_index = rel_var_index + 1;
                  }	
                }
              }

              // Edit mutable relevant variables
              index_data.sort_by(|a, b| b.cmp(a));
              if precondition.0 == 0 {
                for value_to_remove in &index_data {
                  if !index_data.contains(value_to_remove) {
                    mutable_relvar[relevant_variables_index].2.remove(*value_to_remove);
                  }
                }
              } else {
                for value_to_remove in &index_data {
                  mutable_relvar[relevant_variables_index].2.remove(*value_to_remove);
                }
              }

              break;
            }

            relevant_variables_index = relevant_variables_index + 1;
          }
        }

        // println!("Big ol' check: {:?}\nBefore: {:?}\nAfter: {:?}", precondition, relevant_variables, mutable_relvar);

        // let mut line = String::new();
				// let b1 = std::io::stdin().read_line(&mut line).unwrap();

      },
      (2|3, _, _, _) => {
        //  Equals precondition (2 =, 3 not =)
        let mut found_rel_var = false;
        let mut rel_var_index = 0;

        if precondition.0 == 2 {
          
          for relvar in &mutable_relvar {
            if relvar.0 == precondition.1 && mutable_relvar[rel_var_index].2.contains(&precondition.2[0]) {
              found_rel_var = true;
              break; 
            }

            rel_var_index = rel_var_index + 1;
          }

          if found_rel_var {
            mutable_relvar[rel_var_index].2 = precondition.2.clone();
          }
        } else {

          let mut value_index = 0;

          for relvar in &mutable_relvar {
            if relvar.0 == precondition.1 && mutable_relvar[rel_var_index].2.contains(&precondition.2[0]) {

              for value in &relvar.2  {

                if value == &precondition.2[0] {
                  break;
                }

                value_index = value_index + 1;
              }

              break; 
            }
            rel_var_index = rel_var_index + 1;
          }

          mutable_relvar[rel_var_index].2.remove(value_index);
        }

        // println!("before equal {:?}\nafter equal {:?}\n", relevant_variables, mutable_relvar );

        // 						let mut line = String::new();
				// 		let b1 = std::io::stdin().read_line(&mut line).unwrap();
      },
      _ => {}
    } 

  }

  mutable_relvar
} 