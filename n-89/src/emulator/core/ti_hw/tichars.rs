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
    fn malloc(_: cty::c_ulong) -> *mut cty::c_void;
    #[no_mangle]
    fn strlen(_: *const cty::c_char) -> cty::c_ulong;
}
pub type C2RustUnnamed = cty::c_uint;
pub const MAX_TIKEYS: C2RustUnnamed = 85;
pub const TIKEY_VOID: C2RustUnnamed = 84;
pub const TIKEY_PIPE: C2RustUnnamed = 83;
pub const TIKEY_HOME: C2RustUnnamed = 82;
pub const TIKEY_CATALOG: C2RustUnnamed = 81;
pub const TIKEY_EE: C2RustUnnamed = 80;
pub const TIKEY_ALPHA: C2RustUnnamed = 79;
pub const TIKEY_ON: C2RustUnnamed = 78;
pub const TIKEY_MINUS: C2RustUnnamed = 77;
pub const TIKEY_ENTER1: C2RustUnnamed = 76;
pub const TIKEY_A: C2RustUnnamed = 75;
pub const TIKEY_Q: C2RustUnnamed = 74;
pub const TIKEY_F4: C2RustUnnamed = 73;
pub const TIKEY_0: C2RustUnnamed = 72;
pub const TIKEY_PERIOD: C2RustUnnamed = 71;
pub const TIKEY_NEGATE: C2RustUnnamed = 70;
pub const TIKEY_BACKSPACE: C2RustUnnamed = 69;
pub const TIKEY_THETA: C2RustUnnamed = 68;
pub const TIKEY_L: C2RustUnnamed = 67;
pub const TIKEY_O: C2RustUnnamed = 66;
pub const TIKEY_PLUS: C2RustUnnamed = 65;
pub const TIKEY_MODE: C2RustUnnamed = 64;
pub const TIKEY_ESCAPE: C2RustUnnamed = 63;
pub const TIKEY_NU: C2RustUnnamed = 62;
pub const TIKEY_EQUALS: C2RustUnnamed = 61;
pub const TIKEY_M: C2RustUnnamed = 60;
pub const TIKEY_K: C2RustUnnamed = 59;
pub const TIKEY_I: C2RustUnnamed = 58;
pub const TIKEY_F5: C2RustUnnamed = 57;
pub const TIKEY_CLEAR: C2RustUnnamed = 56;
pub const TIKEY_APPS: C2RustUnnamed = 55;
pub const TIKEY_MULTIPLY: C2RustUnnamed = 54;
pub const TIKEY_POWER: C2RustUnnamed = 53;
pub const TIKEY_N: C2RustUnnamed = 52;
pub const TIKEY_J: C2RustUnnamed = 51;
pub const TIKEY_U: C2RustUnnamed = 50;
pub const TIKEY_F1: C2RustUnnamed = 49;
pub const TIKEY_LN: C2RustUnnamed = 48;
pub const TIKEY_ENTER2: C2RustUnnamed = 47;
pub const TIKEY_P: C2RustUnnamed = 46;
pub const TIKEY_DIVIDE: C2RustUnnamed = 45;
pub const TIKEY_B: C2RustUnnamed = 44;
pub const TIKEY_H: C2RustUnnamed = 43;
pub const TIKEY_Y: C2RustUnnamed = 42;
pub const TIKEY_F6: C2RustUnnamed = 41;
pub const TIKEY_SIN: C2RustUnnamed = 40;
pub const TIKEY_COS: C2RustUnnamed = 39;
pub const TIKEY_TAN: C2RustUnnamed = 38;
pub const TIKEY_SPACE: C2RustUnnamed = 37;
pub const TIKEY_V: C2RustUnnamed = 36;
pub const TIKEY_G: C2RustUnnamed = 35;
pub const TIKEY_T: C2RustUnnamed = 34;
pub const TIKEY_F2: C2RustUnnamed = 33;
pub const TIKEY_PALEFT: C2RustUnnamed = 32;
pub const TIKEY_PARIGHT: C2RustUnnamed = 31;
pub const TIKEY_COMMA: C2RustUnnamed = 30;
pub const TIKEY_STORE: C2RustUnnamed = 29;
pub const TIKEY_C: C2RustUnnamed = 28;
pub const TIKEY_F: C2RustUnnamed = 27;
pub const TIKEY_R: C2RustUnnamed = 26;
pub const TIKEY_F7: C2RustUnnamed = 25;
pub const TIKEY_7: C2RustUnnamed = 24;
pub const TIKEY_8: C2RustUnnamed = 23;
pub const TIKEY_9: C2RustUnnamed = 22;
pub const TIKEY_X: C2RustUnnamed = 21;
pub const TIKEY_D: C2RustUnnamed = 20;
pub const TIKEY_E: C2RustUnnamed = 19;
pub const TIKEY_F3: C2RustUnnamed = 18;
pub const TIKEY_4: C2RustUnnamed = 17;
pub const TIKEY_5: C2RustUnnamed = 16;
pub const TIKEY_6: C2RustUnnamed = 15;
pub const TIKEY_Z: C2RustUnnamed = 14;
pub const TIKEY_S: C2RustUnnamed = 13;
pub const TIKEY_W: C2RustUnnamed = 12;
pub const TIKEY_F8: C2RustUnnamed = 11;
pub const TIKEY_1: C2RustUnnamed = 10;
pub const TIKEY_2: C2RustUnnamed = 9;
pub const TIKEY_3: C2RustUnnamed = 8;
pub const TIKEY_2ND: C2RustUnnamed = 7;
pub const TIKEY_DIAMOND: C2RustUnnamed = 6;
pub const TIKEY_SHIFT: C2RustUnnamed = 5;
pub const TIKEY_HAND: C2RustUnnamed = 4;
pub const TIKEY_LEFT: C2RustUnnamed = 3;
pub const TIKEY_UP: C2RustUnnamed = 2;
pub const TIKEY_RIGHT: C2RustUnnamed = 1;
pub const TIKEY_DOWN: C2RustUnnamed = 0;
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
static mut keys: [[cty::c_int; 5]; 256] =
    [[-1i32, -1i32, -1i32, -1i32, -1i32], [-1i32, -1i32, -1i32, -1i32, -1i32],
     [-1i32, -1i32, -1i32, -1i32, -1i32], [-1i32, -1i32, -1i32, -1i32, -1i32],
     [-1i32, -1i32, -1i32, -1i32, -1i32], [-1i32, -1i32, -1i32, -1i32, -1i32],
     [-1i32, -1i32, -1i32, -1i32, -1i32], [-1i32, -1i32, -1i32, -1i32, -1i32],
     [-1i32, -1i32, -1i32, -1i32, -1i32], [-1i32, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_ENTER1 as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [-1i32, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_ESCAPE as cty::c_int,
      TIKEY_DIAMOND as cty::c_int, TIKEY_Q as cty::c_int,
      TIKEY_VOID as cty::c_int],
     [TIKEY_CLEAR as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_1 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_2 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_3 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_4 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_5 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_6 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_7 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_8 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_9 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_A as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_B as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_C as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_D as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_E as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_1 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_2 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_3 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_4 as cty::c_int, -1i32],
     [TIKEY_SPACE as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_1 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_2 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_3 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_4 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_5 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_6 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_7 as cty::c_int, -1i32],
     [TIKEY_PALEFT as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_PARIGHT as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_MULTIPLY as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_PLUS as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_COMMA as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_MINUS as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_PERIOD as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_DIVIDE as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_0 as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_1 as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_2 as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_3 as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_4 as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_5 as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_6 as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_7 as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_8 as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_9 as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_THETA as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_M as cty::c_int, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_0 as cty::c_int, -1i32, -1i32, -1i32],
     [TIKEY_EQUALS as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PERIOD as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_8 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_9 as cty::c_int, -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_A as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_B as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_C as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_D as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_E as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_F as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_G as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_H as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_I as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_J as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_K as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_L as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_M as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_N as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_O as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_P as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_Q as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_R as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_S as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_T as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_U as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_V as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_W as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_X as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_Y as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_SHIFT as cty::c_int, TIKEY_Z as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_COMMA as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_EQUALS as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_DIVIDE as cty::c_int, -1i32, -1i32,
      -1i32], [TIKEY_POWER as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_A as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_B as cty::c_int, -1i32],
     [TIKEY_A as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_B as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_C as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_D as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_E as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_F as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_G as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_H as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_I as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_J as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_K as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_L as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_M as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_N as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_O as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_P as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_Q as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_R as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_S as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_T as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_U as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_V as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_W as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_X as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_Y as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_Z as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PALEFT as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_C as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PARIGHT as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_D as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_4 as cty::c_int, TIKEY_F as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_1 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_2 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_3 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_4 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_5 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_6 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_7 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_8 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_9 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_A as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_C as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_D as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_POWER as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_F as cty::c_int, -1i32],
     [TIKEY_DIAMOND as cty::c_int, TIKEY_G as cty::c_int,
      TIKEY_SHIFT as cty::c_int, TIKEY_S as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_G as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_H as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_I as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_J as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_K as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_L as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_1 as cty::c_int, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_5 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_I as cty::c_int, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_6 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_9 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_A as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_B as cty::c_int, -1i32],
     [TIKEY_DIAMOND as cty::c_int, TIKEY_0 as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_DIAMOND as cty::c_int, TIKEY_EQUALS as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_DIAMOND as cty::c_int, TIKEY_PERIOD as cty::c_int, -1i32, -1i32,
      -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_8 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_E as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_F as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_G as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_H as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_I as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_J as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_K as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_L as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_F as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_M as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_F as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_Q as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_G as cty::c_int, -1i32],
     [TIKEY_NEGATE as cty::c_int, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_N as cty::c_int, -1i32],
     [-1i32, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_7 as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_N as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_I as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_J as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_K as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_1 as cty::c_int, TIKEY_B as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_O as cty::c_int, -1i32],
     [-1i32, -1i32, -1i32, -1i32, -1i32], [-1i32, -1i32, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_H as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_D as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_R as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_8 as cty::c_int,
      TIKEY_BACKSPACE as cty::c_int, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_7 as cty::c_int,
      TIKEY_BACKSPACE as cty::c_int, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_J as cty::c_int, -1i32, -1i32, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_3 as cty::c_int, TIKEY_P as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_1 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_3 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_5 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_7 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_9 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_B as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_D as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_6 as cty::c_int, TIKEY_1 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_2 as cty::c_int, TIKEY_1 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_2 as cty::c_int, TIKEY_3 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_2 as cty::c_int, TIKEY_5 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_2 as cty::c_int, TIKEY_7 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_3 as cty::c_int, TIKEY_1 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_3 as cty::c_int, TIKEY_3 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_3 as cty::c_int, TIKEY_5 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_3 as cty::c_int, TIKEY_7 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_6 as cty::c_int, TIKEY_3 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_6 as cty::c_int, TIKEY_5 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_1 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_3 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_5 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_7 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_9 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_L as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_B as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_5 as cty::c_int, TIKEY_3 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_5 as cty::c_int, TIKEY_5 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_5 as cty::c_int, TIKEY_7 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_6 as cty::c_int, TIKEY_A as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_6 as cty::c_int, TIKEY_8 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_6 as cty::c_int, TIKEY_7 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_2 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_4 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_6 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_8 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_A as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_C as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_1 as cty::c_int, TIKEY_E as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_6 as cty::c_int, TIKEY_2 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_2 as cty::c_int, TIKEY_2 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_2 as cty::c_int, TIKEY_4 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_2 as cty::c_int, TIKEY_6 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_2 as cty::c_int, TIKEY_8 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_3 as cty::c_int, TIKEY_2 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_3 as cty::c_int, TIKEY_4 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_3 as cty::c_int, TIKEY_6 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_3 as cty::c_int, TIKEY_8 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_6 as cty::c_int, TIKEY_4 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_6 as cty::c_int, TIKEY_6 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_2 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_4 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_6 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_8 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_A as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_2 as cty::c_int, TIKEY_M as cty::c_int, -1i32],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int, TIKEY_C as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_5 as cty::c_int, TIKEY_2 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_5 as cty::c_int, TIKEY_4 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_5 as cty::c_int, TIKEY_6 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_5 as cty::c_int, TIKEY_8 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_6 as cty::c_int, TIKEY_B as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_6 as cty::c_int, TIKEY_9 as cty::c_int],
     [TIKEY_2ND as cty::c_int, TIKEY_PLUS as cty::c_int,
      TIKEY_5 as cty::c_int, TIKEY_6 as cty::c_int,
      TIKEY_C as cty::c_int]];
static mut keys89_12: [cty::c_int; 5] =
    [TIKEY_2ND as cty::c_int, TIKEY_ESCAPE as cty::c_int,
     TIKEY_HOME as cty::c_int, TIKEY_VOID as cty::c_int, -1i32];
static mut keys89_58: [cty::c_int; 3] =
    [TIKEY_2ND as cty::c_int, TIKEY_4 as cty::c_int, -1i32];
static mut keys89_59: [cty::c_int; 3] =
    [TIKEY_2ND as cty::c_int, TIKEY_9 as cty::c_int, -1i32];
static mut keys89_92: [cty::c_int; 3] =
    [TIKEY_2ND as cty::c_int, TIKEY_2 as cty::c_int, -1i32];
static mut keys89_142: [cty::c_int; 5] =
    [TIKEY_DIAMOND as cty::c_int, TIKEY_PALEFT as cty::c_int,
     TIKEY_SHIFT as cty::c_int, TIKEY_3 as cty::c_int, -1i32];
static mut keys89_149: [cty::c_int; 2] = [TIKEY_EE as cty::c_int, -1i32];
static mut keys89_151: [cty::c_int; 3] =
    [TIKEY_2ND as cty::c_int, TIKEY_CATALOG as cty::c_int, -1i32];
static mut keys89_190: [cty::c_int; 3] =
    [TIKEY_DIAMOND as cty::c_int, TIKEY_CATALOG as cty::c_int, -1i32];
#[no_mangle]
pub unsafe extern "C" fn chars_to_keys(mut chars: *const cty::c_char,
                                       mut ti89: cty::c_int)
 -> *mut cty::c_int {
    let mut buffer: *mut cty::c_int = 0 as *mut cty::c_int;
    let mut q: *mut cty::c_int = 0 as *mut cty::c_int;
    let mut i: cty::c_int = 0;
    let mut shift: cty::c_int = 0i32;
    let mut p: *const cty::c_uchar = 0 as *const cty::c_uchar;
    let mut row: *const cty::c_int = 0 as *const cty::c_int;
    buffer =
        malloc(strlen(chars).wrapping_mul(6i32 as
                                              cty::c_ulong).wrapping_add(1i32
                                                                              as
                                                                              cty::c_ulong).wrapping_mul(::core::mem::size_of::<cty::c_int>()
                                                                                                              as
                                                                                                              cty::c_ulong))
            as *mut cty::c_int;
    if buffer.is_null() { return 0 as *mut cty::c_int }
    q = buffer;
    p = chars as *const cty::c_uchar;
    while 0 != *p {
        row = keys[*p as usize].as_ptr();
        if 0 != ti89 {
            match *p as cty::c_int {
                12 => {
                    // Form Feed => QUIT + HOME
                    row = keys89_12.as_ptr()
                }
                58 => {
                    // :
                    row = keys89_58.as_ptr()
                }
                59 => {
                    // ;
                    row = keys89_59.as_ptr()
                }
                92 => {
                    // backslash
                    row = keys89_92.as_ptr()
                }
                142 => {
                    // SIGMA
                    row = keys89_142.as_ptr()
                }
                149 => {
                    // EE
                    row = keys89_149.as_ptr()
                }
                151 => {
                    // imaginary i
                    row = keys89_151.as_ptr()
                }
                190 => {
                    // infinity
                    row = keys89_190.as_ptr()
                }
                _ => { }
            }
        }
        i = 0i32;
        while i < 5i32 {
            if *row.offset(i as isize) == -1i32 { break ; }
            if 0 != ti89 {
                match *row.offset(i as isize) {
                    75 => {
                        if 0 == shift {
                            let fresh0 = q;
                            q = q.offset(1);
                            *fresh0 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh1 = q;
                        q = q.offset(1);
                        *fresh1 = TIKEY_EQUALS as cty::c_int;
                        shift = 0i32
                    }
                    44 => {
                        if 0 == shift {
                            let fresh2 = q;
                            q = q.offset(1);
                            *fresh2 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh3 = q;
                        q = q.offset(1);
                        *fresh3 = TIKEY_PALEFT as cty::c_int;
                        shift = 0i32
                    }
                    28 => {
                        if 0 == shift {
                            let fresh4 = q;
                            q = q.offset(1);
                            *fresh4 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh5 = q;
                        q = q.offset(1);
                        *fresh5 = TIKEY_PARIGHT as cty::c_int;
                        shift = 0i32
                    }
                    20 => {
                        if 0 == shift {
                            let fresh6 = q;
                            q = q.offset(1);
                            *fresh6 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh7 = q;
                        q = q.offset(1);
                        *fresh7 = TIKEY_COMMA as cty::c_int;
                        shift = 0i32
                    }
                    19 => {
                        if 0 == shift {
                            let fresh8 = q;
                            q = q.offset(1);
                            *fresh8 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh9 = q;
                        q = q.offset(1);
                        *fresh9 = TIKEY_DIVIDE as cty::c_int;
                        shift = 0i32
                    }
                    27 => {
                        if 0 == shift {
                            let fresh10 = q;
                            q = q.offset(1);
                            *fresh10 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh11 = q;
                        q = q.offset(1);
                        *fresh11 = TIKEY_PIPE as cty::c_int;
                        shift = 0i32
                    }
                    35 => {
                        if 0 == shift {
                            let fresh12 = q;
                            q = q.offset(1);
                            *fresh12 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh13 = q;
                        q = q.offset(1);
                        *fresh13 = TIKEY_7 as cty::c_int;
                        shift = 0i32
                    }
                    43 => {
                        if 0 == shift {
                            let fresh14 = q;
                            q = q.offset(1);
                            *fresh14 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh15 = q;
                        q = q.offset(1);
                        *fresh15 = TIKEY_8 as cty::c_int;
                        shift = 0i32
                    }
                    58 => {
                        if 0 == shift {
                            let fresh16 = q;
                            q = q.offset(1);
                            *fresh16 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh17 = q;
                        q = q.offset(1);
                        *fresh17 = TIKEY_9 as cty::c_int;
                        shift = 0i32
                    }
                    51 => {
                        if 0 == shift {
                            let fresh18 = q;
                            q = q.offset(1);
                            *fresh18 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh19 = q;
                        q = q.offset(1);
                        *fresh19 = TIKEY_MULTIPLY as cty::c_int;
                        shift = 0i32
                    }
                    59 => {
                        if 0 == shift {
                            let fresh20 = q;
                            q = q.offset(1);
                            *fresh20 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh21 = q;
                        q = q.offset(1);
                        *fresh21 = TIKEY_EE as cty::c_int;
                        shift = 0i32
                    }
                    67 => {
                        if 0 == shift {
                            let fresh22 = q;
                            q = q.offset(1);
                            *fresh22 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh23 = q;
                        q = q.offset(1);
                        *fresh23 = TIKEY_4 as cty::c_int;
                        shift = 0i32
                    }
                    60 => {
                        if 0 == shift {
                            let fresh24 = q;
                            q = q.offset(1);
                            *fresh24 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh25 = q;
                        q = q.offset(1);
                        *fresh25 = TIKEY_5 as cty::c_int;
                        shift = 0i32
                    }
                    52 => {
                        if 0 == shift {
                            let fresh26 = q;
                            q = q.offset(1);
                            *fresh26 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh27 = q;
                        q = q.offset(1);
                        *fresh27 = TIKEY_6 as cty::c_int;
                        shift = 0i32
                    }
                    66 => {
                        if 0 == shift {
                            let fresh28 = q;
                            q = q.offset(1);
                            *fresh28 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh29 = q;
                        q = q.offset(1);
                        *fresh29 = TIKEY_MINUS as cty::c_int;
                        shift = 0i32
                    }
                    46 => {
                        if 0 == shift {
                            let fresh30 = q;
                            q = q.offset(1);
                            *fresh30 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh31 = q;
                        q = q.offset(1);
                        *fresh31 = TIKEY_STORE as cty::c_int;
                        shift = 0i32
                    }
                    74 => {
                        if 0 == shift {
                            let fresh32 = q;
                            q = q.offset(1);
                            *fresh32 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh33 = q;
                        q = q.offset(1);
                        *fresh33 = TIKEY_1 as cty::c_int;
                        shift = 0i32
                    }
                    26 => {
                        if 0 == shift {
                            let fresh34 = q;
                            q = q.offset(1);
                            *fresh34 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh35 = q;
                        q = q.offset(1);
                        *fresh35 = TIKEY_2 as cty::c_int;
                        shift = 0i32
                    }
                    13 => {
                        if 0 == shift {
                            let fresh36 = q;
                            q = q.offset(1);
                            *fresh36 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh37 = q;
                        q = q.offset(1);
                        *fresh37 = TIKEY_3 as cty::c_int;
                        shift = 0i32
                    }
                    50 => {
                        if 0 == shift {
                            let fresh38 = q;
                            q = q.offset(1);
                            *fresh38 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh39 = q;
                        q = q.offset(1);
                        *fresh39 = TIKEY_PLUS as cty::c_int;
                        shift = 0i32
                    }
                    36 => {
                        if 0 == shift {
                            let fresh40 = q;
                            q = q.offset(1);
                            *fresh40 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh41 = q;
                        q = q.offset(1);
                        *fresh41 = TIKEY_0 as cty::c_int;
                        shift = 0i32
                    }
                    12 => {
                        if 0 == shift {
                            let fresh42 = q;
                            q = q.offset(1);
                            *fresh42 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh43 = q;
                        q = q.offset(1);
                        *fresh43 = TIKEY_PERIOD as cty::c_int;
                        shift = 0i32
                    }
                    37 => {
                        if 0 == shift {
                            let fresh44 = q;
                            q = q.offset(1);
                            *fresh44 = TIKEY_ALPHA as cty::c_int
                        }
                        let fresh45 = q;
                        q = q.offset(1);
                        *fresh45 = TIKEY_NEGATE as cty::c_int;
                        shift = 0i32
                    }
                    5 => {
                        let fresh46 = q;
                        q = q.offset(1);
                        *fresh46 = TIKEY_SHIFT as cty::c_int;
                        shift = 1i32
                    }
                    _ => {
                        let fresh47 = q;
                        q = q.offset(1);
                        *fresh47 = *row.offset(i as isize);
                        shift = 0i32
                    }
                }
            } else {
                let fresh48 = q;
                q = q.offset(1);
                *fresh48 = *row.offset(i as isize)
            }
            i += 1
        }
        p = p.offset(1isize)
    }
    *q = -1i32;
    return buffer;
}
