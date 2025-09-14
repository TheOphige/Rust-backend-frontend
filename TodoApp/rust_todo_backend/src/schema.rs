use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct RegisterSchema {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct LoginSchema {
    pub email: String,
    pub password: String,
}


/// Query parameters for listing todos with pagination
#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

/// Schema for creating a new todo
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTodoSchema {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_completed: Option<bool>,
}

/// Schema for updating an existing todo
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTodoSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub is_completed: Option<bool>,
}
