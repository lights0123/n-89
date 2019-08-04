#![no_std]

use cstr_core::CString;
use ndless::ffi::OsStrExt;
use ndless::path::Path;

use crate::error::Result;
use crate::ffi::{ti68k_kbd_set_key, ti68k_load_image, ti68k_reset, tihw, TiKey};

pub mod error;
pub mod ffi;

pub fn load_default_configuration() {
    unsafe {
        ffi::ti68k_config_load_default();
    }
}

pub fn load_image(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    let path = CString::new(path.as_os_str().as_bytes())?;
    match unsafe { ti68k_load_image(path.as_ptr()) } {
        0 => Ok(()),
        error => Err(error.into()),
    }
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

pub fn run(min: u32, max: u32) -> Result<()> {
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
