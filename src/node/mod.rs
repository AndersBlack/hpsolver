use crate::problem::Problem;
use crate::domain::*;

#[derive(Debug, Clone)] 
pub enum SubtaskTypes<'a> {
  HtnTask((String, String, Vec<String>)),
  Task(Task),
  Method(&'a Method),
  Action(Action)
} 

#[derive(Debug, Clone)] 
pub struct Node<'a> {
  pub problem: Problem,
  pub subtask_queue: Vec<SubtaskTypes<'a>>,
  /// Tuple of (name, object_type & possible values)
  pub relevant_parameters: Vec<(String, String, Vec<String>)>
}
