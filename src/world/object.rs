use std::fmt;
use crate::world::position::Position;

pub struct Object {
    pub name: String,
    pub kind: Kind,
    pub position: Position,
}

#[derive(Debug)]
pub enum Kind {
    Star,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\nKind: {:?}\n{}", self.name, self.kind, self.position)
    }
}