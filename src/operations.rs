#[derive(Debug)]
pub enum Operations<'a> {
    Insert(&'a str),
    ListAll,
    Delete,
    Clear,
    Get(&'a str),
}

use cli_todo::accept_user_input;
use Operations::*;
pub fn get_operation() -> Option<Operations> {
    println!("Enter the operation you want to perform");
    println!("i -> Insert \nl -> List all \nt -> Toggle Todo\nd -> Delete Todo\ng -> Get a todo");
    let input = match accept_user_input() {
        Ok(o) => o,
        Err(err) => {
            eprintln!("ERROR: accepting user input ");
            "".to_string()
        }
    };
    println!("My input: {input}");
    match &input.as_str() {
        &"i" => {
            println!("Insert");
            Some(Insert(&input))
        }
        &"g" => Some(Get),
        &"l" => Some(ListAll),
        &"c" => Some(Clear),
        &"d" => Some(Delete),
        &_ => {
            eprintln!("Enter correct input from the given options");
            None
        }
    }
}
