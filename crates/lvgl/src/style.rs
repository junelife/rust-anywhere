use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr;
use lvgl_sys::*;

const STYLE_SIZE: usize = mem::size_of::<lv_style_t>();

#[repr(C)]
pub union Style {
    style: lv_style_t,
    empty: [u8; STYLE_SIZE],
}

impl Style {
    #[inline]
    pub fn plain() -> &'static mut Style {
        unsafe { &mut *(&mut lv_style_plain as *mut _ as *mut Style) }
    }

    #[inline]
    pub fn pretty() -> &'static mut Style {
        unsafe { &mut *(&mut lv_style_pretty as *mut _ as *mut Style) }
    }

    #[inline]
    pub fn screen() -> &'static mut Style {
        unsafe { &mut *(&mut lv_style_scr as *mut _ as *mut Style) }
    }

    /// Notify all objects that styles have changed.
    #[inline]
    pub fn report_all_mod() {
        unsafe {
            lv_obj_report_style_mod(ptr::null_mut());
        }
    }

    #[inline]
    pub const fn new_empty() -> Self {
        Style {
            empty: [0u8; STYLE_SIZE],
        }
    }

    /// Initialize from a copy of another style.
    #[inline]
    pub fn copy_from(&mut self, other: &Style) {
        unsafe {
            lv_style_copy(&mut self.style, &other.style);
        }
    }

    /// Notify objects that the style has changed.
    #[inline]
    pub fn report_mod(&self) {
        unsafe {
            let style = &self.style as *const _ as *mut lv_style_t;
            lv_obj_report_style_mod(style);
        }
    }

}

impl Deref for Style {
    type Target = lv_style_t;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &self.style }
    }
}

impl DerefMut for Style {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.style }
    }
}
