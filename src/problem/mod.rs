use crate::domain::Predicate;
use crate::domain::Task;
use std::collections::HashMap;


// Struct that holds the current state
#[derive(Debug, Default)]  
pub struct State {
    pub state_variables: Vec<(Predicate, Vec<Object>)>,
}

#[derive(Debug, Default)]
pub struct Htn {
  pub parameters: Vec<String>, 
  pub subtasks: Vec<(Task, Vec<Object>)>,
}

#[derive(Debug, Default, Clone)]
pub struct Object {
  pub object: (String, String),
}

// The overarching struct for the entire problem
#[derive(Debug, Default)]
pub struct Problem {
  pub name: String,
  pub domain: String,
  pub objects: Vec<Object>,
  pub htn: Htn,
  pub state: State,
}

