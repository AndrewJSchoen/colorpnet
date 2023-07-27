use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use crate::signature::Signature;
use crate::guard::Guard;
use log::warn;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Transition {
    pub id: Uuid,
    pub name: String,
    pub input: HashMap<Uuid,Signature>,
    pub output: HashMap<Uuid,Signature>,
    // Guards are indexed by the place id and the index of the clade in the signature
    pub guard: Guard
}

impl Transition {
    pub fn new(name: String, input: Option<HashMap<Uuid,Signature>>, output: Option<HashMap<Uuid,Signature>>, guard: Option<Guard>) -> Self {
        match (input, guard) {
            (Some(i),Some(g)) => {
                if !g.addresses().iter().all(|address| i.contains_key(&address.place_id) && i.get(&address.place_id).unwrap().clades.len() > address.index) {
                    warn!("Transition {} has guard that references an address not in the input signature. Using an empty guard.", name);
                    return Self { id: Uuid::new_v4(), name, input: i, output: output.unwrap_or(HashMap::new()), guard: Guard::Empty }
                } else {
                    return Self { id: Uuid::new_v4(), name, input: i, output: output.unwrap_or(HashMap::new()), guard: g }
                }
            },
            (Some(i),None) => {
                return Self { id: Uuid::new_v4(), name, input: i, output: output.unwrap_or(HashMap::new()), guard: Guard::Empty }
            },
            (None,Some(_g)) => {
                warn!("Transition {} has guard but no input signature. Using an empty guard.", name);
                return Self { id: Uuid::new_v4(), name, input: HashMap::new(), output: output.unwrap_or(HashMap::new()), guard: Guard::Empty }
            },
            (None,None) => {
                return Self { id: Uuid::new_v4(), name, input: HashMap::new(), output: output.unwrap_or(HashMap::new()), guard: Guard::Empty }
            }
        }
    }
}