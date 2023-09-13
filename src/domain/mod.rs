use std::collections::HashMap;

pub struct Domain {
    name: String,
    tasks: Vec<Task>,
    methods: Vec<Method>,
    actions: Vec<Action>,
    types: Vec<Type>,
    predicates: Vec<Predicate>,
}

pub struct Type {
    object_types: HashMap<String, String>,
}

pub struct Predicate {
    name: String,
    args: Vec<String>,
    neg: bool,
}

pub struct Task {
    name: String,
    parameters: Vec<String>,
}

pub struct Method {
    name: String,
    parameters: Vec<String>, 
    task: Task,
    precondition: Vec<Predicate>,
    subtasks: HashMap<String, Action>, 
    ordering: Vec<String>,
}

pub struct Action {
    name: String,
    parameters: Vec<String>,
    precondition: Vec<Predicate>,
    effect: Vec<Predicate>,
}

