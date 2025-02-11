#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Pixel {
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

    pub fn to_hex(&self) -> String {
        format!("#{:x}{:x}{:x}", self.r, self.g, self.b)
    }
}