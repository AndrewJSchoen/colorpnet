use serde::{Serialize, Deserialize};
use crate::clade::Clade;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Guard {
    Is(Clade),
    GreaterThan(Clade),
    LessThan(Clade),
    GreaterThanOrEqual(Clade),
    LessThanOrEqual(Clade),
    Not(Clade),
    All(Vec<Guard>),
    Any(Vec<Guard>),
    None(Vec<Guard>),
}

impl Guard {
    pub fn eval(&self, candidate:&Clade) -> bool {
        match self {
            Guard::Is(clade) => clade == candidate,
            Guard::GreaterThan(clade) => clade < candidate,
            Guard::LessThan(clade) => clade > candidate,
            Guard::GreaterThanOrEqual(clade) => clade <= candidate,
            Guard::LessThanOrEqual(clade) => clade >= candidate,
            Guard::Not(clade) => clade != candidate,
            Guard::All(guards) => guards.iter().all(|g| g.eval(candidate)),
            Guard::Any(guards) => guards.iter().any(|g| g.eval(candidate)),
            Guard::None(guards) => guards.iter().all(|g| !g.eval(candidate)),
        }
    }
}