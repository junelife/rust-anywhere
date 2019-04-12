# Embedded Rust Anywhere

## Getting Started

** Always make sure your submodules are up to date:
```
git submodule update --init --recursive
```

1) Install the latest version of the Xcode development environment from the [macOS App Store](https://itunes.apple.com/us/app/xcode/id497799835?mt=12).

2) Install Rust according to the [official instructions](https://www.rust-lang.org/tools/install).
```
curl https://sh.rustup.rs -sSf | sh
```

3) Add Rust to your `PATH` environment variable.
```
source $HOME/.cargo/env
```

4) Install the toolchain for the embedded target:
```
rustup target install thumbv7em-none-eabihf
```

5) Install ARM cross-compiler toolchain
```
brew tap ArmMbed/homebrew-formulae
brew install arm-none-eabi-gcc
```

6) Install the ST-LINK utilities
```
brew install stlink
```

## Simulator

1) Open the Xcode project `simulator/RustAnywhere.xcodeproj`
2) Choose the menu item Product -> Run

### Debugging

Install the Rust plugin for Xcode to easily enable breakpoints in Rust files: https://github.com/mtak-/rust-xcode-plugin

## Device

### Build
From the `device`  subdirectory, run `make`.

### Deploy & Debug
1) In another terminal window, run `st-util`.  You must leave this running to deploy and debug your code using `gdb`
2) Run GDB with the ELF binary:
```
arm-none-eabi-gdb build/bin/rust-anywhere.elf
```
3) From the GDB prompt, connect to the remote target
```
tar ext :4242
```
4) Load the binary and start execution:
```
load
continue
```
