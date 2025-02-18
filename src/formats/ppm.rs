use std::io::Write;

use crate::models::{Colors, Pixel};

use super::ImageFormat;

#[derive(Debug, Default)]
pub struct Ppm {
    signature: [u8; 2],
    width: u32,
    height: u32,
    max_color: Option<u8>,
    pixels: Vec<Pixel>
}

impl ImageFormat for Ppm {
    fn get_height(&self) -> usize {
        self.height as usize
    }

    fn get_width(&self) -> usize {
        self.width as usize
    }

    fn get_size(&self) -> u32 {
        self.pixels.len() as u32
    }
    
    fn get_pixel(&self, x: usize, y: usize) -> Option<&Pixel> {
        if x > self.width as usize - 1 || y > self.height as usize - 1 {
            return None
        }

        self.pixels.get((self.width as usize * y) + x)
    }

    fn get_metadata(&self) -> String {
        format!("{:?}", self)
    }

    fn get_signature(&self) -> String {
        let first = self.signature[0];
        let second = self.signature[1];

        format!("{}{}", first as char, second as char)
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) -> Option<()> {
        if x > self.width as usize - 1 || y > self.height as usize - 1 {
            return None
        }

        if self.max_color.is_some() {
            if pixel.r > self.max_color? || pixel.g > self.max_color? || pixel.b > self.max_color? {
                return None
            }
        }

        self.pixels[(self.width as usize * y) + x] = pixel;
        Some(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];

        res.write_all(&self.signature).unwrap();
        res.write_all(&[0x0a]).unwrap();

        res.write_all(format!("{} {}", self.width, self.height).as_bytes()).unwrap();
        res.write(&[0x0a]).unwrap();

        let pixel_bytes: Vec<u8>;

        if self.max_color.is_some() {
            res.write_all(format!("{}", self.max_color.unwrap()).as_bytes()).unwrap();
            res.write_all(&[0x0a]).unwrap();
            
            pixel_bytes = self.pixels.iter().flat_map(|p| p.to_bytes(false)).collect();
        } else {
            pixel_bytes = self.pixels.iter().map(|p| {
                if *p == Colors::WHITE {
                    1
                } else {
                    0
                }
            }).collect();
        }

        res.write_all(&pixel_bytes).unwrap();

        res
    }
}

impl Ppm {
    pub fn parse(data: &[u8]) -> Option<Self> {
        let sections: Vec<&[u8]> = data.split(|x| *x == 0x0a).collect();
        let signature = *sections.get(0)?;
        let dimensions: Vec<&str> = std::str::from_utf8(*sections.get(2)?).ok()?.split(" ").collect();

        let width: u32 = dimensions.get(0)?.parse::<u32>().ok()?;
        let height: u32 = dimensions.get(1)?.parse::<u32>().ok()?;

        let max_color: Option<u8>;
        let pixels: Vec<Pixel>;

        if *signature.get(1)? < 0x04 {
            max_color = None;
            pixels = Ppm::parse_ascii(sections.get(3)?)?;
        } else {
            max_color = Some(std::str::from_utf8(*sections.get(3)?).ok()?.parse().ok()?);
            let data = sections.get(4..)?.concat();
            pixels = Ppm::parse_binary(&data)?;
        }
      
        Some(Ppm { 
            signature: signature.try_into().unwrap(),
            width,
            height,
            max_color,
            pixels 
        })
    }

    fn parse_ascii(_data: &[u8]) -> Option<Vec<Pixel>> {
       todo!()
    }

    fn parse_binary(data: &[u8]) -> Option<Vec<Pixel>> {
        let mut res: Vec<Pixel> = vec![];

        for i in (0..data.len()).step_by(3) {
            let color = data.get(i..i+3);

            if let Some(color) = color {
                let r = color[0];
                let g = color[1];
                let b = color[2];

                res.push(Pixel {
                    r, g, b
                });
            }
        }

        Some(res)
    }
}