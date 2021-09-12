use std::str::FromStr;
use stargazer::units::hms::HMS;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create() {
        let hms = HMS::from_str("18:04:20.99").unwrap();
        assert_eq!(hms.hours, 18);
        assert_eq!(hms.minutes, 04);
        assert_eq!(hms.seconds, 20.99);
    }
}