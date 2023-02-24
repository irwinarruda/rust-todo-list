use actix_web::{get, http::header::ContentType, HttpResponse, Responder};

use crate::repositories::todo::*;

#[get("/todos")]
pub async fn get_todos() -> impl Responder {
    let todos_repository = TodoRepository {};
    let todos = todos_repository
        .get_todos()
        .map_err(|_| HttpResponse::InternalServerError().body(""))
        .unwrap();
    return HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&todos).unwrap());
}
