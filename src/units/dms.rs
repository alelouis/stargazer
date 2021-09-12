use std::fmt;
use std::error;
use std::str::FromStr;

pub struct DMS {
    pub degrees: i32,
    pub minutes: i32,
    pub seconds: f64,
}

impl DMS {
    fn to_degrees(&self) -> f64 {
        let degrees = self.degrees as f64;
        let minutes = self.minutes as f64;
        let seconds = self.seconds;
        let decimal = f64::signum(degrees) * (f64::abs(degrees) + minutes/60. + seconds/3600.);
        decimal
    }
}

impl fmt::Display for DMS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d{}m{}s", self.degrees, self.minutes, self.seconds)
    }
}

impl FromStr for DMS {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self, Box<dyn error::Error>> {
        let split = s.split(":");
        let vec: Vec<&str> = split.clone().collect();
        let degrees = vec[0].parse()?;
        let minutes = vec[1].parse()?;
        let seconds = vec[2].parse()?;
        let dms = DMS {
            degrees: degrees,
            minutes: minutes,
            seconds: seconds,
        };
        Ok(dms)
    }
}

