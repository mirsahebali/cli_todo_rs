#![allow(dead_code)]
use crate::todo::Todo;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TaskList {
    pub tasks: HashMap<u32, Todo>,
}
impl TaskList {
    pub fn new() -> TaskList {
        TaskList {
            tasks: HashMap::new(),
        }
    }
    pub fn insert(&mut self, name: String) {
        match self
            .tasks
            .insert((self.tasks.len() + 1) as u32, Todo::new(&name))
        {
            Some(c) => {
                println!("Todo already exists:{}", c.name)
            }
            None => {
                println!("Todo: {} : inserted", name)
            }
        }
    }
    pub fn toggle_done(&mut self, index: u32, completed: bool) {
        match self.tasks.get_mut(&index) {
            Some(c) => c.is_done = completed,
            None => {
                println!("Todo not found")
            }
        };
    }
    pub fn list_all(&self) {
        for (_, (key, todo)) in self.tasks.iter().enumerate() {
            println!("{}: \nname: {}\ndone? {} ", key, todo.name, todo.is_done);
        }
    }
    pub fn get(&self, index: u32) {}
    pub fn delete(&mut self, index: u32) {}
    pub fn clear(&mut self) {
        self.tasks.clear();
    }
}
