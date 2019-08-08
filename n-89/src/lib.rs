/*
 * lib.rs: provides a Rust interface to the C library
 * Copyright (C) 2019 Ben Schattinger
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111 USA
 */

#![no_std]
extern crate alloc;

use ndless::path::Path;

use crate::error::Result;
use crate::ffi::{ti68k_kbd_set_key, ti68k_reset, tihw, TiKey};
use ndless::fs::File;

pub mod convert;
pub mod error;
pub mod ffi;

pub fn load_default_configuration() {
	unsafe {
		ffi::ti68k_config_load_default();
	}
}

pub fn load_image(path: impl AsRef<Path>) -> Result<()> {
	let path = path.as_ref();
	convert::load_upgrade(&mut File::open(path)?)
}

pub fn init() -> Result<()> {
	match unsafe { ffi::ti68k_init() } {
		0 => Ok(()),
		error => Err(error.into()),
	}
}

pub fn reset() {
	unsafe {
		ti68k_reset();
	}
}

pub fn run_cpu(min: u32, max: u32) -> Result<()> {
	match unsafe { ffi::hw_m68k_run(min as i32, max) } {
		0 => Ok(()),
		error => Err(error.into()),
	}
}

pub fn exit() -> Result<()> {
	match unsafe { ffi::ti68k_exit() } {
		0 => Ok(()),
		error => Err(error.into()),
	}
}

pub fn is_screen_on() -> bool {
	unsafe { tihw.on_off > 0 }
}

pub fn get_pixel(x: usize, y: usize) -> bool {
	assert!(x < 240);
	assert!(y < 128);
	let lcd_offset = ((y * 240 + x) / 8) as isize;
	let byte = unsafe { tihw.ram.offset(tihw.lcd_adr as isize + lcd_offset).read() };
	(byte << (x as u8 % 8) & 0x80) > 0
}

pub fn set_key(key: TiKey, pressed: bool) {
	unsafe { ti68k_kbd_set_key(key as i32, if pressed { 1 } else { 0 }) }
}
