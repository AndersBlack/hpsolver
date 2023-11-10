use crate::datastructures::{domain::*, problem::*};
use std::collections::HashSet;

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
  pub called: (Vec<bool>, Vec<(Method, Vec<(String, String, Vec<String>)>)>, Vec<usize>),
  pub applied_action_list: Vec<(String, Vec<String>)>,
  pub hash_table: HashSet<u64>
}