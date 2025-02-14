use std::{fs, path::Path};

use crate::filters::ImageFilter;
use crate::formats::{bmp::Bmp, ImageFormat};
use crate::models::{ImageType, Pixel};

pub struct Image {
    raw: Box<dyn ImageFormat>
}

impl Image {
    /// Loads the image from specified path
    pub fn load_file(path: &str) -> Option<Self> {
        let p = Path::new(path);
        let ext = p.extension()?.to_str()?;
        let data = fs::read(p).ok()?;

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

    /// Loads the image from byte array
    /// # Example
    /// ```no_run
    /// use std::fs;
    /// use rusty_imager::models::ImageType;
    /// 
    /// let data = fs::read("<path>").unwrap();
    /// let img = Image::load_image(&data, ImageType::Bmp);
    /// // Do something with the image
    /// ```
    pub fn load_image(data: &[u8], image_type: ImageType) -> Option<Self> {
        match image_type {
            ImageType::Bmp => {
                Some(Self {
                    raw: Box::new(Bmp::parse(data)?)
                })
            }

            _ => {
                None
            }
        }
    }

    pub fn apply_filter(&mut self, filter: impl ImageFilter) {
        filter.apply(&mut self.raw);
    }

    // TODO: Add image saving
    // TODO: Add image filter reverting
    // TODO: Add image filter applying
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

    /// Replaces the pixel at x and y with the provided pixel
    /// * `x` - row
    /// * `y` - column
    /// 
    /// # Example
    /// ```no_run
    /// let img = Image::from_file("<path>");
    /// for x in img.get_width() {
    ///     for y in img.get_height() {
    ///         let pixel = img.get_pixel(x, y);
    ///         img.set_pixel(x, y, pixel);
    ///     }
    /// }
    /// ```
    fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) -> Option<()> {
        self.raw.set_pixel(x, y, pixel)
    }

    /// Gets the image file size from image header 
    fn get_size(&self) -> u32 {
        self.raw.get_size()
    }

    /// Gets the image header signature
    fn get_signature(&self) -> String {
        self.raw.get_signature()
    }

    /// Gets the image metadata (header)
    fn get_metadata(&self) -> String {
        self.raw.get_metadata()
    }

    /// Gets the image height
    /// # Example
    /// ```no_run
    /// let img = Image::from_file("<name>");
    /// for x in img.get_width() {
    ///     for y in img.get_height() {
    ///         // Do something
    ///     }
    /// }
    /// ```
    fn get_height(&self) -> usize {
        self.raw.get_height()
    }

    /// Gets the image height
    /// # Example
    /// ```no_run
    /// let img = Image::from_file("<name>");
    /// for x in img.get_width() {
    ///     for y in img.get_height() {
    ///         // Do something
    ///     }
    /// }
    /// ```
    fn get_width(&self) -> usize {
        self.raw.get_width()
    }
}