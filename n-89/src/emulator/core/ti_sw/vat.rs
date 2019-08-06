#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
use crate::libc::*;
extern crate c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    #[no_mangle]
    fn printf(_: *const cty::c_char, _: ...) -> cty::c_int;
    #[no_mangle]
    fn memcpy(_: *mut cty::c_void, _: *const cty::c_void, _: cty::c_ulong)
     -> *mut cty::c_void;
    #[no_mangle]
    fn strcmp(_: *const cty::c_char, _: *const cty::c_char) -> cty::c_int;
    #[no_mangle]
    fn strncmp(_: *const cty::c_char, _: *const cty::c_char,
               _: cty::c_ulong) -> cty::c_int;
    #[no_mangle]
    static mut tihw: Ti68kHardware;
    #[no_mangle]
    fn GUINT16_FROM_BE(num: uint16_t) -> uint16_t;
    #[no_mangle]
    fn ti68k_get_real_address(addr: uint32_t) -> *mut uint8_t;
    #[no_mangle]
    static mut img_infos: IMG_INFO32;
    #[no_mangle]
    static mut mem_get_word_ptr: GETWORD_FUNC;
    #[no_mangle]
    fn heap_get_block_addr_and_size(handle: cty::c_int, addr: *mut uint32_t,
                                    size: *mut uint16_t);
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __int32_t = cty::c_int;
pub type __uint32_t = cty::c_uint;
pub type __time_t = cty::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type time_t = __time_t;
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
/* Hey EMACS -*- linux-c -*- */
/* $Id: images.h 2821 2009-05-04 20:06:12Z roms $ */
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
  Definitions
*/
// increase this number when changing the structure below
// Please update the docs/TiEmu_img_format.txt documentation when making changes
// on the structure below
// If this structure is modified, the SAV_REVISION number (state.c)
// has to be incremented.
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
    // pure data (temporary use, 8 bytes)
}
pub type GETWORD_FUNC = Option<unsafe extern "C" fn(_: uint32_t) -> uint16_t>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TI89_SYM_ENTRY {
    pub name: [cty::c_char; 8],
    pub compat: uint16_t,
    pub flags: C2RustUnnamed,
    pub handle: uint16_t,
}
#[derive ( Copy , Clone )]
#[repr ( C )]
pub union C2RustUnnamed {
    pub flags_n: uint16_t,
    pub bits: C2RustUnnamed_0,
}
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    #[bitfield(name = "busy", ty = "uint16_t", bits = "0..=0")]
    #[bitfield(name = "local", ty = "uint16_t", bits = "1..=1")]
    #[bitfield(name = "flag1_5", ty = "uint16_t", bits = "2..=2")]
    #[bitfield(name = "flag1_4", ty = "uint16_t", bits = "3..=3")]
    #[bitfield(name = "collapsed", ty = "uint16_t", bits = "4..=4")]
    #[bitfield(name = "twin", ty = "uint16_t", bits = "5..=5")]
    #[bitfield(name = "archived", ty = "uint16_t", bits = "6..=6")]
    #[bitfield(name = "in_view", ty = "uint16_t", bits = "7..=7")]
    #[bitfield(name = "folder", ty = "uint16_t", bits = "8..=8")]
    #[bitfield(name = "overwritten", ty = "uint16_t", bits = "9..=9")]
    #[bitfield(name = "checked", ty = "uint16_t", bits = "10..=10")]
    #[bitfield(name = "hidden", ty = "uint16_t", bits = "11..=11")]
    #[bitfield(name = "locked", ty = "uint16_t", bits = "12..=12")]
    #[bitfield(name = "statvar", ty = "uint16_t", bits = "13..=13")]
    #[bitfield(name = "graph_ref_1", ty = "uint16_t", bits = "14..=14")]
    #[bitfield(name = "graph_ref_0", ty = "uint16_t", bits = "15..=15")]
    pub busy_local_flag1_5_flag1_4_collapsed_twin_archived_in_view_folder_overwritten_checked_hidden_locked_statvar_graph_ref_1_graph_ref_0: [u8; 2],
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TI92_SYM_ENTRY {
    pub name: [cty::c_char; 9],
    pub state: uint8_t,
    pub handle: uint16_t,
}
/*
    "a" To "z", "A" To "Z", Chr(128), Chr(129), Chr(130), Chr(131), Chr(132), Chr(133), Chr(134),
    Chr(135), Chr(136), Chr(137), Chr(138), Chr(139), Chr(141), Chr(142), Chr(143), Chr(144),
    Chr(145), Chr(146), Chr(147), Chr(148), "�" To "�", "�" To "�", "�" To "�", "_", Chr(154),
    Chr(155), Chr(178), "\"
    and "0" To "9" (apart from first position)
*/
static mut valid_chars: [cty::c_int; 256] =
    [0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32,
     0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32,
     0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32,
     0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32,
     1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 0i32, 0i32,
     0i32, 0i32, 0i32, 0i32, 0i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32,
     1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32,
     1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 0i32, 1i32, 0i32, 0i32, 1i32,
     0i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32,
     1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32,
     1i32, 1i32, 1i32, 0i32, 0i32, 0i32, 0i32, 0i32, 1i32, 1i32, 1i32, 1i32,
     1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 0i32, 1i32, 1i32, 1i32,
     1i32, 1i32, 1i32, 1i32, 1i32, 0i32, 0i32, 0i32, 0i32, 0i32, 1i32, 1i32,
     0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32,
     0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 1i32, 0i32,
     0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32,
     1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 0i32, 1i32, 1i32, 1i32, 1i32,
     1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 0i32,
     1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32,
     1i32, 1i32, 1i32, 0i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32,
     1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 0i32, 1i32, 1i32, 1i32, 1i32,
     1i32, 1i32, 1i32, 1i32];
//#define is_alnum(c)			isalnum((char)(c))
// Return TRUE if there is a NULL terminated string starting at *mem.
unsafe extern "C" fn is_varname(mut mem: *mut uint8_t) -> cty::c_int {
    let mut i: cty::c_int = 0i32;
    // printf("%i %i %i %02x\n", is_alnum(mem[i]) ? 1 : 0, is_allowed_char(mem[i]), mem[i], mem[i]);
    // The VAT can contain names starting with a number, they are hidden in Var-Link.
    if 0 ==
           valid_chars[(*mem.offset(0isize) as cty::c_int & 0xffi32) as
                           usize] {
        return 0i32
    }
    i = 1i32;
    while i < 8i32 {
        if 0 ==
               valid_chars[(*mem.offset(i as isize) as cty::c_int & 0xffi32)
                               as usize] &&
               0 != *mem.offset(i as isize) as cty::c_int {
            return 0i32
        }
        i += 1
    }
    return (0 == 0i32) as cty::c_int;
}
unsafe extern "C" fn get_folder_list_handle() -> cty::c_int {
    let mut h: cty::c_int = 0;
    let mut i: cty::c_int = 0;
    // search for memory blocks which have a string at [5]
    h = 1i32;
    while h < 2000i32 {
        let mut addr: uint32_t = 0;
        let mut size: uint16_t = 0;
        heap_get_block_addr_and_size(h, &mut addr, &mut size);
        if 0 !=
               is_varname(ti68k_get_real_address(addr.wrapping_add(4i32 as
                                                                       cty::c_uint)))
           {
            // next, be sure we have found the folder list by comparing the number of (possible)
			// folders with the number of identified folders
            let mut nfolders: cty::c_int =
                mem_get_word_ptr.expect("non-null function pointer")(addr.wrapping_add(2i32
                                                                                           as
                                                                                           cty::c_uint))
                    as cty::c_int;
            // The folder list always contains at least the main folder.
            if !(nfolders == 0i32) {
                i = 0i32;
                while i < nfolders {
                    if 0 ==
                           is_varname(ti68k_get_real_address((addr.wrapping_add(4i32
                                                                                    as
                                                                                    cty::c_uint)
                                                                  as
                                                                  cty::c_ulong).wrapping_add((i
                                                                                                   as
                                                                                                   cty::c_ulong).wrapping_mul(::core::mem::size_of::<TI89_SYM_ENTRY>()
                                                                                                                                   as
                                                                                                                                   cty::c_ulong))
                                                                 as uint32_t))
                       {
                        break ;
                    }
                    i += 1
                }
                // not valid, so try the next handle
                if !(i < nfolders) {
                    printf(b"handle $%i, #folders = %i\n\x00" as *const u8 as
                               *const cty::c_char, h, nfolders);
                    return h
                }
            }
        }
        h += 1
    }
    return -1i32;
}
unsafe extern "C" fn sym_find_handle_89(mut dirname: *const cty::c_char,
                                        mut filename: *const cty::c_char)
 -> cty::c_int {
    let mut fa: uint32_t = 0;
    let mut va: uint32_t = 0;
    let mut fs: uint16_t = 0;
    let mut vs: uint16_t = 0;
    let mut nfolders: cty::c_int = 0;
    let mut nvars: cty::c_int = 0;
    let mut i: cty::c_int = 0;
    let mut j: cty::c_int = 0;
    let mut handle: cty::c_int = 0x8i32;
    if tihw.calc_type == 1i32 << 0i32 { return -1i32 }
    // handle: names and handles of all folders (including "main")
    if strcmp(img_infos.version.as_mut_ptr(),
              b"2.00\x00" as *const u8 as *const cty::c_char) >= 0i32
       { // AMS1 (static)
        handle = get_folder_list_handle()
    } else { handle = 0x8i32 } // AMS2 (dynamic)
    if handle == -1i32 {
        return 0i32
    } else { heap_get_block_addr_and_size(handle, &mut fa, &mut fs); }
    // skip maximum number of folders before handle #$B needs to be resized
	// and actual number of folders
    nfolders =
        mem_get_word_ptr.expect("non-null function pointer")(fa.wrapping_add(2i32
                                                                                 as
                                                                                 cty::c_uint))
            as cty::c_int;
    fa =
        (fa as cty::c_uint).wrapping_add(4i32 as cty::c_uint) as uint32_t as
            uint32_t;
    // now, we read a list of SYM_ENTRY structs (list of folders)
    i = 0i32;
    while i < nfolders {
        let mut se: TI89_SYM_ENTRY =
            TI89_SYM_ENTRY{name: [0; 8],
                           compat: 0,
                           flags: C2RustUnnamed{flags_n: 0,},
                           handle: 0,};
        // read struct
        memcpy(&mut se as *mut TI89_SYM_ENTRY as *mut cty::c_void,
               ti68k_get_real_address((fa as
                                           cty::c_ulong).wrapping_add((i as
                                                                            cty::c_ulong).wrapping_mul(::core::mem::size_of::<TI89_SYM_ENTRY>()
                                                                                                            as
                                                                                                            cty::c_ulong))
                                          as uint32_t) as *const cty::c_void,
               ::core::mem::size_of::<TI89_SYM_ENTRY>() as cty::c_ulong);
        se.handle = GUINT16_FROM_BE(se.handle);
        if !(0 !=
                 strncmp(se.name.as_mut_ptr(), dirname,
                         8i32 as cty::c_ulong)) {
            // handle xxxx: names and handles of all variables
            heap_get_block_addr_and_size(se.handle as cty::c_int, &mut va,
                                         &mut vs);
            // skip max num and actual num of vars
            nvars =
                mem_get_word_ptr.expect("non-null function pointer")(va.wrapping_add(2i32
                                                                                         as
                                                                                         cty::c_uint))
                    as cty::c_int;
            va =
                (va as cty::c_uint).wrapping_add(4i32 as cty::c_uint) as
                    uint32_t as uint32_t;
            j = 0i32;
            while j < nvars {
                let mut se_0: TI89_SYM_ENTRY =
                    TI89_SYM_ENTRY{name: [0; 8],
                                   compat: 0,
                                   flags: C2RustUnnamed{flags_n: 0,},
                                   handle: 0,};
                // read struct
                memcpy(&mut se_0 as *mut TI89_SYM_ENTRY as *mut cty::c_void,
                       ti68k_get_real_address((va as
                                                   cty::c_ulong).wrapping_add((j
                                                                                    as
                                                                                    cty::c_ulong).wrapping_mul(::core::mem::size_of::<TI89_SYM_ENTRY>()
                                                                                                                    as
                                                                                                                    cty::c_ulong))
                                                  as uint32_t) as
                           *const cty::c_void,
                       ::core::mem::size_of::<TI89_SYM_ENTRY>() as
                           cty::c_ulong);
                se_0.handle = GUINT16_FROM_BE(se_0.handle);
                // add node
                if 0 !=
                       strncmp(se_0.name.as_mut_ptr(), filename,
                               8i32 as cty::c_ulong) {
                    j += 1
                } else { return se_0.handle as cty::c_int }
            }
        }
        i += 1
    }
    return 0i32;
}
unsafe extern "C" fn sym_find_handle_92(mut dirname: *const cty::c_char,
                                        mut filename: *const cty::c_char)
 -> cty::c_int {
    let mut fa: uint32_t = 0;
    let mut va: uint32_t = 0;
    let mut fs: uint16_t = 0;
    let mut vs: uint16_t = 0;
    let mut nfolders: cty::c_int = 0;
    let mut nvars: cty::c_int = 0;
    let mut i: cty::c_int = 0;
    let mut j: cty::c_int = 0;
    if tihw.calc_type != 1i32 << 0i32 { return 0i32 }
    // handle 000B:	names and handles of all folders (including "main")
    heap_get_block_addr_and_size(0xbi32, &mut fa, &mut fs);
    // skip maximum number of folders before handle #$B needs to be resized
	// and actual number of folders
    nfolders =
        mem_get_word_ptr.expect("non-null function pointer")(fa.wrapping_add(2i32
                                                                                 as
                                                                                 cty::c_uint))
            as cty::c_int;
    fa =
        (fa as cty::c_uint).wrapping_add(4i32 as cty::c_uint) as uint32_t as
            uint32_t;
    // now, we read a list of SYM_ENTRY structs (list of folders)
    i = 0i32;
    while i < nfolders {
        let mut se: TI92_SYM_ENTRY =
            TI92_SYM_ENTRY{name: [0; 9], state: 0, handle: 0,};
        // read struct
        memcpy(&mut se as *mut TI92_SYM_ENTRY as *mut cty::c_void,
               ti68k_get_real_address((fa as
                                           cty::c_ulong).wrapping_add((i as
                                                                            cty::c_ulong).wrapping_mul(::core::mem::size_of::<TI92_SYM_ENTRY>()
                                                                                                            as
                                                                                                            cty::c_ulong))
                                          as uint32_t) as *const cty::c_void,
               ::core::mem::size_of::<TI92_SYM_ENTRY>() as cty::c_ulong);
        se.handle = GUINT16_FROM_BE(se.handle);
        if !(0 !=
                 strncmp(se.name.as_mut_ptr(), dirname,
                         8i32 as cty::c_ulong)) {
            // handle xxxx: names and handles of all variables
            heap_get_block_addr_and_size(se.handle as cty::c_int, &mut va,
                                         &mut vs);
            // skip max num and actual num of vars
            nvars =
                mem_get_word_ptr.expect("non-null function pointer")(va.wrapping_add(2i32
                                                                                         as
                                                                                         cty::c_uint))
                    as cty::c_int;
            va =
                (va as cty::c_uint).wrapping_add(4i32 as cty::c_uint) as
                    uint32_t as uint32_t;
            j = 0i32;
            while j < nvars {
                let mut se_0: TI92_SYM_ENTRY =
                    TI92_SYM_ENTRY{name: [0; 9], state: 0, handle: 0,};
                // read struct
                memcpy(&mut se_0 as *mut TI92_SYM_ENTRY as *mut cty::c_void,
                       ti68k_get_real_address((va as
                                                   cty::c_ulong).wrapping_add((j
                                                                                    as
                                                                                    cty::c_ulong).wrapping_mul(::core::mem::size_of::<TI92_SYM_ENTRY>()
                                                                                                                    as
                                                                                                                    cty::c_ulong))
                                                  as uint32_t) as
                           *const cty::c_void,
                       ::core::mem::size_of::<TI92_SYM_ENTRY>() as
                           cty::c_ulong);
                se_0.handle = GUINT16_FROM_BE(se_0.handle);
                // add node
                if 0 !=
                       strncmp(se_0.name.as_mut_ptr(), filename,
                               8i32 as cty::c_ulong) {
                    j += 1
                } else { return se_0.handle as cty::c_int }
            }
        }
        i += 1
    }
    return 0i32;
}
/* Hey EMACS -*- linux-c -*- */
/* $Id: vat.h 2268 2006-11-06 17:18:51Z roms $ */
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
    Definitions
*/
/*
    Functions
*/
/*
    Allocate and create a tree
*/
#[no_mangle]
pub unsafe extern "C" fn sym_find_handle(mut dirname: *const cty::c_char,
                                         mut filename: *const cty::c_char)
 -> cty::c_int {
    if tihw.calc_type == 1i32 << 0i32 {
        return sym_find_handle_92(dirname, filename)
    } else { return sym_find_handle_89(dirname, filename) };
}
