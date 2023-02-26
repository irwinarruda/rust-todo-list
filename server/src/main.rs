mod controllers;
mod entities;
mod repositories;
mod services;
mod utils;

use actix_web::{App, HttpServer};
use controllers::todo::{create_todo, delete_todo, edit_todo, get_todo_by_id, get_todos};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return HttpServer::new(|| {
        return App::new()
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
