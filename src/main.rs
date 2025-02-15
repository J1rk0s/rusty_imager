use rusty_imager::{filters::Contrast, Image};

fn main() -> () {
    let mut img = Image::from_file("data/reference.bmp").unwrap();
    img.apply_filter(Contrast::new(1.3f32));

    img.save("test.bmp").unwrap();
}