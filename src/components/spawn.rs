use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Spawn {
    pub progress: u32,
    pub entity_id: Uuid,
}