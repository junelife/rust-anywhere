use crate::color::Color;
use crate::obj::ObjT;

use core::ptr;
use lvgl_sys::*;

#[repr(transparent)]
pub struct Gauge(*mut lv_obj_t);

impl Gauge {
    #[inline]
    pub fn new() -> Option<Self> {
        unsafe {
            let gauge = lv_gauge_create(lv_scr_act(), ptr::null());
            if gauge.is_null() {
                None
            } else {
                Some(Gauge(gauge))
            }
        }
    }

    // Setter methods.

    /// Set the number of needles.
    pub fn set_needle_count(&mut self, needle_cnt: u8, colors: &'static [Color]) {
        assert!(
            colors.len() == needle_cnt as usize,
            "Unexpected number of needle colors"
        );
        unsafe {
            lv_gauge_set_needle_count(self.0, needle_cnt, colors.as_ptr() as *const lv_color_t)
        }
    }

    /// Set the value of a needle.   
    #[inline]
    pub fn set_value(&mut self, needle_id: u8, value: i16) {
        unsafe {
            lv_gauge_set_value(self.0, needle_id, value);
        }
    }

    /// Set minimum and the maximum values of the gauge.
    #[inline]
    pub fn set_range(&mut self, min: i16, max: i16) {
        unsafe {
            // NOTE: call the lmeter function directly.
            lv_lmeter_set_range(self.0, min, max);
        }
    }

    /// Set a critical value on the scale. After this value 'line.color' scale
    /// lines will be drawn
    #[inline]
    pub fn set_critical_value(&mut self, value: i16) {
        unsafe {
            // NOTE: call the lmeter function directly.
            lv_lmeter_set_value(self.0, value);
        }
    }

    /// Seet the scale settings.
    #[inline]
    pub fn set_scale(&mut self, angle: u16, line_count: u8, label_count: u8) {
        unsafe {
            lv_gauge_set_scale(self.0, angle, line_count, label_count);
        }
    }
}

impl ObjT for Gauge {
    #[inline]
    unsafe fn as_lv_obj(&self) -> *mut lv_obj_t {
        self.0
    }
}
