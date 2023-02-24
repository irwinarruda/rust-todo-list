mod controllers;
mod entities;
mod repositories;

use actix_web::{App, HttpServer};
use controllers::todo::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return HttpServer::new(|| {
        return App::new().service(get_todos);
    })
    .bind(("localhost", 8080))?
    .run()
    .await;
}
