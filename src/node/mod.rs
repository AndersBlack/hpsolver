use crate::problem::Problem;
use crate::domain::*;

#[derive(Debug, Clone)] 
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
  pub called: (Vec<bool>, Vec<(Method, Vec<(String, String, Vec<String>)>)>, Vec<usize>),
  pub applied_action_list: Vec<(String, Vec<String>)>
}