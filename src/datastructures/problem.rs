#[derive(Debug, Default, Clone, Hash)]
pub struct Htn {
  pub parameters: Vec<(String, String)>, 
  pub subtasks: Vec<(String, String, Vec<String>)>
}

// The overarching struct for the entire problem
#[derive(Debug, Default, Clone, Hash)]
pub struct Problem {
  pub name: String,
  pub domain: String,
  pub objects: Vec<(String, String, Vec<String>)>,
  pub htn: Htn,
  pub state: Vec<(String, Vec<String>)>,
  pub goal: Option<Vec<(String, Vec<String>)>>
}


