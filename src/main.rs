use rusty_imager::{filters::Grayscale, formats::ImageFormat, Image};

fn main() {
    let mut img = Image::from_file("data/reference.png").unwrap();
    img.apply_filter(Grayscale::new());
    img.save("test.png").unwrap();
}