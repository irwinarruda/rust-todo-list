use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub deadline: Option<DateTime<Utc>>,
    pub is_completed: bool,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct CreateTodoDTO {
    pub title: String,
    pub description: String,
    pub deadline: Option<DateTime<Utc>>,
    pub is_completed: bool,
}

#[derive(Deserialize, Debug)]
pub struct EditTodoDTO {
    pub id: Uuid,
    pub description: String,
    pub is_completed: bool,
}
