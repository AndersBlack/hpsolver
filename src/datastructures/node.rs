use crate::datastructures::{domain::*, problem::*};
use std::collections::{HashSet, HashMap};

type RelVars = Vec<(String, String, Vec<String>)>;
type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);
type Called = (Vec<bool>, Vec<(Method, RelVars, Vec<Precondition>)>, Vec<usize>);

#[derive(Debug, Clone, Hash)] 
pub enum SubtaskTypes {
  HtnTask((String, String, Vec<String>)),
  Task(Task),
  Method(Method),
  Action(Action)
} 

#[derive(Debug, Clone)] 
pub struct Node {
  pub problem: Problem,
  /// Tuple of (name, object_type & possible values)
  pub subtask_queue: Vec<(SubtaskTypes, Vec<(String, String, Vec<String>)>)>,
  pub called: (Vec<bool>, Vec<(Method, Vec<(String, String, Vec<String>)>, Vec<Precondition>)>, Vec<usize>),
  pub applied_functions: ((String, Vec<usize>), Vec<(SubtaskTypes, usize, Vec<usize>, RelVars)>),
  pub hash_table: HashSet<u64>,
  pub passing_preconditions: Vec<Precondition>,
  pub goal_functions: Vec<String>
}


#[derive(Debug, Clone)] 
pub struct PartialNode {
  pub problem: Problem,
  pub subtask_queue: Vec< (SubtaskTypes, RelVars, Called, Vec<Precondition>) >,
  pub applied_functions: ((String, Vec<usize>), Vec<(SubtaskTypes, usize, Vec<usize>, RelVars)>),
  pub hash_table: HashSet<u64>,
  pub hash_counter: HashMap<u64, usize>,
  pub goal_functions: Vec<String>
}


