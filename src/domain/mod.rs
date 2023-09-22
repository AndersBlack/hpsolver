use crate::problem::Object;

#[derive(Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub tasks: Vec<Task>,
    pub methods: Vec<Method>,
    pub actions: Vec<Action>,
    pub types: Vec<Type>,
    pub predicates: Vec<Predicate>,
}

#[derive(Debug, Clone)] 
pub struct Type {
    pub object_type: (String, String),
}

#[derive(Debug, Clone)]
pub struct Predicate {
    pub name: String,
    pub args: Vec<Argument>,
}

#[derive(Debug, Clone)] 
pub struct Task {
    pub name: String,
    pub parameters: Vec<Argument>,
    pub alias: String,
}

#[derive(Debug, Clone)] 
pub struct Argument {
    pub name: String,
    pub object_type: Type
}

#[derive(Debug, Clone)] 
pub struct Method {
    pub name: String,
    pub parameters: Vec<Argument>, 
    pub task: Task,
    pub precondition: Vec<(bool,Predicate,Vec<Argument>)>,
    pub subtasks: Vec<(Action, Vec<Argument>)>,
}

#[derive(Debug, Clone)] 
pub struct Action {
    pub name: String,
    pub parameters: Vec<Argument>,
    pub precondition: Vec<(bool,Predicate,Vec<Argument>)>,
    pub effect: Vec<(bool,Predicate,Vec<Argument>)>,
}

