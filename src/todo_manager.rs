use std::path::Path;

use crate::todo::Todo;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TodoManager {
    todos: Vec<Todo>,
}

impl TodoManager {
    pub fn new(todos: Vec<Todo>) -> TodoManager {
        TodoManager { todos }
    }

    pub fn get_todos(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn add_todo(&mut self, name: &str) -> &Todo {
        let id = self.get_new_id();

        self.todos.push(Todo::new(name, id));

        &self.todos[match self.get_index(id) {
            Ok(index) => index,
            Err(_) => 0,
        }]
    }

    pub fn delete_todo(&mut self, id: usize) -> Result<(), ()> {
        match self.get_index(id) {
            Ok(index) => {
                self.todos.remove(index);
                Ok(())
            }
            Err(_) => Err(()),
        }
    }

    pub fn mark_todo(&mut self, id: usize) -> Result<(), ()> {
        match self.get_index(id) {
            Ok(index) => {
                self.todos[index].set_completed(true);
                Ok(())
            }
            Err(_) => Err(()),
        }
    }

    pub fn save(&self, path: &Path) {
        std::fs::write(path, serde_json::to_string_pretty(&self).unwrap()).expect("Save failed");
    }

    fn get_index(&self, id: usize) -> Result<usize, ()> {
        let mut todo_index: isize = -1;

        for (index, todo) in self.todos.iter().enumerate() {
            if todo.get_id() == id {
                todo_index = index as isize;
                break;
            }
        }

        if todo_index >= 0 {
            Ok(todo_index as usize)
        } else {
            Err(())
        }
    }

    fn get_new_id(&self) -> usize {
        let mut id = 0;

        'outer: loop {
            'inner: loop {
                for todo in self.todos.iter() {
                    if todo.get_id() == id {
                        break 'inner;
                    }
                }

                break 'outer;
            }

            id += 1;
        }

        id
    }
}
