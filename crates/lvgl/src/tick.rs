use lvgl_sys::*;

pub struct Tick;

impl Tick {
    /// Increment the system tick count.
    #[inline]
    pub fn inc(tick_period: u32) {
        unsafe {
            lv_tick_inc(tick_period);
        }
    }

    /// Get the elapsed milliseconds since start up.
    #[inline]
    pub fn get() -> u32 {
        unsafe { lv_tick_get() }
    }

    /// Get the elapsed milliseconds since a previous timestamp.
    #[inline]
    pub fn elapsed(prev_tick: u32) -> u32 {
        unsafe { lv_tick_elaps(prev_tick) }
    }
}
