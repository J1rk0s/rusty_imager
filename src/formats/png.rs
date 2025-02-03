use super::format::ImageFormat;
use crate::models::Pixel;

pub struct Png {

}

impl ImageFormat for Png {
    fn get_pixel(&self, x: u32, y: u32) -> Option<Pixel> {
        todo!()
    }

    fn get_size(&self) -> u32 {
        todo!()
    }

    fn get_signature(&self) -> String {
        todo!()
    }
}