//! Color module that helps generating and operating on rgb colors
use std::str::FromStr;
use std::u8;
use std::iter::FromIterator;
use image::Rgba;
use error::Error;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RgbColor(u8, u8, u8);

impl RgbColor {
    pub fn new(red: u8, green: u8, blue: u8) -> RgbColor {
        RgbColor(red, green, blue)
    }

    /// Generate a random rgb color
    pub fn random() -> Self {
        use rand::prelude::*;

        let mut rng = thread_rng();

        RgbColor(rng.gen(), rng.gen(), rng.gen())
    }

    /// Calculate the contrast ratio between colors
    pub fn find_ratio(&self, other: &RgbColor) -> f32 {
        self.calculate_luminance() / other.calculate_luminance()
    }

    /// Convert to rgba (including transparency) for image creation
    pub fn to_rgba(self, alpha: u8) -> Rgba<u8> {
        Rgba {
            data: [self.0, self.1, self.2, alpha]
        }
    }

    fn calculate_luminance(&self) -> f32 {
        0.299 * f32::from(self.0) +
        0.587 * f32::from(self.1) +
        0.114 * f32::from(self.2) +
        0.05
    }
}

/// Parse hex code and generate RGB vector accordingly.
impl FromStr for RgbColor {
    type Err = Error;

    fn from_str(hex: &str) -> Result<RgbColor, Error> {
        if hex.is_empty() {
            return Err(Error::InvalidHexFormat {
                expected: String::from("Color hex code must not be empty"),
                actual: String::from("Color hex was empty"),
            });
        }

        if !hex.starts_with('#') {
            return Err(Error::InvalidHexFormat {
                expected: String::from("Color hex code must start with `#`"),
                actual: format!("Color hex starts with `{}`", hex.chars().nth(0).unwrap())
            });
        }

        if hex.len() != 7 {
            return Err(Error::InvalidHexFormat {
                expected: String::from("Hex code must be `7` characters long. Example: `#00FF00`"),
                actual: format!("Hex code is `{}` characters long!", hex.len())
            });
        }

        // collect characters from the String
        let raw: Vec<char> = hex.chars().collect();

        Ok(RgbColor(
            u8::from_str_radix(&String::from_iter(&raw[1 .. 3]), 16)?,
            u8::from_str_radix(&String::from_iter(&raw[3 .. 5]), 16)?,
            u8::from_str_radix(&String::from_iter(&raw[5 .. 7]), 16)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_hex_empty_str() {
        let res: Result<RgbColor, Error> = "".parse();
        assert!(res.is_err());
        assert_eq!(
            format!("{}", res.unwrap_err()), 
            "unexpected hex color format: expected(Color hex code must not be empty), got(Color hex was empty)"
        );
    }

    #[test]
    fn test_invalid_hex_with_missing_prefix() {
        let res: Result<RgbColor, Error> = "00ff00".parse();
        assert!(res.is_err());
        assert_eq!(
            format!("{}", res.unwrap_err()), 
            "unexpected hex color format: expected(Color hex code must start with `#`), got(Color hex starts with `0`)"
        );
    }

    #[test]
    fn test_invalid_hex_with_wrong_size() {
        let res: Result<RgbColor, Error> = "#ff00".parse();
        assert!(res.is_err());
        assert_eq!(
            format!("{}", res.unwrap_err()), 
            "unexpected hex color format: expected(Hex code must be `7` characters long. Example: `#00FF00`), got(Hex code is `5` characters long!)"
        );
    }

    #[test]
    fn test_invalid_hex_with_parse_error() {
        let res: Result<RgbColor, Error> = "#0qfd00".parse();
        assert!(res.is_err());
        assert_eq!(format!("{}", res.unwrap_err()), "couldn't parse hex value: invalid digit found in string");
    }

    #[test]
    fn test_valid_hex() {
        let res: Result<RgbColor, Error> = "#00ff00".parse();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), RgbColor(0, 255, 0));
    }

    #[test]
    fn test_contrast_ratio() {
        let rgb_white = RgbColor(255, 255, 255);
        let rgb_yellow = RgbColor(255, 255, 0);
        assert_eq!(rgb_white.find_ratio(&rgb_yellow).floor(), 1.);
        let rgb_blue =  RgbColor(0, 0, 255);
        assert_eq!(rgb_white.find_ratio(&rgb_blue).floor(), 8.);
    }
}