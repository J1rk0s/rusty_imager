use super::format::ImageFormat;
use crate::models::Pixel;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct BmpHeader {
    signature: [u8; 2],
    file_size: u32,
    reserved: u32,
    data_offset: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
    important_colors: u32 
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
    header: BmpHeader,
    info: BmpInfo,
    colors: Option<Box<BmpColorTable>>,
    data: Vec<u8>
}
 
impl ImageFormat for Bmp {
    fn get_pixel(&self, x: u32, y: u32) -> Option<Pixel> {
        todo!()
    }

    fn get_size(&self) -> u32 {
        self.header.file_size
    }

    fn get_signature(&self) -> String {
        let first = self.header.signature[0];
        let second = self.header.signature[1];

        format!("{}{}", first as char, second as char)
    }

    fn get_header(&self) -> String {
        format!("{:?}\n{:?}", self.header, self.info)
    }
}

impl Bmp {
    pub fn parse(data: &[u8]) -> Option<Self> {
        let header = Bmp::parse_header(data)?;
        let info = Bmp::parse_info(data)?;
        let pixels = data.get(54..)?;

        Some(Bmp { 
            header, 
            info, 
            colors: None, 
            data: pixels.iter().cloned().collect() 
        })
    }

    fn parse_header(data: &[u8]) -> Option<BmpHeader> {
        let signature = data.get(0..2)?;
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

    fn parse_info(data: &[u8]) -> Option<BmpInfo> {
        let size = u32::from_le_bytes(data.get(14..18)?.try_into().unwrap());
        let width = u32::from_le_bytes(data.get(18..22)?.try_into().unwrap());
        let height = u32::from_le_bytes(data.get(22..26)?.try_into().unwrap());
        let planes = u16::from_le_bytes(data.get(26..28)?.try_into().unwrap());
        let bit_count = u16::from_le_bytes(data.get(28..30)?.try_into().unwrap()); // 32 => A8 R8 G8 B8
        let compression = u32::from_le_bytes(data.get(30..34)?.try_into().unwrap());
        let image_size = u32::from_le_bytes(data.get(34..38)?.try_into().unwrap());
        let x_pixels = u32::from_le_bytes(data.get(38..42)?.try_into().unwrap());
        let y_pixels = u32::from_le_bytes(data.get(42..46)?.try_into().unwrap());
        let colors_used = u32::from_le_bytes(data.get(46..50)?.try_into().unwrap());
        let colors_important = u32::from_le_bytes(data.get(50..54)?.try_into().unwrap());

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
            important_colors: colors_important
        };

        Some(info)
    }

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
}