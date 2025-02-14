use crate::formats::ImageFormat;

pub trait ImageFilter {
    fn apply(&self, img: &mut Box<dyn ImageFormat>);
}