use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, Hash, Serialize, Deserialize)]
pub enum Clade {
    Branch {
        uuid: Uuid,
        name: String,
        children: Vec<Clade>,
    },
    Leaf {
        uuid: Uuid,
        name: String,
    },
}

impl Clade {
    pub fn new(name: String, children: Option<Vec<Clade>>) -> Clade {
        match children {
            Some(c) => Clade::Branch {
                uuid: Uuid::new_v4(),
                name,
                children: c,
            },
            None => Clade::Leaf {
                uuid: Uuid::new_v4(),
                name,
            },
        }
    }

    pub fn id(&self) -> Uuid {
        match self {
            Clade::Branch { uuid, .. } => *uuid,
            Clade::Leaf { uuid, .. } => *uuid,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Clade::Branch { name, .. } => name.clone(),
            Clade::Leaf { name, .. } => name.clone(),
        }
    }

    pub fn children(&self) -> Option<&Vec<Clade>> {
        match self {
            Clade::Branch { children, .. } => Some(children),
            Clade::Leaf { .. } => None,
        }
    }

    pub fn descendent(&self, uuid: &Uuid) -> bool {
        match self {
            Clade::Branch {
                uuid: u, children, ..
            } => {
                if u == uuid {
                    return true;
                }
                return children.iter().any(|c| c.descendent(uuid));
            }
            Clade::Leaf { uuid: u, .. } => return u == uuid,
        }
    }

    pub fn parentage(&self, uuid: &Uuid) -> Option<Vec<Uuid>> {
        match self {
            Clade::Branch {
                uuid: u, children, ..
            } => {
                if u == uuid {
                    return Some(vec![]);
                }
                for child in children {
                    match child.parentage(uuid) {
                        Some(mut p) => {
                            p.push(*u);
                            return Some(p);
                        }
                        None => continue,
                    }
                }
                return None;
            }
            Clade::Leaf { uuid: u, .. } => {
                if u == uuid {
                    return Some(vec![]);
                }
                return None;
            }
        }
    }

    pub fn query(&self, name_query: &String) -> Option<Uuid> {
        match self {
            Clade::Leaf { uuid, name } => {
                if name_query == name {
                    return Some(*uuid);
                }
                return None;
            },
            Clade::Branch { uuid, name, children } => {
                if name_query == name {
                    return Some(*uuid);
                }
                for child in children {
                    match child.query(name_query) {
                        Some(uuid) => return Some(uuid),
                        None => continue,
                    }
                }
                return None;
            }
        }
    }

    pub fn get(&self, id: &Uuid) -> Option<Clade> {
        match self {
            Clade::Leaf { uuid, .. } => {
                if uuid == id {
                    return Some(self.clone());
                }
                return None;
            }
            Clade::Branch { uuid, children, .. } => {
                if uuid == id {
                    return Some(self.clone());
                }
                for child in children {
                    match child.get(id) {
                        Some(tax) => return Some(tax),
                        None => continue,
                    }
                }
                return None;
            }
        }
    }
}

impl PartialOrd for Clade {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.descendent(&other.id()), other.descendent(&self.id())) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Greater),
            (false, true) => Some(Ordering::Less),
            (false, false) => None,
        }
    }
}

impl PartialEq for Clade {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

#[test]
pub fn clade_descendents() {
    let tax1 = Clade::new(
        "root:1".into(),
        Some(vec![Clade::new(
            "child:1".into(),
            Some(vec![
                Clade::new("grandchild1:1".into(), None),
                Clade::new("grandchild2:1".into(), None),
            ]),
        )]),
    );

    let tax2 = Clade::new(
        "root:2".into(),
        Some(vec![Clade::new(
            "child:2".into(),
            None
        )]),
    );


    assert!(tax1.descendent(&tax1.id()));
    assert!(tax1 >= tax1);
    assert!(tax1 == tax1);
    match tax1.children() {
        Some(children) => {
            assert!(children[0].descendent(&children[0].id()));
            assert!(tax1.descendent(&children[0].id()));
            assert!(!(tax1 < children[0]));
            assert!(tax1 > children[0]);
            assert!(tax1 >= children[0]);
            assert!(children[0] == children[0]);
            match children[0].children() {
                Some(grandchildren) => {
                    assert_eq!(grandchildren.len(), 2);
                    assert!(grandchildren[0].descendent(&grandchildren[0].id()));
                    assert!(children[0].descendent(&grandchildren[0].id()));
                    assert!(children[0] > grandchildren[0]);
                    assert!(children[0] >= grandchildren[1]);
                    assert!(tax1 > grandchildren[0]);
                    assert!(!(tax1 < grandchildren[0]));
                    assert!(grandchildren[0] != grandchildren[1]);
                    assert!(tax1.descendent(&grandchildren[0].id()));
                    assert!(tax1.descendent(&grandchildren[1].id()));
                }
                None => assert!(false),
            }
            assert_eq!(children[0].id(),tax1.query(&"child:1".into()).unwrap());
            assert_eq!(children[0],tax1.get(&children[0].id()).unwrap());
        }
        None => assert!(false),
    }
}