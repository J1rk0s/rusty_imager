use super::ImageFormat;

#[allow(dead_code)]
pub struct Png {
    header: PngHeader,
    chunks: Vec<PngChunk>
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Debug)]
pub struct PngHeader {
    high_byte: u8,
    signature: [u8; 3],
    dos_line_end: [u8; 2],
    dos_eof: u8,
    unix_line_end: u8
}

#[derive(Debug)]
#[repr(u8)]
pub enum PngColorType {
    Grayscale = 0,
    RGB = 2,
    Unknown
}

#[derive(Debug)]
#[repr(u8)]
pub enum PngInterlacing {
    None,
    Adam7
}

#[derive(Debug)]
pub enum PngUnit {
    Unknown,
    Meter
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PngIhdr {
    length: u32,
    signature: [u8; 4],

    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: PngColorType,
    compression_method: u8,
    filter_method: u8,
    interlacing: PngInterlacing,

    crc: u32
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PngIccp {
    length: u32,
    signature: [u8; 4],

    keyword: String,
    compression_method: u8,
    compression_profile: Vec<u8>,

    crc: u32
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PngPhys {
    length: u32,
    signature: [u8; 4],

    pixel_per_unit_x: u32,
    pixel_per_unit_y: u32,
    unit: PngUnit,

    crc: u32
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PngTime {
    length: u32,
    signature: [u8; 4],

    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,

    crc: u32
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PngText {
    length: u32,
    signature: [u8; 4],

    keyword: String,
    text: String,

    crc: u32
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PngIdata {
    length: u32,
    signature: [u8; 4],

    data: Vec<u8>,

    crc: u32
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PngIend {
    length: u32,
    signature: [u8; 4],
    crc: u32
}

#[derive(Debug)]
pub enum PngChunk {
    IHDR(PngIhdr),
    IDAT(PngIdata),
    PLTE(PngPhys),
    IEND(PngIend),
    TEXT(PngText),
    ICCP(PngIccp),
    PHYS(PngPhys),
    TIME(PngTime),
    Unknown { length: u32, signature: [u8; 4], data: Vec<u8>, crc: u32 },
}

impl Png {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 64 {
            return None;
        }

        let header: PngHeader = unsafe { std::ptr::read(data[0..8].as_ptr() as *const _) };
        let ihdr: PngIhdr = Png::parse_ihdr(data.get(8..36)?)?;

        println!("{:?}", header);
        println!("{:?}", ihdr);

        todo!()
    }

    fn parse_ihdr(data: &[u8]) -> Option<PngIhdr> {
        let length: u32 = u32::from_be_bytes(data.get(0..4)?.try_into().unwrap());
        let signature = data.get(4..8)?;

        let width = u32::from_be_bytes(data.get(8..12)?.try_into().unwrap());
        let height = u32::from_be_bytes(data.get(12..16)?.try_into().unwrap());

        let bit_depth= data.get(16)?;
        let color_type = match data.get(17)? {
            0 => PngColorType::Grayscale,
            2 => PngColorType::RGB,
            _ => PngColorType::Unknown
        };

        let compression_method = data.get(18)?;
        let filter_method = data.get(19)?;

        let interlacing = match data.get(20)? {
            0 => PngInterlacing::None,
            1 => PngInterlacing::Adam7,
            _ => panic!("Invalid interlacing")
        };

        let crc = u32::from_be_bytes(data.get(21..25)?.try_into().unwrap());
        
        Some(PngIhdr {
            length,
            signature: signature.try_into().unwrap(),
            width,
            height,
            bit_depth: *bit_depth,
            color_type,
            compression_method: *compression_method,
            filter_method: *filter_method,
            interlacing,
            crc
        })
    }
}

impl ImageFormat for Png {
    fn get_height(&self) -> usize {
        todo!()
    }

    fn get_metadata(&self) -> String {
        todo!()
    }

    fn get_pixel(&self, x: usize, y: usize) -> Option<&crate::models::Pixel> {
        todo!()
    }

    fn get_signature(&self) -> String {
        todo!()
    }

    fn get_size(&self) -> u32 {
        todo!()
    }

    fn get_width(&self) -> usize {
        todo!()
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: crate::models::Pixel) -> Option<()> {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}