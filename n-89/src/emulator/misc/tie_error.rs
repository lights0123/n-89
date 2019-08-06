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
    fn strdup(_: *const cty::c_char) -> *mut cty::c_char;
    #[no_mangle]
    fn tiemu_warning(format: *const cty::c_char, _: ...);
}
/* Hey EMACS -*- linux-c -*- */
/* $Id: tie_error.c 2372 2007-02-25 21:43:23Z roms $ */
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
  This function can take 2 parameters:
  - an error to translate or 0
  - a pure message or NULL
 */
#[no_mangle]
pub unsafe extern "C" fn tiemu_err(mut err_code: cty::c_int,
                                   mut err_str: *mut cty::c_char)
 -> cty::c_int {
    let mut s: *mut cty::c_char = 0 as *mut cty::c_char;
    if 0 == err_code && err_str.is_null() { return 0 }
    if !err_str.is_null() { s = strdup(err_str) }
    tiemu_warning(b"%s\x00" as *const u8 as *const cty::c_char, s);
    // free(s);
    err_code
}
