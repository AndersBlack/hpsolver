use std::collections::HashMap;

use super::node::SubtaskTypes;

#[derive(Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub tasks: Vec<Task>,
    pub methods: HashMap<String, Vec<Method>>,
    pub actions: Vec<Action>,
    pub types: Vec<(String, String)>,
    pub constants: Option<Vec<(String, String)>>,
    pub predicates: Vec<Predicate>,
}

#[derive(Debug, Clone, Hash)]
pub struct Predicate {
    pub name: String,
    pub args: Vec<Argument>,
}

#[derive(Debug, Clone, Hash)] 
pub struct Task {
    pub name: String,   
    pub parameters: Vec<Argument>,
    pub alias: String,
    pub id: i32
}

// make into tuple?
#[derive(Debug, Clone, Hash)] 
pub struct Argument {
    pub name: String,
    pub object_type: String
}

// NEW PRECONDITIONS: 0 = True, 1 = False, 2 = True Equal, 3 = False Equal, 4 = forall
#[derive(Debug, Clone, Hash)] 
pub struct Method {
    pub name: String,
    pub parameters: Vec<Argument>, 
    pub task: (String, Vec<String>),
    pub precondition: Option<Vec<(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)>>,
    pub subtasks: Vec<(SubtaskTypes, Vec<Argument>)>,
    pub ordering: bool,
    pub id: usize
}

#[derive(Debug, Clone, Hash)] 
pub struct Action {
    pub name: String,
    pub parameters: Vec<Argument>,
    pub precondition: Option<Vec<(i32,String,Vec<String>, Option<((String, String), Vec<(bool, String, Vec<String>)>)>)>>,
    pub effect: Option<Vec<(bool,String,Vec<String>)>>,
    pub id: i32
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Action name: {}\nParameters: {:?}\nPrecondition: {:?}\nEffect: {:?}", self.name, self.parameters, self.precondition, self.effect)
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Method name: {}\nParameters: {:?}\nTask: {:?}\nPrecondition: {:?}\nSubtasks: {:?}", self.name, self.parameters, self.task, self.precondition, self.subtasks)
    }
}

impl std::fmt::Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Domain: {}\n", self.name)?;

        write!(f, "Types: {:?}\n", self.types)?;

        for t in &self.tasks {
            println!("| Task: {:?}, Parameters: {:?}", t.name, t.parameters);
        }

        write!(f, "\nMethods:\n")?;
        for m in &self.methods {
            let mut count = 0;
            println!("| Methods for task: {}\n", m.0);
            for method in m.1 {
                println!("| #{}: {}\n", count, method);
                count += 1;
            }   
        }

        write!(f, "Actions:\n")?;
        for a in &self.actions {
            println!("| {}\n", a)
        }

        Ok(())
    }
}