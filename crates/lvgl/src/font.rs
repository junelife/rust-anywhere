use lvgl_sys::*;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Font(*const lv_font_t);

impl Font {
    pub fn dejavu_10() -> Font {
        unsafe { Font(&lv_font_dejavu_10) }
    }

    pub fn dejavu_20() -> Font {
        unsafe { Font(&lv_font_dejavu_20) }
    }

    pub fn dejavu_30() -> Font {
        unsafe { Font(&lv_font_dejavu_30) }
    }

    pub fn dejavu_40() -> Font {
        unsafe { Font(&lv_font_dejavu_40) }
    }
}

impl From<*const lv_font_t> for Font {
    #[inline]
    fn from(item: *const lv_font_t) -> Self {
        Font(item)
    }
}

impl From<Font> for *const lv_font_t {
    #[inline]
    fn from(item: Font) -> Self {
        item.0
    }
}
