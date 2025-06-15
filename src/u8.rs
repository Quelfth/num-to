use super::*;

impl AsU8 for u8 { fn as_u8(self) -> u8 { self } }
impl AsU16 for u8 {
    fn as_u16(self) -> u16 {
        self.into()
    }
}
impl AsU32 for u8 {
    fn as_u32(self) -> u32 {
        self.into()
    }
}
impl AsU64 for u8 {
    fn as_u64(self) -> u64 {
        self.into()
    }
}
impl AsU128 for u8 {
    fn as_u128(self) -> u128 {
        self.into()
    }
}

impl ToI8 for u8 {
    fn to_i8(self) -> i8 {
        if self & 0x80 != 0 {
            i8::MAX
        } else {
            self as i8
        }
    }
}
impl AsI16 for u8 {
    fn as_i16(self) -> i16 {
        self.into()
    }
}
impl AsI32 for u8 {
    fn as_i32(self) -> i32 {
        self.into()
    }
}
impl AsI64 for u8 {
    fn as_i64(self) -> i64 {
        self.into()
    }
}
impl AsI128 for u8 {
    fn as_i128(self) -> i128 {
        self.into()
    }
}

impl AsF32 for u8 {
    fn as_f32(self) -> f32 {
        self.into()
    }
}
impl AsF64 for u8 {
    fn as_f64(self) -> f64 {
        self.into()
    }
}
