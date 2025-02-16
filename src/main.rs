use rusty_imager::{filters::GaussianBlur, Image};

fn main() -> () {
    let mut img = Image::from_file("data/reference.bmp").unwrap();
    img.apply_filter(GaussianBlur::new(10f32, 7));

    img.save("test.bmp").unwrap();
}