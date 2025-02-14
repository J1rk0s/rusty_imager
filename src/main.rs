use rusty_imager::Image;

fn main() -> () {
    let img = Image::load_file("data/reference.bmp").unwrap();
    img.save("output.bmp").unwrap();
}