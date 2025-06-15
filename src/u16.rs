use super::*;

impl ToU8 for u16 {
    fn to_u8(self) -> u8 {
        if self & 0xff00 != 0 {
            u8::MAX
        } else {
            self as u8
        }
    }
}
impl AsU16 for u16 {
    fn as_u16(self) -> u16 {
        self
    }
}
impl AsU32 for u16 {
    fn as_u32(self) -> u32 {
        self.into()
    }
}
impl AsU64 for u16 {
    fn as_u64(self) -> u64 {
        self.into()
    }
}
impl AsU128 for u16 {
    fn as_u128(self) -> u128 {
        self.into()
    }
}

impl ToI8 for u16 {
    fn to_i8(self) -> i8 {
        if self & 0xff80 != 0 {
            i8::MAX
        } else {
            self as i8
        }
    }
}
impl AsI16 for u16 {
    fn as_i16(self) -> i16 {
        if self & 0x8000 != 0 {
            i16::MAX
        } else {
            self as i16
        }
    }
}
impl AsI32 for u16 {
    fn as_i32(self) -> i32 {
        self.into()
    }
}
impl AsI64 for u16 {
    fn as_i64(self) -> i64 {
        self.into()
    }
}
impl AsI128 for u16 {
    fn as_i128(self) -> i128 {
        self.into()
    }
}

impl AsF32 for u16 {
    fn as_f32(self) -> f32 {
        self.into()
    }
}
impl AsF64 for u16 {
    fn as_f64(self) -> f64 {
        self.into()
    }
}
