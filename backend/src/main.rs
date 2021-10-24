use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{get, http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
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
    //    web::Json(results)
    let body = serde_json::to_string(&results).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| true)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);
        App::new().wrap(cors).service(index)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
