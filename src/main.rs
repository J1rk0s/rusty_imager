use rusty_imager::Image;

fn main() -> () {
    let mut img = Image::from_file("data/reference.bmp").unwrap();

    img.save("test.bmp").unwrap();
}