use rusty_imager::{filters::Grayscale, Image};

fn main() -> () {
    let mut img = Image::from_file("data/reference.bmp").unwrap();
    img.apply_filter(Grayscale::new());

    img.save("test.bmp").unwrap();
}