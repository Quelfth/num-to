use super::*;

impl ToU8 for i8 {
    fn to_u8(self) -> u8 {
        if self & Self::MIN != 0 {
            0
        } else {
            self as u8
        }
    }
}
impl ToU16 for i8 {
    fn to_u16(self) -> u16 {
        self.to_u8().as_u16()
    }
}
impl ToU32 for i8 {
    fn to_u32(self) -> u32 {
        self.to_u8().as_u32()
    }
}
impl ToU64 for i8 {
    fn to_u64(self) -> u64 {
        self.to_u8().as_u64()
    }
}
impl ToU128 for i8 {
    fn to_u128(self) -> u128 {
        self.to_u8().as_u128()
    }
}

impl AsI8 for i8 {
    fn as_i8(self) -> i8 {
        self
    }
}
impl AsI16 for i8 {
    fn as_i16(self) -> i16 {
        self.into()
    }
}
impl AsI32 for i8 {
    fn as_i32(self) -> i32 {
        self.into()
    }
}
impl AsI64 for i8 {
    fn as_i64(self) -> i64 {
        self.into()
    }
}
impl AsI128 for i8 {
    fn as_i128(self) -> i128 {
        self.into()
    }
}

impl AsF32 for i8 {
    fn as_f32(self) -> f32 {
        self.into()
    }
}
impl AsF64 for i8 {
    fn as_f64(self) -> f64 {
        self.into()
    }
}
