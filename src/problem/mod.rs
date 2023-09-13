use std::collections::HashMap;

// Struct that holds the current state
#[derive(Debug, Default)]  
pub struct State {
    state_variables: HashMap<String, Vec<String>>,
}

impl State {

}

#[derive(Debug, Default)]
pub struct Htn {
  parameters: Vec<String>, 
  subtasks: HashMap<String, Vec<String>>,
  ordering: Vec<String>,
}

// The overarching struct for the entire problem
#[derive(Debug, Default)]
pub struct Problem {
  pub name: String,
  pub domain: String,
  pub objects: HashMap<String, String>,
  pub htn: Htn,
  pub state: State,
}

