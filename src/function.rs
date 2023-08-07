use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub id: Uuid,
}

impl Default for Function {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
        }
    }
}

impl Function {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
        }
    }
}
