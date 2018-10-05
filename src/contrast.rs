//! Contrast module helps to calculate constrast ratio of two RGB color.
fn calculate_luminance(rgb: &Vec<i64>) -> f32 {
    0.299 * rgb[0] as f32 + 0.587 * rgb[1] as f32 + 0.114 * rgb[2] as f32
}

pub fn find_ratio(font_rgb: &Vec<i64>, background_rgb: &Vec<i64>) -> f32 {
    (calculate_luminance(font_rgb) + 0.05) / (calculate_luminance(background_rgb) + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contrast_ratio() {
        let rgb_white = vec![255, 255, 255];
        let rgb_yellow = vec![255, 255, 0];
        assert_eq!(find_ratio(&rgb_white, &rgb_yellow).floor(), 1.);
        let rgb_blue =  vec![0, 0, 255];
        assert_eq!(find_ratio(&rgb_white, &rgb_blue).floor(), 8.);
    }
}