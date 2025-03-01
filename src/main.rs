use rusty_imager::{formats::ImageFormat, Image};

fn main() {
    let img = Image::from_file("data/reference.png").unwrap();
    println!("{:?}", img.get_pixel(0, 2));
    //img.save("test.png").unwrap();
}