mod stargazer;
use std::str::FromStr;

fn main() {
    let star = stargazer::Object {
        name: "myStar".to_string(),
        kind: stargazer::Kind::Star,
        position: stargazer::Position::from_str("18:04:20.99,-31:31:8.9").unwrap(),
    };
    print!("{}", star);
}
