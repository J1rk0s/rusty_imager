use crate::models::Colors;

use super::ImageFilter;

pub struct Threshold {
    treshold: u16 
}

impl Threshold {
    pub fn new(treshold: u16) -> Self {
        Self {
            treshold
        }
    }
}

impl ImageFilter for Threshold {
    fn apply(&self, img: &mut Box<dyn crate::formats::ImageFormat>) {
        for i in 0..img.get_width() {
            for j in 0..img.get_height() {
                if let Some(pixel) = img.get_pixel(i, j) {
                    let color = (pixel.r as u16 + pixel.g as u16 + pixel.b as u16) / 3;

                    if color > self.treshold {
                        img.set_pixel(i, j, Colors::WHITE);
                    } else {
                        img.set_pixel(i, j, Colors::BLACK);
                    }
                }
            }
        }
    }
}