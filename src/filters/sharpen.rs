use crate::models::Pixel;

use super::ImageFilter;

/// Performs a simple sharpening filter on the image
pub struct Sharpen {
    intensity: u32
}

impl Sharpen {
    pub fn new(intensity: u32) -> Self {
        Self { intensity }
    }
}

impl ImageFilter for Sharpen {
    fn apply(&self, img: &mut Box<dyn crate::formats::ImageFormat>) {
        let kernel = vec![
            vec![-1, -1, -1],
            vec![-1, self.intensity as i32, -1],
            vec![-1, -1, -1]
        ];

        let half: isize = kernel.len() as isize / 2;

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
                            r += pixel.r as f32 * weight as f32;
                            g += pixel.g as f32 * weight as f32;
                            b += pixel.b as f32 * weight as f32;
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