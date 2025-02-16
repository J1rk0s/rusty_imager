use rusty_imager::{filters::Emboss, Image};

fn main() -> () {
    let mut img = Image::from_file("data/reference.bmp").unwrap();
    img.apply_filter(Emboss::new());

    img.save("test.bmp").unwrap();
}