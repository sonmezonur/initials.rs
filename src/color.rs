//! An example of opening an image.
extern crate image;

// TODO: change it
use color::image::Rgba;

use std::collections::HashMap;
use std::collections::hash_map::Entry;


/// `AvatarColor` stores colors for the avatar that will be exported.
#[derive(Debug)]
pub struct AvatarColor {
    pub colors: HashMap<String, Rgba<u8>>,
    pub default: String, 
}

impl AvatarColor {
    /// Creates a new AvatarColor object which uses image::Rgba structs to store allowed colors for the avatar.
    ///
    /// Note that default color encoding is '45VDF3' 
    fn new() -> AvatarColor {
        // inializing the AvatarColor struct
        AvatarColor { 
            colors: HashMap::<String, Rgba<u8>>::new(),
            default: String::from("45VDF3"),
        }
    }

    /// Map pixel slices to specific key while returning the mutable reference of object itself.
    fn insert<'a>(&'a mut self, arg: &str, values: [u8; 4]) -> &'a mut AvatarColor {
        match self.colors.entry(arg.to_string()) {
            // if the entry is occupied, do nothing.
            Entry::Occupied(_) => (),
            // if the entry is vacant, insert 'values'.
            Entry::Vacant(_) => {
                self.colors.insert(
                    String::from(arg),
                    Rgba { data: values }
                );
            },
        }
        self 
    }
}

/// Insert colors by wrapping `image::Rgba`.
pub fn build_colors() -> AvatarColor {
    let mut color_map = AvatarColor::new();
    color_map.insert("45BDF3", [69, 189, 243, 255])
        .insert("E08F70", [224, 143, 112, 255])
        .insert("4DB6AC", [77, 182, 172, 255])
        .insert("9575CD", [149, 117, 205, 255])
        .insert("B0855E", [176, 133, 94, 255])
        .insert("F06292", [240, 98, 146, 255])
        .insert("A3D36C", [163, 211, 108, 255])
        .insert("7986CB", [121, 134, 203, 255])
        .insert("F1B91D", [241, 185, 29, 255]);
    color_map
}