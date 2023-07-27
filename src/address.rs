use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct Address {
    pub index: usize,
    pub place_id: Uuid,
}