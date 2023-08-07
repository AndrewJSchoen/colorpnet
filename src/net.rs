use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::signature::Signature;
use crate::aliases::{Time,Cost};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColoredPetriNet {
    pub id: Uuid,
    pub name: String,
    pub places: HashMap<Uuid, Place>,
    pub transitions: HashMap<Uuid, Transition>,
    pub initial_marking: HashMap<Uuid, HashMap<Uuid,Token>>,
    pub current_marking: HashMap<Uuid, HashMap<Uuid,Token>>,
    pub name_lookup: HashMap<Uuid, String>,
}

impl ColoredPetriNet {
    pub fn new(name: String, places: Option<HashMap<Uuid, Place>>, transitions: Option<HashMap<Uuid, Transition>>, initial_marking: Option<HashMap<Uuid, HashMap<Uuid,Token>>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            places: places.unwrap_or(HashMap::new()),
            transitions: transitions.unwrap_or(HashMap::new()),
            initial_marking: initial_marking.unwrap_or(HashMap::new()),
            current_marking: initial_marking.unwrap_or(HashMap::new()),
            name_lookup: HashMap::new(),
        }
    }
}