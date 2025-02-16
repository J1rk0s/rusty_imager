use crate::models::Pixel;

use super::{Grayscale, ImageFilter};

pub struct Emboss {}

impl Emboss {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl ImageFilter for Emboss {
    #[allow(unreachable_code)]
    fn apply(&self, img: &mut Box<dyn crate::formats::ImageFormat>) {
        todo!();
        let width = img.get_width() as isize;
        let height = img.get_height() as isize;

        let kernel = vec![
            vec![1, 1, 0],
            vec![1, 0, -1],
            vec![0, -1, -1],
        ];

        for i in 0..width {
            for j in 0..height {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;

                for k in -1..=1 {
                    for l in -1..=1 {
                        let x = (i + k).clamp(0, width - 1) as usize;
                        let y = (j + l).clamp(0, height - 1) as usize;
                        let kernel_x = (k + 1).clamp(0, width - 1) as usize;
                        let kernel_y = (l + 1).clamp(0, height - 1) as usize;

                        if let Some(pixel) = img.get_pixel(x, y) {
                            let weight = kernel[kernel_x][kernel_y];
                            r += pixel.r as i32 * weight;
                            g += pixel.g as i32 * weight;
                            b += pixel.b as i32 * weight;
                        }
                    }
                }

                r = (r + 128).clamp(0, 255);
                g = (g + 128).clamp(0, 255);
                b = (b + 128).clamp(0, 255);

                let px = Pixel {
                    r: r as u8,
                    g: g as u8,
                    b: b as u8,
                };

                img.set_pixel(i as usize, j as usize, px);
            }
        }

        Grayscale::new().apply(img);
    }
}