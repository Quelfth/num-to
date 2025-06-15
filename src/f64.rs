
use super::*;

impl ToU8 for f64 {
    fn to_u8(self) -> u8 {
        self as u8
    }
}
impl ToU16 for f64 {
    fn to_u16(self) -> u16 {
        self as u16
    }
}
impl ToU32 for f64 {
    fn to_u32(self) -> u32 {
        self as u32
    }
}
impl ToU64 for f64 {
    fn to_u64(self) -> u64 {
        self as u64
    }
}
impl ToU128 for f64 {
    fn to_u128(self) -> u128 {
        self as u128
    }
}

impl ToI8 for f64 {
    fn to_i8(self) -> i8 {
        self as i8
    }
}
impl ToI16 for f64 {
    fn to_i16(self) -> i16 {
        self as i16
    }
}
impl ToI32 for f64 {
    fn to_i32(self) -> i32 {
        self as i32
    }
}
impl ToI64 for f64 {
    fn to_i64(self) -> i64 {
        self as i64
    }
}
impl ToI128 for f64 {
    fn to_i128(self) -> i128 {
        self as i128
    }
}

impl ToF32 for f64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}
impl AsF64 for f64 {
    fn as_f64(self) -> f64 {
        self
    }
}
