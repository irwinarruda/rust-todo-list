use crate::entities::error::AppError;
use crate::entities::success::AppSuccess;
use crate::entities::todo::CreateTodoDTO;
use crate::entities::todo::EditTodoDTO;
use crate::services::create_todo::CreateTodoService;
use crate::services::delete_todo::DeleteTodoService;
use crate::services::edit_todo::EditTodoService;
use crate::services::get_todo_by_id::GetTodoByIdService;
use crate::services::get_todos::GetTodosService;
use actix_web::{delete, get, post, put, web, Responder};
use uuid::Uuid;

#[get("/todos")]
pub async fn get_todos() -> impl Responder {
    return match GetTodosService::execute() {
        Ok(todos) => AppSuccess::ok(&todos),
        Err(err) => AppError::handle(&err),
    };
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(id: web::Path<Uuid>) -> impl Responder {
    return match GetTodoByIdService::execute(*id) {
        Ok(todo) => AppSuccess::ok(&todo),
        Err(err) => AppError::handle(&err),
    };
}

#[post("/todos")]
pub async fn create_todo(req_body: web::Json<CreateTodoDTO>) -> impl Responder {
    return match CreateTodoService::execute(req_body.0) {
        Ok(_) => AppSuccess::no_content(),
        Err(err) => AppError::handle(&err),
    };
}

#[put("/todos")]
pub async fn edit_todo(req_body: web::Json<EditTodoDTO>) -> impl Responder {
    return match EditTodoService::execute(req_body.0) {
        Ok(_) => AppSuccess::no_content(),
        Err(err) => AppError::handle(&err),
    };
}

#[delete("/todos/{id}")]
pub async fn delete_todo(id: web::Path<Uuid>) -> impl Responder {
    return match DeleteTodoService::execute(*id) {
        Ok(_) => AppSuccess::no_content(),
        Err(err) => AppError::handle(&err),
    };
}
