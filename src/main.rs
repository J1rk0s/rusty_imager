use rusty_imager::{formats::{bmp::Bmp, format::ImageFormat}, Image};

fn main() -> () {
    let img = Image::load_file("C:/Users/Kris/Pictures/MegaRender.bmp").unwrap();
    println!("{}", img.get_metadata());
    println!("{:?}", img.get_pixel(0, 0).unwrap());
}