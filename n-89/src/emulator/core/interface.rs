#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
use crate::libc::*;
extern "C" {
    #[no_mangle]
    fn exit(_: cty::c_int) -> !;
    #[no_mangle]
    fn memset(_: *mut cty::c_void, _: cty::c_int, _: cty::c_ulong)
     -> *mut cty::c_void;
    #[no_mangle]
    fn tiemu_error(format: *const cty::c_char, _: ...);
    /* Hey EMACS -*- linux-c -*- */
/* $Id: main.c 245 2004-05-23 20:45:43Z roms $ */
    /*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin
 *  Copyright (c) 2005, Christian Walther (patches for Mac OS-X port)
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street - Fifth Floor, Boston, MA 02110-1301, USA.
 */
    #[no_mangle]
    fn hw_init() -> cty::c_int;
    #[no_mangle]
    fn hw_reset() -> cty::c_int;
    #[no_mangle]
    fn hw_exit() -> cty::c_int;
    #[no_mangle]
    fn hw_m68k_get_cycle_count(_: cty::c_int) -> cty::c_uint;
    // dc = don't care for rom/tib
    // "TiEmu img v2.00" (dc)
    // structure revision (compatibility)
    // size of this structure and offset to pure data (dc)
    // calculator type
    // firmware revision
    // EPROM or FLASH
    // FLASH upgrade does not have boot
    // size of pure data
    // hw1 or hw2
    // ROM base address (MSB)
    // round up struct to 0x40 bytes
    // pure data (temporary use, 8 bytes)
    // dc = don't care for rom/tib
/* *
 * DeviceType:
 *
 * An enumeration which contains some device IDs for FLASH apps:
 **/
    /* *
 * CalcModel:
 *
 * An enumeration which contains several calculator models.
 *
 **/
    /* *
 * FlashPage:
 * @offset: FLASH offset (see TI link guide).
 * @page: FLASH page (see TI link guide).
 * @flag: see link guide.
 * @size: length of pure data (up to 16384 bytes)
 * @data: pure FLASH data.
 *
 * A generic structure used to store the content of a TI-Z80 memory page for FLASH.
 **/
    /* *
 * FlashContent:
 * @model: a calculator model.
 * @revision_major:
 * @revision_minor:
 * @flags:
 * @object_type:
 * @revision_day:
 * @revision_month:
 * @revision_year:
 * @name: name of FLASH app or OS
 * @device_type: a device ID
 * @data_type: a type ID
 * @hw_id: hardware ID (used on TI-68k only, 0 otherwise)
 * @data_length: length of pure data
 * @data_part: pure FLASH data (TI-68k, TI-eZ80) or license or certificate
 * @num_pages: number of FLASH pages (TI-Z80 only)
 * @pages: NULL-terminated array of FLASH pages (TI-Z80 only)
 * @next: pointer to next structure (linked list of contents)
 *
 * A generic structure used to store the content of a FLASH file (os or app).
 **/
    // FlashHeader	header;
    // TI-68k and TI-eZ80 FlashApps.
    // TI-Z80 only
    // TI-Z80 only
    // TI-68k only
    #[no_mangle]
    static mut img_infos: IMG_INFO32;
    #[no_mangle]
    static mut img_loaded: cty::c_int;
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __int32_t = cty::c_int;
pub type __uint32_t = cty::c_uint;
pub type __time_t = cty::c_long;
pub type time_t = __time_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TTIME {
    pub s: time_t,
    pub ms: cty::c_int,
}
/* Equivalences */
/* Constants */
// LCD _memory_ width
// LCD _memory_ height
/* Structures */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Ti68kParameters {
    pub rom_file: *mut cty::c_char,
    pub tib_file: *mut cty::c_char,
    pub sav_file: *mut cty::c_char,
    pub restricted: cty::c_int,
    pub cpu_rate: cty::c_int,
    pub hw_rate: cty::c_int,
    pub lcd_rate: cty::c_int,
    pub hw_protect: cty::c_int,
    pub recv_file: cty::c_int,
    // receive file enabled/disabled
}
// If this structure is modified, the SAV_REVISION number (state.c)
// has to be incremented.
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Ti68kHardware {
    pub calc_type: cty::c_int,
    pub ram_size: cty::c_int,
    pub rom_size: cty::c_int,
    pub io_size: cty::c_int,
    pub io2_size: cty::c_int,
    pub io3_size: cty::c_int,
    pub rom_base: uint32_t,
    pub rom_flash: cty::c_int,
    pub rom_version: [cty::c_char; 5],
    pub hw_type: cty::c_int,
    pub ti92v1: cty::c_int,
    pub ti92v2: cty::c_int,
    pub lcd_w: cty::c_int,
    pub lcd_h: cty::c_int,
    pub on_key: cty::c_int,
    pub lcd_adr: uint32_t,
    pub lcd_ptr: *mut cty::c_char,
    pub contrast: cty::c_int,
    pub log_w: cty::c_int,
    pub log_h: cty::c_int,
    pub on_off: cty::c_int,
    pub lcd_tick: cty::c_ulong,
    pub rom: *mut uint8_t,
    pub ram: *mut uint8_t,
    pub io: *mut uint8_t,
    pub io2: *mut uint8_t,
    pub io3: *mut uint8_t,
    pub unused: *mut uint8_t,
    pub initial_ssp: uint32_t,
    pub initial_pc: uint32_t,
    pub timer_value: uint8_t,
    pub timer_init: uint8_t,
    pub rtc_value: uint8_t,
    pub rtc3_ref: TTIME,
    pub rtc3_beg: TTIME,
    pub rtc3_load: TTIME,
    pub protect: cty::c_int,
    pub archive_limit: uint32_t,
    pub ram_exec: [cty::c_int; 64],
    // RAM page execution protection bitmask
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Ti68kLogging {
    pub pclog_size: cty::c_int,
    pub pclog_buf: *mut uint32_t,
    pub pclog_ptr: cty::c_int,
    pub link_size: cty::c_int,
    pub link_buf: *mut uint16_t,
    pub link_ptr: cty::c_int,
    pub link_mask: cty::c_int,
    // actions (1: S, 2: R)
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct IMG_INFO32 {
    pub signature: [cty::c_char; 16],
    pub revision: int32_t,
    pub header_size: int32_t,
    pub calc_type: cty::c_char,
    pub version: [cty::c_char; 5],
    pub flash: cty::c_char,
    pub has_boot: cty::c_char,
    pub size: int32_t,
    pub hw_type: cty::c_char,
    pub rom_base: uint8_t,
    pub fill: [cty::c_char; 22],
    pub data: *mut cty::c_char,
}
/* Hey EMACS -*- linux-c -*- */
/* $Id: interface.c 2792 2008-05-26 16:48:30Z roms $ */
/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005-2007, Romain Li�vin, Kevin Kofler
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street - Fifth Floor, Boston, MA 02110-1301, USA.
 */
/*
    Interface: exported & misc routines
*/
/* *********************/
/* Internal variables */
/* *********************/
#[no_mangle]
pub static mut params: Ti68kParameters =
    Ti68kParameters{rom_file: 0 as *const cty::c_char as *mut cty::c_char,
                    tib_file: 0 as *const cty::c_char as *mut cty::c_char,
                    sav_file: 0 as *const cty::c_char as *mut cty::c_char,
                    restricted: 0,
                    cpu_rate: 0,
                    hw_rate: 0,
                    lcd_rate: 0,
                    hw_protect: 0,
                    recv_file: 0,};
#[no_mangle]
pub static mut tihw: Ti68kHardware =
    Ti68kHardware{calc_type: 0i32,
                  ram_size: 0,
                  rom_size: 0,
                  io_size: 0,
                  io2_size: 0,
                  io3_size: 0,
                  rom_base: 0,
                  rom_flash: 0,
                  rom_version: [0; 5],
                  hw_type: 0,
                  ti92v1: 0,
                  ti92v2: 0,
                  lcd_w: 0,
                  lcd_h: 0,
                  on_key: 0,
                  lcd_adr: 0,
                  lcd_ptr: 0 as *const cty::c_char as *mut cty::c_char,
                  contrast: 0,
                  log_w: 0,
                  log_h: 0,
                  on_off: 0,
                  lcd_tick: 0,
                  rom: 0 as *const uint8_t as *mut uint8_t,
                  ram: 0 as *const uint8_t as *mut uint8_t,
                  io: 0 as *const uint8_t as *mut uint8_t,
                  io2: 0 as *const uint8_t as *mut uint8_t,
                  io3: 0 as *const uint8_t as *mut uint8_t,
                  unused: 0 as *const uint8_t as *mut uint8_t,
                  initial_ssp: 0,
                  initial_pc: 0,
                  timer_value: 0,
                  timer_init: 0,
                  rtc_value: 0,
                  rtc3_ref: TTIME{s: 0, ms: 0,},
                  rtc3_beg: TTIME{s: 0, ms: 0,},
                  rtc3_load: TTIME{s: 0, ms: 0,},
                  protect: 0,
                  archive_limit: 0,
                  ram_exec: [0; 64],};
#[no_mangle]
pub static mut logger: Ti68kLogging =
    Ti68kLogging{pclog_size: 0i32,
                 pclog_buf: 0 as *const uint32_t as *mut uint32_t,
                 pclog_ptr: 0,
                 link_size: 0,
                 link_buf: 0 as *const uint16_t as *mut uint16_t,
                 link_ptr: 0,
                 link_mask: 0,};
/* **********************************/
/* Entry points for initialization */
/* **********************************/
/*
  This should be the FIRST function to call (unless the 'params'
  structure has been properly initialized.
 */
#[no_mangle]
pub unsafe extern "C" fn ti68k_config_load_default() -> cty::c_int {
    params.restricted = 1i32;
    params.cpu_rate = -1i32;
    params.hw_rate = -1i32;
    params.lcd_rate = -1i32;
    params.hw_protect = 1i32;
    params.recv_file = 1i32;
    return 0i32;
}
/*
  This is the THIRD function to call for completely initializing the
  emulation engine.
*/
#[no_mangle]
pub unsafe extern "C" fn ti68k_init() -> cty::c_int {
    // check if image has been loaded
    if img_loaded == 0i32 { return 772i32 }
    // set calc type and init hardware
    memset(&mut tihw as *mut Ti68kHardware as *mut cty::c_void, 0i32,
           ::core::mem::size_of::<Ti68kHardware>() as cty::c_ulong);
    tihw.calc_type = img_infos.calc_type as cty::c_int;
    let mut aaa_: cty::c_int = 0;
    aaa_ = hw_init();
    if 0 != aaa_ { return aaa_ }
    return 0i32;
}
/*
  This should be the FOURTH function to call.
  It simply resets the hardware engine.
*/
#[no_mangle]
pub unsafe extern "C" fn ti68k_reset() -> cty::c_int {
    hw_reset();
    return 0i32;
}
/*
  Close the library by exiting the emulation engine
  (free ressources).
*/
#[no_mangle]
pub unsafe extern "C" fn ti68k_exit() -> cty::c_int {
    let mut aaa_: cty::c_int = 0;
    aaa_ = hw_exit();
    if 0 != aaa_ { return aaa_ }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_get_cycle_count(mut reset: cty::c_int,
                                               mut diff: *mut cty::c_uint)
 -> cty::c_uint {
    static mut old_cnt: cty::c_uint = 0i32 as cty::c_uint;
    let mut new_cnt: cty::c_uint = 0;
    new_cnt = hw_m68k_get_cycle_count(reset);
    if !diff.is_null() { *diff = new_cnt.wrapping_sub(old_cnt) }
    old_cnt = new_cnt;
    return new_cnt;
}
/* *****************/
/* Misc functions */
/* *****************/
#[no_mangle]
pub static mut ti_rom_base: [cty::c_int; 5] =
    [0i32, 0x200000i32, 0x400000i32, 0x200000i32, 0x800000i32];
#[no_mangle]
pub static mut ti_rom_sizes: [cty::c_int; 5] =
    [1i32 * (1024i32 * 1024i32), 2i32 * (1024i32 * 1024i32),
     2i32 * (1024i32 * 1024i32), 4i32 * (1024i32 * 1024i32),
     4i32 * (1024i32 * 1024i32)];
#[no_mangle]
pub static mut ti_ram_sizes: [cty::c_int; 5] =
    [256i32 * 1024i32, 256i32 * 1024i32, 256i32 * 1024i32, 256i32 * 1024i32,
     256i32 * 1024i32];
#[no_mangle]
pub static mut ti_io_sizes: [cty::c_int; 5] =
    [32i32, 32i32, 32i32, 32i32, 32i32];
#[no_mangle]
pub static mut ti_io2_sizes: [cty::c_int; 5] =
    [0i32, 32i32, 32i32, 32i32, 256i32];
#[no_mangle]
pub static mut ti_io3_sizes: [cty::c_int; 5] =
    [0i32, 0i32, 0i32, 0i32, 256i32];
unsafe extern "C" fn log_b2(mut i: cty::c_int) -> cty::c_int {
    let mut j: cty::c_int = 0;
    let mut v: cty::c_int = 0;
    j = 0i32;
    v = i;
    while v != 0i32 { v >>= 1i32; j += 1 }
    return j - 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_get_rom_size(mut calc_type: cty::c_int)
 -> cty::c_int {
    if calc_type > 1i32 << 4i32 {
        tiemu_error(b"Bad argument!\x00" as *const u8 as *const cty::c_char);
        exit(0i32);
    }
    return ti_rom_sizes[log_b2(calc_type) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_get_ram_size(mut calc_type: cty::c_int)
 -> cty::c_int {
    if calc_type > 1i32 << 4i32 {
        tiemu_error(b"Bad argument!\x00" as *const u8 as *const cty::c_char);
        exit(0i32);
    }
    return ti_ram_sizes[log_b2(calc_type) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_get_io_size(mut calc_type: cty::c_int)
 -> cty::c_int {
    if calc_type > 1i32 << 4i32 {
        tiemu_error(b"Bad argument!\x00" as *const u8 as *const cty::c_char);
        exit(0i32);
    }
    return ti_io_sizes[log_b2(calc_type) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_get_io2_size(mut calc_type: cty::c_int)
 -> cty::c_int {
    if calc_type > 1i32 << 4i32 {
        tiemu_error(b"Bad argument!\x00" as *const u8 as *const cty::c_char);
        exit(0i32);
    }
    return ti_io2_sizes[log_b2(calc_type) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_get_io3_size(mut calc_type: cty::c_int)
 -> cty::c_int {
    if calc_type > 1i32 << 4i32 {
        tiemu_error(b"Bad argument!\x00" as *const u8 as *const cty::c_char);
        exit(0i32);
    }
    return ti_io3_sizes[log_b2(calc_type) as usize];
}
/* Hey EMACS -*- linux-c -*- */
/* $Id: ti68k_int.h 2433 2007-04-13 19:03:45Z roms $ */
/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin
 *  Copyright (c) 2006, Kevin Kofler
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street - Fifth Floor, Boston, MA 02110-1301, USA.
 */
/* ************/
/* Functions */
/* ************/
// Note: [ti68k]_[group]_[short_or_long_name]
// Initialization
// Link
// Keyboard
// Misc
// HW protection for the debugger
/* *******/
/* Misc */
/* *******/
#[no_mangle]
pub unsafe extern "C" fn ti68k_unprotect_64KB_range(mut addr: uint32_t) {
    let mut blockid: cty::c_uint = addr >> 12i32;
    let mut i: cty::c_uint = 0;
    i = blockid;
    while i <= blockid.wrapping_add(16i32 as cty::c_uint) &&
              i < 64i32 as cty::c_uint {
        tihw.ram_exec[i as usize] = 0i32;
        i = i.wrapping_add(1)
    };
}
