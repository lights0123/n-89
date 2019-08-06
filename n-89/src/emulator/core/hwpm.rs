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
    fn tiemu_info(format: *const cty::c_char, _: ...);
    /* Hey EMACS -*- linux-c -*- */
/* $Id: romcalls.h 864 2005-02-22 09:54:05Z roms $ */
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
    /*
    Functions
*/
    #[no_mangle]
    fn rd_word(p: *mut uint8_t) -> uint16_t;
    #[no_mangle]
    fn wr_word(p: *mut uint8_t, d: uint16_t);
    #[no_mangle]
    fn wr_long(p: *mut uint8_t, d: uint32_t);
    #[no_mangle]
    fn rd_long(p: *mut uint8_t) -> uint32_t;
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __uint32_t = cty::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct HW_PARM_BLOCK {
    pub len: uint16_t,
    pub hardwareID: uint32_t,
    pub hardwareRevision: uint32_t,
    pub bootMajor: uint32_t,
    pub bootRevision: uint32_t,
    pub bootBuild: uint32_t,
    pub gateArray: uint32_t,
    pub physDisplayBitsWide: uint32_t,
    pub physDisplayBitsTall: uint32_t,
    pub LCDBitsWide: uint32_t,
    pub LCDBitsTall: uint32_t,
}
/* Hey EMACS -*- linux-c -*- */
/* $Id: hwpm.c 2752 2007-12-30 22:23:54Z roms $ */
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
/*
    This module manages Hardware Parameter Block. This is a 'C' structure like this:

    typedef struct {
    unsigned short len;					// length of parameter block
    unsigned long hardwareID;			// 1 = TI-92 Plus, 3 = TI-89
    unsigned long hardwareRevision;		// hardware revision number
    unsigned long bootMajor;			// boot code version number
    unsigned long bootRevision;			// boot code revision number
    unsigned long bootBuild;			// boot code build number
    unsigned long gateArray;			// gate array version number
    unsigned long physDisplayBitsWide;	// display width
    unsigned long physDisplayBitsTall;	// display height
    unsigned long LCDBitsWide;			// visible display width
    unsigned long LCDBitsTall;			// visible display height
    } HARDWARE_PARM_BLOCK;
*/
//memset
/* -- */
#[no_mangle]
pub unsafe extern "C" fn ti68k_display_hw_param_block(mut s:
                                                          *mut HW_PARM_BLOCK) {
    let mut i: cty::c_int = 0i32;
    tiemu_info(b"Hardware Parameters Block:\x00" as *const u8 as
                   *const cty::c_char);
    tiemu_info(b"  length           : %i\x00" as *const u8 as
                   *const cty::c_char, (*s).len as cty::c_int);
    let fresh0 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh0 {
        tiemu_info(b"  hardwareID       : %i\x00" as *const u8 as
                       *const cty::c_char, (*s).hardwareID);
    }
    let fresh1 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh1 {
        tiemu_info(b"  hardwareRevision : %i\x00" as *const u8 as
                       *const cty::c_char, (*s).hardwareRevision);
    }
    let fresh2 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh2 {
        tiemu_info(b"  bootMajor        : %i\x00" as *const u8 as
                       *const cty::c_char, (*s).bootMajor);
    }
    let fresh3 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh3 {
        tiemu_info(b"  bootRevision     : %i\x00" as *const u8 as
                       *const cty::c_char, (*s).bootRevision);
    }
    let fresh4 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh4 {
        tiemu_info(b"  bootBuild        : %i\x00" as *const u8 as
                       *const cty::c_char, (*s).bootBuild);
    }
    let fresh5 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh5 {
        tiemu_info(b"  gateArray        : %i\x00" as *const u8 as
                       *const cty::c_char, (*s).gateArray);
    }
    let fresh6 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh6 {
        tiemu_info(b"  physDisplayBitsWide : %i\x00" as *const u8 as
                       *const cty::c_char,
                   (*s).physDisplayBitsWide & 0xffi32 as cty::c_uint);
    }
    let fresh7 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh7 {
        tiemu_info(b"  physDisplayBitsTall : %i\x00" as *const u8 as
                       *const cty::c_char,
                   (*s).physDisplayBitsTall & 0xffi32 as cty::c_uint);
    }
    let fresh8 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh8 {
        tiemu_info(b"  LCDBitsWide         : %i\x00" as *const u8 as
                       *const cty::c_char,
                   (*s).LCDBitsWide & 0xffi32 as cty::c_uint);
    }
    let fresh9 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh9 {
        tiemu_info(b"  LCDBitsTall         : %i\x00" as *const u8 as
                       *const cty::c_char,
                   (*s).LCDBitsTall & 0xffi32 as cty::c_uint);
    };
}
/*
    Read hardware parameter block from image.
*/
#[no_mangle]
pub unsafe extern "C" fn ti68k_get_hw_param_block(mut rom_data: *mut uint8_t,
                                                  mut rom_base: uint8_t,
                                                  mut s: *mut HW_PARM_BLOCK)
 -> cty::c_int {
    let mut i: cty::c_int = 0i32;
    let mut addr: uint32_t = 0;
    addr = rd_long(&mut *rom_data.offset(0x104i32 as isize));
    addr &= 0xfffffi32 as cty::c_uint;
    memset(s as *mut cty::c_void, 0i32,
           ::core::mem::size_of::<HW_PARM_BLOCK>() as cty::c_ulong);
    (*s).len =
        rd_word(&mut *rom_data.offset(addr.wrapping_add(0i32 as cty::c_uint)
                                          as isize));
    let fresh10 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh10 {
        (*s).hardwareID =
            rd_long(&mut *rom_data.offset(addr.wrapping_add(2i32 as
                                                                cty::c_uint)
                                              as isize))
    }
    let fresh11 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh11 {
        (*s).hardwareRevision =
            rd_long(&mut *rom_data.offset(addr.wrapping_add(6i32 as
                                                                cty::c_uint)
                                              as isize))
    }
    let fresh12 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh12 {
        (*s).bootMajor =
            rd_long(&mut *rom_data.offset(addr.wrapping_add(10i32 as
                                                                cty::c_uint)
                                              as isize))
    }
    let fresh13 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh13 {
        (*s).bootRevision =
            rd_long(&mut *rom_data.offset(addr.wrapping_add(14i32 as
                                                                cty::c_uint)
                                              as isize))
    }
    let fresh14 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh14 {
        (*s).bootBuild =
            rd_long(&mut *rom_data.offset(addr.wrapping_add(18i32 as
                                                                cty::c_uint)
                                              as isize))
    }
    let fresh15 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh15 {
        (*s).gateArray =
            rd_long(&mut *rom_data.offset(addr.wrapping_add(22i32 as
                                                                cty::c_uint)
                                              as isize))
    }
    let fresh16 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh16 {
        (*s).physDisplayBitsWide =
            rd_long(&mut *rom_data.offset(addr.wrapping_add(26i32 as
                                                                cty::c_uint)
                                              as isize))
    }
    let fresh17 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh17 {
        (*s).physDisplayBitsTall =
            rd_long(&mut *rom_data.offset(addr.wrapping_add(30i32 as
                                                                cty::c_uint)
                                              as isize))
    }
    let fresh18 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh18 {
        (*s).LCDBitsWide =
            rd_long(&mut *rom_data.offset(addr.wrapping_add(34i32 as
                                                                cty::c_uint)
                                              as isize))
    }
    let fresh19 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh19 {
        (*s).LCDBitsTall =
            rd_long(&mut *rom_data.offset(addr.wrapping_add(38i32 as
                                                                cty::c_uint)
                                              as isize))
    }
    if (*s).hardwareID == 8i32 as cty::c_uint &&
           rom_base as cty::c_int == 0x40i32 {
        tiemu_info(b"Detected V200 patched ROM (ExtendeD): emulated as TI92+ by changing the hwID from 8 to 1.\x00"
                       as *const u8 as *const cty::c_char);
        (*s).hardwareID = 1i32 as uint32_t
    }
    if (*s).hardwareID == 9i32 as cty::c_uint &&
           rom_base as cty::c_int == 0x20i32 {
        tiemu_info(b"Detected TI89 Titanium patched ROM (ExtendeD): emulated as TI89 by changing the hwID from 9 to 3.\x00"
                       as *const u8 as *const cty::c_char);
        (*s).hardwareID = 3i32 as uint32_t
    }
    return 0i32;
}
/*
    Functions
*/
/*
    Write hardware parameter block into image.
*/
#[no_mangle]
pub unsafe extern "C" fn ti68k_put_hw_param_block(mut rom_data: *mut uint8_t,
                                                  mut rom_base: uint8_t,
                                                  mut s: *const HW_PARM_BLOCK)
 -> cty::c_int {
    let mut i: cty::c_int = 0i32;
    let mut addr: uint32_t = 0x108i32 as uint32_t;
    wr_long(&mut *rom_data.offset(0x104i32 as isize),
            (0 != (rom_base as cty::c_int) << 16i32 || 0 != addr) as
                cty::c_int as uint32_t);
    wr_word(&mut *rom_data.offset(addr.wrapping_add(0i32 as cty::c_uint) as
                                      isize), (*s).len);
    let fresh20 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh20 {
        wr_long(&mut *rom_data.offset(addr.wrapping_add(2i32 as cty::c_uint)
                                          as isize), (*s).hardwareID);
    }
    let fresh21 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh21 {
        wr_long(&mut *rom_data.offset(addr.wrapping_add(6i32 as cty::c_uint)
                                          as isize), (*s).hardwareRevision);
    }
    let fresh22 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh22 {
        wr_long(&mut *rom_data.offset(addr.wrapping_add(10i32 as cty::c_uint)
                                          as isize), (*s).bootMajor);
    }
    let fresh23 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh23 {
        wr_long(&mut *rom_data.offset(addr.wrapping_add(14i32 as cty::c_uint)
                                          as isize), (*s).bootRevision);
    }
    let fresh24 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh24 {
        wr_long(&mut *rom_data.offset(addr.wrapping_add(18i32 as cty::c_uint)
                                          as isize), (*s).bootBuild);
    }
    let fresh25 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh25 {
        wr_long(&mut *rom_data.offset(addr.wrapping_add(22i32 as cty::c_uint)
                                          as isize), (*s).gateArray);
    }
    let fresh26 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh26 {
        wr_long(&mut *rom_data.offset(addr.wrapping_add(26i32 as cty::c_uint)
                                          as isize),
                (*s).physDisplayBitsWide);
    }
    let fresh27 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh27 {
        wr_long(&mut *rom_data.offset(addr.wrapping_add(30i32 as cty::c_uint)
                                          as isize),
                (*s).physDisplayBitsTall);
    }
    let fresh28 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh28 {
        wr_long(&mut *rom_data.offset(addr.wrapping_add(34i32 as cty::c_uint)
                                          as isize), (*s).LCDBitsWide);
    }
    let fresh29 = i;
    i = i + 1;
    if (*s).len as cty::c_int > 2i32 + 4i32 * fresh29 {
        wr_long(&mut *rom_data.offset(addr.wrapping_add(38i32 as cty::c_uint)
                                          as isize), (*s).LCDBitsTall);
    }
    return 0i32;
}
