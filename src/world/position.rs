use std::fmt;
use std::error;
use std::str::FromStr;

use crate::units::{hms::HMS, dms::DMS};

pub struct Position {
    pub right_ascension: HMS,
    pub declination: DMS,
}
 
impl FromStr for Position {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self, Box<dyn error::Error>> {
        let split = s.split(",");
        let vec: Vec<&str> = split.clone().collect();
        let ra = HMS::from_str(vec[0])?;
        let dec = DMS::from_str(vec[1])?;
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

