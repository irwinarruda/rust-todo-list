use crate::entities::error::AppError;
use crate::repositories::todo::TodoRepository;
use uuid::Uuid;

pub struct DeleteTodoService;

impl DeleteTodoService {
    pub fn execute(id: Uuid) -> Result<(), AppError> {
        let todo_repository = TodoRepository::new();
        todo_repository.find_by_id(&id)?;
        return todo_repository.delete_todo(&id);
    }
}
