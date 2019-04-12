use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    fs::remove_dir_all(&out_dir)?;
    fs::create_dir_all(&out_dir)?;

    let build_dir = out_dir.join("build");
    fs::create_dir_all(build_dir)?;

    let target = env::var("TARGET")?;
    match target.as_ref() {
        "thumbv7em-none-eabi" => {
            env::set_var("AR", "arm-none-eabi-ar");
            env::set_var("AS", "arm-none-eabi-as");
            env::set_var("CC", "arm-none-eabi-gcc");
            env::set_var("CXX", "arm-none-eabi-g++");
            env::set_var("LD", "arm-none-eabi-ld");
            env::set_var("RANLIB", "arm-none-eabi-ranlib");
            env::set_var("STRIP", "arm-none-eabi-strip");

            let flags = "-mcpu=cortex-m4 -mthumb -mfloat-abi=soft";
            env::set_var("CFLAGS", flags.clone());
            env::set_var("CXXFLAGS", flags.clone());
        }
        "thumbv7em-none-eabihf" => {
            env::set_var("AR", "arm-none-eabi-ar");
            env::set_var("AS", "arm-none-eabi-as");
            env::set_var("CC", "arm-none-eabi-gcc");
            env::set_var("CXX", "arm-none-eabi-g++");
            env::set_var("LD", "arm-none-eabi-ld");
            env::set_var("RANLIB", "arm-none-eabi-ranlib");
            env::set_var("STRIP", "arm-none-eabi-strip");

            let flags = "-mcpu=cortex-m4 -mthumb -mfloat-abi=hard";
            env::set_var("CFLAGS", flags.clone());
            env::set_var("CXXFLAGS", flags.clone());
        }
        _ => {}
    }

    let cargo_manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let _ = Command::new("make")
        .current_dir(&cargo_manifest_dir)
        .arg("-j4")
        .status()?;

    let lvgl_dir = cargo_manifest_dir.join("lvgl");
    env::set_current_dir(&lvgl_dir)?; 

    let mut builder = bindgen::Builder::default();
    builder = builder.header("lvgl.h");
    builder = builder.use_core();
    builder = builder.ctypes_prefix("ffi");
    if target == "thumbv7em-none-eabi" || target == "thumbv7em-none-eabihf" {
        builder = builder.clang_arg("--target=thumbv7em-none-eabihf");
        builder = builder.clang_arg("--sysroot=/usr/local/opt/arm-none-eabi-gcc/gcc/arm-none-eabi");
    }

    let bindings = builder.generate().expect("Couldn't write bindings!");
    bindings.write_to_file(out_dir.join("bindings.rs"))?;

    println!("cargo:rustc-link-lib=static=lvgl");
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("lib").display()
    );

    // Make sure these options are synchronized with lv_conf.h.
    println!("cargo:lv_color_depth=16");
    println!("cargo:use_lv_gauge=enabled");
    println!("cargo:use_lv_img=enabled");
    println!("cargo:use_lv_label=enabled");
    println!("cargo:use_lv_lmeter=enabled");

    Ok(())
}
