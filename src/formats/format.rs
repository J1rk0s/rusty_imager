use crate::models::Pixel;

pub trait ImageFormat {
    fn get_pixel(&self, x: u32, y: u32) -> Option<Pixel>;
    fn get_size(&self) -> u32;
    fn get_signature(&self) -> String;
    fn get_header(&self) -> String;
}