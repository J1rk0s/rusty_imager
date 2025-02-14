use rusty_imager::{filters::ColorInversion, Image};

fn main() -> () {
    let mut img = Image::from_file("data/reference.bmp").unwrap();
    img.apply_filter(ColorInversion::new());

    img.save("test.bmp").unwrap();
}