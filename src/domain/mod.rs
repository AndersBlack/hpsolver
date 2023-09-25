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
    pub object_type: String
}

#[derive(Debug, Clone)] 
pub struct Method {
    pub name: String,
    pub parameters: Vec<Argument>, 
    pub task: (String, Vec<String>),
    pub precondition: Option<Vec<(bool,String,Vec<String>)>>,
    pub subtasks: Option<Vec<(String, String, Vec<String>)>>,
    pub contraints: Option<Vec<(bool, String, String)>>
}

#[derive(Debug, Clone)] 
pub struct Action {
    pub name: String,
    pub parameters: Vec<Argument>,
    pub precondition: Option<Vec<(bool,String,Vec<String>)>>,
    pub effect: Vec<(bool,String,Vec<String>)>,
}

