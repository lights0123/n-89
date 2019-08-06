#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
use crate::libc::*;
extern "C" {
    /* Hey EMACS -*- linux-c -*- */
/* $Id: ti68k_def.h 2819 2009-05-02 19:51:29Z roms $ */
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
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street - Fifth Floor, Boston, MA 02110-1301, USA.
 */
    /* Equivalences */
    /* Constants */
    // LCD _memory_ width
    // LCD _memory_ height
    /* Structures */
    // CPU rate of a real calc
    // OSC1
    // OSC2
    // synched OSC2 (hw1) or OSC3 (hw2)
    // HW protection
    // receive file enabled/disabled
    // If this structure is modified, the SAV_REVISION number (state.c)
// has to be incremented.
    // misc (non hardware pseudo-constants)
    // RAM size
    // ROM size
    // HWx io size
    // HW2 io size
    // HW3 io size
    // ROM base address
    // ROM type
    // ROM/AMS version
    // HW1/2/3/4
    // ROM v1.x(y)
    // ROM v2.x
    // LCD physical width
    // LCD physical height
    // keyboard
    // lcd
    // LCD address (as $4c00)
    // direct pointer to LCD in PC RAM
    // LCD logical width
    // LCD logical height
    // used by grayscales
    // memory
    // ROM
    // RAM
    // HW1/2/3 i/o ports
    // HW2/3   i/o ports
    // HW3	   i/o ports
    // unused
    // SSP at vector #0
    // PC  at vector #1
    // timer
    // Current timer value
    // Value to reload
    // rtc (hw2)
    // RTC value
    // rtc (hw3)
    // time reference
    // time value when
    // clock is load
    // protection
    // hw protection state
    // archive memory limit
    // RAM page execution protection bitmask
    // PC
    // Link
    // buffer size
    // buffer (LSB is data, MSB is S/R action)
    // buffer index
    // actions (1: S, 2: R)
    /* Externs */
    #[no_mangle]
    static mut logger: Ti68kLogging;
    // defs similar to UAE's memory.h (interface)
    #[no_mangle]
    fn hw_get_real_address(addr: uint32_t) -> *mut uint8_t;
    #[no_mangle]
    static mut regs: regstruct;
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __uint32_t = cty::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = cty::c_ulong;
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
/* You can set this to long double to be more accurate. However, the
   resulting alignment issues will cost a lot of performance in some
   apps */
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
    /* CYGNUS_SIM */
}
#[inline(always)]
unsafe extern "C" fn m68k_getpc() -> uintptr_t {
    return (regs.pc as cty::c_long +
                (regs.pc_p as
                     *mut cty::c_char).wrapping_offset_from(regs.pc_oldp as
                                                                 *mut cty::c_char)
                    as cty::c_long) as uintptr_t;
}
/* Hey EMACS -*- linux-c -*- */
/* $Id: debug.h 2603 2007-07-14 17:09:56Z roms $ */
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
/* Hey EMACS -*- linux-c -*- */
/* $Id: debug.c 2792 2008-05-26 16:48:30Z roms $ */
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
/*
    Debug: debugging functions
*/
#[no_mangle]
pub unsafe extern "C" fn ti68k_debug_get_pc() -> cty::c_int {
    return m68k_getpc() as cty::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_debug_get_old_pc() -> cty::c_int {
    return *logger.pclog_buf.offset(((logger.pclog_ptr + logger.pclog_size -
                                          1i32) % logger.pclog_size) as isize)
               as cty::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_debug_break() -> cty::c_int {
    regs.spcflags |= 16i32 as cty::c_uint;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_debug_trace() -> cty::c_int {
    // Set up an internal trap (DBTRACE) which will
	// launch/refresh the debugger when encountered
    regs.spcflags |= 1i32 as cty::c_uint;
    return 0i32;
}
// Used to read/modify/write memory directly from debugger
#[no_mangle]
pub unsafe extern "C" fn ti68k_get_real_address(mut addr: uint32_t)
 -> *mut uint8_t {
    return hw_get_real_address(addr);
}
