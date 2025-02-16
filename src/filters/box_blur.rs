use crate::models::Pixel;

use super::ImageFilter;

pub struct BoxBlur {
    size: usize
}

impl BoxBlur {
    pub fn new(size: usize) -> Self {
        assert!(size % 2 != 0, "Size must be odd");

        Self {
            size
        }
    }
}

impl ImageFilter for BoxBlur {
    fn apply(&self, img: &mut Box<dyn crate::formats::ImageFormat>) {
        let half: isize = self.size as isize / 2;
        let width = img.get_width() as isize;
        let height = img.get_height() as isize;

        for i in 0..width {
            for j in 0..height {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;

                for k in -half..=half {
                    for l in -half..=half {
                        let x = (i + k).clamp(0, width - 1) as usize;
                        let y = (j + l).clamp(0, height - 1) as usize;

                        if let Some(pixel) = img.get_pixel(x, y) {
                            r += pixel.r as u32;
                            g += pixel.g as u32;
                            b += pixel.b as u32;
                        }
                    }
                }

                let size = self.size as u32 * self.size as u32; 
                let px = Pixel {
                    r: (r / size) as u8, 
                    g: (g / size) as u8, 
                    b: (b / size) as u8
                };

                img.set_pixel(i as usize, j as usize, px);
            }
        }
    }
}