use lvgl_sys::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub union Color {
    lv_color: lv_color_t,
    value: lv_color_int_t,
}

impl Color {
    pub const WHITE: Color = Color::new(0xFF, 0xFF, 0xFF);
    pub const SILVER: Color = Color::new(0xC0, 0xC0, 0xC0);
    pub const GRAY: Color = Color::new(0x80, 0x80, 0x80);
    pub const BLACK: Color = Color::new(0x00, 0x00, 0x00);
    pub const RED: Color = Color::new(0xFF, 0x00, 0x00);
    pub const MAROON: Color = Color::new(0x80, 0x00, 0x00);
    pub const YELLOW: Color = Color::new(0xFF, 0xFF, 0x00);
    pub const OLIVE: Color = Color::new(0x80, 0x80, 0x00);
    pub const LIME: Color = Color::new(0x00, 0xFF, 0x00);
    pub const GREEN: Color = Color::new(0x00, 0x80, 0x00);
    pub const CYAN: Color = Color::new(0x00, 0xFF, 0xFF);
    pub const AQUA: Color = Color::CYAN;
    pub const TEAL: Color = Color::new(0x00, 0x80, 0x80);
    pub const BLUE: Color = Color::new(0x00, 0x00, 0xFF);
    pub const NAVY: Color = Color::new(0x00, 0x00, 0x80);
    pub const MAGENTA: Color = Color::new(0xFF, 0x00, 0xFF);
    pub const PURPLE: Color = Color::new(0x80, 0x00, 0x80);
    pub const ORANGE: Color = Color::new(0xFF, 0xA5, 0x00);

    #[inline]
    #[cfg(all(lv_color_depth = "1", target_endian = "little"))]
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            value: (b >> 7) | (g >> 7) | (r >> 7),
        }
    }

    #[inline]
    #[cfg(all(lv_color_depth = "16", target_endian = "little"))]
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            value: (((r >> 3) as u16) << 11) | (((g >> 2) as u16) << 5) | ((b >> 3) as u16),
        }
    }
}

impl From<lv_color_t> for Color {
    #[inline]
    fn from(item: lv_color_t) -> Self {
        Color { lv_color: item }
    }
}

impl From<Color> for lv_color_t {
    #[inline]
    fn from(item: Color) -> Self {
        unsafe { item.lv_color }
    }
}
