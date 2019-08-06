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
    static mut mem_get_word_ptr: GETWORD_FUNC;
    #[no_mangle]
    static mut mem_get_long_ptr: GETLONG_FUNC;
    /* Functions */
    #[no_mangle]
    fn romcalls_get_table_infos(base: *mut uint32_t, size: *mut uint32_t);
    #[no_mangle]
    fn romcalls_get_symbol_address(id: cty::c_int, addr: *mut uint32_t);
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __uint32_t = cty::c_uint;
pub type __time_t = cty::c_long;
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
pub type GETWORD_FUNC = Option<unsafe extern "C" fn(_: uint32_t) -> uint16_t>;
pub type GETLONG_FUNC = Option<unsafe extern "C" fn(_: uint32_t) -> uint32_t>;
/* Hey EMACS -*- linux-c -*- */
/* $Id: handles.c 2268 2006-11-06 17:18:51Z roms $ */
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
    Handles/Heap access:
    $5D42 is a pointer on the Handles[] array. An handle is an index in this array
    which poins on a memory allocatedblock :
    Handles[0] => block 0
    Handles[1] => block 1 ....

    The first word just before the beginning of the block is the block size.

    - HeapAlloc: | size | block |
                          |
    HeapDeref           --+

    - HeapAllocPtr:		--+
                          |
        | size | handle | block |
                 |
    HeapDeref  --+

    - PedRom: | size.l | handle | block |
                                  |
    HeapDeref					--+


*/
static mut pedrom: cty::c_int = 0i32;
/* Hey EMACS -*- linux-c -*- */
/* $Id: handles.h 2268 2006-11-06 17:18:51Z roms $ */
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
    Retrieve address of heap (pointed by $5D42 on TI92).
*/
#[no_mangle]
pub unsafe extern "C" fn heap_get_addr(mut base: *mut uint32_t) {
    pedrom =
        (mem_get_word_ptr.expect("non-null function pointer")(0x32i32 as
                                                                  uint32_t) as
             cty::c_int == (('R' as i32) << 8i32) + 'O' as i32) as
            cty::c_int;
    if 0 != pedrom &&
           mem_get_word_ptr.expect("non-null function pointer")(0x30i32 as
                                                                    uint32_t)
               as cty::c_int <= 0x80i32 {
        // PedroM <=0.80
        let mut ptr: uint32_t =
            0x5d58i32 as uint32_t; // fixed by PPhD for AMS1 compat
        *base = mem_get_long_ptr.expect("non-null function pointer")(ptr)
    } else if 0 != tihw.ti92v2 {
        // TI-92 II
        let mut ptr_0: uint32_t =
            (0x4720i32 + 0x1902i32) as
                uint32_t; // tios::main_lcd�equ�tios::globals+$0000
        *base = mem_get_long_ptr.expect("non-null function pointer")(ptr_0)
    } else if 0 != tihw.ti92v1 {
        // TI-92 I
        let mut ptr_1: uint32_t =
            (0x4440i32 + 0x1902i32) as
                uint32_t; // and tios::heap equ tios::globals+$1902
        *base = mem_get_long_ptr.expect("non-null function pointer")(ptr_1)
    } else {
        let mut b: uint32_t = 0;
        let mut size: uint32_t = 0;
        let mut addr: uint32_t = 0;
        let mut ptr_2: uint32_t = 0;
        romcalls_get_table_infos(&mut b, &mut size);
        if size < 0x441i32 as cty::c_uint && 0 == pedrom {
            // AMS 1
            romcalls_get_symbol_address(0x96i32,
                                        &mut addr); // AMS 2, PedroM >=0.81
            ptr_2 =
                mem_get_word_ptr.expect("non-null function pointer")(addr.wrapping_add(8i32
                                                                                           as
                                                                                           cty::c_uint))
                    as uint32_t;
            *base =
                mem_get_long_ptr.expect("non-null function pointer")(ptr_2)
        } else { // tios::HeapDeref (#0x096)
            // MOVEA.W $7592,A0
            romcalls_get_symbol_address(0x441i32,
                                        &mut addr); // tios::HeapTable	(#0x441)
            *base = addr
        }
    };
}
// aliases
/*
    Get address of an allocated block (like HeapDeref)
*/
#[no_mangle]
pub unsafe extern "C" fn heap_deref(mut handle: cty::c_int) -> uint32_t {
    let mut base: uint32_t = 0;
    heap_get_addr(&mut base);
    return mem_get_long_ptr.expect("non-null function pointer")(base.wrapping_add((4i32
                                                                                       *
                                                                                       handle)
                                                                                      as
                                                                                      cty::c_uint));
}
#[no_mangle]
pub unsafe extern "C" fn heap_get_block_addr(mut handle: cty::c_int,
                                             mut addr: *mut uint32_t) {
    *addr = heap_deref(handle);
}
/*
    Get size of an allocated block (like HeapSize)
*/
#[no_mangle]
pub unsafe extern "C" fn heap_size(mut handle: cty::c_int) -> uint16_t {
    let mut base: uint32_t = 0; // remove lock
    let mut addr: uint32_t = 0; // size is twice
    let mut size: uint16_t = 0;
    heap_get_addr(&mut base);
    addr =
        mem_get_long_ptr.expect("non-null function pointer")(base.wrapping_add((4i32
                                                                                    *
                                                                                    handle)
                                                                                   as
                                                                                   cty::c_uint));
    if 0 == pedrom {
        size =
            mem_get_word_ptr.expect("non-null function pointer")(addr.wrapping_sub(2i32
                                                                                       as
                                                                                       cty::c_uint));
        size = (size as cty::c_int & !(1i32 << 16i32)) as uint16_t;
        size = ((size as cty::c_int) << 1i32) as uint16_t;
        size = (size as cty::c_int - 2i32) as uint16_t
    } else if addr >= tihw.rom_base {
        // archived file on PedroM - use file size
        size = mem_get_word_ptr.expect("non-null function pointer")(addr);
        size = (size as cty::c_int + 2i32) as uint16_t
    } else {
        size =
            mem_get_long_ptr.expect("non-null function pointer")(addr.wrapping_sub(6i32
                                                                                       as
                                                                                       cty::c_uint))
                as uint16_t;
        size = (size as cty::c_int - 6i32) as uint16_t
    }
    return size;
}
/*
    Given an handle, retrieve block size and block address
*/
#[no_mangle]
pub unsafe extern "C" fn heap_get_block_addr_and_size(mut handle: cty::c_int,
                                                      mut addr: *mut uint32_t,
                                                      mut size:
                                                          *mut uint16_t) {
    *addr = heap_deref(handle);
    *size = heap_size(handle);
}
/*
    Walk in the heap to search for a block address.
*/
#[no_mangle]
pub unsafe extern "C" fn heap_search_for_address(mut address: uint32_t,
                                                 mut handle:
                                                     *mut cty::c_int) {
    let mut base: uint32_t = 0;
    let mut i: cty::c_int = 0;
    heap_get_addr(&mut base);
    i = 1i32;
    while i < 2000i32 {
        let mut addr: uint32_t =
            mem_get_long_ptr.expect("non-null function pointer")(base.wrapping_add((4i32
                                                                                        *
                                                                                        i)
                                                                                       as
                                                                                       cty::c_uint));
        let mut size: uint16_t =
            mem_get_word_ptr.expect("non-null function pointer")(addr.wrapping_sub(2i32
                                                                                       as
                                                                                       cty::c_uint));
        if 0 != addr && address >= addr &&
               address < addr.wrapping_add(size as cty::c_uint) {
            *handle = i
        }
        i += 1
    }
    *handle = -1i32;
}
//#define HeapDeref(handle) HeapTable[handle]
//#define HeapSize(handle) ((short*)HeapDeref(handle))[-1]<<1;
