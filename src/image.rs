use std::{fs, path::Path};

use crate::filters::ImageFilter;
use crate::formats::{bmp::Bmp, ImageFormat};
use crate::models::{ImageType, Pixel};

pub struct Image {
    raw: Box<dyn ImageFormat>
}

impl Image {
    /// Loads the image from specified path
    /// # Example
    /// ```no_run
    /// use rusty_imager::Image;
    /// 
    /// let img = Image::from_file("<path>").expect("File not found!");
    /// ```
    pub fn from_file(path: &str) -> Option<Self> {
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
    /// use rusty_imager::Image;
    /// 
    /// let data = fs::read("<path>").expect("File not found!");
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
        }
    }

    /// Applies a filter to the image
    /// # Example
    /// ```no_run
    /// use rusty_imager::Image;
    /// use rusty_imager::filters::ColorInversion;
    /// 
    /// let mut img = Image::from_file("<path>").expect("File not found!");
    /// img.apply_filter(ColorInversion::new());
    /// ```
    pub fn apply_filter(&mut self, filter: impl ImageFilter) {
        // TODO: Fix artefacts
        filter.apply(&mut self.raw);
    }

    /// Saves the loaded image
    /// # Example
    /// ```no_run
    /// use rusty_imager::Image;
    /// use rusty_imager::filters::ColorInversion;
    /// 
    /// let mut img = Image::from_file("<path>").expect("File not found!");
    /// img.apply_filter(ColorInversion::new());
    /// 
    /// img.save("<name>.<ext>");
    /// ```
    pub fn save(self, path: &str) -> Result<(), std::io::Error>{
        let bytes = self.to_bytes();

        fs::write(path, bytes)?;

        Ok(())
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
    /// use rusty_imager::{formats::ImageFormat, Image};
    /// 
    /// let img = Image::from_file("<path>").expect("File not found!");
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
    /// use rusty_imager::{formats::ImageFormat, Image};
    /// 
    /// let mut img = Image::from_file("<path>").expect("File not found!");
    /// for x in 0..img.get_width() {
    ///     for y in 0..img.get_height() {
    ///         let pixel = img.get_pixel(x, y).unwrap();
    ///         img.set_pixel(x, y, pixel.clone());
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
    /// # Example
    /// ```no_run
    /// use rusty_imager::{formats::ImageFormat, Image};
    /// 
    /// let img = Image::from_file("<path>").expect("Failed to load file");
    /// println!("{}", img.get_signature()); // For bmp it returns BM
    /// ```
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
    /// use rusty_imager::{formats::ImageFormat, Image};
    /// 
    /// let img = Image::from_file("<name>").expect("File not found!");
    /// for x in 0..img.get_width() {
    ///     for y in 0..img.get_height() {
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
    /// use rusty_imager::{formats::ImageFormat, Image};
    /// 
    /// let img = Image::from_file("<name>").expect("File not found!");
    /// for x in 0..img.get_width() {
    ///     for y in 0..img.get_height() {
    ///         // Do something
    ///     }
    /// }
    /// ```
    fn get_width(&self) -> usize {
        self.raw.get_width()
    }

    /// Converts the image back to bytes
    /// # Example
    /// ```no_run
    /// use rusty_imager::Image;
    /// use rusty_imager::formats::ImageFormat;
    /// use rusty_imager::filters::ColorInversion;
    /// use std::fs;
    /// 
    /// let mut img = Image::from_file("<path>").expect("File not found!");
    /// img.apply_filter(ColorInversion::new());
    /// 
    /// fs::write("<path>", img.to_bytes()).expect("Failed to save the image");
    /// ```
    fn to_bytes(&self) -> Vec<u8> {
        self.raw.to_bytes()
    }
}