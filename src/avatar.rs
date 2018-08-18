use rusttype::{point, Font, Scale};
use image::{DynamicImage, Rgba, ImageBuffer};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use hex;


#[derive(Debug)]
pub struct AvatarBuilder {
    pub name: String,
    pub font_data: Vec<u8>,
    pub font_scale: Scale,
    pub font_color: Vec<i64>,
    pub background_color: Vec<i64>,
    pub padding: f32,
    pub length: u8,
    pub width: u32,
    pub height: u32
}

impl AvatarBuilder {
    pub fn new(name: &str) -> AvatarBuilder {
        let mut text = String::new();
        for word in name.split_whitespace() {
            text.push(word.chars().next().unwrap());
        }
        AvatarBuilder {
            name: text,
            font_data: include_bytes!("fonts/Hiragino_Sans_GB_W3.ttf").to_vec(),
            font_scale: Scale::uniform(500.0),
            padding: 20.0,
            length: 1,
            width: 800,
            height: 800,
            font_color: vec![255, 255, 255], // default white color
            background_color: vec![224, 143, 112], // default background
        }
    }

    pub fn with_font(mut self, font: &str) -> Self {
        let mut f = File::open(font).unwrap();
        let mut font_data = vec![];
        f.read_to_end(&mut font_data).expect("Unable to read data");
        self.font_data = font_data;
        self
    }

    pub fn with_font_color(mut self, color: &str) -> Self {
        let font_color = hex::parse_hex(color).unwrap_or_else(|e| {
            panic!("failed to parse: {}", e);
        });
        self.font_color = font_color;
        self
    }

    pub fn with_font_scale(mut self, scale: f32) -> Self {
        self.font_scale = Scale::uniform(scale);
        self
    }

    pub fn with_background_color(mut self, color: &str) -> Self {
        let font_color = hex::parse_hex(color).unwrap_or_else(|e| {
            panic!("failed to parse: {}", e);
        });
        self.background_color = font_color;
        self
    }

    pub fn with_length(mut self, length: u8) -> Self {
        self.length = length;
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn draw(self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        // This only succeeds if collection consists of one font
        let font = Font::from_bytes(&self.font_data as &[u8]).expect("Error constructing Font");

        let v_metrics = font.v_metrics(self.font_scale);
        
        // layout the glyphs
        let glyphs: Vec<_> = font
            .layout(&self.name, self.font_scale, point(self.padding, self.padding + v_metrics.ascent))
            .collect();


        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as u32
        };

        let mut image = DynamicImage::new_rgba8(self.width, self.height).to_rgba();

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        Rgba {
                            data: [
                                self.font_color[0] as u8, 
                                self.font_color[1] as u8, 
                                self.font_color[2] as u8, 
                                (v * 255.0) as u8
                            ],
                        },
                    )
                });
            }
        }

        for (_, _, pixel) in image.enumerate_pixels_mut() {
            if pixel.data[3] == 0 {
                *pixel = Rgba { 
                    data: [
                        self.background_color[0] as u8, 
                        self.background_color[1] as u8, 
                        self.background_color[2] as u8, 
                        255
                    ] 
                }
            }
        }
        image
    }
}