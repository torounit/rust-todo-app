use std::io::{stdin, Read};
use todo_app_backend::*;

fn main() {
    let connection = establish_connection();
    println!("add todo");
    let mut content = String::new();
    stdin().read_to_string(&mut content).unwrap();
    let _ = create_todo(&connection, &content);
    println!("\nSaved {}", content);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";
