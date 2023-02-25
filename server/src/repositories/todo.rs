use crate::entities::error::AppError;
use crate::entities::todo::Todo;
use serde_json::from_str;
use std::fs::read_to_string;

pub struct TodoRepository;

impl TodoRepository {
    pub fn get_todos(&self) -> Result<Vec<Todo>, AppError> {
        let todos = {
            let todos_string = match read_to_string(String::from("src/db/todos.json")) {
                Ok(json) => json,
                _ => String::from(""),
            };
            match from_str::<Vec<Todo>>(&todos_string) {
                Ok(todos) => Ok(todos),
                Err(err) => Err(AppError::new("Error parsing json string", err.to_string())),
            }
        };
        return todos;
    }
}
