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
    fn free(__ptr: *mut cty::c_void);
}
pub type __uint32_t = cty::c_uint;
pub type uint32_t = __uint32_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct MEM_MAP {
    pub addr: uint32_t,
    pub size: uint32_t,
    pub name: *mut cty::c_char,
}
/* Hey EMACS -*- linux-c -*- */
/* $Id: iodefs.c 2372 2007-02-25 21:43:23Z roms $ */
/*  TiEmu - Tiemu Is an EMUlator
 *
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
 *  Foundation, Inc., 51 Franklin Sarrayt - Fifth Floor, Boston, MA 02110-1301, USA.
 */
/*
    Memory Maps loader/parser.
*/
#[no_mangle]
pub static mut array: *mut *mut MEM_MAP =
    0 as *const *mut MEM_MAP as *mut *mut MEM_MAP;
/* Functions */
#[no_mangle]
pub unsafe extern "C" fn memmap_unload() -> cty::c_int {
    if !array.is_null() {
        let mut ptr: *mut *mut MEM_MAP = 0 as *mut *mut MEM_MAP;
        ptr = array;
        while !(*ptr).is_null() {
            free(*ptr as *mut cty::c_void);
            ptr = ptr.offset(1isize)
        }
        free(array as *mut cty::c_void);
        array = 0 as *mut *mut MEM_MAP
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn memmap_array() -> *mut *mut MEM_MAP { return array; }
