use rusty_imager::{filters::GaussianBlur, formats::ImageFormat, Image};

fn main() -> () {
    let mut img = Image::load_file("data/reference.bmp").unwrap();
    img.apply_filter(GaussianBlur::new(2));
}