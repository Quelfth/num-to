use super::*;

impl<T: AsU8> ToU8 for T {
    fn to_u8(self) -> u8 {
        self.as_u8()
    }
}
impl<T: AsU16> ToU16 for T {
    fn to_u16(self) -> u16 {
        self.as_u16()
    }
}
impl<T: AsU32> ToU32 for T {
    fn to_u32(self) -> u32 {
        self.as_u32()
    }
}
impl<T: AsU64> ToU64 for T {
    fn to_u64(self) -> u64 {
        self.as_u64()
    }
}
impl<T: AsU128> ToU128 for T {
    fn to_u128(self) -> u128 {
        self.as_u128()
    }
}

impl<T: AsI8> ToI8 for T {
    fn to_i8(self) -> i8 {
        self.as_i8()
    }
}
impl<T: AsI16> ToI16 for T {
    fn to_i16(self) -> i16 {
        self.as_i16()
    }
}
impl<T: AsI32> ToI32 for T {
    fn to_i32(self) -> i32 {
        self.as_i32()
    }
}
impl<T: AsI64> ToI64 for T {
    fn to_i64(self) -> i64 {
        self.as_i64()
    }
}
impl<T: AsI128> ToI128 for T {
    fn to_i128(self) -> i128 {
        self.as_i128()
    }
}

impl<T: AsF32> ToF32 for T {
    fn to_f32(self) -> f32 {
        self.as_f32()
    }
}
impl<T: AsF64> ToF64 for T {
    fn to_f64(self) -> f64 {
        self.as_f64()
    }
}

#[cfg(target_pointer_width = "16")]
mod platform_specific {
    use super::*;
    impl<T: AsU16> AsUsize for T {
        fn as_usize(self) -> usize {
            self.as_u16() as usize
        }
    }
    impl<T: ToU16> ToUsize for T {
        fn to_usize(self) -> usize {
            self.to_u16() as usize
        }
    }
    impl<T: AsI16> AsIsize for T {
        fn as_isize(self) -> isize {
            self.as_i16() as isize
        }
    }
    impl<T: ToI16> ToIsize for T {
        fn to_isize(self) -> isize {
            self.to_i16() as isize
        }
    }
}

#[cfg(target_pointer_width = "32")]
mod platform_specific {
    use super::*;
    impl<T: AsU32> AsUsize for T {
        fn as_usize(self) -> usize {
            self.as_u32() as usize
        }
    }
    impl<T: ToU32> ToUsize for T {
        fn to_usize(self) -> usize {
            self.to_u32() as usize
        }
    }
    impl<T: AsI32> AsIsize for T {
        fn as_isize(self) -> isize {
            self.as_i32() as isize
        }
    }
    impl<T: ToI32> ToIsize for T {
        fn to_isize(self) -> isize {
            self.to_i32() as isize
        }
    }
}

#[cfg(target_pointer_width = "64")]
mod platform_specific {
    use super::*;
    impl<T: AsU64> AsUsize for T {
        fn as_usize(self) -> usize {
            self.as_u64() as usize
        }
    }
    impl<T: ToU64> ToUsize for T {
        fn to_usize(self) -> usize {
            self.to_u64() as usize
        }
    }
    impl<T: AsI64> AsIsize for T {
        fn as_isize(self) -> isize {
            self.as_i64() as isize
        }
    }
    impl<T: ToI64> ToIsize for T {
        fn to_isize(self) -> isize {
            self.to_i64() as isize
        }
    }
}
