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
    fn strcmp(_: *const cty::c_char, _: *const cty::c_char) -> cty::c_int;
}
// Breakpoints type
pub type C2RustUnnamed = cty::c_uint;
pub const BK_TYPE_BIT: C2RustUnnamed = 7;
pub const BK_TYPE_PROTECT: C2RustUnnamed = 6;
pub const BK_TYPE_PGMENTRY: C2RustUnnamed = 5;
pub const BK_TYPE_EXCEPTION: C2RustUnnamed = 4;
pub const BK_TYPE_CODE: C2RustUnnamed = 3;
pub const BK_TYPE_RANGE: C2RustUnnamed = 2;
pub const BK_TYPE_ACCESS: C2RustUnnamed = 1;
// Breakpoints cause (ti68k_bkpt_get_cause())
pub type C2RustUnnamed_0 = cty::c_uint;
pub const BK_CAUSE_BIT: C2RustUnnamed_0 = 7;
pub const BK_CAUSE_PROTECT: C2RustUnnamed_0 = 6;
pub const BK_CAUSE_PGMENTRY: C2RustUnnamed_0 = 5;
pub const BK_CAUSE_EXCEPTION: C2RustUnnamed_0 = 4;
pub const BK_CAUSE_ADDRESS: C2RustUnnamed_0 = 3;
pub const BK_CAUSE_RANGE: C2RustUnnamed_0 = 2;
pub const BK_CAUSE_ACCESS: C2RustUnnamed_0 = 1;
/* Hey EMACS -*- linux-c -*- */
/* $Id: type2str.c 2601 2007-07-14 08:49:30Z roms $ */
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
    Type conversion routines
*/
#[no_mangle]
pub unsafe extern "C" fn ti68k_calctype_to_string(mut type_0: cty::c_int)
 -> *const cty::c_char {
    match type_0 {
        2 => { return b"TI89\x00" as *const u8 as *const cty::c_char }
        1 => { return b"TI92\x00" as *const u8 as *const cty::c_char }
        4 => { return b"TI92+\x00" as *const u8 as *const cty::c_char }
        8 => { return b"V200PLT\x00" as *const u8 as *const cty::c_char }
        16 => { return b"TI89t\x00" as *const u8 as *const cty::c_char }
        _ => { return b"none\x00" as *const u8 as *const cty::c_char }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_string_to_calctype(mut str:
                                                      *const cty::c_char)
 -> cty::c_int {
    if 0 == strcmp(str, b"TI89\x00" as *const u8 as *const cty::c_char) {
        return 1i32 << 1i32
    } else {
        if 0 == strcmp(str, b"TI92\x00" as *const u8 as *const cty::c_char) {
            return 1i32 << 0i32
        } else {
            if 0 ==
                   strcmp(str,
                          b"TI92+\x00" as *const u8 as *const cty::c_char) {
                return 1i32 << 2i32
            } else {
                if 0 ==
                       strcmp(str,
                              b"V200PLT\x00" as *const u8 as
                                  *const cty::c_char) {
                    return 1i32 << 3i32
                } else {
                    if 0 ==
                           strcmp(str,
                                  b"TI89t\x00" as *const u8 as
                                      *const cty::c_char) {
                        return 1i32 << 4i32
                    }
                }
            }
        }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_romtype_to_string(mut type_0: cty::c_int)
 -> *const cty::c_char {
    match type_0 {
        0 => { return b"EPROM\x00" as *const u8 as *const cty::c_char }
        2 => { return b"FLASH\x00" as *const u8 as *const cty::c_char }
        _ => { }
    }
    return 0 as *const cty::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_string_to_romtype(mut str: *const cty::c_char)
 -> cty::c_int {
    if 0 == strcmp(str, b"EPROM\x00" as *const u8 as *const cty::c_char) {
        return 0i32
    } else {
        if 0 == strcmp(str, b"FLASH\x00" as *const u8 as *const cty::c_char)
           {
            return 2i32
        }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_hwtype_to_string(mut type_0: cty::c_int)
 -> *const cty::c_char {
    match type_0 {
        1 => { return b"HW1\x00" as *const u8 as *const cty::c_char }
        2 => { return b"HW2\x00" as *const u8 as *const cty::c_char }
        3 => { return b"HW3\x00" as *const u8 as *const cty::c_char }
        4 => { return b"HW4\x00" as *const u8 as *const cty::c_char }
        _ => { return b"none\x00" as *const u8 as *const cty::c_char }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_string_to_hwtype(mut str: *const cty::c_char)
 -> cty::c_int {
    if 0 == strcmp(str, b"HW1\x00" as *const u8 as *const cty::c_char) {
        return 1i32
    } else {
        if 0 == strcmp(str, b"HW2\x00" as *const u8 as *const cty::c_char) {
            return 2i32
        } else {
            if 0 ==
                   strcmp(str, b"HW3\x00" as *const u8 as *const cty::c_char)
               {
                return 3i32
            } else {
                if 0 ==
                       strcmp(str,
                              b"HW4\x00" as *const u8 as *const cty::c_char)
                   {
                    return 4i32
                }
            }
        }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_exception_to_string(mut number: cty::c_int)
 -> *const cty::c_char {
    match number {
        0 => { return b"Initial SSP\x00" as *const u8 as *const cty::c_char }
        1 => { return b"Initial PC\x00" as *const u8 as *const cty::c_char }
        2 => {
            return b"Bus Error vector\x00" as *const u8 as *const cty::c_char
        }
        3 => {
            return b"Address Error vector\x00" as *const u8 as
                       *const cty::c_char
        }
        4 => {
            return b"Illegal Instruction vector\x00" as *const u8 as
                       *const cty::c_char
        }
        5 => {
            return b"Zero Divide vector\x00" as *const u8 as
                       *const cty::c_char
        }
        6 => {
            return b"CHK Instruction vector\x00" as *const u8 as
                       *const cty::c_char
        }
        7 => {
            return b"TRAPV Instruction vector\x00" as *const u8 as
                       *const cty::c_char
        }
        8 => {
            return b"Privilege Violation vector\x00" as *const u8 as
                       *const cty::c_char
        }
        9 => {
            return b"Trace vector\x00" as *const u8 as *const cty::c_char
        }
        10 => {
            return b"Line 1010 Emulator vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        11 => {
            return b"Line 1111 Emulator vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        12 => {
            return b"Unassigned, reserved\x00" as *const u8 as
                       *const cty::c_char
        }
        13 => {
            return b"Unassigned, reserved\x00" as *const u8 as
                       *const cty::c_char
        }
        14 => {
            return b"Unassigned, reserved\x00" as *const u8 as
                       *const cty::c_char
        }
        15 => {
            return b"Uninitialised Interrupt vector\x00" as *const u8 as
                       *const cty::c_char
        }
        16 => {
            return b"Unassigned, reserved\x00" as *const u8 as
                       *const cty::c_char
        }
        17 => {
            return b"Unassigned, reserved\x00" as *const u8 as
                       *const cty::c_char
        }
        18 => {
            return b"Unassigned, reserved\x00" as *const u8 as
                       *const cty::c_char
        }
        19 => {
            return b"Unassigned, reserved\x00" as *const u8 as
                       *const cty::c_char
        }
        20 => {
            return b"Unassigned, reserved\x00" as *const u8 as
                       *const cty::c_char
        }
        21 => {
            return b"Unassigned, reserved\x00" as *const u8 as
                       *const cty::c_char
        }
        22 => {
            return b"Unassigned, reserved\x00" as *const u8 as
                       *const cty::c_char
        }
        23 => {
            return b"Unassigned, reserved\x00" as *const u8 as
                       *const cty::c_char
        }
        24 => {
            return b"Spurious Interrupt vector\x00" as *const u8 as
                       *const cty::c_char
        }
        25 => {
            return b"Level 1 Interrupt auto-vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        26 => {
            return b"Level 2 Interrupt auto-vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        27 => {
            return b"Level 3 Interrupt auto-vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        28 => {
            return b"Level 4 Interrupt auto-vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        29 => {
            return b"Level 5 Interrupt auto-vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        30 => {
            return b"Level 6 Interrupt auto-vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        31 => {
            return b"Level 7 Interrupt auto-vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        32 => {
            return b"TRAP #0 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        33 => {
            return b"TRAP #1 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        34 => {
            return b"TRAP #2 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        35 => {
            return b"TRAP #3 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        36 => {
            return b"TRAP #4 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        37 => {
            return b"TRAP #5 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        38 => {
            return b"TRAP #6 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        39 => {
            return b"TRAP #7 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        40 => {
            return b"TRAP #8 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        41 => {
            return b"TRAP #9 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        42 => {
            return b"TRAP #10 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        43 => {
            return b"TRAP #11 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        44 => {
            return b"TRAP #12 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        45 => {
            return b"TRAP #13 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        46 => {
            return b"TRAP #14 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        47 => {
            return b"TRAP #15 Instruction vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 |
        62 | 63 => {
            return b"Unassigned, reserved\x00" as *const u8 as
                       *const cty::c_char
        }
        64 => {
            return b"User Interrupt vectors\x00" as *const u8 as
                       *const cty::c_char
        }
        _ => {
            return b"User Interrupt vectors\x00" as *const u8 as
                       *const cty::c_char
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_bkpt_cause_to_string(mut type_0: cty::c_int)
 -> *const cty::c_char {
    match type_0 {
        1 => { return b"access\x00" as *const u8 as *const cty::c_char }
        2 => {
            return b"access range\x00" as *const u8 as *const cty::c_char
        }
        3 => { return b"address\x00" as *const u8 as *const cty::c_char }
        4 => { return b"exception\x00" as *const u8 as *const cty::c_char }
        5 => { return b"prgm entry\x00" as *const u8 as *const cty::c_char }
        6 => {
            return b"hw protection\x00" as *const u8 as *const cty::c_char
        }
        7 => { return b"bit change\x00" as *const u8 as *const cty::c_char }
        _ => { return b"unknown\x00" as *const u8 as *const cty::c_char }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_bkpt_type_to_string(mut type_0: cty::c_int)
 -> *const cty::c_char {
    match type_0 {
        1 => { return b"access\x00" as *const u8 as *const cty::c_char }
        2 => { return b"range\x00" as *const u8 as *const cty::c_char }
        3 => { return b"code\x00" as *const u8 as *const cty::c_char }
        4 => { return b"exception\x00" as *const u8 as *const cty::c_char }
        5 => { return b"prgm entry\x00" as *const u8 as *const cty::c_char }
        6 => {
            return b"hw protection\x00" as *const u8 as *const cty::c_char
        }
        7 => { return b"bit change\x00" as *const u8 as *const cty::c_char }
        _ => { return b"unknown\x00" as *const u8 as *const cty::c_char }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_string_to_bkpt_type(mut str:
                                                       *const cty::c_char)
 -> cty::c_int {
    if 0 == strcmp(str, b"access\x00" as *const u8 as *const cty::c_char) {
        return BK_TYPE_ACCESS as cty::c_int
    } else {
        if 0 == strcmp(str, b"range\x00" as *const u8 as *const cty::c_char)
           {
            return BK_TYPE_RANGE as cty::c_int
        } else {
            if 0 ==
                   strcmp(str,
                          b"code\x00" as *const u8 as *const cty::c_char) {
                return BK_TYPE_CODE as cty::c_int
            } else {
                if 0 ==
                       strcmp(str,
                              b"exception\x00" as *const u8 as
                                  *const cty::c_char) {
                    return BK_TYPE_EXCEPTION as cty::c_int
                } else {
                    if 0 ==
                           strcmp(str,
                                  b"prgm entry\x00" as *const u8 as
                                      *const cty::c_char) {
                        return BK_TYPE_PGMENTRY as cty::c_int
                    } else {
                        if 0 ==
                               strcmp(str,
                                      b"hw protection\x00" as *const u8 as
                                          *const cty::c_char) {
                            return BK_TYPE_PROTECT as cty::c_int
                        } else {
                            if 0 ==
                                   strcmp(str,
                                          b"bit change\x00" as *const u8 as
                                              *const cty::c_char) {
                                return BK_TYPE_BIT as cty::c_int
                            }
                        }
                    }
                }
            }
        }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_bkpt_mode_to_string(mut type_0: cty::c_int,
                                                   mut mode: cty::c_int)
 -> *const cty::c_char {
    // don't use type, it's implicit.
    if 0 != mode & 16i32 && 0 == mode & 32i32 {
        if 0 != mode & 1i32 {
            return b"byte-read\x00" as *const u8 as *const cty::c_char
        } else if 0 != mode & 2i32 {
            return b"word-read\x00" as *const u8 as *const cty::c_char
        } else if 0 != mode & 4i32 {
            return b"long-read\x00" as *const u8 as *const cty::c_char
        } else { return b"read\x00" as *const u8 as *const cty::c_char }
    } else {
        if 0 == mode & 16i32 && 0 != mode & 32i32 {
            if 0 != mode & 1i32 {
                return b"byte-write\x00" as *const u8 as *const cty::c_char
            } else if 0 != mode & 2i32 {
                return b"word-write\x00" as *const u8 as *const cty::c_char
            } else if 0 != mode & 4i32 {
                return b"long-write\x00" as *const u8 as *const cty::c_char
            } else { return b"write\x00" as *const u8 as *const cty::c_char }
        } else {
            if 0 != mode & 16i32 && 0 != mode & 32i32 {
                if 0 != mode & 1i32 {
                    return b"r/w byte\x00" as *const u8 as *const cty::c_char
                } else if 0 != mode & 2i32 {
                    return b"r/w word\x00" as *const u8 as *const cty::c_char
                } else if 0 != mode & 4i32 {
                    return b"r/w long\x00" as *const u8 as *const cty::c_char
                } else {
                    return b"r/w\x00" as *const u8 as *const cty::c_char
                }
            }
        }
    }
    return b"unknown (bug)\x00" as *const u8 as *const cty::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_string_to_bkpt_mode(mut str:
                                                       *const cty::c_char)
 -> cty::c_int {
    if 0 == strcmp(str, b"any\x00" as *const u8 as *const cty::c_char) ||
           0 == strcmp(str, b"r/w\x00" as *const u8 as *const cty::c_char) {
        return 16i32 | 32i32
    } else if 0 ==
                  strcmp(str, b"read\x00" as *const u8 as *const cty::c_char)
     {
        return 16i32
    } else if 0 ==
                  strcmp(str,
                         b"write\x00" as *const u8 as *const cty::c_char) {
        return 32i32
    } else if 0 ==
                  strcmp(str,
                         b"byte-read\x00" as *const u8 as *const cty::c_char)
     {
        return 16i32 | 1i32
    } else if 0 ==
                  strcmp(str,
                         b"word-read\x00" as *const u8 as *const cty::c_char)
     {
        return 16i32 | 2i32
    } else if 0 ==
                  strcmp(str,
                         b"long-read\x00" as *const u8 as *const cty::c_char)
     {
        return 16i32 | 4i32
    } else if 0 ==
                  strcmp(str,
                         b"byte-write\x00" as *const u8 as
                             *const cty::c_char) {
        return 32i32 | 1i32
    } else if 0 ==
                  strcmp(str,
                         b"word-write\x00" as *const u8 as
                             *const cty::c_char) {
        return 32i32 | 2i32
    } else if 0 ==
                  strcmp(str,
                         b"long-write\x00" as *const u8 as
                             *const cty::c_char) {
        return 32i32 | 4i32
    } else if 0 ==
                  strcmp(str,
                         b"r/w byte\x00" as *const u8 as *const cty::c_char)
     {
        return 16i32 | 1i32 | (32i32 | 1i32)
    } else if 0 ==
                  strcmp(str,
                         b"r/w word\x00" as *const u8 as *const cty::c_char)
     {
        return 16i32 | 2i32 | (32i32 | 2i32)
    } else if 0 ==
                  strcmp(str,
                         b"r/w long\x00" as *const u8 as *const cty::c_char)
     {
        return 16i32 | 4i32 | (32i32 | 4i32)
    } else { return 0i32 };
}
