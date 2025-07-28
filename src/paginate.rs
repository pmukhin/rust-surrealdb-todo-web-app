use crate::app_state::AppState;
use crate::todo::Todo;
use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("database error")]
    Surreal(#[from] surrealdb::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct Params {
    limit: Option<usize>,
    offset: Option<usize>,
}

pub async fn action(
    State(s): State<Arc<AppState>>,
    Query(params): Query<Params>,
) -> Result<Json<Vec<Todo>>, Error> {
    let limit = params.limit.unwrap_or(20);
    let query = r#"
        SELECT uuid, title, completed
        FROM todos
        LIMIT $limit START $offset
     "#;
    let offset = params.offset.unwrap_or(0);
    let mut r = s
        .surreal_db
        .query(query)
        .bind(("limit", limit))
        .bind(("offset", offset))
        .await?;
    Ok(Json(r.take(0)?))
}
