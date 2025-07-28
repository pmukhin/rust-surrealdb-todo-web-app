use crate::app_state::AppState;
use crate::todo::Todo;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    title: String,
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("empty title")]
    EmptyTitle,
    #[error("title too long")]
    TitleTooLong,
}

impl CreateTodo {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.title.is_empty() {
            return Err(ValidationError::EmptyTitle);
        }
        if self.title.len() > 255 {
            return Err(ValidationError::TitleTooLong);
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum CreateError {
    #[error("Database error: {0}")]
    Surreal(#[from] surrealdb::Error),
    #[error("Database error: {0}")]
    Validation(#[from] ValidationError),
    #[error("Already exists")]
    AlreadyExists,
}

impl IntoResponse for CreateError {
    fn into_response(self) -> Response {
        match &self {
            CreateError::Surreal(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            CreateError::Validation(e) => {
                (StatusCode::BAD_REQUEST, format!("validation failed: {e}")).into_response()
            }
            CreateError::AlreadyExists => StatusCode::CONFLICT.into_response(),
        }
    }
}

pub async fn action(
    State(s): State<Arc<AppState>>,
    Json(input): Json<CreateTodo>,
) -> Result<Json<Todo>, CreateError> {
    match input.validate() {
        Err(err) => Err(err.into()),
        Ok(_) => {
            let uuid = Uuid::new_v5(&Uuid::NAMESPACE_OID, input.title.as_bytes());
            let todo = Todo {
                uuid,
                title: input.title,
                completed: false,
            };
            let created: Todo = s
                .surreal_db
                .create(("todos", todo.uuid.to_string().as_str()))
                .content(todo)
                .await?
                .ok_or(CreateError::AlreadyExists)?;
            Ok(Json(created))
        }
    }
}
