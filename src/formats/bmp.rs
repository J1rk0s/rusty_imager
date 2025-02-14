use std::io::Write;
use super::format::ImageFormat;
use crate::models::Pixel;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BmpHeader {
    signature: [u8; 2],
    file_size: u32,
    reserved: u32,
    data_offset: u32
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BmpInfo {
    size: u32,
    width: u32,
    height: u32,
    planes: u16,
    bits_per_pixel: u16,
    compression: u32,
    image_size: u32,
    h_res: u32,
    v_res: u32,
    colors: u32,
    important_colors: u32,
    padding: Vec<u8> 
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BmpColorTable {
    red: u8,
    green: u8,
    blue: u8,
    reserved: u8
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Bmp {
    pub header: BmpHeader,
    pub info: BmpInfo,
    pub colors: Option<Box<BmpColorTable>>,
    pub data: Vec<Pixel>
}
 
impl ImageFormat for Bmp {
    fn get_pixel(&self, x: usize, y: usize) -> Option<&Pixel> {
        if x > self.info.width as usize - 1 || y > self.info.height as usize - 1 {
            return None
        }

        // TODO: Add indexing from top left instead of bottom left
        self.data.get((self.info.width as usize * x) + y)
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) -> Option<()> {
        if x > self.info.width as usize - 1 || y > self.info.height as usize - 1 {
            return None
        }

        // TODO: Add indexing from top left instead of bottom left
        self.data[(self.info.width as usize * x) + y] = pixel;
        Some(())
    }

    fn get_size(&self) -> u32 {
        self.header.file_size
    }

    fn get_signature(&self) -> String {
        let first = self.header.signature[0];
        let second = self.header.signature[1];

        format!("{}{}", first as char, second as char)
    }

    fn get_metadata(&self) -> String {
        format!("{:?}\n{:?}", self.header, self.info)
    }

    fn get_height(&self) -> usize {
        self.info.height as usize
    }

    fn get_width(&self) -> usize {
        self.info.width as usize
    }

    fn to_bytes(&self) -> Vec<u8> {
        fn convert_header(header: BmpHeader) -> Vec<u8> {
            let mut res: Vec<u8> = vec![];
            res.write_all(&header.signature).unwrap();
            res.write_all(&header.file_size.to_le_bytes()).unwrap();
            res.write_all(&header.reserved.to_le_bytes()).unwrap();
            res.write_all(&header.data_offset.to_le_bytes()).unwrap();

            res
        }
        fn convert_info(info: &BmpInfo) -> Vec<u8> {
            let mut res: Vec<u8> = vec![];
            res.write_all(&info.size.to_le_bytes()).unwrap();
            res.write_all(&info.width.to_le_bytes()).unwrap();
            res.write_all(&info.height.to_le_bytes()).unwrap();
            res.write_all(&info.planes.to_le_bytes()).unwrap();
            res.write_all(&info.bits_per_pixel.to_le_bytes()).unwrap();
            res.write_all(&info.compression.to_le_bytes()).unwrap();
            res.write_all(&info.image_size.to_le_bytes()).unwrap();
            res.write_all(&info.h_res.to_le_bytes()).unwrap();
            res.write_all(&info.v_res.to_le_bytes()).unwrap();
            res.write_all(&info.colors.to_le_bytes()).unwrap();
            res.write_all(&info.important_colors.to_le_bytes()).unwrap();
            res.write_all(&info.padding).unwrap();

            res
        }
        fn convert_data(data: &Vec<Pixel>) -> Vec<u8> {
            data.iter().flat_map(|p| p.to_bytes(true)).collect()
        }

        let mut img: Vec<u8> = vec![];
        img.write_all(&convert_header(self.header)).unwrap();
        img.write_all(&convert_info(&self.info)).unwrap();
        img.write_all(&convert_data(&self.data)).unwrap();
        img.write_all(&[0, 0]).unwrap();

        img
    }
}

impl Bmp {

    // TODO: Switch from Option<Self> to Result<Self, ImageFormatError>
    pub fn parse(data: &[u8]) -> Option<Self> {
        let header = Bmp::parse_header(data)?;
        let info = Bmp::parse_info(data, header.data_offset)?;
        let pixels = Bmp::parse_pixels(&data[header.data_offset as usize..],  &info)?;

        // TODO: Add support for color table and compression

        Some(Bmp { 
            header, 
            info, 
            colors: None, 
            data: pixels
        })
    }

    fn parse_header(data: &[u8]) -> Option<BmpHeader> {
        let signature = data.get(0..2)?;

        if signature != [0x42, 0x4D] {
            return None
        }

        let size = u32::from_le_bytes(data.get(2..6)?.try_into().unwrap());
        let reserved = u32::from_le_bytes(data.get(6..10)?.try_into().unwrap());
        let offset = u32::from_le_bytes(data.get(10..14)?.try_into().unwrap());
        
        Some(BmpHeader {
            signature: signature.try_into().unwrap(),
            file_size: size,
            reserved: reserved,
            data_offset: offset
        })
    }

    fn parse_info(data: &[u8], offset: u32) -> Option<BmpInfo> {
        let size = u32::from_le_bytes(data.get(14..18)?.try_into().unwrap());
        let width = u32::from_le_bytes(data.get(18..22)?.try_into().unwrap());
        let height = u32::from_le_bytes(data.get(22..26)?.try_into().unwrap());
        let planes = u16::from_le_bytes(data.get(26..28)?.try_into().unwrap());
        let bit_count = u16::from_le_bytes(data.get(28..30)?.try_into().unwrap());
        let compression = u32::from_le_bytes(data.get(30..34)?.try_into().unwrap());
        let image_size = u32::from_le_bytes(data.get(34..38)?.try_into().unwrap());
        let x_pixels = u32::from_le_bytes(data.get(38..42)?.try_into().unwrap());
        let y_pixels = u32::from_le_bytes(data.get(42..46)?.try_into().unwrap());
        let colors_used = u32::from_le_bytes(data.get(46..50)?.try_into().unwrap());
        let colors_important = u32::from_le_bytes(data.get(50..54)?.try_into().unwrap());
        let mut padding: Vec<u8> = vec![];

        if size > 54 {
            padding = data.get(54..offset as usize).unwrap().to_vec();
        }

        let info = BmpInfo {
            size,
            width,
            height,
            planes,
            bits_per_pixel: bit_count,
            compression,
            image_size,
            h_res: x_pixels,
            v_res: y_pixels,
            colors: colors_used,
            important_colors: colors_important,
            padding: padding
        };

        Some(info)
    }

    #[allow(dead_code)]
    fn parse_table(data: &[u8]) -> Option<BmpColorTable> {
        let r = data.get(54)?;
        let g = data.get(55)?;
        let b = data.get(56)?;
        let res = data.get(57)?;

        Some(BmpColorTable {
            red: *r,
            green: *g,
            blue: *b,
            reserved: *res
        })
    }

    fn parse_pixels(data: &[u8], bmp_info: &BmpInfo) -> Option<Vec<Pixel>> {
        let mut res: Vec<Pixel> = vec![];

        match bmp_info.bits_per_pixel {
            24 => {
                for i in (0..data.len()).step_by(3) {
                    let color = data.get(i..i+3);

                    if let Some(color) = color {
                        let b = color[0];
                        let g = color[1];
                        let r = color[2];

                        res.push(Pixel {
                            r, g, b
                        });
                    }

                }
            }

            _ => {
                return None
            }
        }

        Some(res)
    }
}