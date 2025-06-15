#![allow(clippy::wrong_self_convention)]
//! This crate provides traits [`To{type}`] and [`As{type}`] providing methods [`to_{type}`] and [`as_{type}`] for converting numbers between types
//! without using [`as`] casts.  The rules for these names are as follows.  Methods named
//! [`as_{type}`] will always convert the value losslessly producing an exact result, whereas
//! [`to_{type]`] are possibly lossy but will always convert to the nearest value of the result type during
//! integer-integer conversions, and for conversions involving floating-point values, the result is
//! the same as that of an [`as`] cast.
//! 
//! Whenever an [`As`] trait is implemented, the corresponding [`To`] trait is also implemented,
//! and has the same behavior.
//! 
//! Existence of [`AsUsize`] and [`AsIsize`] for a particular type is target architecture
//! dependent, so usage of these traits is discouraged.
//! 
//! [`use num-to::*;`] will include only the 14 [`As`] traits, 14 [`To`] traits and nothing
//! else.

mod blanket;
mod u8;
mod u16;
mod u32;
mod u64;
mod u128;
mod usize;

mod i8;
mod i16;
mod i32;
mod i64;
mod i128;
mod isize;

mod f32;
mod f64;

pub trait AsU8 { fn as_u8(self) -> u8; }
pub trait AsU16 { fn as_u16(self) -> u16; }
pub trait AsU32 { fn as_u32(self) -> u32; }
pub trait AsU64 { fn as_u64(self) -> u64; }
pub trait AsU128 { fn as_u128(self) -> u128; }
pub trait AsUsize { fn as_usize(self) -> usize; }

pub trait AsI8 { fn as_i8(self) -> i8; }
pub trait AsI16 { fn as_i16(self) -> i16; }
pub trait AsI32 { fn as_i32(self) -> i32; }
pub trait AsI64 { fn as_i64(self) -> i64; }
pub trait AsI128 { fn as_i128(self) -> i128; }
pub trait AsIsize { fn as_isize(self) -> isize; }

pub trait AsF32 { fn as_f32(self) -> f32; }
pub trait AsF64 { fn as_f64(self) -> f64; }

pub trait ToU8 { fn to_u8(self) -> u8; }
pub trait ToU16 { fn to_u16(self) -> u16; }
pub trait ToU32 { fn to_u32(self) -> u32; }
pub trait ToU64 { fn to_u64(self) -> u64; }
pub trait ToU128 { fn to_u128(self) -> u128; }
pub trait ToUsize { fn to_usize(self) -> usize; }

pub trait ToI8 { fn to_i8(self) -> i8; }
pub trait ToI16 { fn to_i16(self) -> i16; }
pub trait ToI32 { fn to_i32(self) -> i32; }
pub trait ToI64 { fn to_i64(self) -> i64; }
pub trait ToI128 { fn to_i128(self) -> i128; }
pub trait ToIsize { fn to_isize(self) -> isize; }

pub trait ToF32 { fn to_f32(self) -> f32; }
pub trait ToF64 { fn to_f64(self) -> f64; }
