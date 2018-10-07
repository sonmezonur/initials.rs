extern crate initials;

use initials::AvatarBuilder;

pub fn main() {
    let image = AvatarBuilder::new("Anakin Skywalker")
        .draw();
    image.save("minimal.jpg").unwrap();
}