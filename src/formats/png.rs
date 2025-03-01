use std::io::Read;

use flate2::read::ZlibDecoder;
use rkyv::{rancor::Error, Archive, Deserialize, Serialize};

use crate::models::Pixel;

use super::ImageFormat;

#[allow(dead_code)]
pub struct Png {
    header: PngHeader,
    chunks: Vec<PngChunk>,
    pixels: Vec<Pixel>
}

#[allow(dead_code)]
#[repr(C)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Archive, Clone)]
#[rkyv(
    compare(PartialEq),
    derive(Debug),
)]
pub struct PngHeader {
    high_byte: u8,
    signature: [u8; 3],
    dos_line_end: [u8; 2],
    dos_eof: u8,
    unix_line_end: u8
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Archive, Clone)]
#[rkyv(
    compare(PartialEq),
    derive(Debug),
)]
#[repr(u8)]
pub enum PngColorType {
    Grayscale = 0,
    RGB = 2,
    Unknown
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Archive, Clone)]
#[rkyv(
    compare(PartialEq),
    derive(Debug),
)]
#[repr(u8)]
pub enum PngInterlacing {
    None,
    Adam7
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Archive, Clone)]
#[rkyv(
    compare(PartialEq),
    derive(Debug),
)]
pub enum PngUnit {
    Unknown,
    Meter
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Archive, Clone)]
#[rkyv(
    compare(PartialEq),
    derive(Debug),
)]
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
#[derive(Debug, Deserialize, Serialize, PartialEq, Archive, Clone)]
#[rkyv(
    compare(PartialEq),
    derive(Debug),
)]
pub struct PngIccp {
    length: u32,
    signature: [u8; 4],

    keyword: String,
    compression_method: u8,
    compression_profile: Vec<u8>,

    crc: u32
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Archive, Clone)]
#[rkyv(
    compare(PartialEq),
    derive(Debug),
)]
pub struct PngPhys {
    length: u32,
    signature: [u8; 4],

    pixel_per_unit_x: u32,
    pixel_per_unit_y: u32,
    unit: PngUnit,

    crc: u32
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Archive, Clone)]
#[rkyv(
    compare(PartialEq),
    derive(Debug),
)]
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
#[derive(Debug, Deserialize, Serialize, PartialEq, Archive, Clone)]
#[rkyv(
    compare(PartialEq),
    derive(Debug),
)]
pub struct PngText {
    length: u32,
    signature: [u8; 4],

    keyword: String,
    text: String,

    crc: u32
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Archive, Clone)]
#[rkyv(
    compare(PartialEq),
    derive(Debug),
)]
pub struct PngIdata {
    length: u32,
    signature: [u8; 4],

    data: Vec<u8>,

    crc: u32
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, PartialEq, Archive, Clone)]
#[rkyv(
    compare(PartialEq),
    derive(Debug),
)]
pub struct PngIend {
    length: u32,
    signature: [u8; 4],
    crc: u32
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Archive, Clone)]
#[rkyv(
    compare(PartialEq),
    derive(Debug),
)]
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
        if data.len() < 8 {
            return None;
        }

        let header: PngHeader = unsafe { std::ptr::read(data[0..8].as_ptr() as *const _) };
        let chunks = Png::parse_chunks(data.get(8..)?)?;
        let metadata = Png::get_idhr(&chunks)?;
        let pixels = Png::parse_pixels(Png::get_idata_chunks(&chunks), metadata.width as usize);


        Some(Self {
            header,
            chunks,
            pixels
        })
    }

    fn parse_ihdr(length: u32, signature: &[u8], data: &[u8], crc: u32) -> Option<PngIhdr> {
        let width = u32::from_be_bytes(data.get(0..4)?.try_into().unwrap());
        let height = u32::from_be_bytes(data.get(4..8)?.try_into().unwrap());

        let bit_depth= data.get(8)?;
        let color_type = match data.get(9)? {
            0 => PngColorType::Grayscale,
            2 => PngColorType::RGB,
            _ => PngColorType::Unknown
        };

        let compression_method = data.get(10)?;
        let filter_method = data.get(11)?;

        let interlacing = match data.get(12)? {
            0 => PngInterlacing::None,
            1 => PngInterlacing::Adam7,
            _ => panic!("Invalid interlacing")
        };
        
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

    fn parse_iccp(length: u32, signature: &[u8], data: &[u8], crc: u32) -> Option<PngIccp> {
        let mut keyword = String::new();

        let mut index = 0;

        while index < data.len() {
            if *data.get(index)? == 0x00 {
                break;
            }

            keyword.push(*data.get(index)? as char);
            index += 1;
        }

        let compression_method = data.get(index + 1)?;
        let profile = data.get(index + 2..)?;

        Some(PngIccp {
            length,
            signature: signature.try_into().unwrap(),
            keyword,
            compression_method: *compression_method,
            compression_profile: profile.to_vec(),
            crc
        })
    }

    fn parse_time(length: u32, signature: &[u8], data: &[u8], crc: u32) -> Option<PngTime> {
        let year = u16::from_be_bytes(data.get(0..2)?.try_into().unwrap());
        let month = data.get(2)?;
        let day = data.get(3)?;
        let hour = data.get(4)?;
        let minute = data.get(5)?;
        let second = data.get(6)?;

        Some(PngTime {
            length,
            signature: signature.try_into().unwrap(),
            year,
            month: *month,
            day: *day,
            hour: *hour,
            minute: *minute,
            second: *second,
            crc
        })
    }

    fn parse_idat(length: u32, signature: &[u8], data: &[u8], crc: u32) -> Option<PngIdata> {
        Some(PngIdata {
            length,
            signature: signature.try_into().unwrap(),
            data: data.to_vec(),
            crc
        })
    }

    fn parse_iend(length: u32, signature: &[u8], crc: u32) -> Option<PngIend> {
        Some(PngIend {
            length,
            signature: signature.try_into().unwrap(),
            crc
        })
    }

    fn parse_phys(length: u32, signature: &[u8], data: &[u8], crc: u32) -> Option<PngPhys> {
        let pixel_per_x = u32::from_be_bytes(data.get(0..4)?.try_into().ok()?);
        let pixel_per_y = u32::from_be_bytes(data.get(4..8)?.try_into().ok()?);
        let unit = match data.get(8)? {
            0 => PngUnit::Unknown,
            1 => PngUnit::Meter,
            _ => panic!("Unknown png unit format")
        };

        Some(PngPhys {
            length,
            signature: signature.try_into().ok()?,
            pixel_per_unit_x: pixel_per_x,
            pixel_per_unit_y: pixel_per_y,
            unit,
            crc
        })
    }
 
    fn parse_text(length: u32, signature: &[u8], data: &[u8], crc: u32) -> Option<PngText> {
        todo!()
    }

    fn get_idata_chunks(chunks: &Vec<PngChunk>) -> Vec<PngIdata> {
        let mut res: Vec<PngIdata> = vec![];

        for chunk in chunks {
            match chunk {
                PngChunk::IDAT(idata) => {
                    res.push(idata.clone());
                }

                _ => continue
            }
        }

        res
    }

    fn get_idhr<'a>(chunks: &'a Vec<PngChunk>) -> Option<&'a PngIhdr> {
        for chunk in chunks {
            match chunk {
                PngChunk::IHDR(data) => return Some(data),
                _ => continue
            }
        }

        None
    }

    fn parse_chunks(data: &[u8]) -> Option<Vec<PngChunk>> {
        let mut chunks: Vec<PngChunk> = vec![];
        let mut index = 0;

        while index + 12 <= data.len() {
            let length = u32::from_be_bytes(data.get(index..index+4)?.try_into().unwrap()) as usize;
            let signature = data.get(index+4..index+8)?;

            if index + 8 + length >= data.len() {
                return None;
            }

            let chunk_data = data.get((index + 8)..(index + 8 + length))?;
            let crc = u32::from_be_bytes(data.get((index + 8 + length)..(index + 12 + length))?.try_into().unwrap());

            index += 12 + length;

            let chunk = match signature {
                b"IHDR" => {
                    let ihdr = Png::parse_ihdr(length as u32, signature, chunk_data, crc)?;
                    PngChunk::IHDR(ihdr)
                }

                b"iCCP" => {
                    PngChunk::ICCP(Png::parse_iccp(length as u32, signature, chunk_data, crc)?)
                }

                b"pHYs" => {
                    PngChunk::PHYS(Png::parse_phys(length as u32, signature, chunk_data, crc)?)
                }

                b"tIME" => {
                    PngChunk::TIME(Png::parse_time(length as u32, signature, chunk_data, crc)?)
                }

                b"IDAT" => {
                    PngChunk::IDAT(Png::parse_idat(length as u32, signature, chunk_data, crc)?)
                }

                b"IEND" => {
                    PngChunk::IEND(Png::parse_iend(length as u32, signature, crc)?)
                }

                _ => {
                    //PngChunk::Unknown { length: length as u32, signature: signature.try_into().unwrap(), data: chunk_data.to_vec(), crc }
                    todo!("Png chunk not yet implemented")
                }
            };

            chunks.push(chunk);
        }

        Some(chunks)
    }

    fn get_chunk(&self, signature: &[u8]) -> Option<&PngChunk> {
        self.chunks.iter().find(|chunk| match chunk {
            PngChunk::IHDR(_) => signature == b"IHDR",
            PngChunk::IDAT(_) => signature == b"IDAT",
            PngChunk::PLTE(_) => signature == b"PLTE",
            PngChunk::IEND(_) => signature == b"IEND",
            PngChunk::TEXT(_) => signature == b"tEXt",
            PngChunk::ICCP(_) => signature == b"iCCP",
            PngChunk::PHYS(_) => signature == b"pHYs",
            PngChunk::TIME(_) => signature == b"tIME",
            PngChunk::Unknown { signature: sig, .. } => sig == signature,
        })
    }

    fn parse_pixels(idata: Vec<PngIdata>, width: usize) -> Vec<Pixel> {
        let mut pixels: Vec<Pixel> = vec![];
        let mut bytes: Vec<u8> = vec![];

        for data in idata {
            let mut decoder = ZlibDecoder::new(&data.data[..]);
            decoder.read_to_end(&mut bytes).unwrap();
        }

        let data = Png::undo_png_filtering(&bytes, width, PngColorType::RGB);

        for i in (0..data.len()).step_by(3) {
            let color = data.get(i..i+3);

            if let Some(color) = color {
                let r = color[0];
                let g = color[1];
                let b = color[2];

                pixels.push(Pixel {
                    r, g, b
                });
            }

        }

        pixels
    }

    fn undo_png_filtering(data: &[u8], width: usize, color_type: PngColorType) -> Vec<u8> {
        let mut bytes_per_pixel = 0;

        match color_type {
            PngColorType::RGB => bytes_per_pixel = 3,
            PngColorType::Grayscale => bytes_per_pixel = 1,
            PngColorType::Unknown => bytes_per_pixel = 0
        }

        let stride = width * bytes_per_pixel + 1;
        let mut output = Vec::with_capacity(data.len() - data.len() / stride);
    
        let mut prev_row: Vec<u8> = vec![0; width * bytes_per_pixel];
    
        for chunk in data.chunks(stride) {
            let filter_type = chunk[0];
            let scanline = &chunk[1..];
    
            let mut row = vec![0; width * bytes_per_pixel];
    
            match filter_type {
                0 => {
                    // None
                    row.copy_from_slice(scanline);
                }
                1 => {
                    // Sub
                    for i in 0..width * bytes_per_pixel {
                        let left = if i >= bytes_per_pixel { row[i - bytes_per_pixel] } else { 0 };
                        row[i] = scanline[i].wrapping_add(left);
                    }
                }
                2 => {
                    // Up
                    for i in 0..width * bytes_per_pixel {
                        let above = prev_row[i];
                        row[i] = scanline[i].wrapping_add(above);
                    }
                }
                3 => {
                    // Average
                    for i in 0..width * bytes_per_pixel {
                        let left = if i >= bytes_per_pixel { row[i - bytes_per_pixel] } else { 0 };
                        let above = prev_row[i];
                        row[i] = scanline[i].wrapping_add(((left as u16 + above as u16) / 2) as u8);
                    }
                }
                4 => {
                    // Paeth
                    for i in 0..width * bytes_per_pixel {
                        let left = if i >= bytes_per_pixel { row[i - bytes_per_pixel] } else { 0 };
                        let above = prev_row[i];
                        let upper_left = if i >= bytes_per_pixel { prev_row[i - bytes_per_pixel] } else { 0 };
                        row[i] = scanline[i].wrapping_add(Png::paeth_predictor(left, above, upper_left));
                    }
                }
                _ => panic!("Unknown filter type: {}", filter_type),
            }
    
            output.extend_from_slice(&row);
            prev_row = row;
        }
    
        output
    }

    fn paeth_predictor(a: u8, b: u8, c: u8) -> u8 {
        let p = a as i16 + b as i16 - c as i16;
        let pa = (p - a as i16).abs();
        let pb = (p - b as i16).abs();
        let pc = (p - c as i16).abs();
    
        if pa <= pb && pa <= pc {
            a
        } else if pb <= pc {
            b
        } else {
            c
        }
    }
}

impl ImageFormat for Png {
    fn get_height(&self) -> usize {
        let chunk = self.get_chunk(b"IHDR").expect("Image does not have height metadata");

        if let PngChunk::IHDR(ihdr) = chunk {
            ihdr.height as usize
        } else {
            0
        }
    }

    fn get_metadata(&self) -> String {
        todo!()
    }

    fn get_pixel(&self, x: usize, y: usize) -> Option<&crate::models::Pixel> {
        if x > self.get_width() - 1 || y > self.get_height() - 1 {
            return None
        }

        self.pixels.get((self.get_width() * y) + x)
    }

    fn get_signature(&self) -> String {
        todo!()
    }

    fn get_size(&self) -> u32 {
        todo!()
    }

    fn get_width(&self) -> usize {
        let chunk = self.get_chunk(b"IHDR").expect("Image does not have width metadata");

        if let PngChunk::IHDR(ihdr) = chunk {
            ihdr.width as usize
        } else {
            0
        }
    }

    fn set_pixel(&mut self, x: usize, y: usize, pixel: crate::models::Pixel) -> Option<()> {
        if x > self.get_width() - 1 || y > self.get_height() - 1 {
            return None
        }

        let width = self.get_width();

        self.pixels[(width * y) + x] = pixel;
        Some(())
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];
        res.append(rkyv::to_bytes::<Error>(&self.header).unwrap().to_vec().as_mut());
        
        for chunk in self.chunks.iter() {
            if let PngChunk::IDAT(data) = chunk {
                let mut hasher = crc32fast::Hasher::new();
                hasher.update(&data.data);
                let crc = hasher.finalize();

                res.append(crc.to_le_bytes().to_vec().as_mut());
                res.append(data.data.clone().as_mut());
                continue;
            }

            res.append(rkyv::to_bytes::<Error>(chunk).unwrap().to_vec().as_mut());
        }
        
        res
    }
}