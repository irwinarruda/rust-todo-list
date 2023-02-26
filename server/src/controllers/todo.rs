use crate::entities::success::AppSuccess;
use crate::entities::todo::EditTodoDTO;
use crate::entities::{error::AppError, todo::CreateTodoDTO};
use crate::services::create_todo::CreateTodoService;
use crate::services::edit_todo::EditTodoService;
use crate::services::get_todos::GetTodosService;
use actix_web::{get, post, put, web, Responder};

#[get("/todos")]
pub async fn get_todos() -> impl Responder {
    return match GetTodosService::execute() {
        Ok(todos) => AppSuccess::ok(&todos),
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
