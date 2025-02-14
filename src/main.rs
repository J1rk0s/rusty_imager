use rusty_imager::{filters::ColorInversion, Image};

fn main() -> () {
    let mut img = Image::load_file("data/reference.bmp").unwrap();
    img.apply_filter(ColorInversion::new());

    img.save("test.bmp").unwrap();
}