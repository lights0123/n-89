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
    fn memset(_: *mut cty::c_void, _: cty::c_int, _: cty::c_ulong)
     -> *mut cty::c_void;
    #[no_mangle]
    fn memcpy(_: *mut cty::c_void, _: *const cty::c_void, _: cty::c_ulong)
     -> *mut cty::c_void;
    #[no_mangle]
    fn rtc3_init() -> cty::c_int;
    #[no_mangle]
    fn rtc3_state_save() -> cty::c_int;
    #[no_mangle]
    fn rtc3_get_time(tt: *mut TTIME);
    #[no_mangle]
    static mut logger: Ti68kLogging;
    #[no_mangle]
    static mut tihw: Ti68kHardware;
    #[no_mangle]
    static mut regs: regstruct;
    #[no_mangle]
    fn hw_kbd_read_cols() -> uint8_t;
    #[no_mangle]
    fn set_prescaler(_: cty::c_int);
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __uint32_t = cty::c_uint;
pub type __time_t = cty::c_long;
pub type time_t = __time_t;
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
pub struct Ti68kLogging {
    pub pclog_size: cty::c_int,
    pub pclog_buf: *mut uint32_t,
    pub pclog_ptr: cty::c_int,
    pub link_size: cty::c_int,
    pub link_buf: *mut uint16_t,
    pub link_ptr: cty::c_int,
    pub link_mask: cty::c_int,
}
pub type flagtype = cty::c_char;
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
#[inline(always)]
unsafe extern "C" fn m68k_setstopped(mut stop: cty::c_int) {
    regs.stopped = stop as flagtype;
    if 0 != stop &&
           regs.spcflags & 64i32 as cty::c_uint == 0i32 as cty::c_uint {
        regs.spcflags |= 4i32 as cty::c_uint
    };
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
/* Hey EMACS -*- linux-c -*- */
/* $Id: ports.c 2781 2008-05-25 12:38:25Z roms $ */
/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2002, Romain Lievin, Julien Blache
 *  Copyright (c) 2003-2004, Romain Li�vin
 *  Copyright (c) 2005-2007, Romain Li�vin, Kevin Kofler
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRAN7TY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street - Fifth Floor, Boston, MA 02110-1301, USA.
 */
/*
    TI's ASIC management: memory mapped I/O ports
*/
#[no_mangle]
pub unsafe extern "C" fn hw_io_init() -> cty::c_int {
    // clear hw registers
    memset(tihw.io as *mut cty::c_void, 0i32, tihw.io_size as cty::c_ulong);
    memset(tihw.io2 as *mut cty::c_void, 0i32,
           tihw.io2_size as cty::c_ulong);
    memset(tihw.io3 as *mut cty::c_void, 0i32,
           tihw.io3_size as cty::c_ulong);
    // set LCD base address
    if tihw.hw_type > 1i32 { tihw.lcd_adr = 0x4c00i32 as uint32_t }
    // set LCD on
    if tihw.hw_type > 1i32 {
        *tihw.io2.offset(0x1di32 as isize) = 2i32 as uint8_t
    }
    // computes reference
    rtc3_init(); // tihw.io_size-1;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn hw_io_reset() -> cty::c_int { return 0i32; }
#[no_mangle]
pub unsafe extern "C" fn hw_io_exit() -> cty::c_int { return 0i32; }
#[no_mangle]
pub unsafe extern "C" fn io_put_byte(mut addr: uint32_t, mut arg: uint8_t) {
    addr &= 31i32 as cty::c_uint;
    match addr {
        0 => {
            // rw <76...2..>
            // %5: bit 0 of contrast (TI92)
            if tihw.calc_type == 1i32 << 0i32 {
                if 0 != (arg as cty::c_int & 1i32 << 5i32) >> 5i32 {
                    tihw.contrast |= 1i32 << 0i32
                } else { tihw.contrast &= !(1i32 << 0i32) }
            }
        }
        1 => {
            // rw <.....2.0>
	           // %0 clr: interleave RAM (allows use of 256K of RAM)
            if tihw.hw_type == 1i32 {
                tihw.ram_size =
                    if 0 != arg as cty::c_int & 1i32 << 0i32 {
                        128i32 * 1024i32
                    } else { 256i32 * 1024i32 }
            }
        }
        2 => { }
        3 => { }
        5 => {
            // -w <...43210>
            // turn off OSC1 (CPU), wake on int level 6 (ON key) and int level [5..1]
            m68k_setstopped(1i32);
        }
        12 => {
            // rw <765.3210>
	           // %[3:0]: Trigger interrupt level 4 on error, activity, tx empty, rx full
	           // see hardware.c
            // %6: link disable (usually reset link port or direct access to wires)
            if 0 != arg as cty::c_int & 1i32 << 6i32 &&
                   0 != arg as cty::c_int & 1i32 << 5i32 {
                *tihw.io.offset(0xdi32 as isize) = 0x40i32 as uint8_t
            }
        }
        13 => { }
        15 => {
            // rw <76543210>
            // write a byte to the transmit buffer (1 byte buffer)
            let ref mut fresh0 = *tihw.io.offset(0xdi32 as isize);
            *fresh0 = (*fresh0 as cty::c_int & !(1i32 << 0i32)) as uint8_t;
            // STX=0 (tx reg is full)
            if !logger.link_buf.is_null() && 0 != logger.link_mask & 1i32 {
                let fresh1 = logger.link_ptr;
                logger.link_ptr = logger.link_ptr + 1;
                *logger.link_buf.offset((fresh1 % logger.link_size) as isize)
                    = (arg as cty::c_int | 1i32 << 8i32) as uint16_t
            }
        }
        16 => {
            // -w <76543210> (hw1)
            // address of LCD memory divided by 8 (msb)
            if tihw.hw_type == 1i32 {
                tihw.lcd_adr =
                    (((arg as cty::c_int) << 8i32 |
                          *tihw.io.offset(0x11i32 as isize) as cty::c_int) <<
                         3i32) as uint32_t
            }
        }
        17 => {
            // -w <76543210> (hw1)
            // address of LCD memory divided by 8 (lsb)
            if tihw.hw_type == 1i32 {
                tihw.lcd_adr =
                    (((*tihw.io.offset(0x10i32 as isize) as cty::c_int) <<
                          8i32 | arg as cty::c_int) << 3i32) as uint32_t
            }
        }
        18 => {
            // -w <76543210>
            // LCD logical width = (64-n)*2 bytes = (64-n)*16 pixels <=> n = 64-w/16
            tihw.log_w = (64i32 - arg as cty::c_int) * 16i32
        }
        19 => {
            // -w <..543210>
            // LCD logical height = (256-n) <=> n = 256-h
            tihw.log_h = 0x100i32 - arg as cty::c_int
        }
        21 => {
            // rw <7.6543210>
	           // %7 set: Master disable timer interrupts (level 1, 3 and 5)
	           // see hardware.c
            // %[5-4]: Increment rate of $600017 (prescaler)
            set_prescaler(arg as cty::c_int >> 4i32 & 3i32); // reset timer
        }
        23 => {
            // rw <76543210>
            // programmable rate generator
            tihw.timer_value = arg
        }
        24 => { }
        25 => { }
        26 => { }
        28 => {
            // -w <..5432..>
	           // %[5-2] set: LCD RS (row sync) frequency, OSC2/((16-n)*8)
	           // %1111 turns off the RS completely (used when LCD is off)
            tihw.on_off =
                if arg as cty::c_int & 0x3ci32 == 0x3ci32 {
                    0i32
                } else { 1i32 }
        }
        29 => {
            // -w <7..43210>
            // %[3-0]: contrast
            if tihw.calc_type == 1i32 << 0i32 {
                // %[3-0]: bits <4321.> of contrast
                static mut avg: cty::c_int = 0i32; // filter value
                avg = (avg + arg as cty::c_int) / 2i32;
                tihw.contrast = tihw.contrast & 1i32 | (avg & 15i32) << 1i32
            } else {
                // %[4/3-0]: LCD contrast bits 4/3-0 (bit 4/3 is msb on HW2/HW1)
                tihw.contrast =
                    arg as cty::c_int &
                        (if 0 !=
                                *tihw.io2.offset(0x1fi32 as isize) as
                                    cty::c_int & 1i32 << 0i32 {
                             0x1fi32
                         } else { 0xfi32 }); // tihw.io_size-1;
                if tihw.calc_type == 1i32 << 1i32 ||
                       tihw.calc_type == 1i32 << 4i32 {
                    if tihw.hw_type == 1i32 {
                        tihw.contrast = 31i32 - 2i32 * tihw.contrast
                    } else { tihw.contrast = 31i32 - tihw.contrast }
                }
            }
        }
        4 | 6 | 7 | 8 | 9 | 10 | 11 | 20 | 22 | 27 | 30 | 31 | _ => { }
    }
    *tihw.io.offset(addr as isize) = arg;
}
#[no_mangle]
pub unsafe extern "C" fn io_put_word(mut addr: uint32_t, mut arg: uint16_t) {
    io_put_byte(addr, ((arg as cty::c_int & 0xff00i32) >> 8i32) as uint8_t);
    io_put_byte(addr.wrapping_add(1i32 as cty::c_uint),
                (arg as cty::c_int & 0xffi32) as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn io_put_long(mut addr: uint32_t, mut arg: uint32_t) {
    io_put_word(addr, ((arg & 0xffff0000u32) >> 16i32) as uint16_t);
    io_put_word(addr.wrapping_add(2i32 as cty::c_uint),
                (arg & 0xffffi32 as cty::c_uint) as uint16_t);
}
#[no_mangle]
pub unsafe extern "C" fn io_get_byte(mut addr: uint32_t) -> uint8_t {
    let mut v: cty::c_int = 0;
    addr &= 31i32 as cty::c_uint;
    v = *tihw.io.offset(addr as isize) as cty::c_int;
    let mut current_block_28: u64;
    match addr {
        0 => {
            // rw <76...2..>
            // %0: bits <....0> of contrast
            if tihw.calc_type == 1i32 << 0i32 {
                v = (tihw.contrast & 1i32) << 5i32
            }
            // %2: Battery voltage level is *above* the trig level
            v |= 4i32;
            // %[7-6]: keep clear
            current_block_28 = 7659304154607701039;
        }
        1 => {
            // rw <.....2.0>
            current_block_28 = 7659304154607701039;
        }
        3 => {
            // -w <.654.210>
            current_block_28 = 7659304154607701039;
        }
        4 => {
            // ??
            current_block_28 = 7659304154607701039;
        }
        5 => {
            // -w <...43210>
            current_block_28 = 7659304154607701039;
        }
        6 | 7 | 8 | 9 | 10 | 11 => {
            // ??
            return 0x14i32 as uint8_t
        }
        12 => {
            // rw <765.3210>
	           // linkport status
	           // see hardware.c or dbus.c
            current_block_28 = 7659304154607701039;
        }
        13 => {
            // r- <76543210>
            // reading the DBus status register resets that register (as specified by TI)
		// but don't touch the SLE bit
            *tihw.io.offset(0xdi32 as isize) =
                (v & 0x80i32 | 0x40i32) as uint8_t;
            current_block_28 = 7659304154607701039;
        }
        15 => {
            // rw <76543210>
	                         // read one byte from receive (incoming) buffer
            let ref mut fresh2 = *tihw.io.offset(0xdi32 as isize);
            *fresh2 = (*fresh2 as cty::c_int & !(1i32 << 5i32)) as uint8_t;
            // SRX=0 (rx reg is empty)
            if !logger.link_buf.is_null() && 0 != logger.link_mask & 2i32 {
                let fresh3 = logger.link_ptr;
                logger.link_ptr = logger.link_ptr + 1;
                *logger.link_buf.offset((fresh3 % logger.link_size) as isize)
                    = (v | 2i32 << 8i32) as uint16_t
            }
            current_block_28 = 7659304154607701039;
        }
        16 => {
            // -w <76543210> (hw1)
            return 0x14i32 as uint8_t
        }
        17 => {
            // -w <76543210> (hw1)
            return 0x14i32 as uint8_t
        }
        18 => {
            // -w <76543210>
            return 0x14i32 as uint8_t
        }
        19 => {
            // -w <..543210>
            return 0x14i32 as uint8_t
        }
        20 => {
            // ??
            current_block_28 = 7659304154607701039;
        }
        21 => {
            // rw <7.6543210>
            current_block_28 = 7659304154607701039;
        }
        22 => {
            // ??
            current_block_28 = 7659304154607701039;
        }
        23 => {
            // rw <76543210>
            // Programmable rate generator
            return tihw.timer_value
        }
        24 => {
            // rw <76543210>
            current_block_28 = 7659304154607701039;
        }
        25 => {
            // rw <......10>
            current_block_28 = 7659304154607701039;
        }
        26 => {
            // rw <......10>
            // ON key status (0=down, 1=up)
            if 0 == tihw.on_key {
                v |= 1i32 << 1i32
            } else { v &= !(1i32 << 1i32) }
            current_block_28 = 7659304154607701039;
        }
        27 => {
            // r- <76543210>
	           // keyboard column status
            v = hw_kbd_read_cols() as cty::c_int;
            current_block_28 = 6769428223557027391;
        }
        28 => { current_block_28 = 6769428223557027391; }
        2 | 29 => { current_block_28 = 7659304154607701039; }
        30 => {
            // ??
            return 0x14i32 as uint8_t
        }
        31 => {
            // ??
            return 0x14i32 as uint8_t
        }
        _ => { return 0x14i32 as uint8_t }
    }
    match current_block_28 {
        6769428223557027391 =>
        // -w <..5432..>
        {
        }
        _ => { }
    }
    return v as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn io_get_word(mut addr: uint32_t) -> uint16_t {
    return ((io_get_byte(addr) as uint16_t as cty::c_int) << 8i32 |
                io_get_byte(addr.wrapping_add(1i32 as cty::c_uint)) as
                    cty::c_int) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn io_get_long(mut addr: uint32_t) -> uint32_t {
    return (io_get_word(addr) as uint32_t) << 16i32 |
               io_get_word(addr.wrapping_add(2i32 as cty::c_uint)) as
                   cty::c_uint;
}
/* * HW2 **/
#[no_mangle]
pub unsafe extern "C" fn io2_put_byte(mut addr: uint32_t, mut arg: uint8_t) {
    let mut i: cty::c_int = 0; // tihw.io2_size-1;
    addr &= 63i32 as cty::c_uint;
    let mut current_block_50: u64;
    match addr {
        0 | 8 => {
            // rw <76543210>
            if 0 != tihw.protect { return }
            i = 0i32;
            while i < 8i32 {
                // this is the fastest method (an easier method will use 64 bit integer)
                tihw.ram_exec[(8i32 + i) as usize] =
                    arg as cty::c_int & 1i32 << i;
                i += 1
            }
            current_block_50 = 17075014677070940716;
        }
        1 | 9 => {
            // rw <76543210>
            if 0 != tihw.protect { return }
            i = 0i32;
            while i < 8i32 {
                tihw.ram_exec[(0i32 + i) as usize] =
                    arg as cty::c_int & 1i32 << i;
                i += 1
            }
            current_block_50 = 17075014677070940716;
        }
        2 | 10 => {
            // rw <76543210>
            if 0 != tihw.protect { return }
            i = 0i32;
            while i < 8i32 {
                tihw.ram_exec[(24i32 + i) as usize] =
                    arg as cty::c_int & 1i32 << i;
                i += 1
            }
            current_block_50 = 17075014677070940716;
        }
        3 | 11 => {
            // rw <76543210>
            if 0 != tihw.protect { return }
            i = 0i32;
            while i < 8i32 {
                tihw.ram_exec[(16i32 + i) as usize] =
                    arg as cty::c_int & 1i32 << i;
                i += 1
            }
            current_block_50 = 17075014677070940716;
        }
        4 | 12 => {
            // rw <76543210>
            if 0 != tihw.protect { return }
            i = 0i32;
            while i < 8i32 {
                tihw.ram_exec[(40i32 + i) as usize] =
                    arg as cty::c_int & 1i32 << i;
                i += 1
            }
            current_block_50 = 17075014677070940716;
        }
        5 | 13 => {
            // rw <76543210>
            if 0 != tihw.protect { return }
            i = 0i32;
            while i < 8i32 {
                tihw.ram_exec[(32i32 + i) as usize] =
                    arg as cty::c_int & 1i32 << i;
                i += 1
            }
            current_block_50 = 17075014677070940716;
        }
        6 | 14 => {
            // rw <76543210>
            if 0 != tihw.protect { return }
            i = 0i32;
            while i < 8i32 {
                tihw.ram_exec[(56i32 + i) as usize] =
                    arg as cty::c_int & 1i32 << i;
                i += 1
            }
            current_block_50 = 14466989937446026541;
        }
        7 | 15 => { current_block_50 = 14466989937446026541; }
        17 => {
            // -w <76543210>
            current_block_50 = 17075014677070940716;
        }
        18 => {
            // rw <..543210>
            if 0 != tihw.protect { return }
            arg = (arg as cty::c_int & 0x3fi32) as uint8_t;
            current_block_50 = 17075014677070940716;
        }
        20 => {
            // rw <76543210>
            // RTC, incremented every 2^13 seconds. The whole word must be read:
		// reading the port byte by byte can return wrong value
            tihw.rtc_value =
                ((*tihw.io2.offset(0x14i32 as isize) as cty::c_int) << 8i32 |
                     *tihw.io2.offset(0x15i32 as isize) as cty::c_int) as
                    uint8_t;
            current_block_50 = 17075014677070940716;
        }
        21 => {
            // rw <76543210>
            tihw.rtc_value =
                ((*tihw.io2.offset(0x14i32 as isize) as cty::c_int) << 8i32 |
                     *tihw.io2.offset(0x15i32 as isize) as cty::c_int) as
                    uint8_t;
            current_block_50 = 17075014677070940716;
        }
        23 => {
            // rw <......10>
            // Display memory snoop range
            tihw.lcd_adr =
                (0x4c00i32 + 0x1000i32 * (arg as cty::c_int & 3i32)) as
                    uint32_t;
            current_block_50 = 17075014677070940716;
        }
        29 => {
            // rw <7...3210>
            // %1: Screen enable (clear this bit to shut down LCD)
            tihw.on_off =
                if 0 != arg as cty::c_int & 1i32 << 1i32 {
                    1i32
                } else { 0i32 };
            current_block_50 = 17075014677070940716;
        }
        31 => {
            // rw <.....210>
            if 0 == tihw.protect {
                *tihw.io2.offset(addr as isize) = arg
            } else { return }
            current_block_50 = 17075014677070940716;
        }
        19 | _ => { current_block_50 = 17075014677070940716; }
    }
    match current_block_50 {
        14466989937446026541 =>
        // rw <76543210>
        {
            if 0 != tihw.protect { return }
            i = 0i32;
            while i < 8i32 {
                tihw.ram_exec[(48i32 + i) as usize] =
                    arg as cty::c_int & 1i32 << i;
                i += 1
            }
        }
        _ => { }
    }
    *tihw.io2.offset(addr as isize) = arg;
}
#[no_mangle]
pub unsafe extern "C" fn io2_put_word(mut addr: uint32_t, mut arg: uint16_t) {
    io2_put_byte(addr, ((arg as cty::c_int & 0xff00i32) >> 8i32) as uint8_t);
    io2_put_byte(addr.wrapping_add(1i32 as cty::c_uint),
                 (arg as cty::c_int & 0xffi32) as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn io2_put_long(mut addr: uint32_t, mut arg: uint32_t) {
    io2_put_word(addr, ((arg & 0xffff0000u32) >> 16i32) as uint16_t);
    io2_put_word(addr.wrapping_add(2i32 as cty::c_uint),
                 (arg & 0xffffi32 as cty::c_uint) as uint16_t);
}
// ---
#[no_mangle]
pub unsafe extern "C" fn io2_get_byte(mut addr: uint32_t) -> uint8_t {
    let mut v: cty::c_int = 0; // tihw.io2_size-1;
    addr &= 63i32 as cty::c_uint;
    v = *tihw.io2.offset(addr as isize) as cty::c_int;
    match addr {
        20 => {
            // rw <7...3210>
            // RTC hw2 incremented every 2^13 seconds. The whole word must
		// be read: reading the port byte per byte can return wrong value.
            return ((tihw.rtc_value as cty::c_int & 0xff00i32) >> 8i32) as
                       uint8_t
        }
        21 => { return (tihw.rtc_value as cty::c_int & 0xffi32) as uint8_t }
        0 | 8 | 1 | 9 | 2 | 10 | 3 | 11 | 4 | 12 | 5 | 13 | 6 | 14 | 7 | 15 |
        17 | 18 | 19 | 23 | 29 | 31 | _ => {
        }
    }
    return v as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn io2_get_word(mut addr: uint32_t) -> uint16_t {
    return ((io2_get_byte(addr) as uint16_t as cty::c_int) << 8i32 |
                io2_get_byte(addr.wrapping_add(1i32 as cty::c_uint)) as
                    cty::c_int) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn io2_get_long(mut addr: uint32_t) -> uint32_t {
    return (io2_get_word(addr) as uint32_t) << 16i32 |
               io2_get_word(addr.wrapping_add(2i32 as cty::c_uint)) as
                   cty::c_uint;
}
/* * HW3 **/
#[no_mangle]
pub unsafe extern "C" fn io3_put_byte(mut addr: uint32_t, mut arg: uint8_t) {
    addr &= 255i32 as cty::c_uint; // tihw.io3_size-1;
    match addr {
        64 | 65 | 66 | 67 => { }
        68 => {
            // rw <....3210>
            // RTC hw3: 1/16th of seconds, upper digit is always 0 (loading register)
            arg = (arg as cty::c_int & 0xfi32) as uint8_t
        }
        69 => {
            // ro <....3210>
            // RTC hw3: 1/16th of seconds, upper digit is always 0 (counting register)
            arg = (arg as cty::c_int & 0xfi32) as uint8_t;
            return
        }
        70 | 71 | 72 | 73 => {
            // ro <76543210>
            // RTC hw3: seconds since January 1st, 1997 00:00:00 (counting register)
            return
        }
        95 => {
            // ro & rw <......10>
            // RTC hw3 control register
		// bit 0 means clock enabled ($710040 is set to 0 when disabled),
		// bit 1 changing from 0 to 1 loads $710040:44 to $710045-49 and set the clock
            arg = (arg as cty::c_int & 0x3i32) as uint8_t;
            arg = (arg as cty::c_int | 0x80i32) as uint8_t;
            if 0 == arg as cty::c_int & 1i32 << 0i32 {
                // RTC is disabled
                let ref mut fresh6 = *tihw.io3.offset(0x41i32 as isize);
                let ref mut fresh5 = *tihw.io3.offset(0x42i32 as isize);
                let ref mut fresh4 = *tihw.io3.offset(0x43i32 as isize);
                *fresh4 = 0i32 as uint8_t;
                *fresh5 = *fresh4;
                *fresh6 = *fresh5;
                *tihw.io3.offset(0x40i32 as isize) = *fresh6;
                memcpy(&mut tihw.rtc3_beg as *mut TTIME as *mut cty::c_void,
                       &mut tihw.rtc3_ref as *mut TTIME as
                           *const cty::c_void,
                       ::core::mem::size_of::<TTIME>() as cty::c_ulong);
            } else if 0 == arg as cty::c_int & 1i32 << 1i32 {
                // RTC reload
                *tihw.io3.offset(0x46i32 as isize) =
                    *tihw.io3.offset(0x40i32 as isize);
                *tihw.io3.offset(0x47i32 as isize) =
                    *tihw.io3.offset(0x41i32 as isize);
                *tihw.io3.offset(0x48i32 as isize) =
                    *tihw.io3.offset(0x42i32 as isize);
                *tihw.io3.offset(0x49i32 as isize) =
                    *tihw.io3.offset(0x43i32 as isize);
                *tihw.io3.offset(0x45i32 as isize) =
                    *tihw.io3.offset(0x44i32 as isize);
                tihw.rtc3_load.s =
                    ((*tihw.io3.offset(0x46i32 as isize) as cty::c_int) <<
                         24i32 |
                         (*tihw.io3.offset(0x47i32 as isize) as cty::c_int)
                             << 16i32 |
                         (*tihw.io3.offset(0x48i32 as isize) as cty::c_int)
                             << 8i32 |
                         *tihw.io3.offset(0x49i32 as isize) as cty::c_int) as
                        time_t;
                tihw.rtc3_load.ms =
                    125i32 * *tihw.io3.offset(0x45i32 as isize) as cty::c_int
                        >> 1i32;
                rtc3_get_time(&mut tihw.rtc3_beg);
            }
        }
        0 | _ => { }
    }
    *tihw.io3.offset(addr as isize) = arg;
}
#[no_mangle]
pub unsafe extern "C" fn io3_put_word(mut addr: uint32_t, mut arg: uint16_t) {
    io3_put_byte(addr, ((arg as cty::c_int & 0xff00i32) >> 8i32) as uint8_t);
    io3_put_byte(addr.wrapping_add(1i32 as cty::c_uint),
                 (arg as cty::c_int & 0xffi32) as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn io3_put_long(mut addr: uint32_t, mut arg: uint32_t) {
    io3_put_word(addr, ((arg & 0xffff0000u32) >> 16i32) as uint16_t);
    io3_put_word(addr.wrapping_add(2i32 as cty::c_uint),
                 (arg & 0xffffi32 as cty::c_uint) as uint16_t);
}
// ---
#[no_mangle]
pub unsafe extern "C" fn io3_get_byte(mut addr: uint32_t) -> uint8_t {
    let mut v: cty::c_int = 0; // tihw.io3_size-1;
    addr &= 255i32 as cty::c_uint;
    v = *tihw.io3.offset(addr as isize) as cty::c_int;
    let mut current_block_3: u64;
    match addr {
        68 => {
            // rw <....3210>
            // RTC hw3: 1/16th of seconds, upper digit is always 0 (loading register)
            v &= 0xfi32;
            current_block_3 = 13109137661213826276;
        }
        69 => {
            // ro <....3210>
	           // RTC hw3: 1/16th of seconds, upper digit is always 0 (counting register)
	           // beware: this function may be non portable but an equivalent exists for Linux
            current_block_3 = 14740433242466364216;
        }
        70 | 71 | 72 | 73 => { current_block_3 = 14740433242466364216; }
        95 => {
            // rw <......10>
            // RTC hw3 control register
            current_block_3 = 13109137661213826276;
        }
        0 | 64 | 65 | 66 | 67 | _ => {
            current_block_3 = 13109137661213826276;
        }
    }
    match current_block_3 {
        14740433242466364216 =>
        // ro <76543210>
        // RTC hw3: seconds since January 1st, 1997 00:00:00 (counting register)
        {
            rtc3_state_save();
        }
        _ => { }
    }
    return v as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn io3_get_word(mut addr: uint32_t) -> uint16_t {
    return ((io3_get_byte(addr) as uint16_t as cty::c_int) << 8i32 |
                io3_get_byte(addr.wrapping_add(1i32 as cty::c_uint)) as
                    cty::c_int) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn io3_get_long(mut addr: uint32_t) -> uint32_t {
    return (io3_get_word(addr) as uint32_t) << 16i32 |
               io3_get_word(addr.wrapping_add(2i32 as cty::c_uint)) as
                   cty::c_uint;
}
