#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
use crate::libc::*;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TiemuInstPaths {
    pub base_dir: *mut cty::c_char,
    pub locale_dir: *mut cty::c_char,
    pub manpage_dir: *mut cty::c_char,
    pub help_dir: *mut cty::c_char,
    pub pixmap_dir: *mut cty::c_char,
    pub img_dir: *mut cty::c_char,
    pub skin_dir: *mut cty::c_char,
    pub glade_dir: *mut cty::c_char,
    pub home_dir: *mut cty::c_char,
    pub rom_dir: *mut cty::c_char,
    pub misc_dir: *mut cty::c_char,
    pub screen_dir: *mut cty::c_char,
    pub current_dir: [cty::c_char; 1024],
}
/* Hey EMACS -*- linux-c -*- */
/* $Id: paths.c 2268 2006-11-06 17:18:51Z roms $ */
/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin, Kevin Kofler
 *  Copyright (c) 2005, Christian Walther (patches for Mac OS-X port)
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
    Platform independant paths
*/
#[no_mangle]
pub static mut inst_paths: TiemuInstPaths =
    TiemuInstPaths{base_dir: 0 as *const cty::c_char as *mut cty::c_char,
                   locale_dir: 0 as *const cty::c_char as *mut cty::c_char,
                   manpage_dir: 0 as *const cty::c_char as *mut cty::c_char,
                   help_dir: 0 as *const cty::c_char as *mut cty::c_char,
                   pixmap_dir: 0 as *const cty::c_char as *mut cty::c_char,
                   img_dir: 0 as *const cty::c_char as *mut cty::c_char,
                   skin_dir: 0 as *const cty::c_char as *mut cty::c_char,
                   glade_dir: 0 as *const cty::c_char as *mut cty::c_char,
                   home_dir: 0 as *const cty::c_char as *mut cty::c_char,
                   rom_dir: 0 as *const cty::c_char as *mut cty::c_char,
                   misc_dir: 0 as *const cty::c_char as *mut cty::c_char,
                   screen_dir: 0 as *const cty::c_char as *mut cty::c_char,
                   current_dir: [0; 1024],};
/*
  Functions
*/
// int build_home_path(char **path, char *fileaname);
// installation paths
/*
    Called by TiEmu at startup for initializing platform dependant paths.
*/
/*  */
/*
    Same for Win32
*/
/*  */
#[no_mangle]
pub unsafe extern "C" fn initialize_paths() -> cty::c_int { return 0i32; }
