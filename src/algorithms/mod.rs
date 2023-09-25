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
    let subtasks: Vec<(String, String, Vec<String>)> = problem.htn.subtasks;
    let mut state: State = problem.state;
    let methods: Vec<Method> = domain.methods; 
    let actions: Vec<Action> = domain.actions;
    let mut path: Vec<(Action, Vec<Object>)> = Vec::<(Action, Vec<Object>)>::new();

    // Loop every task in the htn
    for task in subtasks {
        let objects: Vec<String> = task.2; // task argument(s)

        // Loop through methods and locate task from htn
        for method in &methods {
            //println!("looping method: {} against {}",method.task.0 , task.0);

            // Found task from htn
            if method.task.0 == task.0 {
                //println!("Found the method!");
                let mut set_parameters = Vec::<(bool,Argument)>::new();

                // Loop through method parameter and check if the htn task provided it
                for method_parameter in &method.parameters{
                    let task_parameters: Vec<String> = method.task.1.clone();
                    let mut i = 0;
                    let mut found: bool = false;

                    while i < task_parameters.len() && !found {
                        //println!("i: {} < param_len: {}",i ,task_parameters.len());

                        // The htn task provided this parameter
                        if task_parameters[i] == method_parameter.name{
                            set_parameters.push(( true, method_parameter.clone() ));
                            found = true; 
                        }
                        
                        i = i + 1;
                    }

                    if !found {
                        // The htn task did not provide this parameter
                        set_parameters.push((false,method_parameter.clone()));
                    }
                }


                let mut possible_permutations: Vec<Vec<String>> = Vec::from([]); // all possible combinations of parameters for the methods
                
                // Look for unprovided parameters
                for parameter in set_parameters {
                    if parameter.0 == false {
                        for object in &problem.objects {
                            if object.object.1 == parameter.1.object_type {
                                println!("Obj: {}\n", object.object.1);
                            }
                        }
                    }
                }

                //resolve_method(set_parameters, state, state.state_variables);
            }     

        }
    }
}

// need to be reworked
fn resolve_method(method: Method, task_objects: Vec<String>, state: State, state_objects: Vec<Object>){

    todo!("implement resolve method!")

    // if precondition_met(method.precondition, set_parameters, state, state_objects){
    //     for action in method.subtasks{
    //         resolve_action(action, set_parameters, problem);
    //         if precondition_met(reform_precondition(action.precondition, objects, domain.tasks, action.parameters), objects, state){
    //             state = change_state(action, state);
    //             path.push(action);
    //         } 
    //     }
    // } else {
        
    // }    
}

fn resolve_action(){
    todo!("Resolve_action not done");
}

fn precondition_met(preconditions: Vec<(bool,String,Vec<Argument>)>, set_parameters: Vec<String>, state: State, state_objects: Vec<Object>) -> bool{
    todo!("do precondition");
    let mut met: bool = true;
    let mut i = 0;
    let mut precondition_parameters: Vec<String> = Vec::from([]);
    
    for precondition in preconditions{
        for argument in precondition.2{
            let mut found: bool = false;
            let mut i = 0;
            while !found && i < set_parameters.len(){
                if set_parameters[i] == argument.name{
                    precondition_parameters.push(argument.name);
                    found = true;
                } else {
                    precondition_parameters.push(argument.name);
                    i = i + 1;
                }
            }
        }
    }

    //let mut i: i32 = 0;
    //while met && i < len(preconditions){

    //    i = i + 1;
    //}
    
    met
}

fn change_state(){
    todo!("Implement change state")
}