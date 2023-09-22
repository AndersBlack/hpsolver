use crate::problem::State;
use crate::problem::Htn;
use crate::problem::Problem;
use crate::problem::Object;
use crate::domain::Action;
use crate::domain::Type;
use crate::domain::Task;
use crate::domain::Domain;
use crate::domain::Predicate;
use crate::domain::Method;
use crate::domain::Argument;

pub fn create_problem () -> (Problem, Domain) {

  let type1: Type = Type {
    object_type: (String::from("key"), String::from("object")),
  };

  let type2: Type = Type {
    object_type: (String::from("box"), String::from("object")),
  };

  // ----------------------------------------------------

  let arg0: Argument = Argument {
    name: "box_arg".to_string(),
    object_type: "box".to_string(),
  };

  let arg1: Argument = Argument {
    name: "key_arg".to_string(),
    object_type: "key".to_string(),
  };

  // -----------------------------------------------------

  let predicate1: Predicate = Predicate {
    name: String::from("open"),
    args: Vec::from([arg0.clone()]),
  };
  
  let predicate2: Predicate = Predicate {
    name: String::from("closed"),
    args: Vec::from([arg0.clone()]),
  };
  
  let predicate3: Predicate = Predicate {
    name: String::from("in"),
    args: Vec::from([arg1.clone(), arg0.clone()]),
  };

  // -----------------------------------------------------


  let object1: Object = Object { object: (String::from("key0"), String::from("key")) };
  let object2: Object = Object { object: (String::from("box0"), String::from("box")) };
  let object3: Object = Object { object: (String::from("box1"), String::from("box")) };
  let object4: Object = Object { object: (String::from("box2"), String::from("box")) };


  // -----------------------------------------------------  
  
  let state_vector = Vec::from([(String::from("closed"), Vec::from([String::from("box0")])), (String::from("closed"), Vec::from([String::from("box0")])), (String::from("closed"), Vec::from([String::from("box0")]))]);
  
  let init_state: State = State {
    state_variables: state_vector,
  };

  // -----------------------------------------------------
  
  let task: Task = Task {
    name: "opened_box".to_string(),
    alias: "task0".to_string(),
    parameters: Vec::from([arg1.clone(), arg0.clone()]),
  };

  // -----------------------------------------------------

  let htn: Htn = Htn {
    parameters: Vec::from([]),
    subtasks: Vec::from([("opened_box".to_string(), "task0".to_string(), Vec::from(["box0".to_string()])),("opened_box".to_string(), "task0".to_string(), Vec::from(["box1".to_string()])),("opened_box".to_string(), "task0".to_string(), Vec::from(["box2".to_string()]))]),
  };

  // -----------------------------------------------------

  let problem: Problem = Problem {
    name: "box_opener".to_string(),
    domain: "box_opener_domain".to_string(),
    objects: Vec::from([object1, object2.clone(), object3.clone(), object4.clone()]),
    htn: htn,
    state: init_state,
  };

  // ---------------------------------------------------

  let action1 = Action {
    name: String::from("insert_key"),
    parameters: Vec::from([arg1.clone(), arg0.clone()]),
    precondition: Vec::from([(false, predicate3.clone(), Vec::from([arg1.clone(), arg0.clone()]))]),
    effect: Vec::from([(true, predicate3.clone(),Vec::from([arg1.clone(),arg0.clone()]))])  
  };

  let action2 = Action {
    name: String::from("open_box"),
    parameters: Vec::from([arg1.clone(), arg0.clone()]),
    precondition: Vec::from([(true, predicate3.clone(), Vec::from([arg1.clone(), arg0.clone()]))]),
    effect: Vec::from([(true, predicate1.clone(),Vec::from([arg0.clone()])),(false, predicate2.clone(),Vec::from([arg0.clone()])),(false, predicate3.clone(),Vec::from([arg1.clone(),arg0.clone()]))])  
  };




  // ----------------------------------------------------

  let method: Method = Method {
    name: "opened_box_method".to_string(),
    parameters: Vec::from([arg1.clone(), arg0.clone()]), 
    task: ("box_opener".to_string(), Vec::from([arg1.name.clone(), arg0.name.clone()])),
    precondition: Vec::from([(false,String::from("open"), Vec::from([arg0.clone()])),(true,String::from("closed"), Vec::from([arg0.clone()]))]),
    subtasks: Vec::from([(action1.clone(), Vec::from([arg1.clone(), arg0.clone()])), (action2.clone(), Vec::from([arg1.clone(), arg0.clone()]))]), 
    contraints: Vec::<(String, String)>::new()
  };

  // ---------------------------------------------------

  let domain = Domain {
    name: "box_opener_domain".to_string(),
    tasks: Vec::from([task.clone()]),
    methods: Vec::from([method]),
    actions: Vec::from([action1.clone(), action2.clone()]),
    types: Vec::from([type1, type2]),
    predicates: Vec::from([predicate1.clone(), predicate2.clone(), predicate3.clone()]),
  };

  (problem, domain)
}