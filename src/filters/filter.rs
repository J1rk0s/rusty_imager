use crate::formats::ImageFormat;

/// All filters must implement this trait
pub trait ImageFilter {
    fn apply(&self, img: &mut Box<dyn ImageFormat>);
}