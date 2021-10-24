use super::schema::todos;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Todo {
    pub id: i32,
    pub content: String,
}
#[derive(Debug, Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a> {
    pub content: &'a str,
}
