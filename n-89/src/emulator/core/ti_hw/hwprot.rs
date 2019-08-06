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
    static mut tihw: Ti68kHardware;
    #[no_mangle]
    static mut mem_get_byte_ptr: GETBYTE_FUNC;
    #[no_mangle]
    static mut mem_get_word_ptr: GETWORD_FUNC;
    #[no_mangle]
    static mut mem_put_byte_ptr: PUTBYTE_FUNC;
    #[no_mangle]
    static mut regs: regstruct;
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
pub type GETBYTE_FUNC = Option<unsafe extern "C" fn(_: uint32_t) -> uint8_t>;
pub type GETWORD_FUNC = Option<unsafe extern "C" fn(_: uint32_t) -> uint16_t>;
pub type PUTBYTE_FUNC
    =
    Option<unsafe extern "C" fn(_: uint32_t, _: uint8_t) -> ()>;
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
/* $Id: hwprot.c 2268 2006-11-06 17:18:51Z roms $ */
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
    Memory management: TI89/92+/V200/Titanium FLASH with Hardware Protection
*/
static mut ba: uint32_t = 0;
// FLASH ROM base address (shortcut)
static mut access1: cty::c_int = 0;
// protection access authorization (hw1)
static mut access2: cty::c_int = 0;
// protection access authorization (hw2)
static mut crash: cty::c_int = 0;
// access counter before crashing
static mut arch_mem_crash: cty::c_int = 0;
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
// same
#[no_mangle]
pub unsafe extern "C" fn hw_hwp_init() -> cty::c_int {
    tihw.protect = 0i32;
    ba = tihw.rom_base.wrapping_sub(0x200000i32 as cty::c_uint);
    arch_mem_crash = 0i32;
    crash = arch_mem_crash;
    access2 = crash;
    access1 = access2;
    if tihw.hw_type >= 2i32 {
        *tihw.io2.offset(0x13i32 as isize) = 0x18i32 as uint8_t
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn hw_hwp_reset() -> cty::c_int { return 0i32; }
#[no_mangle]
pub unsafe extern "C" fn hw_hwp_exit() -> cty::c_int { return 0i32; }
unsafe extern "C" fn freeze_calc() {
    crash = 0i32;
    access2 = crash;
    access1 = access2;
    m68k_setstopped(1i32);
}
// note: "if(!(adr & 1))" is used to avoid multiple increment when reading/writing words.
// This don't work for longs but I don't think protection is accessed by longs.
/*
    Check whether instruction fetch is allowed at adr.
    The returned value can be used by breakpoints to determine the
    origin of violation.
*/
#[no_mangle]
pub unsafe extern "C" fn hwp_fetch(mut adr: uint32_t) -> cty::c_int {
    // protections (hw1)
    if tihw.hw_type == 1i32 {
        if adr >=
               (0x390000i32 as
                    cty::c_uint).wrapping_add(tihw.archive_limit.wrapping_mul(0x10000i32
                                                                                   as
                                                                                   cty::c_uint)).wrapping_add(ba)
               && adr <= (0x3fffffi32 as cty::c_uint).wrapping_add(ba) {
            // archive memory limit (hw1)
            // three consecutive access to any adress >=$390000+limit*$10000 and <$400000 crashes the calc
            if 0 == adr & 1i32 as cty::c_uint { arch_mem_crash += 1 }
            if tihw.hw_type == 1i32 && arch_mem_crash >= 4i32 {
                freeze_calc();
                return 1i32
            }
        }
    } else if adr >= 0i32 as cty::c_uint && adr <= 0x3ffffi32 as cty::c_uint
     {
        // protections (hw2)
        // RAM page execution protection
        if 0 != tihw.ram_exec[(adr >> 12i32) as usize] {
            freeze_calc();
            return 2i32
        }
    } else if adr >= 0x40000i32 as cty::c_uint &&
                  adr <= 0x1fffffi32 as cty::c_uint {
        // RAM page execution protection
        if 0 != *tihw.io2.offset(6isize) as cty::c_int & 1i32 << 7i32 {
            freeze_calc();
            return 2i32
        }
    } else if adr >= (0x210000i32 as cty::c_uint).wrapping_add(ba) &&
                  adr <=
                      ((0x1fffffi32 + tihw.rom_size) as
                           cty::c_uint).wrapping_add(ba) {
        // FLASH page execution protection
        if adr >=
               (0x210000i32 as
                    cty::c_uint).wrapping_add(ba).wrapping_add((*tihw.io2.offset(0x13i32
                                                                                      as
                                                                                      isize)
                                                                     as
                                                                     uint32_t).wrapping_mul(0x10000i32
                                                                                                as
                                                                                                cty::c_uint))
           {
            // printf(">> $%06x-%06x ", adr, 0x210000+ba + (uint32_t)tihw.io2[0x13]*0x10000);
            freeze_calc();
            return 3i32
        }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn hwp_get_byte(mut adr: uint32_t) -> uint8_t {
    // stealth I/O
    if adr >= 0x40000i32 as cty::c_uint && adr <= 0x7ffffi32 as cty::c_uint
       {
        // archive memory limit bit 0 (hw1)
        if 0 == tihw.protect && tihw.hw_type == 1i32 {
            tihw.archive_limit &= !(1i32 << 0i32) as cty::c_uint
        }
    } else if adr >= 0x80000i32 as cty::c_uint &&
                  adr <= 0xbffffi32 as cty::c_uint {
        // archive memory limit bit 1 (hw1)
        if 0 == tihw.protect && tihw.hw_type == 1i32 {
            tihw.archive_limit &= !(1i32 << 1i32) as cty::c_uint
        }
    } else if adr >= 0xc0000i32 as cty::c_uint &&
                  adr <= 0xfffffi32 as cty::c_uint {
        // archive memory limit bit 2 (hw1)
        if 0 == tihw.protect && tihw.hw_type == 1i32 {
            tihw.archive_limit &= !(1i32 << 2i32) as cty::c_uint
        }
    } else if !(adr >= 0x180000i32 as cty::c_uint &&
                    adr <= 0x1bffffi32 as cty::c_uint) {
        if adr >= 0x1c0000i32 as cty::c_uint &&
               adr <= 0x1fffffi32 as cty::c_uint {
            // protection enable
            // FIXME: >=7 is probably too lax, but the actual number can vary a lot due to the prefetch.
		// I guess we should check instructions rather than access counts on HW2.
            if access1 >= 3i32 || { access2 += 1; access2 >= 7i32 } {
                tihw.protect = (0 == 0i32) as cty::c_int;
                access2 = 0i32;
                access1 = access2
            } else if tihw.hw_type == 1i32 {
                // any four consecutive access to $1c0000-1fffff crashes a HW1 calc
                if 0 == adr & 1i32 as cty::c_uint { crash += 1 }
                if crash >= 4i32 { freeze_calc(); }
            }
        } else if adr >= (0x200000i32 as cty::c_uint).wrapping_add(ba) &&
                      adr <= (0x20ffffi32 as cty::c_uint).wrapping_add(ba) {
            // protection access authorization
            if tihw.hw_type == 1i32 {
                if 0 == adr & 1i32 as cty::c_uint { access1 += 1 }
            }
            if tihw.hw_type != 1i32 {
                if 0 == adr & 1i32 as cty::c_uint { access2 += 1 }
            }
        } else if adr >= (0x210000i32 as cty::c_uint).wrapping_add(ba) &&
                      adr <= (0x211fffi32 as cty::c_uint).wrapping_add(ba) {
            // certificate (read protected)
            if 0 != tihw.protect { return 0x14i32 as uint8_t }
        } else if adr >= (0x212000i32 as cty::c_uint).wrapping_add(ba) &&
                      adr <= (0x217fffi32 as cty::c_uint).wrapping_add(ba) {
            // protection access authorization
            if tihw.hw_type == 1i32 {
                if 0 == adr & 1i32 as cty::c_uint { access1 += 1 }
            }
            if tihw.hw_type != 1i32 {
                if 0 == adr & 1i32 as cty::c_uint { access2 += 1 }
            }
        } else if adr >= (0x218000i32 as cty::c_uint).wrapping_add(ba) &&
                      adr <= (0x219fffi32 as cty::c_uint).wrapping_add(ba) {
            // read protected
            if 0 != tihw.protect { return 0x14i32 as uint8_t }
        } else if adr >= (0x21a000i32 as cty::c_uint).wrapping_add(ba) &&
                      adr <= (0x21ffffi32 as cty::c_uint).wrapping_add(ba) {
            // protection access authorization
            if tihw.hw_type == 1i32 {
                if 0 == adr & 1i32 as cty::c_uint { access1 += 1 }
            }
            if tihw.hw_type != 1i32 {
                if 0 == adr & 1i32 as cty::c_uint { access2 += 1 }
            }
        } else {
            access2 = 0i32;
            access1 = access2;
            arch_mem_crash = 0i32;
            crash = arch_mem_crash
        }
    }
    // memory
    return mem_get_byte_ptr.expect("non-null function pointer")(adr);
}
#[no_mangle]
pub unsafe extern "C" fn hwp_get_word(mut adr: uint32_t) -> uint16_t {
    // We can't implement this in terms of hwp_get_byte, because the FlashROM
// hardware behaves differently for byte and word reads (see flash.c).
    // stealth I/O
    if adr >= 0x40000i32 as cty::c_uint && adr <= 0x7ffffi32 as cty::c_uint
       {
        // archive memory limit bit 0 (hw1)
        if 0 == tihw.protect && tihw.hw_type == 1i32 {
            tihw.archive_limit &= !(1i32 << 0i32) as cty::c_uint
        }
    } else if adr >= 0x80000i32 as cty::c_uint &&
                  adr <= 0xbffffi32 as cty::c_uint {
        // archive memory limit bit 1 (hw1)
        if 0 == tihw.protect && tihw.hw_type == 1i32 {
            tihw.archive_limit &= !(1i32 << 1i32) as cty::c_uint
        }
    } else if adr >= 0xc0000i32 as cty::c_uint &&
                  adr <= 0xfffffi32 as cty::c_uint {
        // archive memory limit bit 2 (hw1)
        if 0 == tihw.protect && tihw.hw_type == 1i32 {
            tihw.archive_limit &= !(1i32 << 2i32) as cty::c_uint
        }
    } else if !(adr >= 0x180000i32 as cty::c_uint &&
                    adr <= 0x1bffffi32 as cty::c_uint) {
        if adr >= 0x1c0000i32 as cty::c_uint &&
               adr <= 0x1fffffi32 as cty::c_uint {
            // protection enable
            access2 += 2i32;
            // FIXME: >=7 is probably too lax, but the actual number can vary a lot due to the prefetch.
		// I guess we should check instructions rather than access counts on HW2.
            if access1 >= 3i32 || access2 >= 7i32 {
                tihw.protect = (0 == 0i32) as cty::c_int;
                access2 = 0i32;
                access1 = access2
            } else if tihw.hw_type == 1i32 {
                // any four consecutive access to $1c0000-1fffff crashes a HW1 calc
                crash += 1;
                if crash >= 4i32 { freeze_calc(); }
            }
        } else if adr >= (0x200000i32 as cty::c_uint).wrapping_add(ba) &&
                      adr <= (0x20ffffi32 as cty::c_uint).wrapping_add(ba) {
            // protection access authorization
            if tihw.hw_type == 1i32 { access1 += 1 } else { access2 += 1 }
        } else if adr >= (0x210000i32 as cty::c_uint).wrapping_add(ba) &&
                      adr <= (0x211fffi32 as cty::c_uint).wrapping_add(ba) {
            // certificate (read protected)
            if 0 != tihw.protect { return 0x1414i32 as uint16_t }
        } else if adr >= (0x212000i32 as cty::c_uint).wrapping_add(ba) &&
                      adr <= (0x217fffi32 as cty::c_uint).wrapping_add(ba) {
            // protection access authorization
            if tihw.hw_type == 1i32 { access1 += 1 } else { access2 += 1 }
        } else if adr >= (0x218000i32 as cty::c_uint).wrapping_add(ba) &&
                      adr <= (0x219fffi32 as cty::c_uint).wrapping_add(ba) {
            // read protected
            if 0 != tihw.protect { return 0x1414i32 as uint16_t }
        } else if adr >= (0x21a000i32 as cty::c_uint).wrapping_add(ba) &&
                      adr <= (0x21ffffi32 as cty::c_uint).wrapping_add(ba) {
            // protection access authorization
            if tihw.hw_type == 1i32 { access1 += 1 } else { access2 += 1 }
        } else {
            access2 = 0i32;
            access1 = access2;
            arch_mem_crash = 0i32;
            crash = arch_mem_crash
        }
    }
    // memory
    return mem_get_word_ptr.expect("non-null function pointer")(adr);
}
#[no_mangle]
pub unsafe extern "C" fn hwp_get_long(mut adr: uint32_t) -> uint32_t {
    return ((hwp_get_word(adr.wrapping_add(0i32 as cty::c_uint)) as
                 cty::c_int) << 16i32 |
                hwp_get_word(adr.wrapping_add(2i32 as cty::c_uint)) as
                    cty::c_int) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn hwp_put_byte(mut adr: uint32_t, mut arg: uint8_t) {
    // stealth I/O
    if adr >= 0x40000i32 as cty::c_uint && adr <= 0x7ffffi32 as cty::c_uint
       {
        // archive memory limit bit 0 (hw1)
        if 0 == tihw.protect && tihw.hw_type == 1i32 {
            tihw.archive_limit |= (1i32 << 0i32) as cty::c_uint
        }
    } else if adr >= 0x80000i32 as cty::c_uint &&
                  adr <= 0xbffffi32 as cty::c_uint {
        // archive memory limit bit 1 (hw1)
        if 0 == tihw.protect && tihw.hw_type == 1i32 {
            tihw.archive_limit |= (1i32 << 1i32) as cty::c_uint
        }
    } else if adr >= 0xc0000i32 as cty::c_uint &&
                  adr <= 0xfffffi32 as cty::c_uint {
        // archive memory limit bit 2 (hw1)
        if 0 == tihw.protect && tihw.hw_type == 1i32 {
            tihw.archive_limit |= (1i32 << 2i32) as cty::c_uint
        }
    } else if !(adr >= 0x180000i32 as cty::c_uint &&
                    adr <= 0x1bffffi32 as cty::c_uint) {
        if adr >= 0x1c0000i32 as cty::c_uint &&
               adr <= 0x1fffffi32 as cty::c_uint {
            // protection disable
            // FIXME: >=7 is probably too lax, but the actual number can vary a lot due to the prefetch.
		// I guess we should check instructions rather than access counts on HW2.
            if access1 >= 3i32 || { access2 += 1; access2 >= 7i32 } {
                tihw.protect = 0i32;
                access2 = 0i32;
                access1 = access2
            } else if tihw.hw_type == 1i32 {
                // any four consecutive accesses to $1c0000-1fffff crash an HW1 calc
                if 0 == adr & 1i32 as cty::c_uint { crash += 1 }
                if crash >= 4i32 { freeze_calc(); }
            }
        } else if adr >= (0x200000i32 as cty::c_uint).wrapping_add(ba) &&
                      adr <= (0x20ffffi32 as cty::c_uint).wrapping_add(ba) {
            // protection access authorization
            if tihw.hw_type == 1i32 {
                if 0 == adr & 1i32 as cty::c_uint { access1 += 1 }
            }
            if tihw.hw_type != 1i32 {
                if 0 == adr & 1i32 as cty::c_uint { access2 += 1 }
            }
            // don't write on boot code
            return
        } else {
            if !(adr >= (0x210000i32 as cty::c_uint).wrapping_add(ba) &&
                     adr <= (0x211fffi32 as cty::c_uint).wrapping_add(ba)) {
                if adr >= (0x212000i32 as cty::c_uint).wrapping_add(ba) &&
                       adr <= (0x217fffi32 as cty::c_uint).wrapping_add(ba) {
                    // protection access authorization
                    if tihw.hw_type == 1i32 {
                        if 0 == adr & 1i32 as cty::c_uint { access1 += 1 }
                    }
                    if tihw.hw_type != 1i32 {
                        if 0 == adr & 1i32 as cty::c_uint { access2 += 1 }
                    }
                } else if !(adr >=
                                (0x218000i32 as cty::c_uint).wrapping_add(ba)
                                &&
                                adr <=
                                    (0x219fffi32 as
                                         cty::c_uint).wrapping_add(ba)) {
                    if adr >= (0x21a000i32 as cty::c_uint).wrapping_add(ba)
                           &&
                           adr <=
                               (0x21ffffi32 as cty::c_uint).wrapping_add(ba)
                       {
                        // protection access authorization
                        if tihw.hw_type == 1i32 {
                            if 0 == adr & 1i32 as cty::c_uint {
                                access1 += 1
                            }
                        }
                        if tihw.hw_type != 1i32 {
                            if 0 == adr & 1i32 as cty::c_uint {
                                access2 += 1
                            }
                        }
                    } else {
                        access2 = 0i32;
                        access1 = access2;
                        arch_mem_crash = 0i32;
                        crash = arch_mem_crash
                    }
                }
            }
        }
    }
    // memory
    mem_put_byte_ptr.expect("non-null function pointer")(adr, arg);
}
#[no_mangle]
pub unsafe extern "C" fn hwp_put_word(mut adr: uint32_t, mut arg: uint16_t) {
    hwp_put_byte(adr.wrapping_add(0i32 as cty::c_uint),
                 ((arg as cty::c_int & 0xff00i32) >> 8i32) as uint8_t);
    hwp_put_byte(adr.wrapping_add(1i32 as cty::c_uint),
                 (arg as cty::c_int & 0xffi32) as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn hwp_put_long(mut adr: uint32_t, mut arg: uint32_t) {
    hwp_put_word(adr.wrapping_add(0i32 as cty::c_uint),
                 ((arg & 0xffff0000u32) >> 16i32) as uint16_t);
    hwp_put_word(adr.wrapping_add(2i32 as cty::c_uint),
                 (arg & 0xffffi32 as cty::c_uint) as uint16_t);
}
