use super::ImageFilter;
use crate::{formats::ImageFormat, models::Pixel, utils::calculate_gaussian_kernel};

/// Performs a gaussian blur on the image
/// # Panics
/// This will panic if the size provided is not odd
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
        let kernel = calculate_gaussian_kernel(self.intensity, self.kernel_size);
        let half: isize = self.kernel_size as isize / 2;

        let width = img.get_width() as isize;
        let height = img.get_height() as isize;

        for i in 0..width {
            for j in 0..height {
                let mut r = 0f32;
                let mut g = 0f32;
                let mut b = 0f32;

                for k in -half..=half {
                    for l in -half..=half {
                        let x = (i + k).clamp(0, width - 1) as usize;
                        let y = (j + l).clamp(0, height - 1) as usize;
                        let kernel_x = (k + half) as usize;
                        let kernel_y = (l + half) as usize;

                        if let Some(pixel) = img.get_pixel(x, y) {
                            let weight = kernel[kernel_x][kernel_y];
                            r += pixel.r as f32 * weight;
                            g += pixel.g as f32 * weight;
                            b += pixel.b as f32 * weight;
                        }
                    }
                }

                let px = Pixel {
                    r: r.round().clamp(0.0, 255.0) as u8,
                    g: g.round().clamp(0.0, 255.0) as u8,
                    b: b.round().clamp(0.0, 255.0) as u8,
                };

                img.set_pixel(i as usize, j as usize, px);
            }
        }
    }
}