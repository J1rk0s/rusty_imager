use crate::models::{Colors, Pixel};

use super::ImageFilter;

pub struct EdgeDetection {
    threshold: f32,
    multiplier: f32
}

impl EdgeDetection {
    #[allow(dead_code)]
    pub fn new(threshold: f32, multiplier: f32) -> Self {
        Self { 
            threshold, multiplier
        }
    }
}

impl ImageFilter for EdgeDetection {
    // TODO: Fix this
    fn apply(&self, img: &mut Box<dyn crate::formats::ImageFormat>) {
        let width = img.get_width() as isize;
        let height = img.get_height() as isize;

        let mut g_max = 0f32;
        //let mut intensities: Vec<f32> = vec![];

        for i in 0..width {
            for j in 0..height {
                let sobel_x = EdgeDetection::sobel_x(img, i, j);
                let sobel_y = EdgeDetection::sobel_y(img, i, j); 

                let g = (sobel_x.powi(2) + sobel_y.powi(2)).sqrt();
                //intensities.push(g);
                g_max = g_max.max(g);               
            }
        }

        //let threshold = (intensities.iter().sum::<f32>() / intensities.len() as f32) as u8;

        for i in 0..width {
            for j in 0..height {
                let sobel_x = EdgeDetection::sobel_x(img, i, j);
                let sobel_y = EdgeDetection::sobel_y(img, i, j); 

                let g = (sobel_x.powi(2) + sobel_y.powi(2)).sqrt();
                let normalized = (((g / g_max) * 255f32) * self.multiplier).min(255f32) as u8;
                let threshold = g_max * self.threshold;

                // let px: Pixel = Pixel {
                //     r: normalized,
                //     g: normalized,
                //     b: normalized
                // };
                let px: Pixel;

                if g > threshold {
                    px = Pixel {
                        r: normalized,
                        g: normalized,
                        b: normalized
                    };
                } else {
                    px = Colors::BLACK;
                }

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
                    let kernel_val = x_kernel[(i + 1) as usize][(j + 1) as usize] as f32;
                    if kernel_val != 0.0 {
                        gx += pixel.r as f32 * kernel_val;
                    }
                }
            }
        }
    
        gx
    }

    fn sobel_y(img: &Box<dyn crate::formats::ImageFormat>, x: isize, y: isize) -> f32 {
        let y_kernel = vec![
            vec![-1, -2, -1],
            vec![0, 0, 0],
            vec![1, 2, 1]
        ];
    
        let mut gy = 0.0;
    
        for i in -1 as isize..=1 {
            for j in -1 as isize..=1 {
                if let Some(pixel) = img.get_pixel((x + i) as usize, (y + j) as usize) {
                    let kernel_val = y_kernel[(i + 1) as usize][(j + 1) as usize] as f32;
                    if kernel_val != 0.0 {
                        gy += pixel.r as f32 * kernel_val;
                    }
                }
            }
        }
    
        gy
    }
}