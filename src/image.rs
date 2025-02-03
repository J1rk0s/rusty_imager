use std::{fs::read, path::Path};

use crate::formats::{bmp::Bmp, format::ImageFormat};
use crate::models::Pixel;

pub struct Image {
    pub raw: Box<dyn ImageFormat>
}

impl Image {
    pub fn load_file(path: &str) -> Option<Self> {
        let p = Path::new(path);
        let ext = p.extension()?.to_str()?;
        let data = read(p).ok()?;

        match ext {
            "bmp" => {
                Some(Self {
                    raw: Box::new(Bmp::parse(&data)?)
                })
            }

            _ => {
                None
            }
        }
        
    }
}

impl ImageFormat for Image {
    fn get_pixel(&self, x: u32, y: u32) -> Option<Pixel> {
        self.raw.get_pixel(x, y)
    }

    fn get_size(&self) -> u32 {
        self.raw.get_size()
    }

    fn get_signature(&self) -> String {
        self.raw.get_signature()
    }
}