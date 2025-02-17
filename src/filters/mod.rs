//! Contains some basic filters

mod filter;
mod gaussian_blur;
mod color_inversion;
mod grayscale;
mod contrast;
mod brightness;
mod threshold;
mod box_blur;
mod edge_detection;
mod emboss;
mod oil_painting;
mod sharpen;

pub use filter::ImageFilter;
pub use gaussian_blur::GaussianBlur;
pub use color_inversion::ColorInversion;
pub use grayscale::Grayscale;
pub use contrast::Contrast;
pub use brightness::Brightness;
pub use threshold::Threshold;
pub use box_blur::BoxBlur;
pub use edge_detection::EdgeDetection;
pub use emboss::Emboss;
pub use oil_painting::OilPainting;
pub use sharpen::Sharpen;