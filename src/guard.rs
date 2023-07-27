use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::clade::Clade;
use crate::address::Address;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Guard {
    Is(Address,Clade),
    GreaterThan(Address,Clade),
    LessThan(Address,Clade),
    GreaterThanOrEqual(Address,Clade),
    LessThanOrEqual(Address,Clade),
    Not(Address,Clade),
    All(Vec<Guard>),
    Any(Vec<Guard>),
    None(Vec<Guard>),
    Empty
}

impl Guard {
    pub fn eval(&self, candidates:&HashMap<Address,Clade>) -> bool {
        match self {
            Guard::Is(address, clade) => candidates.get(address).map(|c| c == clade).unwrap_or(false),
            Guard::GreaterThan(address, clade) => candidates.get(address).map(|c| c > clade).unwrap_or(false),
            Guard::LessThan(address, clade) => candidates.get(address).map(|c| c < clade).unwrap_or(false),
            Guard::GreaterThanOrEqual(address, clade) => candidates.get(address).map(|c| c >= clade).unwrap_or(false),
            Guard::LessThanOrEqual(address, clade) => candidates.get(address).map(|c| c <= clade).unwrap_or(false),
            Guard::Not(address, clade) => candidates.get(address).map(|c| c != clade).unwrap_or(false),
            Guard::All(guards) => guards.iter().all(|g| g.eval(candidates)),
            Guard::Any(guards) => guards.iter().any(|g| g.eval(candidates)),
            Guard::None(guards) => guards.iter().all(|g| !g.eval(candidates)),
            Guard::Empty => true
        }
    }

    pub fn addresses(&self) -> Vec<Address> {
        match self {
            Guard::Is(address, _) => vec![address.clone()],
            Guard::GreaterThan(address, _) => vec![address.clone()],
            Guard::LessThan(address, _) => vec![address.clone()],
            Guard::GreaterThanOrEqual(address, _) => vec![address.clone()],
            Guard::LessThanOrEqual(address, _) => vec![address.clone()],
            Guard::Not(address, _) => vec![address.clone()],
            Guard::All(guards) => guards.iter().flat_map(|g| g.addresses()).collect(),
            Guard::Any(guards) => guards.iter().flat_map(|g| g.addresses()).collect(),
            Guard::None(guards) => guards.iter().flat_map(|g| g.addresses()).collect(),
            Guard::Empty => vec![]
        }
    }
}

impl Default for Guard {
    fn default() -> Self {
        Guard::Empty
    }
}