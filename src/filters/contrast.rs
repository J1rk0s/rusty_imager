use crate::models::Pixel;

use super::ImageFilter;

/// Allows you to increase the image contrast
pub struct Contrast {
    intensity: f32
}

impl Contrast {
    #[allow(dead_code)]
    pub fn new(intensity: f32) -> Self {
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
                    let r = (((pixel.r as f32 - 128f32) * self.intensity) + 128f32) as u8;
                    let g = (((pixel.g as f32 - 128f32) * self.intensity) + 128f32) as u8;
                    let b = (((pixel.b as f32 - 128f32) * self.intensity) + 128f32) as u8;

                    let px = Pixel {
                        r, g, b
                    };

                    img.set_pixel(i, j, px);
                }
            }
        }
    }
}