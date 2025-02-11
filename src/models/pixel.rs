#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Pixel {
    pub fn from_hex(hex: &str) -> Option<Self> {
        if !hex.starts_with("#") {
            return None
        }

        Some(Pixel::default())
    }
}