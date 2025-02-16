use crate::models::Pixel;

use super::{Grayscale, ImageFilter};

pub struct EdgeDetection {}

impl EdgeDetection {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {  }
    }
}

impl ImageFilter for EdgeDetection {
    // TODO: Fix this
    #[allow(unreachable_code)]
    fn apply(&self, img: &mut Box<dyn crate::formats::ImageFormat>) {
        todo!();
        Grayscale::new().apply(img);

        let width = img.get_width() as isize;
        let height = img.get_height() as isize;

        let mut g_max = 0f32;

        for i in 0..width {
            for j in 0..height {
                let sobel_x = EdgeDetection::sobel_x(img, i, j);
                let sobel_y = EdgeDetection::sobel_y(img, i, j); 

                let g = (sobel_x.powi(2) + sobel_y.powi(2)).sqrt();
                g_max = g_max.max(g);               
            }
        }

        for i in 0..width {
            for j in 0..height {
                let sobel_x = EdgeDetection::sobel_x(img, i, j);
                let sobel_y = EdgeDetection::sobel_y(img, i, j); 

                let g = (sobel_x.powi(2) + sobel_y.powi(2)).sqrt();
                let normalized = ((g * 255f32) / g_max).min(255f32) as u8;

                let px = Pixel {
                    r: normalized,
                    g: normalized,
                    b: normalized
                };

                img.set_pixel(i as usize, j as usize, px);
            }
        }
    }
}

impl EdgeDetection {
    fn sobel_x(img: &Box<dyn crate::formats::ImageFormat>, x: isize, y: isize) -> f32 {
        let x_kernel = vec![
            vec![-1, 0, 1],
            vec![-2, 0, 2],
            vec![-1, 0, 1]
        ];
    
        let mut gx = 0.0;
    
        for i in -1 as isize..=1 {
            for j in -1 as isize..=1 {
                if let Some(pixel) = img.get_pixel((x + i) as usize, (y + j) as usize) {
                    gx += pixel.r as f32 * x_kernel[(i + 1) as usize][(j + 1) as usize] as f32;
                }
            }
        }
    
        gx
    }

    fn sobel_y(img: &Box<dyn crate::formats::ImageFormat>, x: isize, y: isize) -> f32 {
        let y_kernel = vec![
            vec![-1, -2, -1],
            vec![0, 0, 0],
            vec![-1, -2, -1]
        ];
    
        let mut gy = 0.0;
    
        for i in -1 as isize..=1 {
            for j in -1 as isize..=1 {
                if let Some(pixel) = img.get_pixel((x + i) as usize, (y + j) as usize) {
                    gy += pixel.r as f32 * y_kernel[(i + 1) as usize][(j + 1) as usize] as f32;
                }
            }
        }
    
        gy
    }
}