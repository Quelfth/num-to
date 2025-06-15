use super::*;

#[cfg(target_pointer_width = "16")]
mod platform_dependent {
    use super::*;

    impl ToU8 for isize {
        fn to_u8(self) -> u8 {
            self.to_u16().to_u8()
        }
    }
    impl ToU16 for isize {
        fn to_u16(self) -> u16 {
            if self & Self::MIN != 0 {
                0
            } else {
                self as u16
            }
        }
    }
    impl ToU32 for isize {
        fn to_u32(self) -> u32 {
            self.to_u16().as_u32()
        }
    }
    impl ToU64 for isize {
        fn to_u64(self) -> u64 {
            self.to_u16().as_u64()
        }
    }
    impl ToU128 for isize {
        fn to_u128(self) -> u128 {
            self.to_u16().as_u128()
        }
    }

    impl ToI8 for isize {
        fn to_i8(self) -> i8 {
            match self {
                ..-0x80 => i8::MIN,
                -0x80..0x80 => self as i8,
                0x80.. => i8::MAX,
            }
        }
    }
    impl AsI16 for isize {
        fn as_i16(self) -> i16 {
            self as i16
        }
    }
    impl AsI32 for isize {
        fn as_i32(self) -> i32 {
            (self as i16).into()
        }
    }
    impl AsI64 for isize {
        fn as_i64(self) -> i64 {
            (self as i16).into()
        }
    }
    impl AsI128 for isize {
        fn as_i128(self) -> i128 {
            (self as i16).into()
        }
    }

    impl AsF32 for isize {
        fn as_f32(self) -> f32 {
            self.into()
        }
    }
    impl AsF64 for isize {
        fn as_f64(self) -> f64 {
            self.into()
        }
    }
}

#[cfg(target_pointer_width = "32")]
mod platform_dependent {
    use super::*;

    impl ToU8 for isize {
        fn to_u8(self) -> u8 {
            self.to_u32().to_u8()
        }
    }
    impl ToU16 for isize {
        fn to_u16(self) -> u16 {
            self.to_u32().to_u16()
        }
    }
    impl ToU32 for isize {
        fn to_u32(self) -> u32 {
            if self & Self::MIN != 0 {
                0
            } else {
                self as u32
            }
        }
    }
    impl ToU64 for isize {
        fn to_u64(self) -> u64 {
            self.to_u32().as_u64()
        }
    }
    impl ToU128 for isize {
        fn to_u128(self) -> u128 {
            self.to_u32().as_u128()
        }
    }

    impl ToI8 for isize {
        fn to_i8(self) -> i8 {
            match self {
                ..-0x80 => i8::MIN,
                -0x80..0x80 => self as i8,
                0x80.. => i8::MAX,
            }
        }
    }
    impl ToI16 for isize {
        fn to_i16(self) -> i16 {
            match self {
                ..-0x8000 => i16::MIN,
                -0x8000..0x8000 => self as i16,
                0x8000.. => i16::MAX,
            }
        }
    }
    impl AsI32 for isize {
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI64 for isize {
        fn as_i64(self) -> i64 {
            (self as i32).into()
        }
    }
    impl AsI128 for isize {
        fn as_i128(self) -> i128 {
            (self as i32).into()
        }
    }

    impl ToF32 for isize {
        fn to_f32(self) -> f32 {
            self as f32
        }
    }
    impl AsF64 for isize {
        fn as_f64(self) -> f64 {
            self.into()
        }
    }
}

#[cfg(target_pointer_width = "64")]
mod platform_dependent {
    use super::*;

    impl ToU8 for isize {
        fn to_u8(self) -> u8 {
            self.to_u64().to_u8()
        }
    }
    impl ToU16 for isize {
        fn to_u16(self) -> u16 {
            self.to_u64().to_u16()
        }
    }
    impl ToU32 for isize {
        fn to_u32(self) -> u32 {
            self.to_u64().to_u32()
        }
    }
    impl ToU64 for isize {
        fn to_u64(self) -> u64 {
            if self & Self::MIN != 0 {
                0
            } else {
                self as u64
            }
        }
    }
    impl ToU128 for isize {
        fn to_u128(self) -> u128 {
            self.to_u64().as_u128()
        }
    }

    impl ToI8 for isize {
        fn to_i8(self) -> i8 {
            match self {
                ..-0x80 => i8::MIN,
                -0x80..0x80 => self as i8,
                0x80.. => i8::MAX,
            }
        }
    }
    impl ToI16 for isize {
        fn to_i16(self) -> i16 {
            match self {
                ..-0x8000 => i16::MIN,
                -0x8000..0x8000 => self as i16,
                0x8000.. => i16::MAX,
            }
        }
    }
    impl ToI32 for isize {
        fn to_i32(self) -> i32 {
            match self {
                ..-0x8000_0000 => i32::MIN,
                -0x8000_0000..0x8000_0000 => self as i32,
                0x8000_0000.. => i32::MAX,
            }
        }
    }
    impl AsI64 for isize {
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI128 for isize {
        fn as_i128(self) -> i128 {
            (self as i64).into()
        }
    }

    impl ToF32 for isize {
        fn to_f32(self) -> f32 {
            self as f32
        }
    }
    impl ToF64 for isize {
        fn to_f64(self) -> f64 {
            self as f64
        }
    }
}
