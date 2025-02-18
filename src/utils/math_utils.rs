use std::f32::consts::{E, PI};

/// Calculates the gaussian kernel from given sigma and size
pub fn calculate_gaussian_kernel(sigma: f32, size: u32) -> Vec<Vec<f32>> {
    assert!(size % 2 != 0);

    let mut res: Vec<Vec<f32>> = vec![vec![0f32; size as usize]; size as usize];
    let half: i32 = size as i32 / 2;
    let mut sum: f32 = 0f32;

    for i in -half..=half {
        for j in -half..=half {
            let x = i as f32;
            let y = j as f32;
            let gaus: f32 = (1.0 / (2.0 * PI * sigma.powi(2))) * E.powf(-(x * x + y * y) / (2.0 * sigma.powi(2))); 
            
            res[(i + half) as usize][(j + half) as usize] = gaus;
            sum += gaus;
        }
    }

    // Normalization
    for i in 0..size as usize {
        for j in 0..size as usize {
            res[i][j] /= sum;
        }
    }

    res
}