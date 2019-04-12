use crate::obj::ObjT;

use core::ptr;
use ffi::*;
use lvgl_sys::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LabelLongMode {
    Expand = LV_LABEL_LONG_EXPAND as lv_label_long_mode_t,
    Break = LV_LABEL_LONG_BREAK as lv_label_long_mode_t,
    Scroll = LV_LABEL_LONG_SCROLL as lv_label_long_mode_t,
    Dot = LV_LABEL_LONG_DOT as lv_label_long_mode_t,
    Roll = LV_LABEL_LONG_ROLL as lv_label_long_mode_t,
    Crop = LV_LABEL_LONG_CROP as lv_label_long_mode_t,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LabelAlign {
    Left = LV_LABEL_ALIGN_LEFT as lv_label_align_t,
    Center = LV_LABEL_ALIGN_CENTER as lv_label_align_t,
    Right = LV_LABEL_ALIGN_RIGHT as lv_label_align_t,
}

#[repr(transparent)]
pub struct Label(*mut lv_obj_t);

impl Label {
    /// Create a new label.
    #[inline]
    pub fn new() -> Option<Self> {
        unsafe {
            let label = lv_label_create(lv_scr_act(), ptr::null());
            if label.is_null() {
                None
            } else {
                Some(Label(label))
            }
        }
    }

    // Setter functions.

    /// Set a new text for a label. Memory will be allocated to store the
    /// text by the label.
    #[inline]
    pub fn set_text(&mut self, text: &[u8]) {
        unsafe {
            lv_label_set_text(self.0, text.as_ptr() as *const c_char);
        }
    }

    /// Set a static text. It will not be saved by the label so the 'text'
    /// variable has to be 'alive' while the label exist.
    #[inline]
    pub fn set_static_text(&mut self, text: &'static [u8]) {
        unsafe {
            lv_label_set_static_text(self.0, text.as_ptr() as *const c_char);
        }
    }

    /// Set the behavior of the label with longer text then the object size.
    #[inline]
    pub fn set_long_mode(&mut self, mode: LabelLongMode) {
        let mode = mode as lv_label_long_mode_t;
        unsafe {
            lv_label_set_long_mode(self.0, mode);
        }
    }

    /// Set the align of the label (left or center).
    #[inline]
    pub fn set_align(&mut self, align: LabelAlign) {
        let align = align as lv_label_align_t;
        unsafe {
            lv_label_set_align(self.0, align);
        }
    }

    /// Enable the recoloring by in-line commands.
    #[inline]
    pub fn set_recolor(&mut self, enabled: bool) {
        unsafe {
            lv_label_set_recolor(self.0, enabled);
        }
    }

    /// Set the label to draw (or not draw) background specified in its
    /// style's body.
    #[inline]
    pub fn set_body_draw(&mut self, enabled: bool) {
        unsafe {
            lv_label_set_body_draw(self.0, enabled);
        }
    }

    /// Set the label's animation speed in LV_LABEL_LONG_ROLL and SCROLL
    /// modes.
    #[inline]
    pub fn set_anim_speed(&mut self, speed: u16) {
        unsafe {
            lv_label_set_anim_speed(self.0, speed);
        }
    }
}

impl ObjT for Label {
    #[inline]
    unsafe fn as_lv_obj(&self) -> *mut lv_obj_t {
        self.0
    }
}
