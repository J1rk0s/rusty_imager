use rusty_imager::{filters::OilPainting, Image};

fn main() -> () {
    let mut img = Image::from_file("data/reference.bmo").unwrap();
    img.apply_filter(OilPainting::new(3));

    img.save("test.bmp").unwrap();
}