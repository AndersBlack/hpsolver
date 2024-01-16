use crate::datastructures::{domain::*, problem::*};
use std::collections::HashSet;

type RelVars = Vec<(String, String, Vec<String>)>;
type Precondition = (i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>);

#[derive(Debug, Clone, Hash)] 
pub enum SubtaskTypes {
  HtnTask((String, String, Vec<String>, bool)),
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
  /// Root tuple, List of applied function (Function, id, called id list, relevant variables)
  pub applied_functions: ((String, Vec<usize>), Vec<(SubtaskTypes, usize, Vec<usize>, RelVars)>),
  pub hash_table: HashSet<u64>,
  pub passing_preconditions: Vec<Precondition>
}