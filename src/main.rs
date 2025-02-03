use rusty_imager::{formats::format::ImageFormat, Image};

fn main() -> () {
    let img = Image::load_file("C:/Users/Kris/Pictures/GameIcon.bmp").unwrap();
    println!("{}", img.get_signature())
}