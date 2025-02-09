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
    /// Retrieves the pixel at x and y
    /// * `x` - row
    /// * `y` - column
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use rusty_imager::{formats::format::ImageFormat, Image};
    /// 
    /// let img = Image::load_file("<path>").unwrap();
    /// let pixel = img.get_pixel(0, 0);
    /// ```
    fn get_pixel(&self, x: usize, y: usize) -> Option<&Pixel> {
        self.raw.get_pixel(x, y)
    }

    fn get_size(&self) -> u32 {
        self.raw.get_size()
    }

    fn get_signature(&self) -> String {
        self.raw.get_signature()
    }

    fn get_header(&self) -> String {
        self.raw.get_header()
    }
}