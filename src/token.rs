use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::clade::Clade;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub id: Uuid,
    pub name: String,
    pub clade: Clade
}

impl Token {
    pub fn new(name: String, clade: Clade) -> Self {
        Self { id: Uuid::new_v4(), name, clade }
    }
}