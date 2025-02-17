use crate::models::Pixel;

use super::ImageFilter;

/// Allows you to increase the image brightness
pub struct Brightness {
    intensity: u8
}

impl Brightness {
    pub fn new(intensity: u8) -> Self {
        Self {
            intensity
        }
    }
}

impl ImageFilter for Brightness {
    fn apply(&self, img: &mut Box<dyn crate::formats::ImageFormat>) {
        for i in 0..img.get_width() {
            for j in 0..img.get_height() {
                if let Some(pixel) = img.get_pixel(i, j) {
                    let r = pixel.r.saturating_add(self.intensity);
                    let g = pixel.g.saturating_add(self.intensity);
                    let b = pixel.b.saturating_add(self.intensity);

                    let px = Pixel {
                        r, g, b
                    };

                    img.set_pixel(i, j, px);
                }
            }
        }
    }
}