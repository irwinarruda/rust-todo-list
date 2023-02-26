use crate::entities::error::AppError;
use crate::entities::todo::{CreateTodoDTO, EditTodoDTO, Todo};
use crate::utils::file::File;
use crate::utils::json::Json;
use chrono::Utc;
use uuid::Uuid;

pub struct TodoRepository;

impl TodoRepository {
    pub fn new() -> TodoRepository {
        return Self {};
    }
    pub fn get_todos(&self) -> Result<Vec<Todo>, AppError> {
        let todos = {
            let todos_string = File::read_to_string(String::from("src/db/todos.json"))?;
            Json::from_str::<Vec<Todo>>(&todos_string)
        }?;
        return Ok(todos.iter().filter(|t| !t.is_deleted).cloned().collect());
    }
    pub fn find_by_id(&self, id: &Uuid) -> Result<Todo, AppError> {
        let current_todos = self.get_todos()?;
        if let Some(todo) = current_todos.iter().find(|todo| todo.id.eq(&id)) {
            if !todo.is_deleted {
                return Ok(todo.clone());
            }
        }
        return Err(AppError::new(
            404,
            "Todo with the specified id not found",
            String::from(""),
        ));
    }
    pub fn create_todo(&self, todo: &CreateTodoDTO) -> Result<(), AppError> {
        let mut current_todos = self.get_todos()?;
        let new_todo = Todo {
            id: Uuid::new_v4(),
            title: todo.title.clone(),
            description: todo.description.clone(),
            is_completed: todo.is_completed,
            deadline: todo.deadline.clone(),
            is_deleted: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        current_todos.push(new_todo);
        let new_todos = Json::to_string_pretty(&current_todos)?;
        File::write(String::from("src/db/todos.json"), new_todos)?;
        return Ok(());
    }
    pub fn edit_todo(&self, todo: &EditTodoDTO) -> Result<(), AppError> {
        let mut current_todos = self.get_todos()?;
        if let Some(index) = current_todos.iter().position(|t| t.id.eq(&todo.id)) {
            current_todos[index].description = todo.description.clone();
            current_todos[index].is_completed = todo.is_completed;
            current_todos[index].updated_at = Utc::now();
        }
        let new_todos = Json::to_string_pretty(&current_todos)?;
        File::write(String::from("src/db/todos.json"), new_todos)?;
        return Ok(());
    }
    pub fn delete_todo(&self, id: &Uuid) -> Result<(), AppError> {
        let mut current_todos = self.get_todos()?;
        if let Some(index) = current_todos.iter().position(|t| t.id.eq(&id)) {
            current_todos[index].is_deleted = true;
        }
        let new_todos = Json::to_string_pretty(&current_todos)?;
        File::write(String::from("src/db/todos.json"), new_todos)?;
        return Ok(());
    }
}
