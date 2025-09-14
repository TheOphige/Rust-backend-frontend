use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Database model for a user
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]

pub struct UserModel {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
}


/// Database model for a todo
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct TodoModel {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: i8,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// JSON response model for a todo
#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TodoModelResponse {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
