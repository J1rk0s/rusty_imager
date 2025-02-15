use super::ImageFilter;
use crate::{formats::ImageFormat, models::Pixel, utils::CalculateGaussianKernel};

#[allow(dead_code)]
pub struct GaussianBlur {
    intensity: f32,
    kernel_size: u32
}

impl GaussianBlur {
    pub fn new(intensity: f32, kernel_size: u32) -> Self {
        assert!(kernel_size % 2 != 0, "Size must be odd");

        Self {
            intensity,
            kernel_size
        }
    }
}

impl ImageFilter for GaussianBlur {
    fn apply(&self, img: &mut Box<dyn ImageFormat>) {
        let kernel = CalculateGaussianKernel(self.intensity, self.kernel_size);
        let half: usize = self.kernel_size as usize / 2;

        for i in 0..img.get_width() {
            for j in 0..img.get_height() {
                let mut r = 0f32;
                let mut g = 0f32;
                let mut b = 0f32;

                // TODO: Make this work
                for k in i.saturating_sub(half)..i+half {
                    for l in j.saturating_sub(half)..j+half {
                        if let Some(pixel) = img.get_pixel(k, l) {
                            println!("{} {}", k, l);
                            r += pixel.r as f32 * kernel[k][l];
                            g += pixel.g as f32 * kernel[k][l];
                            b += pixel.b as f32 * kernel[k][l];
                        }
                    }
                }

                let px = Pixel {
                    r: r as u8, g: g as u8, b: b as u8
                };

                img.set_pixel(i, j, px);
            }
        }
    }
}