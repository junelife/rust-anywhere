use core::mem;
use core::ops::{Deref, DerefMut};
use lvgl_sys::*;

const THEME_SIZE: usize = mem::size_of::<lv_theme_t>();

#[repr(C)]
pub union Theme {
    lv_theme: lv_theme_t,
    empty: [u8; THEME_SIZE],
}

impl Theme {
    /// Get the current system theme.
    #[inline]
    pub fn get_current() -> Option<&'static Theme> {
        unsafe {
            let theme = lv_theme_get_current();
            if theme.is_null() {
                None
            } else {
                let theme = &*(theme as *mut Theme);
                Some(theme)
            }
        }
    }

    /// Set the theme as the current theme for the system.
    #[inline]
    pub fn set_current(theme: &'static Theme) {
        let theme = unsafe { &theme.lv_theme as *const _ as *mut lv_theme_t };
        unsafe {
            lv_theme_set_current(theme);
        }
    }

    #[inline]
    pub const fn new_empty() -> Self {
        Theme {
            empty: [0u8; THEME_SIZE],
        }
    }
}

impl Deref for Theme {
    type Target = lv_theme_t;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &self.lv_theme }
    }
}

impl DerefMut for Theme {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.lv_theme }
    }
}
