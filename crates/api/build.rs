use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<Error>> {
    let target = env::var("TARGET")?;

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    fs::remove_dir_all(&out_dir)?;
    fs::create_dir_all(&out_dir)?;

    let build_dir = out_dir.join("build");
    fs::create_dir_all(build_dir)?;

    let mut builder = bindgen::Builder::default();
    builder = builder.header("include/rust-anywhere.h");
    builder = builder.use_core();
    builder = builder.ctypes_prefix("ffi");
    if target == "thumbv7em-none-eabi" || target == "thumbv7em-none-eabihf" {
        builder = builder.clang_arg("--target=thumbv7em-none-eabihf");
        builder = builder.clang_arg("--sysroot=/usr/local/opt/arm-none-eabi-gcc/gcc/arm-none-eabi");
    }

    let bindings = builder.generate().expect("Couldn't create bindings!");
    bindings.write_to_file(out_dir.join("bindings.rs"))?;

    Ok(())
}
