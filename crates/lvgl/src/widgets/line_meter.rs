use crate::obj::ObjT;

use core::ptr;
use lvgl_sys::*;

#[repr(transparent)]
pub struct LineMeter(*mut lv_obj_t);

impl LineMeter {
    #[inline]
    pub fn new() -> Option<Self> {
        unsafe {
            let lmeter = lv_lmeter_create(lv_scr_act(), ptr::null());
            if lmeter.is_null() {
                None
            } else {
                Some(LineMeter(lmeter))
            }
        }
    }

    // Setter methods.

    /// Set a new value on the line meter.
    #[inline]
    pub fn set_value(&mut self, value: i16) {
        unsafe {
            lv_lmeter_set_value(self.0, value);
        }
    }

    /// Set minimum and the maximum values of the line meter.
    #[inline]
    pub fn set_range(&mut self, min: i16, max: i16) {
        unsafe {
            lv_lmeter_set_range(self.0, min, max);
        }
    }

    /// Set the scale settings on the line meter.
    #[inline]
    pub fn set_scale(&mut self, angle: u16, line_cnt: u8) {
        unsafe {
            lv_lmeter_set_scale(self.0, angle, line_cnt);
        }
    }

    // Getter methods.

    // Get the value of the line meter.
}

impl ObjT for LineMeter {
    #[inline]
    unsafe fn as_lv_obj(&self) -> *mut lv_obj_t {
        self.0
    }
}
