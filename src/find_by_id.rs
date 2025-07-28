use crate::app_state::AppState;

use crate::todo::Todo;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;
#[derive(Debug, Error)]
pub enum FindByIdError {
    #[error("Database error: {0}")]
    Surreal(#[from] surrealdb::Error),
    #[error("Not found")]
    NotFound,
}

impl IntoResponse for FindByIdError {
    fn into_response(self) -> axum::response::Response {
        match &self {
            FindByIdError::Surreal(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            FindByIdError::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

pub async fn action(
    State(s): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>, FindByIdError> {
    match s.cache.read().await.get(&id) {
        None => {}
        Some(todo) => return Ok(Json(todo.clone())),
    }
    let query = r#"
        SELECT uuid, title, completed
        FROM todos
        WHERE uuid = $uuid
    "#;
    let mut r = s.surreal_db.query(query).bind(("uuid", id)).await?;
    let maybe_todo: Option<Todo> = r.take(0)?;

    match maybe_todo {
        None => Err(FindByIdError::NotFound),
        Some(todo) => {
            let mut write_guard = s.cache.write().await;
            write_guard.insert(todo.uuid, todo.clone());
            Ok(Json(todo))
        }
    }
}
