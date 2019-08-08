#!/usr/bin/env bash
read -r -d '' PREFIX <<"EOF"
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
EOF

bindgen include.h -o src/ffi.rs \
	--raw-line "$PREFIX" \
	--ctypes-prefix=cty \
	--use-core \
	--no-layout-tests \
	--whitelist-function "(ti68k_config_load_default|ti68k_load_image|ti68k_init|ti68k_reset|ti68k_exit|hw_m68k_run|ti68k_kbd_set_key|IMG_INFO32)" \
	--whitelist-type "(TTIME|Ti68kHardware|TiKey|DeviceType)" \
	--whitelist-var "(tihw|img_loaded|img_changed|img_infos)" \
	--rustified-enum "TiKey" \
	--rust-target nightly
