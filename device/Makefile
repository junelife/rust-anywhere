MAKEFILE_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

CARGO := cargo

AS := arm-none-eabi-as
CC := arm-none-eabi-gcc
ASFLAGS := -mcpu=cortex-m4 -mthumb -mfloat-abi=hard -mfpu=fpv4-sp-d16 -g
CFLAGS := -mcpu=cortex-m4 -mthumb -mfloat-abi=hard -mfpu=fpv4-sp-d16 -g -Os
LDFLAGS := -flto -mcpu=cortex-m4 -mthumb -mfloat-abi=hard -mfpu=fpv4-sp-d16

# Build directory for artifacts.
BUILD_DIR := $(MAKEFILE_DIR)/build

# Where compiled objects are stored.
BINDIR := $(BUILD_DIR)/bin
OBJDIR := $(BUILD_DIR)/obj

# The Rust static library.
RUST_DIR := $(MAKEFILE_DIR)/../crates/target/thumbv7em-none-eabihf/release
RUST_PATH := $(RUST_DIR)/libdevice.a

# The binary is the main target for this Makefile.
BIN_NAME := rust-anywhere.elf
BIN_PATH := $(BINDIR)/$(BIN_NAME)

# Configure device HAL library.
DEFINES := -DSTM32F429ZITx -DSTM32F429I_DISCO -DSTM32F4 -DSTM32 -DUSE_HAL_DRIVER -DSTM32F429xx

# Include directories.
INCLUDES := \
	-I$(MAKEFILE_DIR) \
	-I$(MAKEFILE_DIR)/include \
	-I$(MAKEFILE_DIR)/CMSIS/core \
	-I$(MAKEFILE_DIR)/CMSIS/device \
	-I$(MAKEFILE_DIR)/HAL_Driver/Inc \
	-I$(MAKEFILE_DIR)/../crates/api/include \
	-I$(MAKEFILE_DIR)/../crates/lvgl/lvgl-sys

# External libraries.
LIBS := \
	-L$(RUST_DIR) \
	-llibdevice

# Linker script.
LINKER_SCRIPT := LinkerScript.ld

# C Sources.
CSRCS := \
	src/main.c \
	src/stm32f4xx_it.c \
	src/syscalls.c \
	src/system_stm32f4xx.c \
	HAL_Driver/Src/stm32f4xx_hal.c \
	HAL_Driver/Src/stm32f4xx_hal_cortex.c \
	HAL_Driver/Src/stm32f4xx_hal_dma.c \
	HAL_Driver/Src/stm32f4xx_hal_gpio.c \
	HAL_Driver/Src/stm32f4xx_hal_i2c.c \
	HAL_Driver/Src/stm32f4xx_hal_ltdc.c \
	HAL_Driver/Src/stm32f4xx_hal_pwr_ex.c \
	HAL_Driver/Src/stm32f4xx_hal_rcc.c \
	HAL_Driver/Src/stm32f4xx_hal_rcc_ex.c \
	HAL_Driver/Src/stm32f4xx_hal_sdram.c \
	HAL_Driver/Src/stm32f4xx_hal_spi.c \
	HAL_Driver/Src/stm32f4xx_hal_tim.c \
	HAL_Driver/Src/stm32f4xx_hal_tim_ex.c \
	HAL_Driver/Src/stm32f4xx_ll_fmc.c \
	hal_stm_lvgl/stm32f429i_discovery.c \
	hal_stm_lvgl/tft/ili9341.c \
	hal_stm_lvgl/tft/tft.c \
	hal_stm_lvgl/touchpad/stmpe811.c \
	hal_stm_lvgl/touchpad/touchpad.c \

# Assembler sources.
ASM_SRCS := startup/startup_stm32f429xx.s

ALL_SRCS := $(CSRCS) $(ASM_SRCS)

BIN_OBJS := $(addprefix $(OBJDIR), \
$(patsubst %.cc,%.o,$(patsubst %.c,%.o,$(patsubst %.s,%.o, $(ALL_SRCS)))))

$(OBJDIR)%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) $(DEFINES) $(INCLUDES) -c $< -o $@

$(OBJDIR)%.o: %.s
	@mkdir -p $(dir $@)
	$(AS) $(ASFLAGS) -c $< -o $@

$(BIN_PATH): $(BIN_OBJS) $(RUST_PATH) $(LINKER_SCRIPT)
	@mkdir -p $(dir $@)
	$(CC) $(LDFLAGS) -T$(LINKER_SCRIPT) -o $@ $(BIN_OBJS) $(LIBS)

# The Rust static library.
$(RUST_PATH):
	cd $(MAKEFILE_DIR)/../crates/libdevice && $(CARGO) build --release --target=thumbv7em-none-eabihf

# The target that's compiled if there's no command line arguments.
all: $(BIN_PATH)

# Other targets.
clean:
	cd $(MAKEFILE_DIR)/../crates/libdevice && $(CARGO) clean
	rm -rf $(BUILD_DIR)

.PHONY: all clean
