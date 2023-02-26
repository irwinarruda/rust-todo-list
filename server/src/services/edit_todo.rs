use chrono::Utc;

use crate::entities::error::AppError;
use crate::entities::todo::EditTodoDTO;
use crate::repositories::todo::TodoRepository;

pub struct EditTodoService;

impl EditTodoService {
    pub fn execute(todo: EditTodoDTO) -> Result<(), AppError> {
        let todo_repository = TodoRepository::new();
        let current_todo = todo_repository.find_by_id(todo.id)?;
        if let Some(deadline) = current_todo.deadline {
            if deadline < Utc::now() && todo.is_completed != current_todo.is_completed {
                return Err(AppError::new(
                    400,
                    "Cannot update completion for a past deadline",
                    String::from(""),
                ));
            }
        }
        todo_repository.edit_todo(todo)?;
        return Ok(());
    }
}
