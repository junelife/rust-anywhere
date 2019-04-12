ifeq ($(origin MAKEFILE_DIR), undefined)
	MAKEFILE_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
endif

# Find Cargo's output directory.
ifeq ($(origin OUT_DIR), undefined)
	OUT_DIR := $(MAKEFILE_DIR)/build
endif

# Extra include directory, required by lv_examples.
INCLUDEDIR := -I$(MAKEFILE_DIR)

# Where compiled objects are stored.
OBJDIR := $(OUT_DIR)/obj/
LIBDIR := $(OUT_DIR)/lib/

# This library is the main target for this Makefile.
LIB_NAME := liblvgl.a
LIB_PATH := $(LIBDIR)$(LIB_NAME)

# Let included makefiles find the lvgl directories.
LVGL_DIR := $(MAKEFILE_DIR)

# Library sources
include ./lvgl/lv_core/lv_core.mk
include ./lvgl/lv_hal/lv_hal.mk
include ./lvgl/lv_objx/lv_objx.mk
include ./lvgl/lv_fonts/lv_fonts.mk
include ./lvgl/lv_misc/lv_misc.mk
include ./lvgl/lv_themes/lv_themes.mk
include ./lvgl/lv_draw/lv_draw.mk

ALL_SRCS := $(CSRCS)

LIB_OBJS := $(addprefix $(OBJDIR), \
$(patsubst %.cc,%.o,$(patsubst %.c,%.o,$(ALL_SRCS))))

$(OBJDIR)%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) $(INCLUDEDIR) -c $< -o $@

# The target that's compiled if there's no command-line arguments.
all: $(LIB_PATH)

# Gathers together all the objects we've compiled into a single '.a' archive.
$(LIB_PATH): $(LIB_OBJS)
	@mkdir -p $(dir $@)
	$(AR) $(ARFLAGS) $(LIB_PATH) $(LIB_OBJS)
