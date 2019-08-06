#!/usr/bin/env bash
read -r -d '' PREFIX <<"EOF"
#![allow(non_camel_case_types)]
EOF

bindgen include.h -o src/ffi.rs \
	--raw-line "$PREFIX" \
	--ctypes-prefix=cty \
	--use-core \
	--no-layout-tests \
	--whitelist-function "(ti68k_config_load_default|ti68k_load_image|ti68k_init|ti68k_reset|ti68k_exit|hw_m68k_run|ti68k_kbd_set_key)" \
	--whitelist-type "(TTIME|Ti68kHardware|TiKey)" \
	--whitelist-var "tihw" \
	--rustified-enum "TiKey" \
	--rust-target nightly
