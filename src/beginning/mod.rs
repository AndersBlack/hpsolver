use crate::datastructures::{domain::*, problem::*};

pub fn create_problem () -> (Problem, Domain) {

  let type1 = (String::from("key"), String::from("object"));

  let type2 = (String::from("box"), String::from("object"));

  // ----------------------------------------------------

  let arg0: Argument = Argument {
    name: "?box_arg".to_string(),
    object_type: "box".to_string(),
  };

  let arg1: Argument = Argument {
    name: "?key_arg".to_string(),
    object_type: "key".to_string(),
  };

  // -----------------------------------------------------

  let predicate1: Predicate = Predicate {
    name: String::from("open"),
    args: Vec::from([arg0.clone()]),
  };
  
  // let predicate2: Predicate = Predicate {
  //   name: String::from("closed"),
  //   args: Vec::from([arg0.clone()]),
  // };
  
  let predicate3: Predicate = Predicate {
    name: String::from("in"),
    args: Vec::from([arg1.clone(), arg0.clone()]),
  };

  // -----------------------------------------------------

  let object0: (String, String, Vec<String>) = (String::from("key0"), String::from("key"), Vec::<String>::new());
  let object1: (String, String, Vec<String>) = (String::from("key1"), String::from("key"), Vec::<String>::new());
  let object2: (String, String, Vec<String>) = (String::from("box0"), String::from("box"), Vec::<String>::new());
  let object3: (String, String, Vec<String>) = (String::from("box1"), String::from("box"), Vec::<String>::new());
  let object4: (String, String, Vec<String>) = (String::from("box2"), String::from("box"), Vec::<String>::new());

  // -----------------------------------------------------  
  
  let state_vector = Vec::from([]);
  
  let init_state = state_vector;

  // -----------------------------------------------------
  
  let task: Task = Task {
    name: "opened_box".to_string(),
    alias: "task0".to_string(),
    parameters: Vec::from([arg0.clone()]),
    id: 1000
  };

  // -----------------------------------------------------

  let htn: Htn = Htn {
    parameters: Vec::from([]),
    subtasks: Vec::from([("opened_box".to_string(), "task0".to_string(), Vec::from(["box0".to_string()]), false),("opened_box".to_string(), "task0".to_string(), Vec::from(["box1".to_string()]), false),("opened_box".to_string(), "task0".to_string(), Vec::from(["box2".to_string()]), false)]),
  };

  // -----------------------------------------------------

  let problem: Problem = Problem {
    name: "box_opener".to_string(),
    domain: "box_opener_domain".to_string(),
    objects: Vec::from([object0, object1, object2, object3, object4]),
    htn: htn,
    state: init_state,
    goal: None
  };

  // ---------------------------------------------------

  let action1 = Action {
    name: String::from("insert_key"),
    parameters: Vec::from([arg1.clone(), arg0.clone()]),
    precondition: Some(
      Vec::from([
        (1, String::from("in"), Vec::from(["?key_arg".to_string(), "?box_arg".to_string()]), None),
        (1, String::from("open"), Vec::from(["?box_arg".to_string()]), None)
      ])
    ),
    effect: Some(Vec::from([(true,String::from("in"),Vec::from(["?key_arg".to_string(),"?box_arg".to_string()]))])),
    id: 1
  };

  let action2 = Action {
    name: String::from("open_box"),
    parameters: Vec::from([arg1.clone(), arg0.clone()]),
    precondition: Some(
      Vec::from([
        (0, String::from("in"), Vec::from(["?key_arg".to_string(), "?box_arg".to_string()]), None),
        (1, String::from("open"), Vec::from(["?box_arg".to_string()]), None)
      ])
      ),
    effect: Some(Vec::from([(true, String::from("open"),Vec::from(["?box_arg".to_string()])),(false,String::from("in"),Vec::from(["?key_arg".to_string(),"?box_arg".to_string()]))])),
    id: 2
  };




  // ----------------------------------------------------

  let method: Method = Method {
    name: "opened_box_method".to_string(),
    parameters: Vec::from([arg1.clone(), arg0.clone()]), 
    task: ("opened_box".to_string(), Vec::from([arg0.name.clone()])),
    precondition: Some(Vec::from([(1,String::from("open"), Vec::from(["?box_arg".to_string()]), None)])),
    subtasks: Vec::from([("insert_key".to_string(), "task0".to_string(), Vec::from(["?key_arg".to_string(), "?box_arg".to_string()]), false), ("open_box".to_string(), "task0".to_string(), Vec::from(["?key_arg".to_string(), "?box_arg".to_string()]), false)]), 
    constraints: None,
    id: 10
  };

  // ---------------------------------------------------

  let domain = Domain {
    name: "box_opener_domain".to_string(),
    tasks: Vec::from([task.clone()]),
    methods: Vec::from([method]),
    actions: Vec::from([action1.clone(), action2.clone()]),
    types: Vec::from([type1, type2]),
    constants: None,
    predicates: Vec::from([predicate1.clone(), predicate3.clone()]),
  };

  (problem, domain)
}