use crate::entities::error::AppError;
use crate::entities::todo::Todo;
use crate::repositories::todo::TodoRepository;
use uuid::Uuid;

pub struct GetTodoByIdService;

impl GetTodoByIdService {
    pub fn execute(id: Uuid) -> Result<Todo, AppError> {
        let todo_repository = TodoRepository::new();
        return todo_repository.find_by_id(&id);
    }
}
