mod world;
mod units;

use std::str::FromStr;
use world::object::{Object, Kind};
use world::position::Position;

fn main() {
    let star = Object {
        name: "myStar".to_string(),
        kind: Kind::Star,
        position: Position::from_str("18:04:20.99,-31:31:8.9").unwrap(),
    };
    print!("{}", star);
}
