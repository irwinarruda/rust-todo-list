use crate::entities::error::AppError;
use crate::entities::success::AppSuccess;
use crate::repositories::todo::TodoRepository;
use actix_web::{get, Responder};

#[get("/todos")]
pub async fn get_todos() -> impl Responder {
    let todos_repository = TodoRepository::new();
    return match todos_repository.get_todos() {
        Ok(todos) => AppSuccess::ok(&todos),
        Err(err) => AppError::handle(&err),
    };
}
