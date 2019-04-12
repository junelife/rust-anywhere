//! Mappings to platform dependent types especially designed for use with
//! ffi bindings to C language functions.

#![allow(non_camel_case_types)]
#![cfg_attr(target_os = "none", no_std)]

#[cfg(not(target_os = "none"))]
pub use std::os::raw::*;

#[cfg(all(target_os = "none", target_pointer_width = "32"))]
pub use self::nostd32::*;

#[cfg(all(target_os = "none", target_pointer_width = "32"))]
mod nostd32 {
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
    pub type c_float = f32;
    pub type c_double = f64;
    pub type c_void = core::ffi::c_void;
}
