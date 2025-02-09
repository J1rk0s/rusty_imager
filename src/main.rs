use rusty_imager::{formats::format::ImageFormat, Image};

fn main() -> () {
    let img = Image::load_file("data/prototype.bmp").unwrap();
    println!("{}", img.get_metadata());
    println!("{:?}", img.get_pixel(0, 0).unwrap());
}