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
    fn strlen(_: *const cty::c_char) -> cty::c_ulong;
    #[no_mangle]
    fn malloc(_: cty::c_ulong) -> *mut cty::c_void;
    #[no_mangle]
    fn memcpy(_: *mut cty::c_void, _: *const cty::c_void, _: cty::c_ulong)
     -> *mut cty::c_void;
    #[no_mangle]
    fn strrchr(_: *const cty::c_char, _: cty::c_int) -> *mut cty::c_char;
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
pub type __uint16_t = cty::c_ushort;
pub type __uint32_t = cty::c_uint;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = cty::c_ulong;
pub type size_t = cty::c_ulong;
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn GUINT16_FROM_BE(mut num: uint16_t) -> uint16_t {
    return ((num as cty::c_int & 0xffi32) >> 8i32 |
                (num as cty::c_int) << 8i32) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn GUINT32_FROM_BE(mut num: uint32_t) -> uint32_t {
    return (num & 0xff000000u32) >> 24i32 |
               (num & 0xff0000i32 as cty::c_uint) >> 8i32 |
               (num & 0xff00i32 as cty::c_uint) << 8i32 | num << 24i32;
}
#[no_mangle]
pub unsafe extern "C" fn g_stpcpy(mut dest: *mut cty::c_char,
                                  mut src: *const cty::c_char)
 -> *mut cty::c_char {
    let mut d: *mut cty::c_char = dest;
    let mut s: *const cty::c_char = src;
    loop  {
        let fresh0 = d;
        d = d.offset(1);
        *fresh0 = *s;
        let fresh1 = s;
        s = s.offset(1);
        if !(*fresh1 as cty::c_int != '\u{0}' as i32) { break ; }
    }
    return d.offset(-1isize);
}
#[no_mangle]
pub unsafe extern "C" fn g_strconcat(mut string1: *const cty::c_char,
                                     mut args: ...) -> *mut cty::c_char {
    let mut l: uintptr_t = 0;
    let mut s: *mut cty::c_char = 0 as *mut cty::c_char;
    let mut concat: *mut cty::c_char = 0 as *mut cty::c_char;
    let mut ptr: *mut cty::c_char = 0 as *mut cty::c_char;
    if string1.is_null() { return 0 as *mut cty::c_char }
    l = (1i32 as cty::c_ulong).wrapping_add(strlen(string1));
    s = args.as_va_list().arg::<*mut cty::c_char>();
    while !s.is_null() {
        l =
            (l as cty::c_ulong).wrapping_add(strlen(s)) as uintptr_t as
                uintptr_t;
        s = args.as_va_list().arg::<*mut cty::c_char>()
    }
    concat = malloc(l) as *mut cty::c_char;
    ptr = concat;
    ptr = g_stpcpy(ptr, string1);
    s = args.as_va_list().arg::<*mut cty::c_char>();
    while !s.is_null() {
        ptr = g_stpcpy(ptr, s);
        s = args.as_va_list().arg::<*mut cty::c_char>()
    }
    return concat;
}
#[no_mangle]
pub unsafe extern "C" fn _g_gnulib_vasprintf(mut result:
                                                 *mut *mut cty::c_char,
                                             mut format: *const cty::c_char,
                                             mut args_0: ::core::ffi::VaList)
 -> cty::c_int {
    let mut length: size_t = 0;
    *result =
        vasnprintf(0 as *mut _, &mut length as *mut size_t as _, format,
                   args_0.as_va_list()) as *mut cty::c_char;
    if (*result).is_null() { return -1i32 }
    return length as cty::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn g_vasprintf(mut string: *mut *mut cty::c_char,
                                     mut format: *const cty::c_char,
                                     mut args_0: ::core::ffi::VaList)
 -> cty::c_int {
    let mut len: cty::c_int = 0;
    len = _g_gnulib_vasprintf(string, format, args_0.as_va_list());
    if len < 0i32 { *string = 0 as *mut cty::c_char }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn g_strdup_vprintf(mut format: *const cty::c_char,
                                          mut args_0: ::core::ffi::VaList)
 -> *mut cty::c_char {
    let mut string: *mut cty::c_char = 0 as *mut cty::c_char;
    g_vasprintf(&mut string, format, args_0.as_va_list());
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn g_strdup_printf(mut format: *const cty::c_char,
                                         mut args_0: ...)
 -> *mut cty::c_char {
    let mut buffer: *mut cty::c_char = 0 as *mut cty::c_char;
    buffer = g_strdup_vprintf(format, args_0.as_va_list());
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn g_strdup(mut str: *const cty::c_char)
 -> *mut cty::c_char {
    let mut new_str: *mut cty::c_char = 0 as *mut cty::c_char;
    let mut length: uintptr_t = 0;
    if !str.is_null() {
        length = strlen(str).wrapping_add(1i32 as cty::c_ulong);
        new_str = malloc(length) as *mut cty::c_char;
        memcpy(new_str as *mut cty::c_void, str as *const cty::c_void,
               length);
    } else { new_str = 0 as *mut cty::c_char }
    return new_str;
}
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
#[no_mangle]
pub unsafe extern "C" fn g_basename(mut file_name: *const cty::c_char)
 -> *const cty::c_char {
    let mut base: *mut cty::c_char = 0 as *mut cty::c_char;
    base = strrchr(file_name, '/' as i32);
    if !base.is_null() { return base.offset(1isize) }
    return file_name as *mut cty::c_char;
}
