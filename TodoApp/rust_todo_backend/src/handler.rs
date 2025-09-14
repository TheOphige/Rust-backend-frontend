use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use argon2::{
    Argon2,
    password_hash::{SaltString, rand_core::OsRng, PasswordHasher, PasswordVerifier, PasswordHash},
};

use crate::{
    model::{TodoModel, TodoModelResponse, UserModel},
    schema::{CreateTodoSchema, FilterOptions, UpdateTodoSchema, RegisterSchema, LoginSchema},
    AppState,
    middleware::AuthUser,
    auth::generate_jwt,
};



/* -------------------------- AUTH HANDLERS -------------------------- */

pub async fn register_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RegisterSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // generate a random salt and hash the password using Argon2
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Password hashing failed"}))))?
        .to_string();

    let id = Uuid::new_v4().to_string();

    sqlx::query!(
        "INSERT INTO users (id, username, email, password_hash) VALUES (?, ?, ?, ?)",
        &id, &body.username, &body.email, &password_hash
    )
    .execute(&data.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("{:?}", e)}))))?;

    Ok(Json(json!({"status": "success", "user_id": id})))
}

pub async fn login_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE email = ?", &body.email)
        .fetch_one(&data.db)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid email or password"}))))?;

    // parse stored PHC string into a PasswordHash
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Stored password hash is invalid"}))))?;

    // verify - returns Result<(), _> (Ok => valid)
    Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .map_err(|_| (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid email or password"}))))?;

    // generate JWT
    let token = generate_jwt(&user.id)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Token creation failed"}))))?;

    Ok(Json(json!({
        "status": "success",
        "token": token
    })))
}

/* -------------------------- HEALTH -------------------------- */

pub async fn health_check_handler() -> impl IntoResponse {
    let json_response = json!({
        "status": "ok",
        "message": "API Services"
    });
    Json(json_response)
}

/* -------------------------- TODO HANDLERS -------------------------- */

// NOTE: AuthUser(...) returns the claims from the middleware; we extract the user_id from claims.sub
pub async fn todo_list_handler(
    Query(opts): Query<FilterOptions>,
    State(data): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = claims.sub;
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos = sqlx::query_as!(
        TodoModel,
        r#"SELECT * FROM todos WHERE user_id = ? ORDER BY id LIMIT ? OFFSET ?"#,
        user_id,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status": "error","message": format!("Database error: {:?}", e)})))
    })?;

    let todo_responses = todos.iter().map(to_todo_response).collect::<Vec<TodoModelResponse>>();

    Ok(Json(json!({
        "status": "ok",
        "count": todo_responses.len(),
        "todos": todo_responses
    })))
}

pub async fn create_todo_handler(
    State(data): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Json(body): Json<CreateTodoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = claims.sub;
    let id = Uuid::new_v4().to_string();
    let query_result = sqlx::query(
        r#"INSERT INTO todos (id, user_id, title, description, is_completed) VALUES (?, ?, ?, ?, ?)"#
    )
        .bind(&id)
        .bind(&user_id)
        .bind(&body.title)
        .bind(&body.description)
        .bind(body.is_completed.unwrap_or(false))
        .execute(&data.db)
        .await
        .map_err(|e| e.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            return Err((StatusCode::CONFLICT, Json(json!({"status": "error","message": "Todo already exists"}))));
        }
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status": "error","message": err}))));
    }

    let todo = sqlx::query_as!(TodoModel, r#"SELECT * FROM todos WHERE id = ?"#, &id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status":"error","message": format!("{:?}", e)}))))?;

    Ok(Json(json!({"status":"success","data":{"todo": to_todo_response(&todo)}})))
}

pub async fn get_todo_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = claims.sub;
    let query_result = sqlx::query_as!(
        TodoModel,
        r#"SELECT * FROM todos WHERE id = ? AND user_id = ?"#,
        &id,
        user_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(todo) => Ok(Json(json!({"status":"success","data":{"todo": to_todo_response(&todo)}}))),
        Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, Json(json!({"status":"fail","message": format!("Todo with ID: {} not found", id)})))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status":"error","message": format!("{:?}", e)})))),
    }
}

pub async fn edit_todo_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Json(body): Json<UpdateTodoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = claims.sub;
    let query_result = sqlx::query_as!(
        TodoModel,
        r#"SELECT * FROM todos WHERE id = ? AND user_id = ?"#,
        &id,
        user_id
    )
    .fetch_one(&data.db)
    .await;

    let todo = match query_result {
        Ok(todo) => todo,
        Err(sqlx::Error::RowNotFound) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Todo with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                })),
            ));
        }
    };

    // parse data
    let is_completed = body.is_completed.unwrap_or(todo.is_completed != 0);
    let i8_is_completed = is_completed as i8;

    sqlx::query(
        r#"UPDATE todos SET title = ?, description = ?, is_completed = ? WHERE id = ? AND user_id = ?"#
    )
        .bind(body.title.unwrap_or(todo.title))
        .bind(body.description.unwrap_or(todo.description.unwrap_or_default()))
        .bind(i8_is_completed)
        .bind(&id)
        .bind(&user_id)
        .execute(&data.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status":"error","message": format!("{:?}", e)}))))?;

    let updated_todo = sqlx::query_as!(
        TodoModel,
        r#"SELECT * FROM todos WHERE id = ? AND user_id = ?"#,
        &id,
        user_id
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status":"error","message": format!("{:?}", e)}))))?;

    Ok(Json(json!({"status":"success","data":{"todo": to_todo_response(&updated_todo)}})))
}

pub async fn delete_todo_handler(
    Path(id): Path<String>,
    State(data): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = claims.sub;
    let query_result = sqlx::query!(
        r#"DELETE FROM todos WHERE id = ? AND user_id = ?"#,
        &id,
        user_id
    )
    .execute(&data.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"status":"error","message": format!("{:?}", e)}))))?;

    if query_result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"status":"error","message": format!("Todo with ID: {} not found", id)}))));
    }

    Ok(StatusCode::OK)
}

/* -------------------------- HELPERS -------------------------- */

fn to_todo_response(todo: &TodoModel) -> TodoModelResponse {
    TodoModelResponse {
        id: todo.id.to_owned(),
        title: todo.title.to_owned(),
        description: todo.description.clone(),
        is_completed: todo.is_completed != 0,
        created_at: todo.created_at.unwrap(),
        updated_at: todo.updated_at.unwrap(),
    }
}
