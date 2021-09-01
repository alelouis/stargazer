use hifitime::{Epoch};
use chrono::prelude::*;
use std::str::FromStr;

pub fn deg_to_hms(deg: f64) -> (f64, f64, f64) {
    // Convert degrees to (hour, minutes, seconds)
    let time = (deg % 360.) * 24. / 360.;
    let gmst_h = time.floor();
    let gmst_m = (time*60.) % 60.;
    let gmst_s = (time*3600.) % 60.;
    (gmst_h, gmst_m, gmst_s)
}

pub fn utc_str() -> String {
    // Return UTC hour in String format
    let format = "%Y-%m-%dT%H:%M:%S%.6f UTC";
    Utc::now().format(format).to_string()
}

pub fn utc_str_simple() -> String {
    // Return UTC hour in String format
    let format = "%H:%M:%S";
    Utc::now().format(format).to_string()
}

pub fn jd(utc_string: &str) -> f64 {
    // Returns julian date for a given UTC String
    let utc: Epoch = Epoch::from_str(&utc_string).unwrap();
    utc.as_jde_utc_days()
}

pub fn era(jd_ut1: f64) -> f64{
    // Compute Earth Rotation Angle from julian date
    let t_u = jd_ut1 - 2451545.0;
    let era_rad = 2.*std::f64::consts::PI*(0.7790572732640 + 1.0027378119113546 * t_u);
    era_rad * 180. / std::f64::consts::PI
}

pub fn lst_at_lon(lon: f64, gmst: f64) -> f64{
    // Compute local sidereal angle for a given longitude
    gmst + lon
}
