use std::fmt;
use std::error;
use std::str::FromStr;

pub struct HMS {
    pub hours: i32,
    pub minutes: i32,
    pub seconds: f64,
}

impl HMS {
    fn to_degrees(&self) -> f64 {
        let hours = self.hours as f64;
        let minutes = self.minutes as f64;
        let seconds = self.seconds;
        let decimal = hours + minutes/60. + seconds/3600.;
        decimal * 15.
    }

    fn from_degrees(&self, degrees: f64) -> HMS {
        let time = (degrees % 360.) * 24. / 360.;
        let hours = time.floor();
        let minutes = (time*60.) % 60.;
        let seconds = (time*3600.) % 60.;
        HMS {
            hours: hours as i32,
            minutes: minutes as i32,
            seconds,
        }
    }
}

impl fmt::Display for HMS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}h{}m{}s", self.hours, self.minutes, self.seconds)
    }
}

impl FromStr for HMS {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self, Box<dyn error::Error>> {
        let split = s.split(":");
        let vec: Vec<&str> = split.clone().collect();
        let hours = vec[0].parse()?;
        let minutes = vec[1].parse()?;
        let seconds = vec[2].parse()?;
        let hms = HMS {
            hours,
            minutes,
            seconds,
        };
        Ok(hms)
    }
}

