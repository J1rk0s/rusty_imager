use crate::models::Pixel;

pub trait ImageFormat {
    fn get_pixel(&self, x: usize, y: usize) -> Option<&Pixel>;
    fn get_size(&self) -> u32;
    fn get_signature(&self) -> String;
    fn get_metadata(&self) -> String;
}