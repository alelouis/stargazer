use std::fmt;
use std::error;
use std::str::FromStr;
pub mod coord;

#[derive(Debug)]
pub enum Kind {
    Star,
    Planet,
    Satellite,
    ArtificialSatellite,
}

pub struct Object {
    pub name: String,
    pub kind: Kind,
    pub position: Position,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}\nKind: {:?}\n{}", self.name, self.kind, self.position)
    }
}

pub struct Position {
    pub right_ascension: coord::HMS,
    pub declination: coord::DMS,
}

impl FromStr for Position {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self, Box<dyn error::Error>> {
        let split = s.split(",");
        let vec: Vec<&str> = split.clone().collect();
        let ra = coord::HMS::from_str(vec[0])?;
        let dec = coord::DMS::from_str(vec[1])?;
        let pos = Position {
            right_ascension: ra,
            declination: dec,
        };
        Ok(pos)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RA: {}, DEC : {}", self.right_ascension, self.declination)
    }
}




