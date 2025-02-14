use crate::{formats::ImageFormat, models::Pixel};

pub trait ImageFilter {
    fn apply(&self, img: &mut Box<dyn ImageFormat>);
}