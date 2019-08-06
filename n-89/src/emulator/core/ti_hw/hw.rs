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
    fn strcpy(_: *mut cty::c_char, _: *const cty::c_char)
     -> *mut cty::c_char;
    #[no_mangle]
    fn strcmp(_: *const cty::c_char, _: *const cty::c_char) -> cty::c_int;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
     -> cty::c_int;
    /* Externs */
    #[no_mangle]
    static mut params: Ti68kParameters;
    #[no_mangle]
    static mut tihw: Ti68kHardware;
    #[no_mangle]
    static mut img_infos: IMG_INFO32;
    #[no_mangle]
    fn hw_mem_exit() -> cty::c_int;
    #[no_mangle]
    fn hw_mem_reset() -> cty::c_int;
    #[no_mangle]
    fn hw_mem_init() -> cty::c_int;
    /* Functions */
    #[no_mangle]
    fn hw_flash_init() -> cty::c_int;
    #[no_mangle]
    fn hw_flash_reset() -> cty::c_int;
    #[no_mangle]
    fn hw_flash_exit() -> cty::c_int;
    /* Hey EMACS -*- linux-c -*- */
/* $Id: hw.h 1455 2005-05-31 18:38:03Z roms $ */
    /*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2005, Romain Lievin
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
    fn lcd_hook_hw1();
    /* Hey EMACS -*- linux-c -*- */
/* $Id: main.c 245 2004-05-23 20:45:43Z roms $ */
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
    /* Functions */
    #[no_mangle]
    fn hw_hwp_init() -> cty::c_int;
    #[no_mangle]
    fn hw_hwp_reset() -> cty::c_int;
    #[no_mangle]
    fn hw_hwp_exit() -> cty::c_int;
    /* Hey EMACS -*- linux-c -*- */
/* $Id: main.c 245 2004-05-23 20:45:43Z roms $ */
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
    #[no_mangle]
    fn hw_kbd_init() -> cty::c_int;
    #[no_mangle]
    fn hw_kbd_reset() -> cty::c_int;
    #[no_mangle]
    fn hw_kbd_exit() -> cty::c_int;
    #[no_mangle]
    fn hw_kbd_update() -> cty::c_int;
    /* Hey EMACS -*- linux-c -*- */
/* $Id: main.c 245 2004-05-23 20:45:43Z roms $ */
    /*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin
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
    fn hw_m68k_init() -> cty::c_int;
    #[no_mangle]
    fn hw_m68k_reset() -> cty::c_int;
    #[no_mangle]
    fn hw_m68k_exit() -> cty::c_int;
    #[no_mangle]
    fn hw_m68k_irq(n: cty::c_int);
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
    #[no_mangle]
    fn hw_io_init() -> cty::c_int;
    #[no_mangle]
    fn hw_io_reset() -> cty::c_int;
    #[no_mangle]
    fn hw_io_exit() -> cty::c_int;
}
pub type __uint8_t = cty::c_uchar;
pub type __int32_t = cty::c_int;
pub type __uint32_t = cty::c_uint;
pub type __time_t = cty::c_long;
pub type __suseconds_t = cty::c_long;
pub type time_t = __time_t;
pub type int32_t = __int32_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: cty::c_int,
    pub tz_dsttime: cty::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
pub type uint8_t = __uint8_t;
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
/* Equivalences */
/* Constants */
// LCD _memory_ width
// LCD _memory_ height
/* Structures */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Ti68kParameters {
    pub rom_file: *mut cty::c_char,
    pub tib_file: *mut cty::c_char,
    pub sav_file: *mut cty::c_char,
    pub restricted: cty::c_int,
    pub cpu_rate: cty::c_int,
    pub hw_rate: cty::c_int,
    pub lcd_rate: cty::c_int,
    pub hw_protect: cty::c_int,
    pub recv_file: cty::c_int,
    // receive file enabled/disabled
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
/* Hey EMACS -*- linux-c -*- */
/* $Id: images.h 2821 2009-05-04 20:06:12Z roms $ */
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
  Definitions
*/
// increase this number when changing the structure below
// Please update the docs/TiEmu_img_format.txt documentation when making changes
// on the structure below
// If this structure is modified, the SAV_REVISION number (state.c)
// has to be incremented.
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct IMG_INFO32 {
    pub signature: [cty::c_char; 16],
    pub revision: int32_t,
    pub header_size: int32_t,
    pub calc_type: cty::c_char,
    pub version: [cty::c_char; 5],
    pub flash: cty::c_char,
    pub has_boot: cty::c_char,
    pub size: int32_t,
    pub hw_type: cty::c_char,
    pub rom_base: uint8_t,
    pub fill: [cty::c_char; 22],
    pub data: *mut cty::c_char,
    // pure data (temporary use, 8 bytes)
}
// 16MHz / (2^19/2^5)
// Timer masks at 2^5, 2^9, 2^12, 2^18 (port $600015)
#[no_mangle]
pub static mut timer_masks: [cty::c_uint; 4] =
    [0i32 as cty::c_uint, 15i32 as cty::c_uint, 127i32 as cty::c_uint,
     8191i32 as cty::c_uint];
#[no_mangle]
pub static mut timer_mask: cty::c_uint = 15i32 as cty::c_uint;
#[no_mangle]
pub static mut cycle_instr: cty::c_uint = 427i32 as cty::c_uint;
#[no_mangle]
pub static mut cycle_count: cty::c_uint = 0i32 as cty::c_uint;
#[no_mangle]
pub unsafe extern "C" fn set_prescaler(mut i: cty::c_int) {
    timer_mask = timer_masks[i as usize];
}
/*
    Init hardware...
    A ROM image must have been loaded before calling this function.
*/
#[no_mangle]
pub unsafe extern "C" fn hw_init() -> cty::c_int {
    let mut img: *mut IMG_INFO32 = &mut img_infos;
    // Get infos from image
    tihw.calc_type = img_infos.calc_type as cty::c_int;
    tihw.rom_base = (((*img).rom_base as cty::c_int) << 16i32) as uint32_t;
    tihw.rom_flash = (*img).flash as cty::c_int;
    strcpy(tihw.rom_version.as_mut_ptr(), (*img).version.as_mut_ptr());
    tihw.hw_type = (*img).hw_type as cty::c_int;
    tihw.ti92v1 =
        (tihw.calc_type == 1i32 << 0i32 &&
             strcmp(tihw.rom_version.as_mut_ptr(),
                    b"2.0\x00" as *const u8 as *const cty::c_char) < 0i32) as
            cty::c_int;
    tihw.ti92v2 =
        (tihw.calc_type == 1i32 << 0i32 &&
             strcmp(tihw.rom_version.as_mut_ptr(),
                    b"2.0\x00" as *const u8 as *const cty::c_char) >= 0i32)
            as cty::c_int;
    match tihw.calc_type {
        2 | 16 => {
            tihw.lcd_w = 160i32;
            tihw.log_w = tihw.lcd_w;
            tihw.lcd_h = 100i32;
            tihw.log_h = tihw.lcd_h
        }
        1 | 4 | 8 => {
            tihw.lcd_w = 240i32;
            tihw.log_w = tihw.lcd_w;
            tihw.lcd_h = 128i32;
            tihw.log_h = tihw.lcd_h
        }
        _ => { return -1i32 }
    }
    // Do sub-initializations.
    let mut aaa_: cty::c_int = 0;
    aaa_ = hw_mem_init();
    if 0 != aaa_ { return aaa_ }
    let mut aaa__0: cty::c_int = 0;
    aaa__0 = hw_flash_init();
    if 0 != aaa__0 { return aaa__0 }
    let mut aaa__1: cty::c_int = 0;
    aaa__1 = hw_io_init();
    if 0 != aaa__1 { return aaa__1 }
    let mut aaa__2: cty::c_int = 0;
    aaa__2 = hw_hwp_init();
    if 0 != aaa__2 { return aaa__2 }
    let mut aaa__3: cty::c_int = 0;
    aaa__3 = hw_kbd_init();
    if 0 != aaa__3 { return aaa__3 }
    let mut aaa__4: cty::c_int = 0;
    aaa__4 = hw_m68k_init();
    if 0 != aaa__4 { return aaa__4 }
    // Set hardware update rate (dependant from io[0x15])
    if params.hw_rate != -1i32 {
        cycle_instr = params.hw_rate as cty::c_uint
    } else if tihw.hw_type == 1i32 {
        cycle_instr = 427i32 as cty::c_uint
    } else if tihw.hw_type <= 3i32 {
        cycle_instr = 732i32 as cty::c_uint
    } else { cycle_instr = 977i32 as cty::c_uint }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn hw_reset() -> cty::c_int {
    let mut aaa_: cty::c_int = 0;
    aaa_ = hw_mem_reset();
    if 0 != aaa_ { return aaa_ }
    let mut aaa__0: cty::c_int = 0;
    aaa__0 = hw_flash_reset();
    if 0 != aaa__0 { return aaa__0 }
    let mut aaa__1: cty::c_int = 0;
    aaa__1 = hw_io_reset();
    if 0 != aaa__1 { return aaa__1 }
    let mut aaa__2: cty::c_int = 0;
    aaa__2 = hw_hwp_reset();
    if 0 != aaa__2 { return aaa__2 }
    let mut aaa__3: cty::c_int = 0;
    aaa__3 = hw_kbd_reset();
    if 0 != aaa__3 { return aaa__3 }
    let mut aaa__4: cty::c_int = 0;
    aaa__4 = hw_m68k_reset();
    if 0 != aaa__4 { return aaa__4 }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn hw_exit() -> cty::c_int {
    let mut aaa_: cty::c_int = 0;
    aaa_ = hw_m68k_exit();
    if 0 != aaa_ { return aaa_ }
    let mut aaa__0: cty::c_int = 0;
    aaa__0 = hw_kbd_exit();
    if 0 != aaa__0 { return aaa__0 }
    let mut aaa__1: cty::c_int = 0;
    aaa__1 = hw_io_exit();
    if 0 != aaa__1 { return aaa__1 }
    let mut aaa__2: cty::c_int = 0;
    aaa__2 = hw_flash_exit();
    if 0 != aaa__2 { return aaa__2 }
    let mut aaa__3: cty::c_int = 0;
    aaa__3 = hw_mem_exit();
    if 0 != aaa__3 { return aaa__3 }
    let mut aaa__4: cty::c_int = 0;
    aaa__4 = hw_hwp_exit();
    if 0 != aaa__4 { return aaa__4 }
    return 0i32;
}
/*
    This function is called by do_cycles to regularly updates the hardware.
    Rate is the same as the timer tick rate.
*/
#[no_mangle]
pub unsafe extern "C" fn hw_update() {
    static mut timer: cty::c_uint = 0;
    // time_t curr_clock;
    // OSC2 enable (bit clear means oscillator stopped!)
    let mut osc2_enabled: cty::c_int =
        *tihw.io.offset(0x15i32 as isize) as cty::c_int & 1i32 << 1i32;
    timer = timer.wrapping_add(1);
    if 0 != osc2_enabled {
        // Increment timer
        if 0 == timer & timer_mask &&
               0 !=
                   *tihw.io.offset(0x15i32 as isize) as cty::c_int &
                       1i32 << 3i32 {
            if tihw.timer_value as cty::c_int == 0i32 {
                tihw.timer_value = *tihw.io.offset(0x17i32 as isize)
            } else { tihw.timer_value = tihw.timer_value.wrapping_add(1) }
        }
    }
    // Increment HW2 RTC timer every 8192 seconds
    if tihw.hw_type >= 2i32 &&
           0 !=
               *tihw.io2.offset(0x1fi32 as isize) as cty::c_int &
                   1i32 << 2i32 &&
           0 !=
               *tihw.io2.offset(0x1fi32 as isize) as cty::c_int &
                   1i32 << 1i32 {
        static mut ref_0: timeval =
            timeval{tv_sec: 0i32 as __time_t,
                    tv_usec: 0i32 as __suseconds_t,};
        let mut tmp: timeval =
            timeval{tv_sec: 0i32 as __time_t,
                    tv_usec: 0i32 as __suseconds_t,};
        gettimeofday(&mut tmp, 0 as *mut timezone);
        // Check if 8192 seconds elapsed, avoiding 32-bit overflow
        if ((tmp.tv_sec - ref_0.tv_sec) as
                cty::c_uint).wrapping_mul(500000u32).wrapping_add((tmp.tv_usec
                                                                        -
                                                                        ref_0.tv_usec)
                                                                       as
                                                                       cty::c_uint
                                                                       >>
                                                                       1u32)
               >= 4096000000u32 {
            gettimeofday(&mut ref_0, 0 as *mut timezone);
            tihw.rtc_value = tihw.rtc_value.wrapping_add(1)
        }
    }
    // Toggles every FS (every time the LCD restarts at line 0) -> 90 Hz ~ timer/192
	// Don't use the actual LCD count (and use 192 rather than 182) to keep exposure
	// times consistent
    if 0 == timer.wrapping_rem(192i32 as cty::c_uint) && tihw.hw_type >= 2i32
       {
        let ref mut fresh0 = *tihw.io2.offset(0x1di32 as isize);
        *fresh0 = (*fresh0 as cty::c_int ^ 0x80i32) as uint8_t
    }
    /* Auto-int management */
    if 0 != osc2_enabled {
        // Auto-int 1: 1/2^6 of timer rate
		// Triggered at a fixed rate: OSC2/2^11 = (OSC2/2^5)/2^6
        if 0 == timer & 63i32 as cty::c_uint {
            if 0 ==
                   *tihw.io.offset(0x15i32 as isize) as cty::c_int &
                       1i32 << 7i32 {
                if tihw.hw_type == 1i32 ||
                       !(0 !=
                             *tihw.io2.offset(0x1fi32 as isize) as cty::c_int
                                 & 1i32 << 2i32 &&
                             0 ==
                                 *tihw.io2.offset(0x1fi32 as isize) as
                                     cty::c_int & 1i32 << 1i32) {
                    hw_m68k_irq(1i32);
                }
            }
        }
        // Auto-int 2: keyboard scan
		// see keyboard.c
    }
    if 0 != osc2_enabled || tihw.hw_type == 2i32 {
        // Auto-int 3: disabled by default by AMS
		// When enabled, it is triggered at a fixed rate: OSC2/2^19 = 1/16384 of timer rate = 1Hz
        if 0 == timer & 16383i32 as cty::c_uint {
            if 0 ==
                   *tihw.io.offset(0x15i32 as isize) as cty::c_int &
                       1i32 << 7i32 &&
                   0 !=
                       *tihw.io.offset(0x15i32 as isize) as cty::c_int &
                           1i32 << 2i32 {
                if tihw.hw_type == 1i32 ||
                       !(0 !=
                             *tihw.io2.offset(0x1fi32 as isize) as cty::c_int
                                 & 1i32 << 2i32 &&
                             0 ==
                                 *tihw.io2.offset(0x1fi32 as isize) as
                                     cty::c_int & 1i32 << 1i32) {
                    hw_m68k_irq(3i32);
                }
            }
        }
    }
    // DBus: External link activity ?
	/*
	if(!ticables_cable_get_d0(cable_handle) || !ticables_cable_get_d1(cable_handle))
	{
	    io_bit_set(0x0d,3);	//SA
	    io_bit_set(0x0d,2);	//EA
	}
	*/
    // DBUS enabled ?
    if 0 == *tihw.io.offset(0xci32 as isize) as cty::c_int & 1i32 << 6i32 {
        // Auto-int 4: triggered by linkport (error, link act, txbuf empty or rxbuf full)
        if 0 != *tihw.io.offset(0xci32 as isize) as cty::c_int & 1i32 << 3i32
               &&
               0 !=
                   *tihw.io.offset(0xdi32 as isize) as cty::c_int &
                       1i32 << 7i32 ||
               0 !=
                   *tihw.io.offset(0xci32 as isize) as cty::c_int &
                       1i32 << 2i32 &&
                   0 !=
                       *tihw.io.offset(0xdi32 as isize) as cty::c_int &
                           1i32 << 3i32 ||
               0 !=
                   *tihw.io.offset(0xci32 as isize) as cty::c_int &
                       1i32 << 1i32 &&
                   0 !=
                       *tihw.io.offset(0xdi32 as isize) as cty::c_int &
                           1i32 << 6i32 ||
               0 !=
                   *tihw.io.offset(0xci32 as isize) as cty::c_int &
                       1i32 << 0i32 &&
                   0 !=
                       *tihw.io.offset(0xdi32 as isize) as cty::c_int &
                           1i32 << 5i32 {
            hw_m68k_irq(4i32);
        }
    }
    if 0 != osc2_enabled {
        // Auto-int 5: triggered by the programmable timer.
		// The default rate is OSC2/(K*2^9), where K=79 for HW1 and K=53 for HW2
		// Make sure AI5 is triggered only if the timer was actually incremented.
        if 0 == timer & timer_mask &&
               0 !=
                   *tihw.io.offset(0x15i32 as isize) as cty::c_int &
                       1i32 << 3i32 && tihw.timer_value as cty::c_int == 0i32
           {
            if 0 ==
                   *tihw.io.offset(0x15i32 as isize) as cty::c_int &
                       1i32 << 7i32 {
                if tihw.hw_type == 1i32 ||
                       !(0 !=
                             *tihw.io2.offset(0x1fi32 as isize) as cty::c_int
                                 & 1i32 << 2i32 &&
                             0 ==
                                 *tihw.io2.offset(0x1fi32 as isize) as
                                     cty::c_int & 1i32 << 1i32) {
                    hw_m68k_irq(5i32);
                }
            }
        }
    }
    // Auto-int 6: triggered when [ON] is pressed.
	// see keyboard.c
    // Auto-int 7: "vector table write protection" & "stack overflow"
	// see memory.c
    /* Hardware refresh */
    // Update keyboard (~600Hz). Not related to timer but as a convenience
    if 0 == timer & 127i32 as cty::c_uint {
        // 31 and 63 don't work, 127 and 255 are ok
        hw_kbd_update();
    }
    // Update LCD (HW1: every 256th timer tick, HW2: unrelated)
    if tihw.hw_type == 1i32 && 0 == timer & 255i32 as cty::c_uint {
        lcd_hook_hw1();
    };
}
/*
    This function is used to regularly update the hardware from CPU loop.
    Note that CPU is running against OSC1 (HW1 @10Mhz, HW2 @12MHz)
    while hardware is synched against OSC2 (HW1 @700kHz,  HW2 @~520 kHz).
    OSC2 is the timing base for the timers, the link I/O hardware and
    (HW1 only) the LCD controller.
    These 2 oscillators are independants.

    See hw.h for inline definition.
*/
/*
static void INLINE do_cycles(void);
*/
