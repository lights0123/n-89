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
    #[no_mangle]
    static mut tihw: Ti68kHardware;
    #[no_mangle]
    fn hw_m68k_irq(n: cty::c_int);
    /*  TiEmu - a TI calculator emulator
 *
 *  Character to key conversion routine
 *  Copyright (c) 2006 Kevin Kofler
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
    fn chars_to_keys(chars: *const cty::c_char, ti89: cty::c_int)
     -> *mut cty::c_int;
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __uint32_t = cty::c_uint;
pub type __time_t = cty::c_long;
pub type time_t = __time_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
/* Hey EMACS -*- linux-c -*- */
/* $Id: hw.h 1455 2005-05-31 18:38:03Z roms $ */
/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TTIME {
    pub s: time_t,
    pub ms: cty::c_int,
}
// If this structure is modified, the SAV_REVISION number (state.c)
// has to be incremented.
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
    // RAM page execution protection bitmask
}
pub type TiKey = cty::c_uint;
pub const MAX_TIKEYS: TiKey = 85;
pub const TIKEY_VOID: TiKey = 84;
pub const TIKEY_PIPE: TiKey = 83;
pub const TIKEY_HOME: TiKey = 82;
pub const TIKEY_CATALOG: TiKey = 81;
pub const TIKEY_EE: TiKey = 80;
pub const TIKEY_ALPHA: TiKey = 79;
pub const TIKEY_ON: TiKey = 78;
pub const TIKEY_MINUS: TiKey = 77;
pub const TIKEY_ENTER1: TiKey = 76;
pub const TIKEY_A: TiKey = 75;
pub const TIKEY_Q: TiKey = 74;
pub const TIKEY_F4: TiKey = 73;
pub const TIKEY_0: TiKey = 72;
pub const TIKEY_PERIOD: TiKey = 71;
pub const TIKEY_NEGATE: TiKey = 70;
pub const TIKEY_BACKSPACE: TiKey = 69;
pub const TIKEY_THETA: TiKey = 68;
pub const TIKEY_L: TiKey = 67;
pub const TIKEY_O: TiKey = 66;
pub const TIKEY_PLUS: TiKey = 65;
pub const TIKEY_MODE: TiKey = 64;
pub const TIKEY_ESCAPE: TiKey = 63;
pub const TIKEY_NU: TiKey = 62;
pub const TIKEY_EQUALS: TiKey = 61;
pub const TIKEY_M: TiKey = 60;
pub const TIKEY_K: TiKey = 59;
pub const TIKEY_I: TiKey = 58;
pub const TIKEY_F5: TiKey = 57;
pub const TIKEY_CLEAR: TiKey = 56;
pub const TIKEY_APPS: TiKey = 55;
pub const TIKEY_MULTIPLY: TiKey = 54;
pub const TIKEY_POWER: TiKey = 53;
pub const TIKEY_N: TiKey = 52;
pub const TIKEY_J: TiKey = 51;
pub const TIKEY_U: TiKey = 50;
pub const TIKEY_F1: TiKey = 49;
pub const TIKEY_LN: TiKey = 48;
pub const TIKEY_ENTER2: TiKey = 47;
pub const TIKEY_P: TiKey = 46;
pub const TIKEY_DIVIDE: TiKey = 45;
pub const TIKEY_B: TiKey = 44;
pub const TIKEY_H: TiKey = 43;
pub const TIKEY_Y: TiKey = 42;
pub const TIKEY_F6: TiKey = 41;
pub const TIKEY_SIN: TiKey = 40;
pub const TIKEY_COS: TiKey = 39;
pub const TIKEY_TAN: TiKey = 38;
pub const TIKEY_SPACE: TiKey = 37;
pub const TIKEY_V: TiKey = 36;
pub const TIKEY_G: TiKey = 35;
pub const TIKEY_T: TiKey = 34;
pub const TIKEY_F2: TiKey = 33;
pub const TIKEY_PALEFT: TiKey = 32;
pub const TIKEY_PARIGHT: TiKey = 31;
pub const TIKEY_COMMA: TiKey = 30;
pub const TIKEY_STORE: TiKey = 29;
pub const TIKEY_C: TiKey = 28;
pub const TIKEY_F: TiKey = 27;
pub const TIKEY_R: TiKey = 26;
pub const TIKEY_F7: TiKey = 25;
pub const TIKEY_7: TiKey = 24;
pub const TIKEY_8: TiKey = 23;
pub const TIKEY_9: TiKey = 22;
pub const TIKEY_X: TiKey = 21;
pub const TIKEY_D: TiKey = 20;
pub const TIKEY_E: TiKey = 19;
pub const TIKEY_F3: TiKey = 18;
pub const TIKEY_4: TiKey = 17;
pub const TIKEY_5: TiKey = 16;
pub const TIKEY_6: TiKey = 15;
pub const TIKEY_Z: TiKey = 14;
pub const TIKEY_S: TiKey = 13;
pub const TIKEY_W: TiKey = 12;
pub const TIKEY_F8: TiKey = 11;
pub const TIKEY_1: TiKey = 10;
pub const TIKEY_2: TiKey = 9;
pub const TIKEY_3: TiKey = 8;
pub const TIKEY_2ND: TiKey = 7;
pub const TIKEY_DIAMOND: TiKey = 6;
pub const TIKEY_SHIFT: TiKey = 5;
pub const TIKEY_HAND: TiKey = 4;
pub const TIKEY_LEFT: TiKey = 3;
pub const TIKEY_UP: TiKey = 2;
pub const TIKEY_RIGHT: TiKey = 1;
pub const TIKEY_DOWN: TiKey = 0;
/* Hey EMACS -*- linux-c -*- */
/* $Id: kbd.c 2268 2006-11-06 17:18:51Z roms $ */
/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin
 *  Copyright (c) 2006, Kevin Kofler
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
    Keyboard management
*/
static mut key_states: [TiKey; 120] = [TIKEY_DOWN; 120];
static mut key_row: *mut cty::c_int =
    0 as *const cty::c_int as *mut cty::c_int;
static mut key_change: cty::c_int = 0;
static mut on_change: cty::c_int = 0;
static mut key_buffer: *mut cty::c_int =
    0 as *const cty::c_int as *mut cty::c_int;
static mut key_buffer_ptr: *mut cty::c_int =
    0 as *const cty::c_int as *mut cty::c_int;
static mut key_buffer_state: cty::c_int = 0;
#[no_mangle]
pub static mut keyRow92: [[cty::c_int; 8]; 10] =
    [[TIKEY_DOWN as cty::c_int, TIKEY_RIGHT as cty::c_int,
      TIKEY_UP as cty::c_int, TIKEY_LEFT as cty::c_int,
      TIKEY_HAND as cty::c_int, TIKEY_SHIFT as cty::c_int,
      TIKEY_DIAMOND as cty::c_int, TIKEY_2ND as cty::c_int],
     [TIKEY_3 as cty::c_int, TIKEY_2 as cty::c_int, TIKEY_1 as cty::c_int,
      TIKEY_F8 as cty::c_int, TIKEY_W as cty::c_int, TIKEY_S as cty::c_int,
      TIKEY_Z as cty::c_int, TIKEY_VOID as cty::c_int],
     [TIKEY_6 as cty::c_int, TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int,
      TIKEY_F3 as cty::c_int, TIKEY_E as cty::c_int, TIKEY_D as cty::c_int,
      TIKEY_X as cty::c_int, TIKEY_VOID as cty::c_int],
     [TIKEY_9 as cty::c_int, TIKEY_8 as cty::c_int, TIKEY_7 as cty::c_int,
      TIKEY_F7 as cty::c_int, TIKEY_R as cty::c_int, TIKEY_F as cty::c_int,
      TIKEY_C as cty::c_int, TIKEY_STORE as cty::c_int],
     [TIKEY_COMMA as cty::c_int, TIKEY_PARIGHT as cty::c_int,
      TIKEY_PALEFT as cty::c_int, TIKEY_F2 as cty::c_int,
      TIKEY_T as cty::c_int, TIKEY_G as cty::c_int, TIKEY_V as cty::c_int,
      TIKEY_SPACE as cty::c_int],
     [TIKEY_TAN as cty::c_int, TIKEY_COS as cty::c_int,
      TIKEY_SIN as cty::c_int, TIKEY_F6 as cty::c_int,
      TIKEY_Y as cty::c_int, TIKEY_H as cty::c_int, TIKEY_B as cty::c_int,
      TIKEY_DIVIDE as cty::c_int],
     [TIKEY_P as cty::c_int, TIKEY_ENTER2 as cty::c_int,
      TIKEY_LN as cty::c_int, TIKEY_F1 as cty::c_int,
      TIKEY_U as cty::c_int, TIKEY_J as cty::c_int, TIKEY_N as cty::c_int,
      TIKEY_POWER as cty::c_int],
     [TIKEY_MULTIPLY as cty::c_int, TIKEY_APPS as cty::c_int,
      TIKEY_CLEAR as cty::c_int, TIKEY_F5 as cty::c_int,
      TIKEY_I as cty::c_int, TIKEY_K as cty::c_int, TIKEY_M as cty::c_int,
      TIKEY_EQUALS as cty::c_int],
     [TIKEY_NU as cty::c_int, TIKEY_ESCAPE as cty::c_int,
      TIKEY_MODE as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_O as cty::c_int, TIKEY_L as cty::c_int,
      TIKEY_THETA as cty::c_int, TIKEY_BACKSPACE as cty::c_int],
     [TIKEY_NEGATE as cty::c_int, TIKEY_PERIOD as cty::c_int,
      TIKEY_0 as cty::c_int, TIKEY_F4 as cty::c_int, TIKEY_Q as cty::c_int,
      TIKEY_A as cty::c_int, TIKEY_ENTER1 as cty::c_int,
      TIKEY_MINUS as cty::c_int]];
#[no_mangle]
pub static mut keyRow89: [[cty::c_int; 8]; 10] =
    [[TIKEY_ALPHA as cty::c_int, TIKEY_DIAMOND as cty::c_int,
      TIKEY_SHIFT as cty::c_int, TIKEY_2ND as cty::c_int,
      TIKEY_RIGHT as cty::c_int, TIKEY_DOWN as cty::c_int,
      TIKEY_LEFT as cty::c_int, TIKEY_UP as cty::c_int],
     [TIKEY_F5 as cty::c_int, TIKEY_CLEAR as cty::c_int,
      TIKEY_POWER as cty::c_int, TIKEY_DIVIDE as cty::c_int,
      TIKEY_MULTIPLY as cty::c_int, TIKEY_MINUS as cty::c_int,
      TIKEY_PLUS as cty::c_int, TIKEY_ENTER1 as cty::c_int],
     [TIKEY_F4 as cty::c_int, TIKEY_BACKSPACE as cty::c_int,
      TIKEY_T as cty::c_int, TIKEY_COMMA as cty::c_int,
      TIKEY_9 as cty::c_int, TIKEY_6 as cty::c_int, TIKEY_3 as cty::c_int,
      TIKEY_NEGATE as cty::c_int],
     [TIKEY_F3 as cty::c_int, TIKEY_CATALOG as cty::c_int,
      TIKEY_Z as cty::c_int, TIKEY_PARIGHT as cty::c_int,
      TIKEY_8 as cty::c_int, TIKEY_5 as cty::c_int, TIKEY_2 as cty::c_int,
      TIKEY_PERIOD as cty::c_int],
     [TIKEY_F2 as cty::c_int, TIKEY_MODE as cty::c_int,
      TIKEY_Y as cty::c_int, TIKEY_PALEFT as cty::c_int,
      TIKEY_7 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_1 as cty::c_int,
      TIKEY_0 as cty::c_int],
     [TIKEY_F1 as cty::c_int, TIKEY_HOME as cty::c_int,
      TIKEY_X as cty::c_int, TIKEY_EQUALS as cty::c_int,
      TIKEY_PIPE as cty::c_int, TIKEY_EE as cty::c_int,
      TIKEY_STORE as cty::c_int, TIKEY_APPS as cty::c_int],
     [TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int,
      TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int,
      TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int,
      TIKEY_VOID as cty::c_int, TIKEY_ESCAPE as cty::c_int],
     [TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int,
      TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int,
      TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int,
      TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int],
     [TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int,
      TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int,
      TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int,
      TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int],
     [TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int,
      TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int,
      TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int,
      TIKEY_VOID as cty::c_int, TIKEY_VOID as cty::c_int]];
#[no_mangle]
pub static mut keyRowV200: [[cty::c_int; 8]; 10] =
    [[TIKEY_DOWN as cty::c_int, TIKEY_RIGHT as cty::c_int,
      TIKEY_UP as cty::c_int, TIKEY_LEFT as cty::c_int,
      TIKEY_HAND as cty::c_int, TIKEY_SHIFT as cty::c_int,
      TIKEY_DIAMOND as cty::c_int, TIKEY_2ND as cty::c_int],
     [TIKEY_3 as cty::c_int, TIKEY_2 as cty::c_int, TIKEY_1 as cty::c_int,
      TIKEY_F8 as cty::c_int, TIKEY_W as cty::c_int, TIKEY_S as cty::c_int,
      TIKEY_Z as cty::c_int, TIKEY_NU as cty::c_int],
     [TIKEY_6 as cty::c_int, TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int,
      TIKEY_F3 as cty::c_int, TIKEY_E as cty::c_int, TIKEY_D as cty::c_int,
      TIKEY_X as cty::c_int, TIKEY_NU as cty::c_int],
     [TIKEY_9 as cty::c_int, TIKEY_8 as cty::c_int, TIKEY_7 as cty::c_int,
      TIKEY_F7 as cty::c_int, TIKEY_R as cty::c_int, TIKEY_F as cty::c_int,
      TIKEY_C as cty::c_int, TIKEY_STORE as cty::c_int],
     [TIKEY_COMMA as cty::c_int, TIKEY_PARIGHT as cty::c_int,
      TIKEY_PALEFT as cty::c_int, TIKEY_F2 as cty::c_int,
      TIKEY_T as cty::c_int, TIKEY_G as cty::c_int, TIKEY_V as cty::c_int,
      TIKEY_SPACE as cty::c_int],
     [TIKEY_TAN as cty::c_int, TIKEY_COS as cty::c_int,
      TIKEY_SIN as cty::c_int, TIKEY_F6 as cty::c_int,
      TIKEY_Y as cty::c_int, TIKEY_H as cty::c_int, TIKEY_B as cty::c_int,
      TIKEY_DIVIDE as cty::c_int],
     [TIKEY_P as cty::c_int, TIKEY_ENTER2 as cty::c_int,
      TIKEY_LN as cty::c_int, TIKEY_F1 as cty::c_int,
      TIKEY_U as cty::c_int, TIKEY_J as cty::c_int, TIKEY_N as cty::c_int,
      TIKEY_POWER as cty::c_int],
     [TIKEY_MULTIPLY as cty::c_int, TIKEY_APPS as cty::c_int,
      TIKEY_CLEAR as cty::c_int, TIKEY_F5 as cty::c_int,
      TIKEY_I as cty::c_int, TIKEY_K as cty::c_int, TIKEY_M as cty::c_int,
      TIKEY_EQUALS as cty::c_int],
     [TIKEY_NU as cty::c_int, TIKEY_ESCAPE as cty::c_int,
      TIKEY_MODE as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_O as cty::c_int, TIKEY_L as cty::c_int,
      TIKEY_THETA as cty::c_int, TIKEY_BACKSPACE as cty::c_int],
     [TIKEY_NEGATE as cty::c_int, TIKEY_PERIOD as cty::c_int,
      TIKEY_0 as cty::c_int, TIKEY_F4 as cty::c_int, TIKEY_Q as cty::c_int,
      TIKEY_A as cty::c_int, TIKEY_ENTER1 as cty::c_int,
      TIKEY_MINUS as cty::c_int]];
#[no_mangle]
pub unsafe extern "C" fn hw_kbd_init() -> cty::c_int {
    let mut i: cty::c_int = 0;
    key_change = 0i32;
    on_change = 0i32;
    tihw.on_key = 0i32;
    match tihw.calc_type {
        2 | 16 => { key_row = keyRow89.as_mut_ptr() as *mut cty::c_int }
        1 | 4 => { key_row = keyRow92.as_mut_ptr() as *mut cty::c_int }
        8 => { key_row = keyRowV200.as_mut_ptr() as *mut cty::c_int }
        _ => { }
    }
    i = 0i32;
    while i < MAX_TIKEYS as cty::c_int {
        key_states[i as usize] = TIKEY_DOWN;
        i += 1
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn hw_kbd_reset() -> cty::c_int { return 0i32; }
#[no_mangle]
pub unsafe extern "C" fn hw_kbd_exit() -> cty::c_int {
    key_row = 0 as *mut cty::c_int;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_kbd_set_key(mut key: cty::c_int,
                                           mut active: cty::c_int) {
    if key == TIKEY_ALPHA as cty::c_int {
        if 0 != active {
            key_states[key as usize] += 1;
            key_change = (0 == 0i32) as cty::c_int
        } else { key_states[key as usize] -= 1 }
    } else if key == TIKEY_ON as cty::c_int {
        tihw.on_key = active;
        if 0 != active { on_change = (0 == 0i32) as cty::c_int }
    } else {
        key_states[key as usize] = active as TiKey;
        if 0 != active { key_change = (0 == 0i32) as cty::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_kbd_is_key_pressed(mut key: cty::c_int)
 -> cty::c_int {
    return key_states[key as usize] as cty::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn hw_kbd_update() -> cty::c_int
 // ~600Hz
 {
    // Push the keys we have been asked to push by ti68k_kbd_push_chars.
    if !key_buffer.is_null() {
        let mut key: cty::c_int = *key_buffer_ptr;
        if key == -1i32 {
            free(key_buffer as *mut cty::c_void);
            key_buffer = 0 as *mut cty::c_int;
            key_buffer_ptr = 0 as *mut cty::c_int;
            key_buffer_state = 0i32
        } else if key == TIKEY_VOID as cty::c_int {
            // give the calculator some time to react
            key_buffer_state += 1;
            if key_buffer_state == 30i32 {
                key_buffer_state = 0i32;
                key_buffer_ptr = key_buffer_ptr.offset(1isize)
            }
        } else {
            let fresh0 = key_buffer_state;
            key_buffer_state = key_buffer_state + 1;
            match fresh0 {
                0 => { ti68k_kbd_set_key(key, 1i32); }
                7 => { ti68k_kbd_set_key(key, 0i32); }
                16 => {
                    key_buffer_state = 0i32;
                    key_buffer_ptr = key_buffer_ptr.offset(1isize)
                }
                _ => { }
            }
        }
    }
    if 0 != key_change {
        // Auto-Int 2 is triggered when the first unmasked key is pressed. Keeping the key
		// pressed, or pressing another one without releasing the first key, will not generate
		// additional interrupts.
        if tihw.hw_type == 1i32 ||
               !(0 !=
                     *tihw.io2.offset(0x1fi32 as isize) as cty::c_int &
                         1i32 << 2i32 &&
                     0 ==
                         *tihw.io2.offset(0x1fi32 as isize) as cty::c_int &
                             1i32 << 1i32) {
            hw_m68k_irq(2i32);
        }
        key_change = 0i32
    }
    if on_change == 1i32 && 0 != tihw.on_key {
        // Auto-Int 6 triggered when [ON] is pressed.
        hw_m68k_irq(6i32);
        on_change = 0i32
    }
    return 0i32;
}
unsafe extern "C" fn get_rowmask(mut r: uint8_t) -> uint8_t {
    let mut rc: uint8_t = 0i32 as uint8_t;
    let mut i: cty::c_int = 0;
    let mut row: *mut cty::c_int =
        key_row.offset(((r as cty::c_int) << 3i32) as isize);
    i = 0i32;
    while i < 8i32 {
        rc =
            (rc as cty::c_uint |
                 (key_states[*row.offset(i as isize) as usize] as cty::c_uint
                      & 1i32 as cty::c_uint) << 7i32 - i) as uint8_t;
        i += 1
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn hw_kbd_read_cols() -> uint8_t {
    static mut i: uint8_t = 0;
    static mut arg: uint8_t = 0;
    static mut mask: uint16_t = 0;
    mask =
        ((*tihw.io.offset(0x18i32 as isize) as uint16_t as cty::c_int) <<
             8i32 | *tihw.io.offset(0x19i32 as isize) as cty::c_int) as
            uint16_t;
    i = 0i32 as uint8_t;
    arg = 0i32 as uint8_t;
    while (i as cty::c_int) < 10i32 {
        if 0 == mask as cty::c_int & 1i32 << i as cty::c_int {
            arg =
                (arg as cty::c_int | get_rowmask(i) as cty::c_int) as
                    uint8_t
        }
        i = i.wrapping_add(1)
    }
    return !(arg as cty::c_int) as uint8_t;
}
// "chars" is expected to be in the TI calculator charset.
// Only works (and returns TRUE) if the internal keyboard buffer was empty,
// otherwise returns FALSE.
#[no_mangle]
pub unsafe extern "C" fn ti68k_kbd_push_chars(mut chars: *const cty::c_char)
 -> cty::c_int {
    if !key_buffer.is_null() { return 0i32 }
    key_buffer =
        chars_to_keys(chars,
                      (tihw.calc_type == 1i32 << 1i32 ||
                           tihw.calc_type == 1i32 << 4i32) as cty::c_int);
    key_buffer_ptr = key_buffer;
    return 1i32;
}
