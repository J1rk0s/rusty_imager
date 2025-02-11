use crate::models::Pixel;

pub trait ImageFormat {
    fn get_pixel(&self, x: usize, y: usize) -> Option<&Pixel>;
    fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) -> Option<()>;
    fn get_size(&self) -> u32;
    fn get_signature(&self) -> String;
    fn get_metadata(&self) -> String;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}