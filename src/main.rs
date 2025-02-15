use rusty_imager::{filters::{Contrast, GaussianBlur}, Image};

fn main() -> () {
    let mut img = Image::from_file("C:/Users/Kris/Pictures/MegaRender.bmp").unwrap();
    img.apply_filter(GaussianBlur::new(1f32, 3));

    img.save("test.bmp").unwrap();
}