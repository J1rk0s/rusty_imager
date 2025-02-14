use crate::models::Pixel;

use super::ImageFilter;

pub struct Contrast {
    intensity: u8
}

impl Contrast {
    #[allow(dead_code)]
    pub fn new(intensity: u8) -> Self {
        Self {
            intensity
        }
    }
}

impl ImageFilter for Contrast {
    fn apply(&self, img: &mut Box<dyn crate::formats::ImageFormat>) {
        for i in 0..img.get_width() {
            for j in 0..img.get_height() {
                if let Some(pixel) = img.get_pixel(i, j) {
                    let r = ((pixel.r - 128) * self.intensity) + 128;
                    let g = ((pixel.g - 128) * self.intensity) + 128;
                    let b = ((pixel.b - 128) * self.intensity) + 128;

                    let px = Pixel {
                        r, g, b
                    };

                    img.set_pixel(i, j, px);
                }
            }
        }
    }
}