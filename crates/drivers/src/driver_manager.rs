#![allow(non_upper_case_globals)]

use api::*;
use board::hal::prelude::*;
use board::gpio::*;
use board::gpio::gpioa::*;
use board::gpio::gpiog::*;
use board::hal::stm32;

pub struct DriverManager {
    led_green: Option<PG13<Output<PushPull>>>,
    led_red: Option<PG14<Output<PushPull>>>,
    button: Option<PA0<Input<PullDown>>>,
}

impl DriverManager {
    pub const fn new_empty() -> Self {
        DriverManager {
            led_green: None,
            led_red: None,
            button: None,
        }
    }

    pub fn init(&mut self) {
        if let Some(p) = stm32::Peripherals::take() {
            let gpioa = p.GPIOA.split();
            let gpiog = p.GPIOG.split();

            // (Re-)configure PG13 (green LED) as output.
            self.led_green = Some(gpiog.pg13.into_push_pull_output());
            // (Re-)configure PG14 (red LED) as output.
            self.led_red = Some(gpiog.pg14.into_push_pull_output());
            // (Re-)configure PA0 (user button) as input.
            self.button = Some(gpioa.pa0.into_pull_down_input());
        }
    }

    pub fn set_led(&mut self, which: LEDs, on: bool) {
        match (which, on) {
            (LEDs_GREEN, true) => {
                if let Some(ref mut led) = self.led_green {
                    led.set_high();
                }
            }
            (LEDs_GREEN, false) => {
                if let Some(ref mut led) = self.led_green {
                    led.set_low();
                }
            }
            (LEDs_RED, true) => {
                if let Some(ref mut led) = self.led_red {
                    led.set_high();
                }
            }
            (LEDs_RED, false) => {
                if let Some(ref mut led) = self.led_red {
                    led.set_low();
                }
            }
            (_, _) => unreachable!()
        }
    }

    pub fn is_button_pressed(&self) -> bool {
        if let Some(ref button) = self.button {
            button.is_high()
        } else {
            false
        }
    }
}
