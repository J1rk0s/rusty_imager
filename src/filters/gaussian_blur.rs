use super::ImageFilter;
use crate::formats::ImageFormat;

#[allow(dead_code)]
pub struct GaussianBlur {
    intensity: u16
}

impl GaussianBlur {
    pub fn new(intensity: u16) -> Self {
        Self {
            intensity
        }
    }
}

impl ImageFilter for GaussianBlur {
    fn apply(&self, img: &mut Box<dyn ImageFormat>) {
        println!("{}", img.get_metadata())
    }
}