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
    data: Vec<Pixel>
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
}

impl Bmp {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 54 {
            return None
        }

        let header = Bmp::parse_header(data)?;

        
        

        todo!()
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
        todo!()
    }
}