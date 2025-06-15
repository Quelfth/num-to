use super::*;

#[cfg(target_pointer_width = "16")]
mod platform_dependent {
    use super::*;

    impl ToU8 for usize {
        fn to_u8(self) -> u8 {
            if self & 0xff00 != 0 {
                u8::MAX
            } else {
                self as u8
            }
        }
    }
    impl AsU16 for usize {
        fn as_u16(self) -> u16 {
            self as u16
        }
    }
    impl AsU32 for usize {
        fn as_u32(self) -> u32 {
            (self as u16).into()
        }
    }
    impl AsU64 for usize {
        fn as_u64(self) -> u64 {
            (self as u16).into()
        }
    }
    impl AsU128 for usize {
        fn as_u128(self) -> u128 {
            (self as u16).into()
        }
    }

    impl ToI8 for usize {
        fn to_i8(self) -> i8 {
            if self & 0xff80 != 0 {
                i8::MAX
            } else {
                self as i8
            }
        }
    }
    impl AsI16 for usize {
        fn as_i16(self) -> i16 {
            if self & 0x8000 != 0 {
                i16::MAX
            } else {
                self as i16
            }
        }
    }
    impl AsI32 for usize {
        fn as_i32(self) -> i32 {
            (self as u16).into()
        }
    }
    impl AsI64 for usize {
        fn as_i64(self) -> i64 {
            (self as u16).into()
        }
    }
    impl AsI128 for usize {
        fn as_i128(self) -> i128 {
            (self as u16).into()
        }
    }

    impl AsF32 for usize {
        fn as_f32(self) -> f32 {
            (self as u16).into()
        }
    }
    impl AsF64 for usize {
        fn as_f64(self) -> f64 {
            (self as u16).into()
        }
    }
}

#[cfg(target_pointer_width = "32")]
mod platform_dependent {
    use super::*;

    impl ToU8 for usize {
        fn to_u8(self) -> u8 {
            if self & 0xffff_ff00 != 0 {
                u8::MAX
            } else {
                self as u8
            }
        }
    }
    impl AsU16 for usize {
        fn as_u16(self) -> u16 {
            if self & 0xffff_0000 != 0 {
                u16::MAX
            } else {
                self as u16
            }
        }
    }
    impl AsU32 for usize {
        fn as_u32(self) -> u32 {
            self as u32
        }
    }
    impl AsU64 for usize {
        fn as_u64(self) -> u64 {
            (self as u32).into()
        }
    }
    impl AsU128 for usize {
        fn as_u128(self) -> u128 {
            (self as u32).into()
        }
    }

    impl ToI8 for usize {
        fn to_i8(self) -> i8 {
            if self & 0xffff_ff80 != 0 {
                i8::MAX
            } else {
                self as i8
            }
        }
    }
    impl AsI16 for usize {
        fn as_i16(self) -> i16 {
            if self & 0xffff_8000 != 0 {
                i16::MAX
            } else {
                self as i16
            }
        }
    }
    impl ToI32 for usize {
        fn to_i32(self) -> i32 {
            if self & 0x8000_0000 != 0 {
                i32::MAX
            } else {
                self as i32
            }
        }
    }
    impl AsI64 for usize {
        fn as_i64(self) -> i64 {
            (self as u32).into()
        }
    }
    impl AsI128 for usize {
        fn as_i128(self) -> i128 {
            (self as u32).into()
        }
    }

    impl ToF32 for usize {
        fn to_f32(self) -> f32 {
            self as f32
        }
    }
    impl AsF64 for usize {
        fn as_f64(self) -> f64 {
            (self as u32).into()
        }
    }
}

#[cfg(target_pointer_width = "64")]
mod platform_dependent {
    use super::*;

    impl ToU8 for usize {
        fn to_u8(self) -> u8 {
            if self & 0xffff_ffff_ffff_ff00 != 0 {
                u8::MAX
            } else {
                self as u8
            }
        }
    }
    impl AsU16 for usize {
        fn as_u16(self) -> u16 {
            if self & 0xffff_ffff_ffff_0000 != 0 {
                u16::MAX
            } else {
                self as u16
            }
        }
    }
    impl ToU32 for usize {
        fn to_u32(self) -> u32 {
            if self & 0xffff_ffff_0000_0000 != 0 {
                u32::MAX
            } else {
                self as u32
            }
        }
    }
    impl AsU64 for usize {
        fn as_u64(self) -> u64 {
            self as u64
        }
    }
    impl AsU128 for usize {
        fn as_u128(self) -> u128 {
            (self as u64).into()
        }
    }

    impl ToI8 for usize {
        fn to_i8(self) -> i8 {
            if self & 0xffff_ffff_ffff_ff80 != 0 {
                i8::MAX
            } else {
                self as i8
            }
        }
    }
    impl AsI16 for usize {
        fn as_i16(self) -> i16 {
            if self & 0xffff_ffff_ffff_8000 != 0 {
                i16::MAX
            } else {
                self as i16
            }
        }
    }
    impl ToI32 for usize {
        fn to_i32(self) -> i32 {
            if self & 0xffff_ffff_8000_0000 != 0 {
                i32::MAX
            } else {
                self as i32
            }
        }
    }
    impl AsI64 for usize {
        fn as_i64(self) -> i64 {
            if self & 0x8000_0000_0000_0000 != 0 {
                i64::MAX
            } else {
                self as i64
            }
        }
    }
    impl AsI128 for usize {
        fn as_i128(self) -> i128 {
            (self as u64).into()
        }
    }

    impl ToF32 for usize {
        fn to_f32(self) -> f32 {
            self as f32
        }
    }
    impl ToF64 for usize {
        fn to_f64(self) -> f64 {
            self as f64
        }
    }
}
