#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
use crate::libc::*;
/* Hey EMACS -*- linux-c -*- */
/* $Id: error.c 2378 2007-02-26 18:07:09Z roms $ */
/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005-2006, Romain Li�vin
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
    Transcoding of error codes into message strings
*/
/*
   This function put in err_msg the error message corresponding to the
   error code.
   If the error code has been handled, the function returns 0 else it
   propagates the error code by returning it.
*/
#[no_mangle]
pub unsafe extern "C" fn ti68k_error_get(mut err_num: cty::c_int,
                                         mut error_msg:
                                             *mut *mut cty::c_char)
 -> cty::c_int {
    match err_num {
        0 => {
            *error_msg =
                g_strdup(b"No error.\x00" as *const u8 as *const cty::c_char)
                    as *mut cty::c_char
        }
        768 => {
            *error_msg =
                g_strdup(b"Can not open file.\x00" as *const u8 as
                             *const cty::c_char) as *mut cty::c_char
        }
        780 => {
            *error_msg =
                g_strdup(b"Can not open state image: file is corrupted or missing.\x00"
                             as *const u8 as *const cty::c_char) as
                    *mut cty::c_char
        }
        781 => {
            *error_msg =
                g_strdup(b"Can not open state image: revision changed. You have to recreate the state image.\x00"
                             as *const u8 as *const cty::c_char) as
                    *mut cty::c_char
        }
        782 => {
            *error_msg =
                g_strdup(b"Can not open state image: state image header does not match ROM image header: have you changed/updated your ROM image?\x00"
                             as *const u8 as *const cty::c_char) as
                    *mut cty::c_char
        }
        783 => {
            *error_msg =
                g_strdup(b"Can not open state image: this state image is not targetted for your current emulator image (calculator model and/or OS version must match!). Choose another image before.\x00"
                             as *const u8 as *const cty::c_char) as
                    *mut cty::c_char
        }
        770 => {
            *error_msg =
                g_strdup(b"Invalid emulator image. File is corrupted or revision changed.\x00"
                             as *const u8 as *const cty::c_char) as
                    *mut cty::c_char
        }
        771 => {
            *error_msg =
                g_strdup(b"Invalid FLASH upgrade.\x00" as *const u8 as
                             *const cty::c_char) as *mut cty::c_char
        }
        779 => {
            *error_msg =
                g_strdup(b"Invalid ROM dump.\x00" as *const u8 as
                             *const cty::c_char) as *mut cty::c_char
        }
        772 => {
            *error_msg =
                g_strdup(b"No image.\x00" as *const u8 as *const cty::c_char)
                    as *mut cty::c_char
        }
        774 => {
            *error_msg =
                g_strdup(b"ROM dump has a weird size.\x00" as *const u8 as
                             *const cty::c_char) as *mut cty::c_char
        }
        775 => {
            *error_msg =
                g_strdup(b"This is not recognized as a TI file.\x00" as
                             *const u8 as *const cty::c_char) as
                    *mut cty::c_char
        }
        777 => {
            *error_msg =
                g_strdup(b"Can\'t parse folder.\x00" as *const u8 as
                             *const cty::c_char) as *mut cty::c_char
        }
        778 => {
            *error_msg =
                g_strdup(b"Can\'t upgrade calculator.\x00" as *const u8 as
                             *const cty::c_char) as *mut cty::c_char
        }
        _ => {
            *error_msg =
                g_strdup(b"Error code not found in the list.\nThis is a bug. Please report it.\x00"
                             as *const u8 as *const cty::c_char) as
                    *mut cty::c_char;
            return err_num
        }
    }
    return 0i32;
}
