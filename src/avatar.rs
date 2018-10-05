//! Avatar module helps to generate avatars according to the initial names.
use rusttype::{point, Font, Scale};
use image::{DynamicImage, Rgba, ImageBuffer};
use std::io::prelude::*;
use std::fs::File;
use std::cmp;
use hex;
use contrast;


/// Avatar builder that stores the metrics of the image.
#[derive(Debug)]
pub struct AvatarBuilder {
    /// Initials name string
    pub name: String,
    /// Vectorized font data
    pub font_data: Vec<u8>,
    /// Scale of the font
    pub font_scale: Scale,
    /// RGB color of the font
    pub font_color: Vec<i64>,
    /// RGB color of the background
    pub background_color: Vec<i64>,
    /// Size of the inner-text
    pub length: usize,
    /// Width of the avatar
    pub width: u32,
    /// Height of the avatar
    pub height: u32,
    /// Contrast ratio for the colors
    pub contrast_ratio: f32,
    /// Private property to hold if colors should be randomly generated
    randomized_colors: (bool, bool)
}

/// Generate vectorized RGB color with range 0 to 255
fn generate_random_rgb() -> Vec<i64> {
    use rand::prelude::*;
        
    let mut rng = thread_rng();
    let mut vec = Vec::with_capacity(3);
    for _ in 0..3 {
        vec.push(rng.gen_range(0, 256));
    }
    vec
}


impl AvatarBuilder {
    /// Construct new AvatarBuilder.
    pub fn new(name: &str) -> AvatarBuilder {
        // unwrap first chars for the each word and store them
        // inside the <String>
        let mut text = String::new();
        for word in name.split_whitespace() {
            text.push(word.chars().next().unwrap());
        }

        // default Avatar settings
        AvatarBuilder {
            name: text,
            font_data: include_bytes!("fonts/Hiragino_Sans_GB_W3.ttf").to_vec(),
            font_scale: Scale::uniform(150.0),
            length: 2,
            width: 300,
            height: 300,
            randomized_colors: (true, true),
            contrast_ratio: 4.5,
            font_color: vec![255, 255, 255], // default white color
            background_color: vec![224, 143, 112], // default background
        }
    }

    /// Change the font of the avatar text. You need to include `.ttf` file.
    /// Default style is `Hiragino_Sans`.
    pub fn with_font(mut self, font: &str) -> Self {
        let mut f = File::open(font).unwrap_or_else(|e| {
            panic!("failed to open file: {}", e);
        });
        let mut font_data = Vec::new();
        f.read_to_end(&mut font_data).expect("unable to read data");
        self.font_data = font_data;
        self
    }

    /// Change the font color. You need to specify hex color code.
    pub fn with_font_color(mut self, color: &str) -> Self {
        let font_color = hex::parse_hex(color).unwrap_or_else(|e| {
            panic!("failed to parse: {}", e);
        });
        self.font_color = font_color;
        self.randomized_colors.0 = false;
        self
    }

    /// Change the uniform scale of font.
    /// Default to `150.0`.
    pub fn with_font_scale(mut self, scale: f32) -> Self {
        self.font_scale = Scale::uniform(scale);
        self
    }

    /// Change the background color of the avatar. You need to specify hex color code.
    pub fn with_background_color(mut self, color: &str) -> Self {
        let background_color = hex::parse_hex(color).unwrap_or_else(|e| {
            panic!("failed to parse: {}", e);
        });
        self.background_color = background_color;
        self.randomized_colors.1 = false;
        self
    }

    /// Change the length of initials characters taken from the name.
    /// Default to `2`. 
    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }

    /// Change the width of the avatar.
    /// Default to `300`. 
    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Change the height of the avatar.
    /// Default to `300`. 
    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Change the contrast ratio for the randomly generated avatar.
    /// Default to `4.5`. Increase the ratio for more clear avatar.
    pub fn with_contrast_ratio(mut self, ratio: f32) -> Self {
        self.contrast_ratio = ratio;
        self
    }


    /// Draw the image according to the metrics given.
    pub fn draw(self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        // convert font-data vector to rusttype::Font
        let font = Font::from_bytes(&self.font_data as &[u8]).expect("Error constructing Font");

        // substract metrics from the font according to the font scale
        let v_metrics = font.v_metrics(self.font_scale);

        // get the number of characters from the given name
        let text: String = self.name
            .chars()
            .take(cmp::min(self.length, self.name.len()))
            .collect();

        // layout the glyphs
        let glyphs: Vec<_> = font
            .layout(&text, self.font_scale, point(0.0, v_metrics.ascent))
            .collect();

        // substract height/width from the glyphs
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as u32;

        // calculate padding for glyphs
        let left_padding = (self.width - glyphs_width) / 2;
        let top_padding = (self.height - glyphs_height) / 2;

        // create dynamic RGBA image
        let mut image = DynamicImage::new_rgba8(self.width, self.height).to_rgba();

        // randomize colors if not being settled
        let mut colors = self.randomized_colors.clone();
        let mut background_color = self.background_color.clone();
        let mut font_color = self.font_color.clone();
        loop {
            match colors {
                (false, false) => break,
                (_, _) => {
                    if colors.0 {
                        font_color = generate_random_rgb();
                    }

                    if colors.1 {
                        background_color = generate_random_rgb();
                    }

                    colors = match contrast::find_ratio(&font_color, &background_color) {
                        // match if contrast ratio between colors is as expected
                        r if r > self.contrast_ratio || r < 1. / self.contrast_ratio => (false, false),
                        _ => {
                            if colors.0 | colors.1 {
                                colors
                            } else {
                                (false, true)
                            }
                        }
                    }
                },
            }
        }


        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // draw the glyph into the image according to font color
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        x + bounding_box.min.x as u32 + left_padding,
                        y + bounding_box.min.y as u32 + top_padding,
                        Rgba {
                            data: [
                                font_color[0] as u8, 
                                font_color[1] as u8, 
                                font_color[2] as u8, 
                                (v * 255.0) as u8
                            ],
                        },
                    )
                });
            }
        }

        for (_, _, pixel) in image.enumerate_pixels_mut() {
            // put background pixels for the uncovered alpha channels
            if pixel.data[3] == 0 {
                *pixel = Rgba {
                    data: [
                        background_color[0] as u8,
                        background_color[1] as u8,
                        background_color[2] as u8,
                        255
                    ]
                }
            }
        }
        image
    }
}
