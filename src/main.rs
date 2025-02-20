use rusty_imager::{formats::ImageFormat, Image};

fn main() {
    let img = Image::from_file("data/reference.png").unwrap();
    println!("{}", img.get_height());
    //img.save("test.png").unwrap();
}