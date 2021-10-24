use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use diesel::prelude::*;
use todo_app_backend::models::Todo;
use todo_app_backend::schema::todos::dsl::todos;
use todo_app_backend::*;

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    let connection = establish_connection();
    let results = todos
        .load::<Todo>(&connection)
        .expect("Error loading todos");
    web::Json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
