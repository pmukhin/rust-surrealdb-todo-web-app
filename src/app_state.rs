use crate::todo::Todo;
use std::collections::HashMap;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct AppState {
    pub surreal_db: Surreal<Client>,
    pub cache: RwLock<HashMap<Uuid, Todo>>, // too simple, but works as an example
}
