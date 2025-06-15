use super::*;

impl ToU8 for u32 {
    fn to_u8(self) -> u8 {
        if self & 0xffff_ff00 != 0 {
            u8::MAX
        } else {
            self as u8
        }
    }
}
impl AsU16 for u32 {
    fn as_u16(self) -> u16 {
        if self & 0xffff_0000 != 0 {
            u16::MAX
        } else {
            self as u16
        }
    }
}
impl AsU32 for u32 {
    fn as_u32(self) -> u32 {
        self
    }
}
impl AsU64 for u32 {
    fn as_u64(self) -> u64 {
        self.into()
    }
}
impl AsU128 for u32 {
    fn as_u128(self) -> u128 {
        self.into()
    }
}

impl ToI8 for u32 {
    fn to_i8(self) -> i8 {
        if self & 0xffff_ff80 != 0 {
            i8::MAX
        } else {
            self as i8
        }
    }
}
impl AsI16 for u32 {
    fn as_i16(self) -> i16 {
        if self & 0xffff_8000 != 0 {
            i16::MAX
        } else {
            self as i16
        }
    }
}
impl ToI32 for u32 {
    fn to_i32(self) -> i32 {
        if self & 0x8000_0000 != 0 {
            i32::MAX
        } else {
            self as i32
        }
    }
}
impl AsI64 for u32 {
    fn as_i64(self) -> i64 {
        self.into()
    }
}
impl AsI128 for u32 {
    fn as_i128(self) -> i128 {
        self.into()
    }
}

impl ToF32 for u32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}
impl AsF64 for u32 {
    fn as_f64(self) -> f64 {
        self.into()
    }
}
