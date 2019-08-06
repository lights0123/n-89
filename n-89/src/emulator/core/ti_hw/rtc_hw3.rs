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
    fn memcpy(_: *mut cty::c_void, _: *const cty::c_void, _: cty::c_ulong)
     -> *mut cty::c_void;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn difftime(__time1: time_t, __time0: time_t) -> cty::c_double;
    #[no_mangle]
    fn mktime(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
    #[no_mangle]
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t)
     -> cty::c_int;
    // PC
    // Link
    // buffer size
    // buffer (LSB is data, MSB is S/R action)
    // buffer index
    // actions (1: S, 2: R)
    /* Externs */
    #[no_mangle]
    static mut tihw: Ti68kHardware;
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __uint32_t = cty::c_uint;
pub type __time_t = cty::c_long;
pub type __suseconds_t = cty::c_long;
pub type time_t = __time_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct tm {
    pub tm_sec: cty::c_int,
    pub tm_min: cty::c_int,
    pub tm_hour: cty::c_int,
    pub tm_mday: cty::c_int,
    pub tm_mon: cty::c_int,
    pub tm_year: cty::c_int,
    pub tm_wday: cty::c_int,
    pub tm_yday: cty::c_int,
    pub tm_isdst: cty::c_int,
    pub tm_gmtoff: cty::c_long,
    pub tm_zone: *const cty::c_char,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: cty::c_int,
    pub tz_dsttime: cty::c_int,
}
pub type __timezone_ptr_t = *mut timezone;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TTIME {
    pub s: time_t,
    pub ms: cty::c_int,
}
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
}
/* Hey EMACS -*- linux-c -*- */
/* $Id: rtc_hw3.c 2559 2007-06-24 17:33:54Z roms $ */
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
/*
    TI's HW3 RTC helpers.
 */
#[no_mangle]
pub unsafe extern "C" fn rtc3_init() -> cty::c_int {
    let mut ref_0: tm =
        tm{tm_sec: 0,
           tm_min: 0,
           tm_hour: 0,
           tm_mday: 0,
           tm_mon: 0,
           tm_year: 0,
           tm_wday: 0,
           tm_yday: 0,
           tm_isdst: 0,
           tm_gmtoff: 0,
           tm_zone: 0 as *const cty::c_char,};
    let mut now: time_t = 0;
    if tihw.hw_type < 3i32 { return 0i32 }
    // Computes pseudo-constant (difference between PC ref and TI ref) because:
	// - TI counts seconds since January 1st, 1997 00:00:00
	// - PC counts seconds since January 1st, 1970 00:00:00
    time(&mut now); // get tm_isdst field (DST status)
    memcpy(&mut ref_0 as *mut tm as *mut cty::c_void,
           localtime(&mut now) as *const cty::c_void,
           ::core::mem::size_of::<tm>() as cty::c_ulong);
    ref_0.tm_year = 1997i32 - 1900i32;
    ref_0.tm_mon = 0i32;
    ref_0.tm_yday = 0i32;
    ref_0.tm_mday = 1i32;
    ref_0.tm_wday = 3i32;
    ref_0.tm_hour = 0i32;
    ref_0.tm_min = 0i32;
    ref_0.tm_sec = 0i32;
    tihw.rtc3_ref.s = mktime(&mut ref_0);
    tihw.rtc3_beg.s = tihw.rtc3_ref.s;
    tihw.rtc3_ref.ms = 0i32;
    tihw.rtc3_beg.ms = tihw.rtc3_ref.ms;
    // printf("<<%s>>\n", asctime(&ref));
    *tihw.io3.offset(0x5fi32 as isize) = 0x80i32 as uint8_t;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn rtc3_reset() -> cty::c_int { return 0i32; }
#[no_mangle]
pub unsafe extern "C" fn rtc3_exit() -> cty::c_int { return 0i32; }
// return seconds and milli-seconds
// FIXME: kill this stupid wrapper which loses precision and use gettimeofday
//        and struct timeval directly instead
// Problem: We'll have to bump the savefile revision if we do that. :-(
#[no_mangle]
pub unsafe extern "C" fn rtc3_get_time(mut tt: *mut TTIME) {
    let mut tp: timeval = timeval{tv_sec: 0, tv_usec: 0,};
    gettimeofday(&mut tp, 0 as *mut timezone);
    (*tt).s = tp.tv_sec;
    (*tt).ms = (tp.tv_usec / 1000i32 as cty::c_long) as cty::c_int;
}
// tt = t2 - t1 and take care of reporting milli-seconds
#[no_mangle]
pub unsafe extern "C" fn rtc3_diff_time(mut t2: *mut TTIME,
                                        mut t1: *mut TTIME,
                                        mut tt: *mut TTIME) {
    (*tt).s = difftime((*t2).s, (*t1).s) as time_t;
    (*tt).ms = (*t2).ms - (*t1).ms;
    if (*tt).ms < 0i32 { (*tt).ms += 1000i32; (*tt).s -= 1 };
}
// tt = t2 + t1 and take care of reporting milli-seconds
#[no_mangle]
pub unsafe extern "C" fn rtc3_add_time(mut t2: *mut TTIME, mut t1: *mut TTIME,
                                       mut tt: *mut TTIME) {
    (*tt).ms = (*t1).ms + (*t2).ms;
    (*tt).s = (*t1).s + (*t2).s;
    if (*tt).ms > 1000i32 { (*tt).ms -= 1000i32; (*tt).s += 1 };
}
// Call it before ti68k_state_save to update registers with current clock
// so that clock is correcly saved
#[no_mangle]
pub unsafe extern "C" fn rtc3_state_save() -> cty::c_int {
    let mut rtc3_cur: TTIME = TTIME{s: 0, ms: 0,};
    let mut d: TTIME = TTIME{s: 0, ms: 0,};
    let mut a: TTIME = TTIME{s: 0, ms: 0,};
    if tihw.hw_type < 3i32 { return 0i32 }
    // get time and computes time elapsed since reload (cur - beg + load)
    rtc3_get_time(&mut rtc3_cur);
    rtc3_diff_time(&mut rtc3_cur, &mut tihw.rtc3_beg, &mut d);
    rtc3_add_time(&mut d, &mut tihw.rtc3_load, &mut a);
    // 1/16th of seconds
    *tihw.io3.offset(0x45i32 as isize) = ((a.ms + a.ms) / 125i32) as uint8_t;
    // seconds since January 1st, 1997 00:00:00
    *tihw.io3.offset(0x46i32 as isize) =
        ((((a.s & 0xffff0000u32 as cty::c_long) >> 16i32) as uint16_t as
              cty::c_int & 0xff00i32) >> 8i32) as uint8_t;
    *tihw.io3.offset(0x47i32 as isize) =
        (((a.s & 0xffff0000u32 as cty::c_long) >> 16i32) as uint16_t as
             cty::c_int & 0xffi32) as uint8_t;
    *tihw.io3.offset(0x48i32 as isize) =
        (((a.s & 0xffffi32 as cty::c_long) as uint16_t as cty::c_int &
              0xff00i32) >> 8i32) as uint8_t;
    *tihw.io3.offset(0x49i32 as isize) =
        ((a.s & 0xffffi32 as cty::c_long) as uint16_t as cty::c_int &
             0xffi32) as uint8_t;
    // printf("%i.%i\n", tihw.io3[0x49], tihw.io3[0x45]);
    /*
	rtc3_diff_time(&rtc3_cur, &tihw.rtc3_beg, &r);
	printf("%i.%i - %i.%i = %i.%i\n",
	    rtc3_cur.s, rtc3_cur.ms, tihw.rtc3_beg.s, tihw.rtc3_beg.ms, r.s, r.ms);
	*/
	/*
	rtc3_add_time(&rtc3_cur, &tihw.rtc3_beg, &r);
	printf("%i.%i - %i.%i = %i.%i\n",
	    rtc3_cur.s, rtc3_cur.ms, tihw.rtc3_beg.s, tihw.rtc3_beg.ms, r.s, r.ms);
	*/
    return 0i32;
}
// Call it after ti68k_state_load to update current clock so that the calc always
// display the right time even if calc has been shudown.
#[no_mangle]
pub unsafe extern "C" fn rtc3_state_load() -> cty::c_int {
    let mut rtc3_cur: TTIME = TTIME{s: 0, ms: 0,};
    if tihw.hw_type < 3i32 { return 0i32 }
    // clock disabled ?
    if 0 == *tihw.io3.offset(0x5fi32 as isize) as cty::c_int & 1i32 << 0i32 {
        return 0i32
    }
    // update current time
    rtc3_get_time(&mut rtc3_cur);
    return 0i32;
}
// When compiling without GDB, build the Win32 implementation of gettimeofday
// here to avoid the makefile hackery. GDB-enabled builds build it as part of
// libiberty.
