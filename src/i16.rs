use super::*;

impl ToU8 for i16 {
    fn to_u8(self) -> u8 {
        self.to_u16().to_u8()
    }
}
impl ToU16 for i16 {
    fn to_u16(self) -> u16 {
        if self & Self::MIN != 0 {
            0
        } else {
            self as u16
        }
    }
}
impl ToU32 for i16 {
    fn to_u32(self) -> u32 {
        self.to_u16().as_u32()
    }
}
impl ToU64 for i16 {
    fn to_u64(self) -> u64 {
        self.to_u16().as_u64()
    }
}
impl ToU128 for i16 {
    fn to_u128(self) -> u128 {
        self.to_u16().as_u128()
    }
}

impl ToI8 for i16 {
    fn to_i8(self) -> i8 {
        match self {
            ..-0x80 => i8::MIN,
            -0x80..0x80 => self as i8,
            0x80.. => i8::MAX,
        }
    }
}
impl AsI16 for i16 {
    fn as_i16(self) -> i16 {
        self
    }
}
impl AsI32 for i16 {
    fn as_i32(self) -> i32 {
        self.into()
    }
}
impl AsI64 for i16 {
    fn as_i64(self) -> i64 {
        self.into()
    }
}
impl AsI128 for i16 {
    fn as_i128(self) -> i128 {
        self.into()
    }
}

impl AsF32 for i16 {
    fn as_f32(self) -> f32 {
        self.into()
    }
}
impl AsF64 for i16 {
    fn as_f64(self) -> f64 {
        self.into()
    }
}
