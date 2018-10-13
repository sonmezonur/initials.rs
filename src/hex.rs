//! Hex module helps to convert Color Hex Code to RGB.
use std::i64;
use std::iter::FromIterator;
use error::Error;

/// Parse hex code and generate RGB vector accordingly.
pub fn parse_hex(hex_str: &str) -> Result<Vec<i64>, Error> {
    if !hex_str.starts_with("#") {
        return Err(Error::InvalidHexFormat {
            expected: String::from("Color hex code must start with `#`"),
            actual: String::from(format!("Color hex starts with `{}`", hex_str.chars().next().unwrap()))
        });
    }
    if hex_str.len() != 7 {
        return Err(Error::InvalidHexFormat {
            expected: String::from("Hex code must be `7` characters long. Example: `#00FF00`"),
            actual: String::from(format!("Hex code is `{}` characters long!", hex_str.len()))
        });
    }
    // collect characters from the String
    let raw: Vec<char> = hex_str.chars().collect();
    Ok(
        vec![
            try!(i64::from_str_radix(&String::from_iter(&raw[1 .. 3]), 16)),
            try!(i64::from_str_radix(&String::from_iter(&raw[3 .. 5]), 16)),
            try!(i64::from_str_radix(&String::from_iter(&raw[5 .. 7]), 16)),
        ]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_hex_with_missing_prefix() {
        let hex_value = "00ff00";
        let res = parse_hex(hex_value);
        assert!(res.is_err());
        assert_eq!(
            format!("{}", res.unwrap_err()), 
            "unexpected hex color format: expected(Color hex code must start with `#`), got(Color hex starts with `0`)"
        );
    }

    #[test]
    fn test_invalid_hex_with_wrong_size() {
        let hex_value = "#ff00";
        let res = parse_hex(hex_value);
        assert!(res.is_err());
        assert_eq!(
            format!("{}", res.unwrap_err()), 
            "unexpected hex color format: expected(Hex code must be `7` characters long. Example: `#00FF00`), got(Hex code is `5` characters long!)"
        );
    }

    #[test]
    fn test_invalid_hex_with_parse_error() {
        let hex_value = "#0qfd00";
        let res = parse_hex(hex_value);
        assert!(res.is_err());
        assert_eq!(format!("{}", res.unwrap_err()), "couldn't parse hex value: invalid digit found in string");
    }

    #[test]
    fn test_valid_hex() {
        let hex_value = "#00ff00";
        let res = parse_hex(hex_value);        
        assert!(parse_hex(hex_value).is_ok());
        assert_eq!(res.unwrap(), vec![0, 255, 0]);
    }
}
