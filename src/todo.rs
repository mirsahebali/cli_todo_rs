#![allow(dead_code)]

#[derive(Debug)]
pub struct Todo {
    pub name: String,
    pub is_done: bool,
}
impl Todo {
    pub fn new(name: &String) -> Todo {
        println!("Init new list...");
        Todo {
            name: name.to_string(),
            is_done: false,
        }
    }
}
