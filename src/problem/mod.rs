// Struct that holds the current state
#[derive(Debug, Default, Clone)]  
pub struct State {
    pub state_variables: Vec<(String, Vec<String>)>
}

#[derive(Debug, Default, Clone)]
pub struct Htn {
  pub parameters: Vec<String>, 
  pub subtasks: Vec<(String, String, Vec<String>)>
}

// The overarching struct for the entire problem
#[derive(Debug, Default, Clone)]
pub struct Problem {
  pub name: String,
  pub domain: String,
  pub objects: Vec<(String, String, Vec<String>)>,
  pub htn: Htn,
  pub state: State,
  pub goal: Option<Vec<(String, Vec<String>)>>
}

