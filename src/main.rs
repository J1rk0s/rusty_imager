use rusty_imager::{formats::format::ImageFormat, Image};

fn main() -> () {
    let img = Image::load_file("data/reference.bmp").unwrap();

    println!("{}", img.get_metadata());
    println!("{:?}", img.get_pixel(1, 0).unwrap());
}