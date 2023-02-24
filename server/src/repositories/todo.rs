use crate::entities::todo::*;

pub struct TodoRepository;

impl TodoRepository {
    pub fn get_todos(&self) -> Result<Vec<Todo>, ()> {
        let todos = {
            let todos_string = std::fs::read_to_string(String::from("src/db/todos.json")).unwrap();
            serde_json::from_str::<Vec<Todo>>(&todos_string).map_err(|e| println!("{:?}", e))
        };
        return todos;
    }
}
