use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Hash, Serialize, PartialEq, Eq, Deserialize)]
pub struct Symbol(String);