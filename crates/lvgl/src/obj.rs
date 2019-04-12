use crate::style::Style;

use core::ptr;
use lvgl_sys::*;

#[repr(u8)]
pub enum Align {
    Center = LV_ALIGN_CENTER as lv_align_t,
    InTopLeft = LV_ALIGN_IN_TOP_LEFT as lv_align_t,
    InTopMid = LV_ALIGN_IN_TOP_MID as lv_align_t,
    InTopRight = LV_ALIGN_IN_TOP_RIGHT as lv_align_t,
    InBottomLeft = LV_ALIGN_IN_BOTTOM_LEFT as lv_align_t,
    InBottomMid = LV_ALIGN_IN_BOTTOM_MID as lv_align_t,
    InBottomRight = LV_ALIGN_IN_BOTTOM_RIGHT as lv_align_t,
    InLeftMid = LV_ALIGN_IN_LEFT_MID as lv_align_t,
    InRightMid = LV_ALIGN_IN_RIGHT_MID as lv_align_t,
    OutTopLeft = LV_ALIGN_OUT_TOP_LEFT as lv_align_t,
    OutTopMid = LV_ALIGN_OUT_TOP_MID as lv_align_t,
    OutTopRight = LV_ALIGN_OUT_TOP_RIGHT as lv_align_t,
    OutBottomLeft = LV_ALIGN_OUT_BOTTOM_LEFT as lv_align_t,
    OutBottomMid = LV_ALIGN_OUT_BOTTOM_MID as lv_align_t,
    OutBottomRight = LV_ALIGN_OUT_BOTTOM_RIGHT as lv_align_t,
    OutLeftTop = LV_ALIGN_OUT_LEFT_TOP as lv_align_t,
    OutLeftMid = LV_ALIGN_OUT_LEFT_MID as lv_align_t,
    OutLeftBottom = LV_ALIGN_OUT_LEFT_BOTTOM as lv_align_t,
    OutRightTop = LV_ALIGN_OUT_RIGHT_TOP as lv_align_t,
    OutRightMid = LV_ALIGN_OUT_RIGHT_MID as lv_align_t,
    OutRightBottom = LV_ALIGN_OUT_RIGHT_BOTTOM as lv_align_t,
}

pub trait ObjT {
    unsafe fn as_lv_obj(&self) -> *mut lv_obj_t;

    /// Mark the object as invalid therefore its current position will be
    /// redrawn by 'lv_refr_task'.
    #[inline]
    fn invalidate(&mut self) {
        unsafe {
            lv_obj_invalidate(self.as_lv_obj());
        }
    }

    // Coordinate setters.

    /// Set relative the position of the object (relative to its parent).
    #[inline]
    fn set_pos(&mut self, x: lv_coord_t, y: lv_coord_t) {
        unsafe {
            lv_obj_set_pos(self.as_lv_obj(), x, y);
        }
    }

    /// Set the x coordinate of the object.  
    #[inline]
    fn set_x(&mut self, x: lv_coord_t) {
        unsafe {
            lv_obj_set_x(self.as_lv_obj(), x);
        }
    }

    /// Set the y coordinate of the object.
    #[inline]
    fn set_y(&mut self, y: lv_coord_t) {
        unsafe {
            lv_obj_set_y(self.as_lv_obj(), y);
        }
    }

    /// Set the size of the object.
    #[inline]
    fn set_size(&mut self, width: lv_coord_t, height: lv_coord_t) {
        unsafe {
            lv_obj_set_size(self.as_lv_obj(), width, height);
        }
    }

    /// Set the width of the object.
    #[inline]
    fn set_width(&mut self, width: lv_coord_t) {
        unsafe {
            lv_obj_set_width(self.as_lv_obj(), width);
        }
    }

    /// Set the height of the object.
    #[inline]
    fn set_height(&mut self, height: lv_coord_t) {
        unsafe {
            lv_obj_set_height(self.as_lv_obj(), height);
        }
    }

    /// Align the object to its parent.
    #[inline]
    fn align_to_parent(&mut self, align: Align, x_mod: lv_coord_t, y_mod: lv_coord_t) {
        unsafe {
            lv_obj_align(
                self.as_lv_obj(),
                ptr::null(),
                align as lv_align_t,
                x_mod,
                y_mod,
            );
        }
    }

    /// Align the object to another object.
    #[inline]
    fn align_to_object(
        &mut self,
        other: &impl ObjT,
        align: Align,
        x_mod: lv_coord_t,
        y_mod: lv_coord_t,
    ) {
        unsafe {
            lv_obj_align(
                self.as_lv_obj(),
                other.as_lv_obj(),
                align as lv_align_t,
                x_mod,
                y_mod,
            );
        }
    }

    /// Realign the object based on the last `lv_obj_align` parameters.
    #[inline]
    fn realign(&mut self) {
        unsafe {
            lv_obj_realign(self.as_lv_obj());
        }
    }

    /// Enable the automatic realign of the object when its size has changed
    /// based on the last `lv_obj_align` parameters.
    #[inline]
    fn set_auto_realign(&mut self, enabled: bool) {
        unsafe {
            lv_obj_set_auto_realign(self.as_lv_obj(), enabled);
        }
    }

    // Appearance setters.

    /// Set a new style for an object.
    #[inline]
    fn set_style(&mut self, style: &'static Style) {
        let style = &**style as *const _ as *mut lv_style_t;
        unsafe {
            lv_obj_set_style(self.as_lv_obj(), style);
        }
    }

    /// Notify an object that its style has been modified.
    #[inline]
    fn refresh_style(&mut self) {
        unsafe {
            lv_obj_refresh_style(self.as_lv_obj());
        }
    }

    // Coordinate getters.

    /// Get the x coordinate of the object.
    #[inline]
    fn get_x(&self) -> lv_coord_t {
        unsafe { lv_obj_get_x(self.as_lv_obj()) }
    }

    /// Get the y coordinate of the object.
    #[inline]
    fn get_y(&self) -> lv_coord_t {
        unsafe { lv_obj_get_y(self.as_lv_obj()) }
    }

    /// Get the width of the object.
    #[inline]
    fn get_width(&self) -> lv_coord_t {
        unsafe { lv_obj_get_width(self.as_lv_obj()) }
    }

    /// Get the height of the object.
    #[inline]
    fn get_height(&self) -> lv_coord_t {
        unsafe { lv_obj_get_height(self.as_lv_obj()) }
    }

    /// Get the extended size attribute of the object.
    #[inline]
    fn get_ext_size(&self) -> lv_coord_t {
        unsafe { lv_obj_get_ext_size(self.as_lv_obj()) }
    }

    /// Get the automatic realign property of the object.
    #[inline]
    fn get_auto_realign(&self) -> bool {
        unsafe { lv_obj_get_auto_realign(self.as_lv_obj()) }
    }

    // Attribute getters.

    /// Get the hidden attribute of an object.
    #[inline]
    fn get_hidden(&self) -> bool {
        unsafe { lv_obj_get_hidden(self.as_lv_obj()) }
    }

    /// Get the click enable attribute of the object.
    #[inline]
    fn get_click(&self) -> bool {
        unsafe { lv_obj_get_click(self.as_lv_obj()) }
    }
}
