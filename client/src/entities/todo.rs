use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub deadline: Option<DateTime<Utc>>,
    pub is_completed: bool,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
