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
    static mut mem_get_byte_ptr: GETBYTE_FUNC;
    #[no_mangle]
    static mut mem_put_byte_ptr: PUTBYTE_FUNC;
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
    fn GUINT16_FROM_BE(num: uint16_t) -> uint16_t;
    #[no_mangle]
    fn GUINT32_FROM_BE(num: uint32_t) -> uint32_t;
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __uint32_t = cty::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type PUTBYTE_FUNC
    =
    Option<unsafe extern "C" fn(_: uint32_t, _: uint8_t) -> ()>;
/* Hey EMACS -*- linux-c -*- */
/* $Id: timem.c 2268 2006-11-06 17:18:51Z roms $ */
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
    Memory access (take care of little/big endian issues)
*/
/* General functions (not related to TI memory) */
#[no_mangle]
pub unsafe extern "C" fn rd_word(mut p: *mut uint8_t) -> uint16_t {
    let mut p16: *mut uint16_t = p as *mut uint16_t;
    return GUINT16_FROM_BE(*p16);
}
#[no_mangle]
pub unsafe extern "C" fn rd_long(mut p: *mut uint8_t) -> uint32_t {
    let mut p32: *mut uint32_t = p as *mut uint32_t;
    return GUINT32_FROM_BE(*p32);
}
#[no_mangle]
pub unsafe extern "C" fn wr_word(mut p: *mut uint8_t, mut d: uint16_t) {
    let mut p16: *mut uint16_t = p as *mut uint16_t;
    *p16 = GUINT16_FROM_BE(d);
}
#[no_mangle]
pub unsafe extern "C" fn wr_long(mut p: *mut uint8_t, mut d: uint32_t) {
    let mut p32: *mut uint32_t = p as *mut uint32_t;
    *p32 = GUINT32_FROM_BE(d);
}
/* Memory access functions (TI memory) */
#[no_mangle]
pub unsafe extern "C" fn mem_rd_block(mut a: uint32_t, mut d: *mut uint8_t,
                                      mut len: uint16_t) {
    let mut i: cty::c_int = 0;
    i = 0i32;
    while i < len as cty::c_int {
        *d.offset(i as isize) =
            mem_get_byte_ptr.expect("non-null function pointer")(a.wrapping_add(i
                                                                                    as
                                                                                    cty::c_uint));
        i += 1
    };
}
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
pub unsafe extern "C" fn mem_wr_block(mut a: uint32_t, mut d: *mut uint8_t,
                                      mut len: uint16_t) {
    let mut i: cty::c_int = 0;
    i = 0i32;
    while i < len as cty::c_int {
        mem_put_byte_ptr.expect("non-null function pointer")(a.wrapping_add(i
                                                                                as
                                                                                cty::c_uint),
                                                             *d.offset(i as
                                                                           isize));
        i += 1
    };
}
