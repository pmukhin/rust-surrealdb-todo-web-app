use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub uuid: Uuid,
    pub title: String,
    pub completed: bool,
}
