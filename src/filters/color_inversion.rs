use super::ImageFilter;

pub struct ColorInversion {}

impl ColorInversion {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {  }
    }
}

impl ImageFilter for ColorInversion {
    fn apply(&self, img: &mut Box<dyn crate::formats::ImageFormat>) {
        for i in 0..img.get_width() {
            for j in 0..img.get_height() {
                if let Some(pixel) = img.get_pixel(i, j) {
                    img.set_pixel(i, j, pixel.invert());
                }
            }
        }
    }
}