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
    fn strcpy(_: *mut cty::c_char, _: *const cty::c_char)
     -> *mut cty::c_char;
    #[no_mangle]
    fn strcmp(_: *const cty::c_char, _: *const cty::c_char) -> cty::c_int;
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
    // defs similar to UAE's memory.h (interface)
    #[no_mangle]
    fn hw_get_real_address(addr: uint32_t) -> *mut uint8_t;
    #[no_mangle]
    static mut regs: regstruct;
    /* CYGNUS_SIM */
    /* These are only used by the 68020/68881 code, and therefore don't
 * need to handle prefetch.  */
    /* A traced STOP instruction drops through immediately without
	   actually stopping.  */
    /* 0 */
    /* NO_GDB */
    /* 0 */
    #[no_mangle]
    fn fill_prefetch_slow();
    #[no_mangle]
    fn MakeFromSR();
    #[no_mangle]
    fn sscanf(_: *const cty::c_char, _: *const cty::c_char, _: ...)
     -> cty::c_int;
    #[no_mangle]
    fn MakeSR();
    #[no_mangle]
    fn printf(_: *const cty::c_char, _: ...) -> cty::c_int;
    #[no_mangle]
    fn sprintf(_: *mut cty::c_char, _: *const cty::c_char, _: ...)
     -> cty::c_int;
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __uint32_t = cty::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = cty::c_ulong;
/*
 * UAE - The Un*x Amiga Emulator
 *
 * MC68000 emulation
 *
 * Copyright 1995 Bernd Schmidt
 * $Id: newcpu.h 2085 2006-05-16 19:28:18Z roms $
 */
/* You can set this to long double to be more accurate. However, the
   resulting alignment issues will cost a lot of performance in some
   apps */
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
#[inline(always)]
unsafe extern "C" fn m68k_setpc(mut newpc: uintptr_t) {
    regs.pc_oldp = hw_get_real_address(newpc as uint32_t);
    regs.pc_p = regs.pc_oldp;
    regs.pc = (newpc & 0xffffffi32 as cty::c_ulong) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn m68k_getpc() -> uintptr_t {
    return (regs.pc as cty::c_long +
                (regs.pc_p as
                     *mut cty::c_char).wrapping_offset_from(regs.pc_oldp as
                                                                 *mut cty::c_char)
                    as cty::c_long) as uintptr_t;
}
// SR bits set/get modifiers
// Previous state to detect change
static mut old_d: [uint32_t; 8] = [0; 8];
static mut old_a: [uint32_t; 8] = [0; 8];
static mut old_sp: uint32_t = 0;
static mut old_usp: uint32_t = 0;
static mut old_ssp: uint32_t = 0;
static mut old_pc: uint32_t = 0;
static mut old_sr: uint16_t = 0;
static mut old_sf: [cty::c_char; 32] = [0; 32];
static mut old_uf: [cty::c_char; 32] = [0; 32];
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_set_data(mut n: cty::c_int,
                                                 mut val: uint32_t) {
    if n >= 0i32 && n < 8i32 { regs.regs[n as usize] = val };
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_set_addr(mut n: cty::c_int,
                                                 mut val: uint32_t) {
    if n >= 0i32 && n < 8i32 {
        *regs.regs.as_mut_ptr().offset(8isize).offset(n as isize) = val
    };
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_set_sp(mut val: uint32_t) {
    *regs.regs.as_mut_ptr().offset(8isize).offset(7isize) = val;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_set_usp(mut val: uint32_t) {
    if 0 == regs.s {
        *regs.regs.as_mut_ptr().offset(8isize).offset(7isize) = val
    } else { regs.usp = val as uintptr_t };
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_set_ssp(mut val: uint32_t) {
    if 0 != regs.s {
        *regs.regs.as_mut_ptr().offset(8isize).offset(7isize) = val
    } else { regs.isp = val as uintptr_t };
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_set_pc(mut val: uint32_t) {
    m68k_setpc(val as uintptr_t);
    fill_prefetch_slow();
    /* Force reloading the prefetch. */
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_set_sr(mut val: uint32_t) {
    regs.sr = val as cty::c_int as uint16_t;
    MakeFromSR();
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_set_flag(mut flag: uint8_t) {
    // TODO
	/* T  0  S  0  0  I2 I1 I0 0  0  0  X  N  Z  V  C */
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_set_flags(mut sf: *const cty::c_char,
                                                  mut uf: *const cty::c_char)
 -> cty::c_int {
    /* SR: T 0 S 0 0 I2 I1 I0 0 0 0 X N Z V C */
    let mut t: cty::c_int = 0;
    let mut s: cty::c_int = 0;
    let mut i: cty::c_int = 0;
    let mut x: cty::c_int = 0;
    let mut n: cty::c_int = 0;
    let mut z: cty::c_int = 0;
    let mut v: cty::c_int = 0;
    let mut c: cty::c_int = 0;
    let mut nargs: cty::c_int = 0;
    MakeSR();
    if !sf.is_null() {
        nargs =
            sscanf(sf,
                   b"T=%d S=%d I=%d\x00" as *const u8 as *const cty::c_char,
                   &mut t as *mut cty::c_int, &mut s as *mut cty::c_int,
                   &mut i as *mut cty::c_int);
        if nargs < 3i32 { return 0i32 }
        if i < 0i32 || i > 7i32 { return 0i32 }
        regs.sr = (regs.sr as cty::c_int | 1i32 << 15i32) as uint16_t;
        if 0 != t {
            regs.sr = (regs.sr as cty::c_int | 1i32 << 15i32) as uint16_t
        } else {
            regs.sr = (regs.sr as cty::c_int & !(1i32 << 15i32)) as uint16_t
        }
        if 0 != s {
            regs.sr = (regs.sr as cty::c_int | 1i32 << 13i32) as uint16_t
        } else {
            regs.sr = (regs.sr as cty::c_int & !(1i32 << 13i32)) as uint16_t
        }
        regs.sr = (regs.sr as cty::c_int & !(7i32 << 8i32)) as uint16_t;
        regs.sr = (regs.sr as cty::c_int | i << 8i32) as uint16_t
    }
    if !uf.is_null() {
        nargs =
            sscanf(uf,
                   b"X=%d N=%d \nZ=%d V=%d C=%d\x00" as *const u8 as
                       *const cty::c_char, &mut x as *mut cty::c_int,
                   &mut n as *mut cty::c_int, &mut z as *mut cty::c_int,
                   &mut v as *mut cty::c_int, &mut c as *mut cty::c_int);
        if nargs < 5i32 { return 0i32 }
        if 0 != x {
            regs.sr = (regs.sr as cty::c_int | 1i32 << 4i32) as uint16_t
        } else {
            regs.sr = (regs.sr as cty::c_int & !(1i32 << 4i32)) as uint16_t
        }
        if 0 != n {
            regs.sr = (regs.sr as cty::c_int | 1i32 << 3i32) as uint16_t
        } else {
            regs.sr = (regs.sr as cty::c_int & !(1i32 << 3i32)) as uint16_t
        }
        if 0 != z {
            regs.sr = (regs.sr as cty::c_int | 1i32 << 2i32) as uint16_t
        } else {
            regs.sr = (regs.sr as cty::c_int & !(1i32 << 2i32)) as uint16_t
        }
        if 0 != v {
            regs.sr = (regs.sr as cty::c_int | 1i32 << 1i32) as uint16_t
        } else {
            regs.sr = (regs.sr as cty::c_int & !(1i32 << 1i32)) as uint16_t
        }
        if 0 != c {
            regs.sr = (regs.sr as cty::c_int | 1i32 << 0i32) as uint16_t
        } else {
            regs.sr = (regs.sr as cty::c_int & !(1i32 << 0i32)) as uint16_t
        }
    }
    MakeFromSR();
    return (0 == 0i32) as cty::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_get_data(mut n: cty::c_int,
                                                 mut val: *mut uint32_t)
 -> cty::c_int {
    let mut c: cty::c_int = 0i32;
    if n >= 0i32 && n < 8i32 { *val = regs.regs[n as usize] }
    if regs.regs[n as usize] != old_d[n as usize] {
        c = (0 == 0i32) as cty::c_int
    }
    old_d[n as usize] = regs.regs[n as usize];
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_get_addr(mut n: cty::c_int,
                                                 mut val: *mut uint32_t)
 -> cty::c_int {
    let mut c: cty::c_int = 0i32;
    if n >= 0i32 && n < 8i32 {
        *val = *regs.regs.as_mut_ptr().offset(8isize).offset(n as isize)
    }
    if *regs.regs.as_mut_ptr().offset(8isize).offset(n as isize) !=
           old_a[n as usize] {
        c = (0 == 0i32) as cty::c_int
    }
    old_a[n as usize] =
        *regs.regs.as_mut_ptr().offset(8isize).offset(n as isize);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_get_sp(mut val: *mut uint32_t)
 -> cty::c_int {
    let mut c: cty::c_int = 0i32;
    *val = *regs.regs.as_mut_ptr().offset(8isize).offset(7isize);
    if *regs.regs.as_mut_ptr().offset(8isize).offset(7isize) != old_sp {
        c = (0 == 0i32) as cty::c_int
    }
    old_sp = *regs.regs.as_mut_ptr().offset(8isize).offset(7isize);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_get_usp(mut val: *mut uint32_t)
 -> cty::c_int {
    let mut c: cty::c_int = 0i32;
    let mut reg: *mut uint32_t = 0 as *mut uint32_t;
    if 0 == regs.s {
        reg =
            &mut *regs.regs.as_mut_ptr().offset(8isize).offset(7isize) as
                *mut uint32_t
    } else { reg = &mut regs.usp as *mut uintptr_t as *mut uint32_t }
    *val = *reg;
    if *reg != old_usp { c = (0 == 0i32) as cty::c_int }
    old_usp = *reg;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_get_ssp(mut val: *mut uint32_t)
 -> cty::c_int {
    let mut c: cty::c_int = 0i32;
    let mut reg: *mut uint32_t = 0 as *mut uint32_t;
    if 0 != regs.s {
        reg =
            &mut *regs.regs.as_mut_ptr().offset(8isize).offset(7isize) as
                *mut uint32_t
    } else { reg = &mut regs.isp as *mut uintptr_t as *mut uint32_t }
    *val = *reg;
    if *reg != old_ssp { c = (0 == 0i32) as cty::c_int }
    old_ssp = *reg;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_get_pc(mut val: *mut uint32_t)
 -> cty::c_int {
    let mut c: cty::c_int = 0i32;
    *val = m68k_getpc() as uint32_t;
    if *val != old_pc { c = (0 == 0i32) as cty::c_int }
    old_pc = regs.pc;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_get_sr(mut val: *mut uint32_t)
 -> cty::c_int {
    let mut c: cty::c_int = 0i32;
    MakeSR();
    *val = regs.sr as uint32_t;
    if regs.sr as cty::c_int != old_sr as cty::c_int {
        c = (0 == 0i32) as cty::c_int
    }
    old_sr = regs.sr;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_get_flag() -> *const cty::c_char {
    static mut str: [cty::c_char; 64] = [0; 64];
    /* T  0  S  0  0  I2 I1 I0 0  0  0  X  N  Z  V  C */
    MakeSR();
    printf(b"T=%d S=%d I=%d | X=%d N=%d\nZ=%d V=%d C=%d\n\x00" as *const u8 as
               *const cty::c_char,
           (regs.sr as cty::c_int & 1i32 << 15i32) >> 15i32,
           (regs.sr as cty::c_int & 1i32 << 13i32) >> 13i32,
           regs.sr as cty::c_int >> 8i32 & 7i32,
           (regs.sr as cty::c_int & 1i32 << 4i32) >> 4i32,
           (regs.sr as cty::c_int & 1i32 << 3i32) >> 3i32,
           (regs.sr as cty::c_int & 1i32 << 2i32) >> 2i32,
           (regs.sr as cty::c_int & 1i32 << 1i32) >> 1i32,
           (regs.sr as cty::c_int & 1i32 << 0i32) >> 0i32);
    return str.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_register_get_flags(mut sf: *mut cty::c_char,
                                                  mut uf: *mut cty::c_char)
 -> cty::c_int {
    let mut c: cty::c_int = 0i32;
    /* SR: T 0 S 0 0 I2 I1 I0 0 0 0 X N Z V C */
    MakeSR();
    sprintf(sf, b"T=%d S=%d I=%d\x00" as *const u8 as *const cty::c_char,
            (regs.sr as cty::c_int & 1i32 << 15i32) >> 15i32,
            (regs.sr as cty::c_int & 1i32 << 13i32) >> 13i32,
            regs.sr as cty::c_int >> 8i32 & 7i32);
    sprintf(uf,
            b"X=%d N=%d \nZ=%d V=%d C=%d\x00" as *const u8 as
                *const cty::c_char,
            (regs.sr as cty::c_int & 1i32 << 4i32) >> 4i32,
            (regs.sr as cty::c_int & 1i32 << 3i32) >> 3i32,
            (regs.sr as cty::c_int & 1i32 << 2i32) >> 2i32,
            (regs.sr as cty::c_int & 1i32 << 1i32) >> 1i32,
            (regs.sr as cty::c_int & 1i32 << 0i32) >> 0i32);
    if 0 != strcmp(sf, old_sf.as_mut_ptr()) ||
           0 != strcmp(uf, old_uf.as_mut_ptr()) {
        c = (0 == 0i32) as cty::c_int
    }
    strcpy(old_sf.as_mut_ptr(), sf);
    strcpy(old_uf.as_mut_ptr(), uf);
    return c;
}
