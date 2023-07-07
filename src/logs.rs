use serde_derive::{Deserialize, Serialize};

use crate::vector::Vector3D;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: usize,
    pub operation: Operation,
    pub vector: Vector3D,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Operation {
    Insert,
    Update,
    Delete,
}
