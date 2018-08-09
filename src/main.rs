pub mod color; // content of network.rs

fn main() {
    let color_map = color::build_colors();
    println!("type : {:?}", color_map);
    println!("Default avatar color: {}", color_map.default)

}