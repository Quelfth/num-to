use super::*;

impl ToU8 for i32 {
    fn to_u8(self) -> u8 {
        self.to_u32().to_u8()
    }
}
impl ToU16 for i32 {
    fn to_u16(self) -> u16 {
        self.to_u32().to_u16()
    }
}
impl ToU32 for i32 {
    fn to_u32(self) -> u32 {
        if self & Self::MIN != 0 {
            0
        } else {
            self as u32
        }
    }
}
impl ToU64 for i32 {
    fn to_u64(self) -> u64 {
        self.to_u32().as_u64()
    }
}
impl ToU128 for i32 {
    fn to_u128(self) -> u128 {
        self.to_u32().as_u128()
    }
}

impl ToI8 for i32 {
    fn to_i8(self) -> i8 {
        match self {
            ..-0x80 => i8::MIN,
            -0x80..0x80 => self as i8,
            0x80.. => i8::MAX,
        }
    }
}
impl ToI16 for i32 {
    fn to_i16(self) -> i16 {
        match self {
            ..-0x8000 => i16::MIN,
            -0x8000..0x8000 => self as i16,
            0x8000.. => i16::MAX,
        }
    }
}
impl AsI32 for i32 {
    fn as_i32(self) -> i32 {
        self
    }
}
impl AsI64 for i32 {
    fn as_i64(self) -> i64 {
        self.into()
    }
}
impl AsI128 for i32 {
    fn as_i128(self) -> i128 {
        self.into()
    }
}

impl ToF32 for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}
impl AsF64 for i32 {
    fn as_f64(self) -> f64 {
        self.into()
    }
}
