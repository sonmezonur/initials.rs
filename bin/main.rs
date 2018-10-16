extern crate initials;

use initials::{AvatarBuilder, AvatarResult};

fn avatar() -> AvatarResult {
    AvatarBuilder::new("A")
        .with_length(2)?
        .with_contrast_ratio(3.)
}

pub fn main() {
    let avatar = avatar().unwrap();
    let image = avatar.draw();
    image.save("test.jpg").unwrap();
}