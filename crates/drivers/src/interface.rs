use api::*;

use crate::driver_manager::DriverManager;

fn get_driver_manager() -> &'static mut DriverManager {
    static mut DRIVER_MANAGER: DriverManager = DriverManager::new_empty();
    unsafe { &mut DRIVER_MANAGER }
}

#[no_mangle]
pub extern "C" fn drivers_init() {
    let drivers = get_driver_manager();
    drivers.init();
}

#[no_mangle]
pub extern "C" fn drivers_set_led(which: LEDs, on: bool) {
    let drivers = get_driver_manager();
    drivers.set_led(which, on);
}

#[no_mangle]
pub extern "C" fn drivers_is_button_pressed() -> bool {
    let drivers = get_driver_manager();
    drivers.is_button_pressed()
}
