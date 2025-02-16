use std::collections::HashMap;

use crate::models::Pixel;

use super::ImageFilter;

pub struct OilPainting {
    size: u32
}

impl OilPainting {
    #[allow(dead_code)]
    pub fn new(size: u32) -> Self { Self { size } }
}

impl ImageFilter for OilPainting {
    fn apply(&self, img: &mut Box<dyn crate::formats::ImageFormat>) {
        assert!(self.size % 2 != 0, "Size must be odd");

        let half: isize = self.size as isize / 2;
        let width = img.get_width() as isize;
        let height = img.get_height() as isize;

        for i in 0..width {
            for j in 0..height {
                let mut freq: HashMap<Pixel, u32> = HashMap::new();

                for k in -half..=half {
                    for l in -half..=half {
                        let x = (i + k).clamp(0, width - 1) as usize;
                        let y = (j + l).clamp(0, height - 1) as usize;

                        if let Some(pixel) = img.get_pixel(x, y) {
                            *freq.entry(pixel.clone()).or_insert(0) += 1;
                        }
                    }
                }

                let common = freq.iter().max_by_key(|x| x.1).unwrap();

                img.set_pixel(i as usize, j as usize, common.0.clone());
            }
        }
    }
}