#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::NewTodo;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_todo(conn: &SqliteConnection, content: &str) -> usize {
    use schema::todos;

    let new_todo = NewTodo { content };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(conn)
        .expect("Error saving new post")
}
