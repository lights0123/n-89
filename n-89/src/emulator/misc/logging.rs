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
    fn printf(_: *const cty::c_char, _: ...) -> cty::c_int;
    #[no_mangle]
    fn vprintf(_: *const cty::c_char, _: ::core::ffi::VaList) -> cty::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: cty::c_uint,
    pub fp_offset: cty::c_uint,
    pub overflow_arg_area: *mut cty::c_void,
    pub reg_save_area: *mut cty::c_void,
}
pub type va_list = __builtin_va_list;
/* printf */
#[no_mangle]
pub unsafe extern "C" fn tiemu_debug(mut format: *const cty::c_char,
                                     mut args: ...) {
    printf(b"[DBG] : \x00" as *const u8 as *const cty::c_char);
    vprintf(format, args.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn tiemu_info(mut format: *const cty::c_char,
                                    mut args_0: ...) {
    printf(b"[INFO]: \x00" as *const u8 as *const cty::c_char);
    vprintf(format, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn tiemu_message(mut format: *const cty::c_char,
                                       mut args_1: ...) {
    printf(b"[MSG] : \x00" as *const u8 as *const cty::c_char);
    vprintf(format, args_1.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn tiemu_warning(mut format: *const cty::c_char,
                                       mut args_2: ...) {
    printf(b"[WARN]: \x00" as *const u8 as *const cty::c_char);
    vprintf(format, args_2.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn tiemu_critical(mut format: *const cty::c_char,
                                        mut args_3: ...) {
    printf(b"[CRIT]: \x00" as *const u8 as *const cty::c_char);
    vprintf(format, args_3.as_va_list());
}
/* Hey EMACS -*- linux-c -*- */
/* $Id: print.h 522 2004-04-08 10:12:55Z roms $ */
/*  TiEmu - Tiemu Is an EMUlator
 *  Copyright (C) 2007  Romain Lievin
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
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
 */
/*
    Domain name logging.
*/
#[no_mangle]
pub unsafe extern "C" fn tiemu_error(mut format: *const cty::c_char,
                                     mut args_4: ...) {
    printf(b"[ERR] : \x00" as *const u8 as *const cty::c_char);
    vprintf(format, args_4.as_va_list());
}
