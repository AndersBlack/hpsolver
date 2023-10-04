use crate::problem::State;
use crate::problem::Htn;
use crate::problem::Problem;
use crate::problem::Object;
use crate::domain::Action;
use crate::domain::Type;
use crate::domain::Task;
use crate::domain::Domain;
use crate::domain::Predicate;
use crate::domain::Method;
use crate::domain::Argument;

pub fn depth_first(problem: Problem, domain: Domain) {

    // Loop every task in the htn
    for task in &problem.htn.subtasks {
				println!("------------------------------------- {} ------------------------------------- \n", task.1);
				execute_subtasks( task, &problem, &domain, &None );
    }
}

fn execute_subtasks(task: &(String, String, Vec<String>),  problem: &Problem, domain: &Domain, param_list: &Option<Vec<(String, Vec<String>, String)>>){
	// Loop through methods and locate task from htn
		//println!("looping method: {} against {}",method.task.0 , task.0);

		match param_list {
			Some(inner) => {
				for method in &domain.methods{
					if task.0 == method.task.0 {
						resolve_method(param_list, problem, domain, method);
					}
				}
				
				for action in &domain.actions{
					if task.0 == action.name{
						resolve_action(param_list, problem, domain, action)
					}
				}
			},
			None => {
				for method in &domain.methods {	
					if method.task.0 == task.0 {
						// Found task from htn
						prep_method(method, problem, domain, &task.2); 
					}		
				}
			}
		}
}


fn prep_method( method: &Method, problem: &Problem, domain: &Domain, objects: &Vec<String>) {
	//println!("Found the method!");
	let mut set_parameters = Vec::<(bool, Argument)>::new();

	// Loop through method parameter and check if the htn task provided it
	for method_parameter in &method.parameters{
		let task_parameters: Vec<String> = method.task.1.clone();
		let mut i = 0;
		let mut found: bool = false;

		while i < task_parameters.len() && !found {
			//println!("i: {} < param_len: {}",i ,task_parameters.len());

			// The htn task provided this parameter
			if task_parameters[i] == method_parameter.name{
				set_parameters.push(( true, method_parameter.clone()));
				found = true; 
			}
			
			i = i + 1;
		}

		if !found {
			// The htn task did not provide this parameter
			set_parameters.push(( false, method_parameter.clone()));
		}
	}


	let mut param_list = Vec::<(String, Vec<String>, String)>::new(); // all possible combinations of parameters for the methods
	let mut int = 0;
	// Look for unprovided parameters
	for parameter in set_parameters {
		//println!("Task: {}, Task args: {:?}, Set param:{:?}", task.1, objects, parameter);

		if parameter.0 == false {

			let mut unset_param_object_list = Vec::<String>::new();
			let param_name = parameter.1.name.clone();

			for object in &problem.objects {
					if object.object.1 == parameter.1.object_type {
							unset_param_object_list.push(object.object.0.clone());
					}
			}

			param_list.push((param_name, unset_param_object_list, parameter.1.object_type));
		} else {
			let mut set_param_object_list = Vec::<String>::new();
			
			set_param_object_list.push(objects[int].clone());

			for object in &problem.objects {
				if objects[int] == object.object.0 {
					param_list.push((method.task.1[int].clone(), set_param_object_list.clone(), object.object.1.clone()));
				}
			} 
			
			int = int + 1;
		}
	}

	//println!("PARAM LIST - Method: {}\nParams: {:?} \n", method.name, param_list);

	resolve_method(&Some(param_list), problem, domain, method);
} 

// Needs return type
fn resolve_method(param_list: &Option<Vec<(String, Vec<String>, String)>>, problem: &Problem, domain: &Domain, method: &Method){
  
	println!("{}\n", method);

	match &method.subtasks {
		Some(subtasks) => {
			for subtask in subtasks {
				//println!("GIMME UBS:{:?}",subtask);
				execute_subtasks(subtask, problem, domain, param_list);
			}
		},
		None => {
			//Check preconditions
		}
	}

	// // Check preconditions
	// match &method.precondition {
	// 	Some(inner) => {
  //     //prep precondition parameters
	// 		for precon in inner {

  //       let mut precon_param_list = Vec::<String>::new();
  //       for param in &precon.2{
  //         for object in &param_list{
	// 					// DET HER TROR JEG IKKE VIRKER 
						
  //           if &object.1 == param {
  //             precon_param_list.push(object.0.clone());
  //           }
  //         }
  //       }

  //       //check precondition in state
  //       for predicate in &problem.state.state_variables{
  //         if precon.0{
  //           if predicate.0 == precon.1{
  //             let mut params_hold = true;
  //             let mut i = 0;

  //             while i < predicate.1.len() && params_hold{
  //               if predicate.1[i] != precon_param_list[i]{
  //                 params_hold = false;
  //               } 
  //             }
  //             if (params_hold){
  //               // precondition met
  //             }
  //             else{
  //               // precondition not met
  //             }
              
  //           }
  //           else{
  //             if predicate.0 == precon.1 {
  //               let mut params_hold = true;
  //               let mut i = 0;
  
  //               while i < predicate.1.len() && params_hold{
  //                 if predicate.1[i] != precon_param_list[i]{
  //                   params_hold = false;
  //                 } 
  //               }
  //               if (params_hold){
  //                 // precondition not met
  //               }
  //               else{
  //                 // precondition met
  //               }
  //             }
  //           } 
  //         }
  //       }


  //     }
      
	// 	},
	// 	None => {
	// 		// Do nothing
	// 	}   
	// } 

	// match method.constraints {
	// 	Some(inner) => {},
	// 	None => {
	// 		// Do nothing
	// 	}   
	// }
}

fn prep_action( ) {

} 

fn resolve_action(param_list: &Option<Vec<(String, Vec<String>, String)>>, problem: &Problem, domain: &Domain, action: &Action){

	//println!("PARAM LIST - Action: {}\nParams: {:?} \n", action.name, param_list);

	println!("{}\n", action);

	for precon in action.precondition.iter().flatten() {
		//println!("Precon: {:?}", precon);
	}
}

fn check_precondition(precondition: (bool,String,Vec<String>), state: &State, ) {

}


