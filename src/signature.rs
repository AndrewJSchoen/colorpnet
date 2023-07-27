use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use crate::clade::Clade;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Signature {
    pub clades: Vec<Clade>
}

impl Signature {
    pub fn new(clades: Vec<Clade>) -> Self {
        Self { clades }
    }
}

impl PartialOrd for Signature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.clades.len() != other.clades.len() {
            return None;
        }
        let relations:Vec<Option<Ordering>> = self.clades.iter().zip(other.clades.iter()).map(|(c1, c2)| c1.partial_cmp(c2)).collect();
        if relations.iter().any(|r| r.is_none()) {
            return None;
        }
        if relations.iter().all(|r| r.unwrap() == Ordering::Equal) {
            return Some(Ordering::Equal);
        } else if relations.iter().all(|r| r.unwrap() == Ordering::Greater || r.unwrap() == Ordering::Equal) {
            return Some(Ordering::Greater);
        } else if relations.iter().all(|r| r.unwrap() == Ordering::Less || r.unwrap() == Ordering::Equal) {
            return Some(Ordering::Less);
        }
        return None;
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.clades == other.clades
    }
}

#[test]
pub fn signature_logic() {
    let grandchild1 = Clade::new("grandchild1".into(), None);
    let grandchild2 = Clade::new("grandchild2".into(), None);
    let child = Clade::new("child".into(), Some(vec![grandchild1.clone(), grandchild2.clone()]));
    let tax = Clade::new(
        "root".into(),
        Some(vec![child.clone()]));

    let sig1 = Signature::new(vec![child.clone(), grandchild2.clone()]);
    let sig2 = Signature::new(vec![grandchild1.clone(), grandchild2.clone()]);
    let sig3 = Signature::new(vec![grandchild1.clone(), grandchild2.clone()]);
    let sig4 = Signature::new(vec![grandchild1.clone()]);
    let sig5 = Signature::new(vec![child.clone()]);
    assert!(sig3 == sig2);
    assert!(sig5 > sig4);
    assert!(!(sig5 < sig4));
    assert!(sig1 > sig2);
    assert!(tax > grandchild1);
    assert!(tax != grandchild1);
    assert!(tax >= child);
    assert!(sig1 != sig5);
    assert!(!(sig1 >= sig5));
}