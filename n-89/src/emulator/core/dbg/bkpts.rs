#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
use crate::libc::*;
/* Hey EMACS -*- linux-c -*- */
/* $Id: bkpts.c 2770 2008-03-16 17:42:13Z roms $ */
/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin, Kevin Kofler
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
 *  Foundation, Inc., 51 Franklin Street - Fifth Floor, Boston, MA 02110-1301, USA.
 */
/*
    Breakpoints management
    Note: addresses are 24 bits but arguments are 32 bits. The MSB is used to store
    extra informations and save speed for comparison.
    Searching does not take into account the MSB (24 bits).
*/
/* Clear */
#[no_mangle]
pub unsafe extern "C" fn ti68k_bkpt_clear_address() { }
#[no_mangle]
pub unsafe extern "C" fn ti68k_bkpt_clear_access() { }
#[no_mangle]
pub unsafe extern "C" fn ti68k_bkpt_clear_range() { }
#[no_mangle]
pub unsafe extern "C" fn ti68k_bkpt_clear_exception() { }
#[no_mangle]
pub unsafe extern "C" fn ti68k_bkpt_clear_pgmentry() { }
