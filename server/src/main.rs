mod controllers;
mod entities;
mod repositories;
mod services;
mod utils;

use actix_web::{App, HttpServer};
use controllers::todo::{create_todo, edit_todo, get_todos};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return HttpServer::new(|| {
        return App::new()
            .service(get_todos)
            .service(create_todo)
            .service(edit_todo);
    })
    .bind(("localhost", 8080))?
    .run()
    .await;
}
