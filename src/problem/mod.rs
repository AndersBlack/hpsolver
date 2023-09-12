use std::collections::HashMap;

// Struct that holds the current state
#[derive(Debug)]  
pub struct State {
    state_variables: HashMap<String, Vec<String>>,
}

impl State {

}

pub struct Htn {
  parameters: Vec<String>, 
  subtasks: HashMap<String, Vec<String>>,
  ordering: Vec<String>,
}


// The overarching module for the entire problem
pub struct Problem {
  name: String,
  domain: String,
  objects: HashMap<String, String>,
  htn: Htn,
  state: State,
}

