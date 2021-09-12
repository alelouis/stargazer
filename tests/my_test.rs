use std::str::FromStr;
use stargazer::world::object::{Object, Kind};
use stargazer::world::position::Position;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
    let star = Object {
        name: "myStar".to_string(),
        kind: Kind::Star,
        position: Position::from_str("18:04:20.99,-31:31:8.9").unwrap(),
    };
    print!("{}", star);
    }
}