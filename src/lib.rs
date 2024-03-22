use std::io;

pub fn accept_user_input() -> Result<String, ()> {
    let mut input = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut input) {
        Ok(..) => Ok(input),
        Err(e) => {
            eprintln!("Error occoured accepting from STDIN: {e}");
            Err(())
        }
    }
}
