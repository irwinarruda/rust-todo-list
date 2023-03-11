use leptos::*;

use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use reqwasm::http::Request;
use serde::Serialize;
use serde_json::to_string;

use crate::entities::todo::Todo;

#[derive(Serialize, Debug, Clone)]
pub struct CreateTodoDTO {
    pub title: String,
    pub description: String,
    pub deadline: Option<DateTime<Utc>>,
    pub is_completed: bool,
}
pub struct TodoService {}

impl TodoService {
    pub async fn get_todos() -> Result<Vec<Todo>> {
        let res = Request::get("http://localhost:8080/todos").send().await?;
        let todos = res.json::<Vec<Todo>>().await?;
        log!("{:?}", todos);
        return Ok(todos);
    }
    pub async fn create_todo(todo: &CreateTodoDTO) -> Result<()> {
        let todo = to_string(&todo)?;
        Request::post("http://localhost:8080/todos")
            .header("Content-Type", "application/json")
            .body(todo)
            .send()
            .await?;
        return Ok(());
    }
    pub async fn delete_todo(id: &String) -> Result<()> {
        Request::delete(&format!("http://localhost:8080/todos/{}", id))
            .send()
            .await?;
        return Ok(());
    }
}
