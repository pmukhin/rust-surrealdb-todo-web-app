mod app_state;
mod create;
mod find_by_id;
mod paginate;
mod todo;

use crate::app_state::AppState;
use axum::body::Body;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::{extract::Request, middleware, routing::get};
use std::str::FromStr;
use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::Span;

async fn auth(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // our authentication logic here...
    Ok(next.run(request).await)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let server_url = std::env::var("SERVER_URL")?;
    let log_level = std::env::var("LOG_LEVEL")?;

    let db = Surreal::new::<Ws>("localhost:8000").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("test").use_db("misc").await?;

    let app_state = Arc::new(AppState {
        surreal_db: db,
        cache: Default::default(),
    });

    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::from_str(&log_level)?)
        .init();

    let app = axum::Router::new()
        .route("/todos", get(paginate::action).post(create::action))
        .route("/todos/{id}", get(find_by_id::action))
        .layer(
            ServiceBuilder::new().layer(TraceLayer::new_for_http().on_request(
                |req: &Request, _span: &Span| tracing::debug!("{} {}", req.method(), req.uri()),
            )),
        )
        .layer(middleware::from_fn(auth))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&server_url).await?;

    tracing::info!("listening on {}", &server_url);

    Ok(axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = signal::ctrl_c().await;
            tracing::info!("app is shutting down gracefully");
        })
        .await?)
}
