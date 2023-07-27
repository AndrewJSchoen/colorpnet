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
    pub guards: HashMap<(Uuid,usize),Guard>
}

impl Transition {
    pub fn new(name: String, input: Option<HashMap<Uuid,Signature>>, output: Option<HashMap<Uuid,Signature>>, guards: Option<HashMap<(Uuid,usize),Guard>>) -> Self {
        let name_clone = name.clone();
        let try_init = || -> Result<Transition, Error> {
            match (input,guards) {
                (Some(i),Some(g)) => {
                    if g.keys().all(|(id,size)| i.contains_key(id) && i.get(id).unwrap().clades.len() == *size) {
                        Ok(Self { id: Uuid::new_v4(), name, input: i, output: output.unwrap_or(HashMap::new()), guards: g })
                    } else {
                        Err(Error::new(ErrorKind::InvalidInput, "Guard size does not match input signature size"))
                    }
                },
                (Some(i),None) => {
                    warn!("Transition {} has input signature but no guards. Assuming direct mapping of input types from signature.", name);
                    let mut g = HashMap::new();
                    for (id,signature) in i.iter() {
                        for (size, clade) in signature.clades.iter().enumerate() {
                            g.insert((*id,size),Guard::LessThanOrEqual(clade.clone()));
                        }
                    }
                    Ok(Self { id: Uuid::new_v4(), name, input: i, output: output.unwrap_or(HashMap::new()), guards: g })
                },
                (None,Some(_g)) => {
                    Err(Error::new(ErrorKind::InvalidInput, "Guards require input signature"))
                },
                (None,None) => {
                    Ok(Self { id: Uuid::new_v4(), name, input: HashMap::new(), output: HashMap::new(), guards: HashMap::new() })
                }
            }
        };
        
        match try_init() {
            Ok(t) => t,
            Err(e) => {
                warn!("Could not reconcile inputs and guards for transition {}. Using an empty set of guards and inputs. Error: {}", name_clone, e);
                Self { id: Uuid::new_v4(), name:name_clone, input: HashMap::new(), output: HashMap::new(), guards: HashMap::new() }
            }
        }
    }
}