use crate::models::Pixel;

use super::ImageFilter;

pub struct Grayscale;

impl Grayscale {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl ImageFilter for Grayscale {
    fn apply(&self, img: &mut Box<dyn crate::formats::ImageFormat>) {
        for i in 0..img.get_width() {
            for j in 0..img.get_height() {
                if let Some(pixel) = img.get_pixel(i, j) {
                    let gray = ((pixel.r as u16 + pixel.g as u16 + pixel.b as u16) / 3) as u8;
                    let px = Pixel {
                        r: gray,
                        g: gray,
                        b: gray
                    };

                    img.set_pixel(i, j, px);
                }
            }
        }
    }
}