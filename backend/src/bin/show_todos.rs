use diesel::prelude::*;
use todo_app_backend::models::Todo;
use todo_app_backend::schema::todos::dsl::todos;
use todo_app_backend::*;

fn main() {
    let connection = establish_connection();
    let results = todos
        .load::<Todo>(&connection)
        .expect("Error loading todos");

    println!("Displaying {} todos", results.len());
    for todo in results {
        println!("{}", todo.content);
    }
}
