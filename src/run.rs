use cli_todo::accept_user_input;

use crate::list::TaskList;
use crate::operations::get_operation;
pub fn init() -> ! {
    let mut todos = TaskList::new();
    loop {
        let input = accept_user_input();
        match get_operation() {
            Some(o) => {}
            None => {}
        }
    }
}
