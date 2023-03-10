mod controllers;
mod entities;
mod repositories;
mod services;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use controllers::todo::{create_todo, delete_todo, edit_todo, get_todo_by_id, get_todos};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        return App::new()
            .wrap(cors)
            .service(get_todos)
            .service(create_todo)
            .service(edit_todo)
            .service(delete_todo)
            .service(get_todo_by_id);
    })
    .bind(("localhost", 8080))?
    .run()
    .await;
}
