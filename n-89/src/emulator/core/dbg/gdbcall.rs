#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
use crate::libc::*;
/* Hey EMACS -*- linux-c -*- */
/* $Id: gdbcall.h 2155 2006-07-20 19:53:06Z kevinkofler $ */
/*  TiEmu - a TI emulator
 *
 *  This file Copyright (c) 2005-2006, Kevin Kofler
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
    Gdbcall: GDB interfacing functions
*/
#[no_mangle]
pub unsafe extern "C" fn gdbcall_run() { }
#[no_mangle]
pub unsafe extern "C" fn gdbcall_continue() { }
#[no_mangle]
pub unsafe extern "C" fn gdb_add_symbol_file(mut filename:
                                                 *const cty::c_char,
                                             mut address: cty::c_uint) {
}
#[no_mangle]
pub unsafe extern "C" fn gdb_hbreak(mut funcname: *const cty::c_char) { }
