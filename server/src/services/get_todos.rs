use crate::entities::error::AppError;
use crate::entities::todo::Todo;
use crate::repositories::todo::TodoRepository;

pub struct GetTodosService;

impl GetTodosService {
    pub fn execute() -> Result<Vec<Todo>, AppError> {
        let todo_repository = TodoRepository::new();
        return todo_repository.get_todos();
    }
}
