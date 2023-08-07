use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::clade::Clade;
use crate::symbol::Symbol;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Guard {
    Is(Symbol,Clade),
    GreaterThan(Symbol,Clade),
    LessThan(Symbol,Clade),
    GreaterThanOrEqual(Symbol,Clade),
    LessThanOrEqual(Symbol,Clade),
    Not(Symbol,Clade),
    All(Vec<Guard>),
    Any(Vec<Guard>),
    None(Vec<Guard>),
    Empty
}

impl Guard {
    pub fn eval(&self, candidates:&HashMap<Symbol,Clade>) -> bool {
        match self {
            Guard::Is(symbol, clade) => candidates.get(symbol).map(|c| c == clade).unwrap_or(false),
            Guard::GreaterThan(symbol, clade) => candidates.get(symbol).map(|c| c > clade).unwrap_or(false),
            Guard::LessThan(symbol, clade) => candidates.get(symbol).map(|c| c < clade).unwrap_or(false),
            Guard::GreaterThanOrEqual(symbol, clade) => candidates.get(symbol).map(|c| c >= clade).unwrap_or(false),
            Guard::LessThanOrEqual(symbol, clade) => candidates.get(symbol).map(|c| c <= clade).unwrap_or(false),
            Guard::Not(symbol, clade) => candidates.get(symbol).map(|c| c != clade).unwrap_or(false),
            Guard::All(guards) => guards.iter().all(|g| g.eval(candidates)),
            Guard::Any(guards) => guards.iter().any(|g| g.eval(candidates)),
            Guard::None(guards) => guards.iter().all(|g| !g.eval(candidates)),
            Guard::Empty => true
        }
    }

    pub fn symbols(&self) -> Vec<Symbol> {
        match self {
            Guard::Is(symbol, _) => vec![symbol.clone()],
            Guard::GreaterThan(symbol, _) => vec![symbol.clone()],
            Guard::LessThan(symbol, _) => vec![symbol.clone()],
            Guard::GreaterThanOrEqual(symbol, _) => vec![symbol.clone()],
            Guard::LessThanOrEqual(symbol, _) => vec![symbol.clone()],
            Guard::Not(symbol, _) => vec![symbol.clone()],
            Guard::All(guards) => guards.iter().flat_map(|g| g.symbols()).collect(),
            Guard::Any(guards) => guards.iter().flat_map(|g| g.symbols()).collect(),
            Guard::None(guards) => guards.iter().flat_map(|g| g.symbols()).collect(),
            Guard::Empty => vec![]
        }
    }
}

impl Default for Guard {
    fn default() -> Self {
        Guard::Empty
    }
}