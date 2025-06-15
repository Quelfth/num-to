use super::*;

impl ToU8 for i128 {
    fn to_u8(self) -> u8 {
        self.to_u128().to_u8()
    }
}
impl ToU16 for i128 {
    fn to_u16(self) -> u16 {
        self.to_u128().to_u16()
    }
}
impl ToU32 for i128 {
    fn to_u32(self) -> u32 {
        self.to_u128().to_u32()
    }
}
impl ToU64 for i128 {
    fn to_u64(self) -> u64 {
        self.to_u128().to_u64()
    }
}
impl ToU128 for i128 {
    fn to_u128(self) -> u128 {
        if self & Self::MIN != 0 {
            0
        } else {
            self as u128
        }
    }
}

impl ToI8 for i128 {
    fn to_i8(self) -> i8 {
        match self {
            ..-0x80 => i8::MIN,
            -0x80..0x80 => self as i8,
            0x80.. => i8::MAX,
        }
    }
}
impl ToI16 for i128 {
    fn to_i16(self) -> i16 {
        match self {
            ..-0x8000 => i16::MIN,
            -0x8000..0x8000 => self as i16,
            0x8000.. => i16::MAX,
        }
    }
}
impl ToI32 for i128 {
    fn to_i32(self) -> i32 {
        match self {
            ..-0x8000_0000 => i32::MIN,
            -0x8000_0000..0x8000_0000 => self as i32,
            0x8000_0000.. => i32::MAX,
        }
    }
}
impl ToI64 for i128 {
    fn to_i64(self) -> i64 {
        match self {
            ..-0x8000_0000_0000_0000 => i64::MIN,
            -0x8000_0000_0000_0000..0x8000_0000_0000_0000 => self as i64,
            0x8000_0000_0000_0000.. => i64::MAX,
        }
    }
}
impl AsI128 for i128 {
    fn as_i128(self) -> i128 {
        self
    }
}

impl ToF32 for i128 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}
impl ToF64 for i128 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}
