use crate::obj::ObjT;

use core::ffi::c_void;
use core::ptr;
use lvgl_sys::*;

#[repr(transparent)]
pub struct Image(*mut lv_obj_t);

impl Image {
    #[inline]
    pub fn new() -> Option<Self> {
        unsafe {
            let image = lv_img_create(lv_scr_act(), ptr::null());
            if image.is_null() {
                None
            } else {
                Some(Image(image))
            }
        }
    }

    // Setter methods.

    /// Set the pixel map to display by the image.
    #[inline]
    pub fn set_src(&mut self, src: &'static ImageSource) {
        unsafe {
            lv_img_set_src(self.0, src.as_img_src_ptr());
        }
    }

    /// Enable the auto size feature.
    #[inline]
    pub fn set_auto_size(&mut self, enabled: bool) {
        unsafe {
            lv_img_set_auto_size(self.0, enabled);
        }
    }
}

impl ObjT for Image {
    #[inline]
    unsafe fn as_lv_obj(&self) -> *mut lv_obj_t {
        self.0
    }
}

pub trait ImageSource {
    fn as_img_src_ptr(&self) -> *const c_void;
}

#[repr(transparent)]
pub struct ImageDescriptor(lv_img_dsc_t);

impl ImageSource for ImageDescriptor {
    #[inline]
    fn as_img_src_ptr(&self) -> *const c_void {
        self as *const _ as *const c_void
    }
}
