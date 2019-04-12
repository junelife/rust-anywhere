use api::*;
use lvgl::*;

const LONG_PRESS_MS: u32 = 400;

enum TimerState {
    Stopped,
    Running,
    Paused,
}

pub struct HmiManager {
    title: Option<Label>,
    gauge: Option<Gauge>,
    timestamp: Option<Label>,
    timer_state: TimerState,
    timer_elapsed: u32,
    last_ticks: u32,
    last_button_pressed: bool,
    last_button_ticks: u32,
}

impl HmiManager {
    pub const fn new_empty() -> Self {
        HmiManager {
            title: None,
            gauge: None,
            timestamp: None,
            timer_state: TimerState::Stopped,
            timer_elapsed: 0,
            last_ticks: 0,
            last_button_pressed: false,
            last_button_ticks: 0,
        }
    }

    pub fn init(&mut self) {
        let screen_style = Style::screen();
        (*screen_style).body.main_color = Color::BLUE.into();
        (*screen_style).body.grad_color = Color::NAVY.into();

        static mut TITLE_STYLE: Style = Style::new_empty();
        let title_style = unsafe { &mut TITLE_STYLE };
        title_style.copy_from(Style::pretty());
        (*title_style).text.font = Font::dejavu_20().into();
        (*title_style).text.color = Color::WHITE.into();

        if let Some(mut title) = Label::new() {
            title.set_style(title_style);
            title.set_static_text(b"Stopwatch\0");
            title.align_to_parent(Align::InTopMid, 0, 10);
            self.title = Some(title);
        }

        static mut GAUGE_STYLE: Style = Style::new_empty();
        let gauge_style = unsafe { &mut GAUGE_STYLE };
        gauge_style.copy_from(Style::pretty());
        (*gauge_style).text.font = Font::dejavu_20().into();
        (*gauge_style).text.color = Color::WHITE.into();

        if let Some(mut gauge) = Gauge::new() {
            gauge.set_style(gauge_style);
            gauge.set_size(220, 220);
            gauge.set_scale(270, 61, 7);
            gauge.set_range(0, 60);
            gauge.align_to_parent(Align::Center, 0, 5);
            self.gauge = Some(gauge);
        }

        static mut TIMESTAMP_STYLE: Style = Style::new_empty();
        let timestamp_style = unsafe { &mut TIMESTAMP_STYLE };
        timestamp_style.copy_from(Style::pretty());
        (*timestamp_style).text.font = Font::dejavu_40().into();
        (*timestamp_style).text.color = Color::WHITE.into();

        if let Some(mut timestamp) = Label::new() {
            timestamp.set_style(timestamp_style);
            timestamp.set_static_text(b"00:00.00\0");
            timestamp.align_to_parent(Align::InBottomMid, 0, 0);
            self.timestamp = Some(timestamp);
        }

        // Update display after changing styles.
        Style::report_all_mod();
    }

    pub unsafe fn update(&mut self) {
        let ticks = Tick::get();
        let button_pressed = drivers_is_button_pressed();

        match self.timer_state {
            TimerState::Stopped => {
                if !button_pressed && button_pressed != self.last_button_pressed {
                    self.timer_state = TimerState::Running;
                    self.display_leds();
                } 
            }
            TimerState::Running => {
                self.timer_elapsed += ticks - self.last_ticks;
                self.display_timer();
                if !button_pressed && button_pressed != self.last_button_pressed {
                    self.timer_state = TimerState::Paused;
                    self.display_leds();
                }
            }
            TimerState::Paused => {
                if !button_pressed && button_pressed != self.last_button_pressed {
                    let press_duration = ticks - self.last_button_ticks;
                    if press_duration < LONG_PRESS_MS {
                        self.timer_state = TimerState::Running;
                    } else {
                        self.timer_state = TimerState::Stopped;
                        self.timer_elapsed = 0;
                        self.display_timer();
                    }
                    self.display_leds();
                }
            }
        }

        if button_pressed != self.last_button_pressed {
            self.last_button_pressed = button_pressed;
            self.last_button_ticks = ticks;
        }
        self.last_ticks = ticks;
        self.last_button_pressed = button_pressed;
    }

    fn display_timer(&mut self) {
        let elapsed_min = (self.timer_elapsed / 60_000) % 60;
        let elapsed_sec = (self.timer_elapsed / 1_000) % 60;
        let elapsed_ds = (self.timer_elapsed / 10) % 100;
        if let Some(ref mut gauge) = self.gauge {
            gauge.set_value(0, elapsed_sec as i16);
        }

        let mut buffer = [0u8; 9];
        buffer[0] = ('0' as u8) + (elapsed_min / 10) as u8;
        buffer[1] = ('0' as u8) + (elapsed_min % 10) as u8;
        buffer[2] = ':' as u8;
        buffer[3] = ('0' as u8) + (elapsed_sec / 10) as u8;
        buffer[4] = ('0' as u8) + (elapsed_sec % 10) as u8;
        buffer[5] = '.' as u8;
        buffer[6] = ('0' as u8) + (elapsed_ds / 10) as u8;
        buffer[7] = ('0' as u8) + (elapsed_ds % 10) as u8;
        buffer[8] = '\0' as u8;
        if let Some(ref mut timestamp) = self.timestamp {
            timestamp.set_text(&buffer);
        }
    }

    unsafe fn display_leds(&self) {
        match self.timer_state {
            TimerState::Stopped => {
                drivers_set_led(LEDs_GREEN, false);
                drivers_set_led(LEDs_RED, false);
            }
            TimerState::Running => {
                drivers_set_led(LEDs_GREEN, true);
                drivers_set_led(LEDs_RED, false);
            }
            TimerState::Paused => {
                drivers_set_led(LEDs_GREEN, false);
                drivers_set_led(LEDs_RED, true);
            }
        }
    }
}
