use crate::hmi_manager::HmiManager;

fn get_hmi_manager() -> &'static mut HmiManager {
    static mut HMI_MANAGER: HmiManager = HmiManager::new_empty();
    unsafe { &mut HMI_MANAGER }
}

#[no_mangle]
pub extern "C" fn hmi_init() {
    let hmi = get_hmi_manager();
    hmi.init();
}

#[no_mangle]
pub extern "C" fn hmi_update() {
    let hmi = get_hmi_manager();
    unsafe {
        hmi.update();
    }
}
