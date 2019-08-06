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
    fn malloc(_: cty::c_ulong) -> *mut cty::c_void;
    #[no_mangle]
    fn free(__ptr: *mut cty::c_void);
    #[no_mangle]
    fn memset(_: *mut cty::c_void, _: cty::c_int, _: cty::c_ulong)
     -> *mut cty::c_void;
    #[no_mangle]
    fn memcpy(_: *mut cty::c_void, _: *const cty::c_void, _: cty::c_ulong)
     -> *mut cty::c_void;
    /* Externs */
    #[no_mangle]
    static mut params: Ti68kParameters;
    #[no_mangle]
    static mut tihw: Ti68kHardware;
    #[no_mangle]
    static mut img_infos: IMG_INFO32;
    // Misc
    #[no_mangle]
    fn ti68k_get_rom_size(calc_type: cty::c_int) -> cty::c_int;
    #[no_mangle]
    fn ti68k_get_ram_size(calc_type: cty::c_int) -> cty::c_int;
    #[no_mangle]
    fn ti68k_get_io_size(calc_type: cty::c_int) -> cty::c_int;
    #[no_mangle]
    fn ti68k_get_io2_size(calc_type: cty::c_int) -> cty::c_int;
    #[no_mangle]
    fn ti68k_get_io3_size(calc_type: cty::c_int) -> cty::c_int;
    #[no_mangle]
    static mut regs: regstruct;
    #[no_mangle]
    fn hwp_get_byte(addr: uint32_t) -> uint8_t;
    #[no_mangle]
    fn hwp_get_word(addr: uint32_t) -> uint16_t;
    #[no_mangle]
    fn hwp_get_long(addr: uint32_t) -> uint32_t;
    #[no_mangle]
    fn hwp_put_long(addr: uint32_t, arg: uint32_t);
    #[no_mangle]
    fn hwp_put_word(addr: uint32_t, arg: uint16_t);
    #[no_mangle]
    fn hwp_put_byte(addr: uint32_t, arg: uint8_t);
    #[no_mangle]
    fn hw_m68k_irq(n: cty::c_int);
    /* Hey EMACS -*- linux-c -*- */
/* $Id: main.c 245 2004-05-23 20:45:43Z roms $ */
    /*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin
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
    /* Functions */
    #[no_mangle]
    fn ti89_mem_init() -> cty::c_int;
    /* Hey EMACS -*- linux-c -*- */
/* $Id: main.c 245 2004-05-23 20:45:43Z roms $ */
    /*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin
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
    /* Functions */
    #[no_mangle]
    fn ti89t_mem_init() -> cty::c_int;
    /* Hey EMACS -*- linux-c -*- */
/* $Id: main.c 245 2004-05-23 20:45:43Z roms $ */
    /*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin
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
    /* Functions */
    #[no_mangle]
    fn ti92_mem_init() -> cty::c_int;
    /* Hey EMACS -*- linux-c -*- */
/* $Id: main.c 245 2004-05-23 20:45:43Z roms $ */
    /*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin
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
    /* Functions */
    #[no_mangle]
    fn ti92p_mem_init() -> cty::c_int;
    /* Hey EMACS -*- linux-c -*- */
/* $Id: main.c 245 2004-05-23 20:45:43Z roms $ */
    /*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin
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
    /* Functions */
    #[no_mangle]
    fn v200_mem_init() -> cty::c_int;
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
pub type uintptr_t = cty::c_ulong;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TTIME {
    pub s: time_t,
    pub ms: cty::c_int,
}
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
}
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
pub type GETBYTE_FUNC = Option<unsafe extern "C" fn(_: uint32_t) -> uint8_t>;
pub type GETWORD_FUNC = Option<unsafe extern "C" fn(_: uint32_t) -> uint16_t>;
pub type GETLONG_FUNC = Option<unsafe extern "C" fn(_: uint32_t) -> uint32_t>;
pub type PUTBYTE_FUNC
    =
    Option<unsafe extern "C" fn(_: uint32_t, _: uint8_t) -> ()>;
pub type PUTWORD_FUNC
    =
    Option<unsafe extern "C" fn(_: uint32_t, _: uint16_t) -> ()>;
pub type PUTLONG_FUNC
    =
    Option<unsafe extern "C" fn(_: uint32_t, _: uint32_t) -> ()>;
pub type REALADR_FUNC
    =
    Option<unsafe extern "C" fn(_: uint32_t) -> *mut uint8_t>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct regstruct {
    pub regs: [uint32_t; 16],
    pub usp: uintptr_t,
    pub isp: uintptr_t,
    pub sr: uint16_t,
    pub t1: flagtype,
    pub s: flagtype,
    pub x: flagtype,
    pub stopped: flagtype,
    pub flags: flag_struct,
    pub intmask: cty::c_int,
    pub pc: uint32_t,
    pub pc_p: *mut uint8_t,
    pub pc_oldp: *mut uint8_t,
    pub vbr: uint32_t,
    pub sfc: uint32_t,
    pub dfc: uint32_t,
    pub fp: [fptype; 8],
    pub fp_result: fptype,
    pub fpcr: uint32_t,
    pub fpsr: uint32_t,
    pub fpiar: uint32_t,
    pub fpsr_highbyte: uint32_t,
    pub spcflags: uint32_t,
    pub kick_mask: uint32_t,
    pub address_space_mask: uint32_t,
    pub irc: uint16_t,
    pub ir: uint16_t,
    pub panic: uint8_t,
    pub panic_pc: uint32_t,
    pub panic_addr: uint32_t,
    pub insn_end: *mut cty::c_uchar,
    pub prevlock: cty::c_int,
    pub thislock: cty::c_int,
    pub exception: cty::c_int,
    pub end_of_registers: cty::c_int,
    pub msize: cty::c_int,
    pub profile: cty::c_int,
    pub profile_hist: *mut cty::c_ushort,
    pub memory: *mut cty::c_uchar,
    pub xyram_select: cty::c_int,
    pub xram_start: cty::c_int,
    pub yram_start: cty::c_int,
    pub xmem: *mut cty::c_uchar,
    pub ymem: *mut cty::c_uchar,
    pub xmem_offset: *mut cty::c_uchar,
    pub ymem_offset: *mut cty::c_uchar,
}
pub type fptype = cty::c_double;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct flag_struct {
    pub c: cty::c_uint,
    pub z: cty::c_uint,
    pub n: cty::c_uint,
    pub v: cty::c_uint,
    pub x: cty::c_uint,
}
pub type flagtype = cty::c_char;
/* Hey EMACS -*- linux-c -*- */
/* $Id: mem.c 2601 2007-07-14 08:49:30Z roms $ */
/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin, Kevin Kofler
 *  Copyright (c) 2007, Romain Li�vin
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
    Memory management: RAM, PROM/FLASH, I/O ports and bkpts
*/
static mut img: *mut IMG_INFO32 =
    unsafe { &img_infos as *const IMG_INFO32 as *mut IMG_INFO32 };
// 000000-0fffff : RAM (128 or 256 KB)
// 100000-1fffff :
// 200000-2fffff : internal ROM (TI92, TI89, V200) or unused
// 300000-3fffff : idem
// 400000-4fffff : external ROM (TI92, TI92-II, TI92+) or unused
// 500000-5fffff : idem
// 600000-6fffff : memory mapped I/O (all HW)
// 700000-7fffff : memory mapped I/O (HW2, HW3)
// 800000-8fffff : ROM (TI89 Titanium) or unused
// 900000-9fffff : idem
// a00000-afffff : idem
// b00000-bfffff : idem
// c00000-cfffff : unused
// d00000-dfffff :	 ...
// e00000-efffff :   ...
// d00000-ffffff : unused
static mut get_byte_ptr: GETBYTE_FUNC = None;
// set on memXX.c or hwprot.c
static mut get_word_ptr: GETWORD_FUNC = None;
static mut get_long_ptr: GETLONG_FUNC = None;
static mut put_byte_ptr: PUTBYTE_FUNC = None;
static mut put_word_ptr: PUTWORD_FUNC = None;
static mut put_long_ptr: PUTLONG_FUNC = None;
#[no_mangle]
pub static mut mem_get_byte_ptr: GETBYTE_FUNC = None;
// set by memXX.c:tiXX_mem_init
#[no_mangle]
pub static mut mem_get_word_ptr: GETWORD_FUNC = None;
#[no_mangle]
pub static mut mem_get_long_ptr: GETLONG_FUNC = None;
#[no_mangle]
pub static mut mem_put_byte_ptr: PUTBYTE_FUNC = None;
#[no_mangle]
pub static mut mem_put_word_ptr: PUTWORD_FUNC = None;
#[no_mangle]
pub static mut mem_put_long_ptr: PUTLONG_FUNC = None;
#[no_mangle]
pub static mut mem_get_real_addr_ptr: REALADR_FUNC = None;
/* Mem init/exit */
#[no_mangle]
pub unsafe extern "C" fn hw_mem_init() -> cty::c_int {
    // get memory sizes
    if 0 != tihw.ti92v2 {
        // TI92 II is same as TI92+ in memory size
        tihw.rom_size = ti68k_get_rom_size(1i32 << 2i32);
        tihw.ram_size = ti68k_get_ram_size(1i32 << 2i32);
        tihw.io_size = ti68k_get_io_size(1i32 << 2i32)
    } else {
        tihw.rom_size = ti68k_get_rom_size(tihw.calc_type);
        tihw.ram_size = ti68k_get_ram_size(tihw.calc_type);
        tihw.io_size = ti68k_get_io_size(tihw.calc_type);
        tihw.io2_size = ti68k_get_io2_size(tihw.calc_type);
        tihw.io3_size = ti68k_get_io3_size(tihw.calc_type)
    }
    // allocate mem
    tihw.ram = malloc(tihw.ram_size as cty::c_ulong) as *mut uint8_t;
    tihw.rom = malloc(tihw.rom_size as cty::c_ulong) as *mut uint8_t;
    tihw.io = malloc(tihw.io_size as cty::c_ulong) as *mut uint8_t;
    tihw.io2 = malloc(tihw.io2_size as cty::c_ulong) as *mut uint8_t;
    tihw.io3 = malloc(tihw.io3_size as cty::c_ulong) as *mut uint8_t;
    tihw.unused = malloc(16i32 as cty::c_ulong) as *mut uint8_t;
    // clear RAM/ROM/IO
    memset(tihw.ram as *mut cty::c_void, 0i32,
           tihw.ram_size as cty::c_ulong);
    memset(tihw.io as *mut cty::c_void, 0i32, tihw.io_size as cty::c_ulong);
    memset(tihw.io2 as *mut cty::c_void, 0i32,
           tihw.io2_size as cty::c_ulong);
    memset(tihw.io2 as *mut cty::c_void, 0i32,
           tihw.io3_size as cty::c_ulong);
    memset(tihw.rom as *mut cty::c_void, 0xffi32,
           tihw.rom_size as cty::c_ulong);
    memset(tihw.unused as *mut cty::c_void, 0x14i32, 16i32 as cty::c_ulong);
    // set banks and mappers on per calc basis
    match tihw.calc_type {
        1 => { ti92_mem_init(); }
        4 => { ti92p_mem_init(); }
        2 => { ti89_mem_init(); }
        8 => { v200_mem_init(); }
        16 => { ti89t_mem_init(); }
        _ => { }
    }
    // blit ROM
    memcpy(tihw.rom as *mut cty::c_void, (*img).data as *const cty::c_void,
           (*img).size as cty::c_ulong);
    free((*img).data as *mut cty::c_void);
    if tihw.ram.is_null() || tihw.rom.is_null() || tihw.io.is_null() ||
           tihw.io2.is_null() {
        return -1i32
    }
    // set memory mappers for hw protection
    if 0 != params.hw_protect && tihw.calc_type != 1i32 << 0i32 {
        get_byte_ptr =
            Some(hwp_get_byte as
                     unsafe extern "C" fn(_: uint32_t) -> uint8_t);
        get_word_ptr =
            Some(hwp_get_word as
                     unsafe extern "C" fn(_: uint32_t) -> uint16_t);
        get_long_ptr =
            Some(hwp_get_long as
                     unsafe extern "C" fn(_: uint32_t) -> uint32_t);
        put_byte_ptr =
            Some(hwp_put_byte as
                     unsafe extern "C" fn(_: uint32_t, _: uint8_t) -> ());
        put_word_ptr =
            Some(hwp_put_word as
                     unsafe extern "C" fn(_: uint32_t, _: uint16_t) -> ());
        put_long_ptr =
            Some(hwp_put_long as
                     unsafe extern "C" fn(_: uint32_t, _: uint32_t) -> ())
    } else {
        get_byte_ptr = mem_get_byte_ptr;
        get_word_ptr = mem_get_word_ptr;
        get_long_ptr = mem_get_long_ptr;
        put_byte_ptr = mem_put_byte_ptr;
        put_word_ptr = mem_put_word_ptr;
        put_long_ptr = mem_put_long_ptr
    }
    return 0i32;
}
/* Hey EMACS -*- linux-c -*- */
/* $Id: main.c 245 2004-05-23 20:45:43Z roms $ */
/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin, Kevin Kofler
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
/* Typedefs */
/* Functions */
#[no_mangle]
pub unsafe extern "C" fn hw_mem_reset() -> cty::c_int { return 0i32; }
#[no_mangle]
pub unsafe extern "C" fn hw_mem_exit() -> cty::c_int {
    // free memory
    if !tihw.ram.is_null() { free(tihw.ram as *mut cty::c_void); }
    tihw.ram = 0 as *mut uint8_t;
    if !tihw.rom.is_null() { free(tihw.rom as *mut cty::c_void); }
    tihw.rom = 0 as *mut uint8_t;
    if !tihw.io.is_null() { free(tihw.io as *mut cty::c_void); }
    tihw.io = 0 as *mut uint8_t;
    if !tihw.io2.is_null() { free(tihw.io2 as *mut cty::c_void); }
    tihw.io2 = 0 as *mut uint8_t;
    if !tihw.io3.is_null() { free(tihw.io3 as *mut cty::c_void); }
    tihw.io3 = 0 as *mut uint8_t;
    // clear breakpoints
    ti68k_bkpt_clear_access();
    ti68k_bkpt_clear_range();
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn hw_get_real_address(mut adr: uint32_t)
 -> *mut uint8_t {
    return mem_get_real_addr_ptr.expect("non-null function pointer")(adr);
}
#[no_mangle]
pub unsafe extern "C" fn hw_get_long(mut adr: uint32_t) -> uint32_t {
    adr &= 0xffffffi32 as cty::c_uint;
    // Odd address: exception !
    if 0 != adr & 1i32 as cty::c_uint {
        regs.spcflags |= 256i32 as cty::c_uint;
        return 0i32 as uint32_t
    }
    return get_long_ptr.expect("non-null function pointer")(adr);
}
#[no_mangle]
pub unsafe extern "C" fn hw_get_word(mut adr: uint32_t) -> uint16_t {
    adr &= 0xffffffi32 as cty::c_uint;
    // Odd address: exception !
    if 0 != adr & 1i32 as cty::c_uint {
        regs.spcflags |= 256i32 as cty::c_uint;
        return 0i32 as uint16_t
    }
    return get_word_ptr.expect("non-null function pointer")(adr);
}
#[no_mangle]
pub unsafe extern "C" fn hw_get_byte(mut adr: uint32_t) -> uint8_t {
    adr &= 0xffffffi32 as cty::c_uint;
    return get_byte_ptr.expect("non-null function pointer")(adr);
}
// defs similar to UAE's memory.h (interface)
#[no_mangle]
pub unsafe extern "C" fn hw_get_byte_noexcept(mut adr: uint32_t) -> uint8_t {
    adr &= 0xffffffi32 as cty::c_uint;
    return get_byte_ptr.expect("non-null function pointer")(adr);
}
#[no_mangle]
pub unsafe extern "C" fn hw_put_long(mut adr: uint32_t, mut arg: uint32_t) {
    adr &= 0xffffffi32 as cty::c_uint;
    // Odd address: exception !
    if 0 != adr & 1i32 as cty::c_uint {
        regs.spcflags |= 256i32 as cty::c_uint;
        return
    }
    // Protected memory violation. Triggered when memory below [$000120] is
	// written while bit 2 of [$600001] is set
    if adr < 0x120i32 as cty::c_uint &&
           0 != *tihw.io.offset(0x1i32 as isize) as cty::c_int & 1i32 << 2i32
       {
        hw_m68k_irq(7i32);
    } else { put_long_ptr.expect("non-null function pointer")(adr, arg); };
}
#[no_mangle]
pub unsafe extern "C" fn hw_put_word(mut adr: uint32_t, mut arg: uint16_t) {
    adr &= 0xffffffi32 as cty::c_uint;
    // Odd address: exception !
    if 0 != adr & 1i32 as cty::c_uint {
        regs.spcflags |= 256i32 as cty::c_uint;
        return
    }
    // Protected memory violation. Triggered when memory below [$000120] is
	// written while bit 2 of [$600001] is set
    if adr < 0x120i32 as cty::c_uint &&
           0 != *tihw.io.offset(0x1i32 as isize) as cty::c_int & 1i32 << 2i32
       {
        hw_m68k_irq(7i32);
    } else { put_word_ptr.expect("non-null function pointer")(adr, arg); };
}
#[no_mangle]
pub unsafe extern "C" fn hw_put_byte(mut adr: uint32_t, mut arg: uint8_t) {
    adr &= 0xffffffi32 as cty::c_uint;
    // Protected memory violation. Triggered when memory below [$000120] is
	// written while bit 2 of [$600001] is set
    if adr < 0x120i32 as cty::c_uint &&
           0 != *tihw.io.offset(0x1i32 as isize) as cty::c_int & 1i32 << 2i32
       {
        hw_m68k_irq(7i32);
    } else { put_byte_ptr.expect("non-null function pointer")(adr, arg); };
}
#[no_mangle]
pub unsafe extern "C" fn hw_put_byte_noexcept(mut adr: uint32_t,
                                              mut arg: uint8_t) {
    adr &= 0xffffffi32 as cty::c_uint;
    put_byte_ptr.expect("non-null function pointer")(adr, arg);
}
