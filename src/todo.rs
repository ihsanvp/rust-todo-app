use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Todo {
    id: usize,
    name: String,
    completed: bool,
}

impl Todo {
    pub fn new(name: &str, id: usize) -> Self {
        Todo {
            id,
            name: String::from(name),
            completed: false,
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn set_completed(&mut self, value: bool) {
        self.completed = value;
    }
}
