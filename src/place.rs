use uuid::Uuid;
use serde::{Serialize, Deserialize};
// use crate::petri::token::TokenSet;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    pub id: Uuid,
    pub name: String,
    // pub tokens: TokenSet
}

impl Place {
    pub fn new(name: String) -> Self {
        Self { id: Uuid::new_v4(), name }
    }
}