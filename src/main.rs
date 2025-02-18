use rusty_imager::Image;

fn main() {
    let img = Image::from_file("data/reference.ppm").unwrap();
    //img.apply_filter(Grayscale::new());

    img.save("test.ppm").unwrap();
}