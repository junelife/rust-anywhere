use std::env;

fn main() {
    set_config_value("DEP_LVGL_LV_COLOR_DEPTH", "lv_color_depth");
    set_config_bool("DEP_LVGL_USE_LV_GAUGE", "use_lv_gauge");
    set_config_bool("DEP_LVGL_USE_LV_IMG", "use_lv_img");
    set_config_bool("DEP_LVGL_USE_LV_LABEL", "use_lv_label");
    set_config_bool("DEP_LVGL_USE_LV_LMETER", "use_lv_lmeter");
}

fn set_config_bool(varname: &str, name: &str) {
    if let Ok(value) = env::var(varname) {
        if value == "enabled" {
            println!("cargo:rustc-cfg={}", name);
        }
    }
}

fn set_config_value(varname: &str, name: &str) {
    if let Ok(value) = env::var(varname) {
        println!("cargo:rustc-cfg={}=\"{}\"", name, value);
    }
}
