use crate::entities::error::AppError;
use crate::entities::todo::CreateTodoDTO;
use crate::repositories::todo::TodoRepository;

pub struct CreateTodoService;

impl CreateTodoService {
    pub fn execute(data: CreateTodoDTO) -> Result<(), AppError> {
        let todo_repository = TodoRepository::new();
        return todo_repository.create_todo(&data);
    }
}
