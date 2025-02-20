use rusty_imager::Image;

fn main() {
    let img = Image::from_file("data/reference.png").unwrap();

    //img.save("test.png").unwrap();
}