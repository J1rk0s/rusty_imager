#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
/// Basic RGB pixel struct
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Pixel {
    /// Creates a pixel from given hex
    /// 
    /// Hex code must always start with # and continue with 6 hex digits
    /// 
    /// # Example
    /// ```
    /// use rusty_imager::models::Pixel;
    /// 
    /// let white = Pixel::from_hex("#ffffff").unwrap();
    /// ```
    pub fn from_hex(hex: &str) -> Option<Self> {
        if !hex.starts_with("#") || hex.len() != 7 {
            return None
        }

        let r = u8::from_str_radix(hex.get(1..3).unwrap_or_default(), 16).ok()?;
        let g = u8::from_str_radix(hex.get(3..5).unwrap_or_default(), 16).ok()?;
        let b = u8::from_str_radix(hex.get(5..7).unwrap_or_default(), 16).ok()?;

        Some(Pixel {
            r, g, b
        })
    }

    /// Converts the pixel to hex string
    /// # Example 
    /// ```
    /// use rusty_imager::models::Pixel;
    /// 
    /// let white = Pixel {
    ///     r: 255,
    ///     g: 255,
    ///     b: 255
    /// };
    /// 
    /// let hex = white.to_hex(); 
    /// // Returns #ffffff
    /// ```
    pub fn to_hex(&self) -> String {
        format!("#{:x}{:x}{:x}", self.r, self.g, self.b)
    }

    /// Converts the pixel to bytes
    /// 
    /// If reverse is true, RGB values will be stored backwards (BGR)
    /// 
    /// # Example
    /// ```
    /// use rusty_imager::models::Pixel;
    /// 
    /// let white = Pixel::from_hex("#ff0f00").unwrap();
    /// 
    /// let bytes = white.to_bytes(false);
    /// // Returns Vec { 255, 128, 0 }
    /// ```
    pub fn to_bytes(&self, reverse: bool) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];
        if reverse {
            res.push(self.b);
            res.push(self.g);
            res.push(self.r);
        } else {
            res.push(self.r);
            res.push(self.g);
            res.push(self.b);
        }

        res
    }

    /// Inverts the pixel RGB values
    pub fn invert(&self) -> Self {
        Self { 
            r: 255 - self.r,
            g: 255 - self.g,
            b: 255 - self.b
        }
    }
}