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
    let mut path Vec<(Action, Vec<Object>)> = Vec::from([]);
    for task in subtasks {
        let objects: Vec<String> = task.2;
        for method in methods{
            if method.task.0 == task.0{
                let mut set_parameters: Vec<bool,String>;
                for method_parameter in method.parameters{
                    let task_parameters: Vec<String> = method.task.1;
                    let mut i: i32 = 0;
                    let mut found: bool = false;
                    while i < len(task_parameters) && !found{
                        if task_parameters[i] == method_parameter.name{
                            set_parameters.push((true,objects[i]));
                            found = true; 
                        }
                        i = i + 1;
                    }
                    if !found {
                       set_parameters.push((false,method_parameter.name));
                    }
                }
                let mut possible_permutations: Vec<Vec<Strings>> = Vec::from([]); // all possible combinations of parameters for the methods
                for object in state.objects{
                    // Find all possible permutations
                }
                resolve_method(set_parameters, state, state.objects);
            }                
        }
    }
    path
}

// need to be reworked
fn resolve_method(method: Method, task_objects: Vec<String>, state: State, state_objects: Vec<Object>){
    if precondition_met(method.precondition, set_parameters, state, state_objects){
        for action in method.subtasks{
            resolve_action(action, set_parameters, problem);
            if precondition_met(reform_precondition(action.precondition, objects, domain.tasks, action.parameters), objects, state){
                state = change_state(action, state);
                path.push(action);
            } 
        }
    } else {
        
    }    
}

fn resolve_action(){
    todo!("Resolve_action not done");
}

fn precondition_met(preconditions: Vec<(bool,String,Vec<Argument>)>, set_parameters: Vec<String>, state: State, state_objects: <Object>) -> bool{
    todo!("do precondition")
    let mut met: bool = True;
    let mut i: i32 = 0;
    let mut precondition_parameters Vec<String> = Vec::from([]);
    
    for precondition in preconditions{
        for argument in precondition.2{
            let mut found: bool = false;
            let mut i: i32 = 0;
            while !found && i < len(set_parameters){
                if set_parameters[j] == argument.name{
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

}