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
    fn memcmp(_: *const cty::c_void, _: *const cty::c_void,
              _: cty::c_ulong) -> cty::c_int;
    #[no_mangle]
    static mut tihw: Ti68kHardware;
    #[no_mangle]
    static mut regs: regstruct;
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __uint32_t = cty::c_uint;
pub type __time_t = cty::c_long;
pub type time_t = __time_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = cty::c_ulong;
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
pub type flagtype = cty::c_char;
pub type fptype = cty::c_double;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct flag_struct {
    pub c: cty::c_uint,
    pub z: cty::c_uint,
    pub n: cty::c_uint,
    pub v: cty::c_uint,
    pub x: cty::c_uint,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct regstruct {
    pub regs: [uint32_t; 16],
    pub usp: uintptr_t,
    pub isp: uintptr_t,
    pub sr: uint16_t,
    pub t1: flagtype,
    pub s: flagtype,
    pub x: flagtype,
    pub stopped: flagtype,
    pub flags: flag_struct,
    pub intmask: cty::c_int,
    pub pc: uint32_t,
    pub pc_p: *mut uint8_t,
    pub pc_oldp: *mut uint8_t,
    pub vbr: uint32_t,
    pub sfc: uint32_t,
    pub dfc: uint32_t,
    pub fp: [fptype; 8],
    pub fp_result: fptype,
    pub fpcr: uint32_t,
    pub fpsr: uint32_t,
    pub fpiar: uint32_t,
    pub fpsr_highbyte: uint32_t,
    pub spcflags: uint32_t,
    pub kick_mask: uint32_t,
    pub address_space_mask: uint32_t,
    pub irc: uint16_t,
    pub ir: uint16_t,
    pub panic: uint8_t,
    pub panic_pc: uint32_t,
    pub panic_addr: uint32_t,
    pub insn_end: *mut cty::c_uchar,
    pub prevlock: cty::c_int,
    pub thislock: cty::c_int,
    pub exception: cty::c_int,
    pub end_of_registers: cty::c_int,
    pub msize: cty::c_int,
    pub profile: cty::c_int,
    pub profile_hist: *mut cty::c_ushort,
    pub memory: *mut cty::c_uchar,
    pub xyram_select: cty::c_int,
    pub xram_start: cty::c_int,
    pub yram_start: cty::c_int,
    pub xmem: *mut cty::c_uchar,
    pub ymem: *mut cty::c_uchar,
    pub xmem_offset: *mut cty::c_uchar,
    pub ymem_offset: *mut cty::c_uchar,
}
// must be a multiple of BUFSIZE
#[no_mangle]
pub static mut lcd_planes: [uint32_t; 5] = [0; 5];
#[no_mangle]
pub static mut lcd_planebufs: [*mut uint8_t; 3] =
    [0 as *const uint8_t as *mut uint8_t; 3];
#[no_mangle]
pub static mut lcd_changed: cty::c_int = 0i32;
#[no_mangle]
pub static mut ngc: cty::c_int = 1i32;
/*
    Grayscale management (common)
*/
unsafe extern "C" fn process_address(mut plane_addr: uint32_t) {
    static mut lcd_addrs: [uint32_t; 16] =
        [0; 16]; // cycle = [cycle_start, cycle_end[ (semi-open interval)
    static mut lcd_buffers: [[uint8_t; 3840]; 48] = [[0; 3840]; 48];
    static mut lcd_cumulative_tickcounts: [cty::c_int; 16] = [0; 16];
    static mut lcd_last_plane_count: cty::c_int = 0;
    static mut lcd_last_planes: [uint32_t; 3] = [0; 3];
    let mut lcd_exptime: [cty::c_int; 3] = [0i32, 0i32, 0i32];
    let mut lcd_apparitions: [cty::c_int; 3] = [0i32, 0i32, 0i32];
    let mut lcd_first_apparition: [cty::c_int; 3] = [0i32, 0i32, 0i32];
    let mut cycle_start: cty::c_int = 0i32;
    let mut cycle_end: cty::c_int = 0;
    static mut cnt: cty::c_int = 0;
    static mut already_reset: cty::c_int = 0;
    let mut tmp: uint32_t = 0;
    // Detect plane switches and sync on them
    if 0 == already_reset {
        let mut current_block_4: u64;
        match lcd_last_plane_count {
            2 => {
                if plane_addr == lcd_last_planes[1usize] {
                    current_block_4 = 11584701595673473500;
                } else if ngc <= 4i32 {
                    cnt += 16i32 - cnt % 16i32;
                    already_reset = 1i32;
                    current_block_4 = 11584701595673473500;
                } else { current_block_4 = 912017249085148103; }
            }
            1 => { current_block_4 = 912017249085148103; }
            0 => { current_block_4 = 10534247358613860045; }
            _ => {
                if plane_addr == lcd_last_planes[0usize] ||
                       plane_addr == lcd_last_planes[1usize] ||
                       plane_addr == lcd_last_planes[2usize] {
                    current_block_4 = 11584701595673473500;
                } else {
                    // We have a third plane we haven't had before. Try resetting.
                    // We have a 4th plane, reset the buffer.
                    cnt += 16i32 - cnt % 16i32;
                    already_reset = 1i32;
                    current_block_4 = 11584701595673473500;
                }
            }
        }
        match current_block_4 {
            912017249085148103 => {
                if plane_addr == lcd_last_planes[0usize] {
                    current_block_4 = 11584701595673473500;
                } else { current_block_4 = 10534247358613860045; }
            }
            _ => { }
        }
        match current_block_4 {
            10534247358613860045 => {
                let fresh0 = lcd_last_plane_count;
                lcd_last_plane_count = lcd_last_plane_count + 1;
                lcd_last_planes[fresh0 as usize] = plane_addr
            }
            _ => { }
        }
    }
    lcd_addrs[(cnt % 16i32) as usize] = plane_addr;
    memcpy(lcd_buffers[(cnt % (16i32 * 3i32)) as usize].as_mut_ptr() as
               *mut cty::c_void,
           &mut *tihw.ram.offset(plane_addr as isize) as *mut uint8_t as
               *const cty::c_void, 3840i32 as cty::c_ulong);
    if plane_addr == lcd_planes[0usize] {
        lcd_planebufs[0usize] =
            lcd_buffers[(cnt % (16i32 * 3i32)) as usize].as_mut_ptr();
        lcd_changed = 1i32
    } else if ngc > 1i32 && plane_addr == lcd_planes[1usize] {
        lcd_planebufs[1usize] =
            lcd_buffers[(cnt % (16i32 * 3i32)) as usize].as_mut_ptr();
        lcd_changed = 1i32
    } else if ngc > 4i32 && plane_addr == lcd_planes[2usize] {
        lcd_planebufs[2usize] =
            lcd_buffers[(cnt % (16i32 * 3i32)) as usize].as_mut_ptr();
        lcd_changed = 1i32
    }
    let fresh1 = cnt;
    cnt = cnt + 1;
    lcd_cumulative_tickcounts[(fresh1 % 16i32) as usize] =
        tihw.lcd_tick as cty::c_int;
    if 0 == cnt % 16i32 {
        let mut np: cty::c_int = 0;
        let mut i: cty::c_int = 0;
        let mut buffer_offset: cty::c_int = 0;
        let mut ngp: cty::c_int = 1i32;
        /* 0 */
        already_reset =
            0i32; // -1 because we'll need to read
		                         // up to cycle_end for the
		                         // exposition times
        lcd_last_plane_count = 0i32;
        cycle_end = 16i32 - 1i32;
        // get address of plane #0
        np = 1i32;
        lcd_planes[4usize] = lcd_addrs[0usize];
        lcd_planes[3usize] = lcd_planes[4usize];
        lcd_planes[2usize] = lcd_planes[3usize];
        lcd_planes[1usize] = lcd_planes[2usize];
        lcd_planes[0usize] = lcd_planes[1usize];
        // get address of plane #1
        i = 1i32;
        while i < 16i32 {
            if lcd_addrs[i as usize] != lcd_planes[0usize] {
                lcd_planes[1usize] = lcd_addrs[i as usize];
                np += 1;
                break ;
            } else { i += 1 }
        }
        // get address of plane #2
        i = 1i32;
        while i < 16i32 {
            if lcd_addrs[i as usize] != lcd_planes[0usize] &&
                   lcd_addrs[i as usize] != lcd_planes[1usize] {
                lcd_planes[2usize] = lcd_addrs[i as usize];
                np += 1;
                break ;
            } else { i += 1 }
        }
        // get address of plane #3 (plane-switching)
        i = 1i32;
        while i < 16i32 {
            if lcd_addrs[i as usize] != lcd_planes[0usize] &&
                   lcd_addrs[i as usize] != lcd_planes[1usize] &&
                   lcd_addrs[i as usize] != lcd_planes[2usize] {
                // plane-switching: set plane switch as last possible end of cycle
                cycle_end = i;
                lcd_planes[3usize] = lcd_addrs[i as usize];
                np += 1;
                break ;
            } else { i += 1 }
        }
        // get address of plane #4 (plane-switching)
        i = 1i32;
        while i < 16i32 {
            if lcd_addrs[i as usize] != lcd_planes[0usize] &&
                   lcd_addrs[i as usize] != lcd_planes[1usize] &&
                   lcd_addrs[i as usize] != lcd_planes[2usize] &&
                   lcd_addrs[i as usize] != lcd_planes[3usize] {
                lcd_planes[4usize] = lcd_addrs[i as usize];
                np += 1;
                break ;
            } else { i += 1 }
        }
        // ignore plane #5 (can be produced by plane-switching, but we don't need it)
        if np < 4i32 {
            ngp = np
        } else if np == 4i32 {
            ngp = 2i32;
            // plane-switching: set plane switch as last possible end of cycle
            i = 1i32;
            while i < 16i32 {
                if lcd_addrs[i as usize] == lcd_planes[2usize] {
                    cycle_end = i;
                    break ;
                } else { i += 1 }
            }
            // reset plane 2
            lcd_planes[2usize] = lcd_planes[0usize]
        } else { ngp = 3i32 }
        // search for a full cycle
		// Use the fact that if there are only 2 planes, plane 2 is the same as plane 0.
        if ngp >= 2i32 {
            // skip the first plane (not a complete exposition)
            i = 0i32;
            while lcd_addrs[i as usize] == lcd_addrs[0usize] { i += 1 }
            while i <= cycle_end {
                // only count an apparition if the plane has actually changed
                if lcd_addrs[i as usize] == lcd_planes[0usize] &&
                       lcd_addrs[(i - 1i32) as usize] != lcd_planes[0usize] {
                    let fresh2 = lcd_apparitions[0usize];
                    lcd_apparitions[0usize] = lcd_apparitions[0usize] + 1;
                    if 0 == fresh2 { lcd_first_apparition[0usize] = i }
                }
                if lcd_addrs[i as usize] == lcd_planes[1usize] &&
                       lcd_addrs[(i - 1i32) as usize] != lcd_planes[1usize] {
                    let fresh3 = lcd_apparitions[1usize];
                    lcd_apparitions[1usize] = lcd_apparitions[1usize] + 1;
                    if 0 == fresh3 { lcd_first_apparition[1usize] = i }
                }
                if lcd_addrs[i as usize] == lcd_planes[2usize] &&
                       lcd_addrs[(i - 1i32) as usize] != lcd_planes[2usize] {
                    let fresh4 = lcd_apparitions[2usize];
                    lcd_apparitions[2usize] = lcd_apparitions[2usize] + 1;
                    if 0 == fresh4 { lcd_first_apparition[2usize] = i }
                }
                // stop (WITHOUT incrementing i) if all planes appeared at least twice
                if lcd_apparitions[0usize] >= 2i32 &&
                       lcd_apparitions[1usize] >= 2i32 &&
                       lcd_apparitions[2usize] >= 2i32 {
                    break ;
                }
                i += 1
            }
            if i > cycle_end {
                //				printf("Warning: no full grayscale cycle found (BUFSIZE too small?)\n");
                cycle_start = 0i32
            } else {
                if lcd_addrs[i as usize] == lcd_planes[0usize] {
                    cycle_start = lcd_first_apparition[0usize]
                } else if lcd_addrs[i as usize] == lcd_planes[1usize] {
                    cycle_start = lcd_first_apparition[1usize]
                } else if lcd_addrs[i as usize] == lcd_planes[2usize] {
                    cycle_start = lcd_first_apparition[2usize]
                }
                cycle_end = i
            }
            // compute exposure time of plane #0 within the cycle
            i = cycle_start;
            while i < cycle_end {
                if lcd_addrs[i as usize] == lcd_planes[0usize] {
                    lcd_exptime[0usize] +=
                        lcd_cumulative_tickcounts[(i + 1i32) as usize] -
                            lcd_cumulative_tickcounts[i as usize]
                }
                i += 1
            }
            // compute exposure time of plane #1 within the cycle
            i = cycle_start;
            while i < cycle_end {
                if lcd_addrs[i as usize] == lcd_planes[1usize] {
                    lcd_exptime[1usize] +=
                        lcd_cumulative_tickcounts[(i + 1i32) as usize] -
                            lcd_cumulative_tickcounts[i as usize]
                }
                i += 1
            }
            // compute exposure time of plane #2 within the cycle
            i = cycle_start;
            while i < cycle_end {
                if lcd_addrs[i as usize] == lcd_planes[2usize] {
                    lcd_exptime[2usize] +=
                        lcd_cumulative_tickcounts[(i + 1i32) as usize] -
                            lcd_cumulative_tickcounts[i as usize]
                }
                i += 1
            }
            // sort plane addresses by exposition times
            if lcd_exptime[0usize] < lcd_exptime[1usize] {
                tmp = lcd_planes[0usize];
                lcd_planes[0usize] = lcd_planes[1usize];
                lcd_planes[1usize] = tmp;
                tmp = lcd_exptime[0usize] as uint32_t;
                lcd_exptime[0usize] = lcd_exptime[1usize];
                lcd_exptime[1usize] = tmp as cty::c_int
            }
            if ngp >= 3i32 && lcd_exptime[0usize] < lcd_exptime[2usize] {
                tmp = lcd_planes[0usize];
                lcd_planes[0usize] = lcd_planes[2usize];
                lcd_planes[2usize] = tmp;
                tmp = lcd_exptime[0usize] as uint32_t;
                lcd_exptime[0usize] = lcd_exptime[2usize];
                lcd_exptime[2usize] = tmp as cty::c_int
            }
            if ngp >= 3i32 && lcd_exptime[1usize] < lcd_exptime[2usize] {
                tmp = lcd_planes[1usize];
                lcd_planes[1usize] = lcd_planes[2usize];
                lcd_planes[2usize] = tmp;
                tmp = lcd_exptime[1usize] as uint32_t;
                lcd_exptime[1usize] = lcd_exptime[2usize];
                lcd_exptime[2usize] = tmp as cty::c_int
            }
        }
        /* 0 */
        // now, determine number of grayscales (kevin)
        if ngp == 1i32 {
            ngc = 1i32
        } else if ngp == 2i32 {
            ngc = 3i32
        } else if ngp == 3i32 {
            // check using fast integer computation and no possible divisions by 0
			// whether lcd_exptime[0] / lcd_exptime[2] > 3.5 = 7/2
            if lcd_exptime[0usize] * 2i32 > lcd_exptime[2usize] * 7i32 {
                ngc = 8i32
            } else { ngc = 7i32 }
        }
        // set the pointers to the plane buffers
        match cnt / 16i32 % 3i32 {
            1 => { buffer_offset = 0i32 }
            2 => { buffer_offset = 16i32 }
            _ => { buffer_offset = 2i32 * 16i32 }
        }
        i = 16i32 - 1i32;
        while i >= 0i32 {
            if lcd_addrs[i as usize] == lcd_planes[0usize] {
                lcd_planebufs[0usize] =
                    lcd_buffers[(i + buffer_offset) as usize].as_mut_ptr();
                break ;
            } else { i -= 1 }
        }
        if ngp >= 2i32 {
            i = 16i32 - 1i32;
            while i >= 0i32 {
                if lcd_addrs[i as usize] == lcd_planes[1usize] {
                    lcd_planebufs[1usize] =
                        lcd_buffers[(i + buffer_offset) as
                                        usize].as_mut_ptr();
                    break ;
                } else { i -= 1 }
            }
        }
        if ngp >= 3i32 {
            i = 16i32 - 1i32;
            while i >= 0i32 {
                if lcd_addrs[i as usize] == lcd_planes[2usize] {
                    lcd_planebufs[2usize] =
                        lcd_buffers[(i + buffer_offset) as
                                        usize].as_mut_ptr();
                    break ;
                } else { i -= 1 }
            }
        }
        lcd_changed = 1i32
    };
}
/*
    HW1 grayscale management
*/
#[no_mangle]
pub unsafe extern "C" fn lcd_hook_hw1() { process_address(tihw.lcd_adr); }
/*
    HW2 grayscale management
*/
#[no_mangle]
pub unsafe extern "C" fn lcd_hook_hw2(mut refresh: cty::c_int) {
    static mut dead_cnt: cty::c_int = 0i32;
    static mut d0d7a2a6_moveml_d0d7a2a6_a1: [cty::c_char; 6] =
        [0x7ci32 as cty::c_char, 0xffi32 as cty::c_char,
         0x48i32 as cty::c_char, 0xd1i32 as cty::c_char,
         0x7ci32 as cty::c_char, 0xffi32 as cty::c_char];
    static mut d1d7a2a6_moveml_d1d7a2a6_a1: [cty::c_char; 6] =
        [0x7ci32 as cty::c_char, 0xfei32 as cty::c_char,
         0x48i32 as cty::c_char, 0xd1i32 as cty::c_char,
         0x7ci32 as cty::c_char, 0xfei32 as cty::c_char];
    static mut d0d7a2a7_moveml_d0d7a2a7_a1: [cty::c_char; 6] =
        [0xfci32 as cty::c_char, 0xffi32 as cty::c_char,
         0x48i32 as cty::c_char, 0xd1i32 as cty::c_char,
         0xfci32 as cty::c_char, 0xffi32 as cty::c_char];
    // if refresh from GTK (calc.c), set 1 plane, read directly from LCD_MEM
    if 0 != refresh {
        dead_cnt += 1;
        if dead_cnt < 5i32 { return }
        lcd_planes[0usize] = tihw.lcd_adr;
        lcd_planebufs[0usize] =
            &mut *tihw.ram.offset(tihw.lcd_adr as isize) as *mut uint8_t;
        ngc = 1i32;
        lcd_changed = 1i32
    } else {
        // if refresh from CPU loop (m68k.c), search for opcode signature:
        let mut pc_p: *mut cty::c_uchar = regs.pc_p;
        let fresh5 = pc_p;
        pc_p = pc_p.offset(1);
        if *fresh5 as cty::c_int == 0x4ci32 &&
               {
                   let fresh6 = pc_p;
                   pc_p = pc_p.offset(1);
                   *fresh6 as cty::c_int == 0xd8i32
               } {
            // search for the movem (%a0)+,... instruction
            if 0 ==
                   memcmp(pc_p as *const cty::c_void,
                          d0d7a2a6_moveml_d0d7a2a6_a1.as_ptr() as
                              *const cty::c_void, 6i32 as cty::c_ulong) ||
                   0 ==
                       memcmp(pc_p as *const cty::c_void,
                              d0d7a2a7_moveml_d0d7a2a7_a1.as_ptr() as
                                  *const cty::c_void, 6i32 as cty::c_ulong)
               {
                let mut a0: uint32_t =
                    (*regs.regs.as_mut_ptr().offset(8isize).offset(0isize)).wrapping_sub(0xa00i32
                                                                                             as
                                                                                             cty::c_uint);
                let mut a1: uint32_t =
                    (*regs.regs.as_mut_ptr().offset(8isize).offset(1isize)).wrapping_sub(0xa00i32
                                                                                             as
                                                                                             cty::c_uint);
                if a1 == 0x4c0ci32 as cty::c_uint {
                    a0 =
                        (a0 as
                             cty::c_uint).wrapping_sub(12i32 as cty::c_uint)
                            as uint32_t as uint32_t
                } else if a1 != 0x4c00i32 as cty::c_uint { return }
                process_address(a0);
                dead_cnt = 0i32
            } else if 0 ==
                          memcmp(pc_p as *const cty::c_void,
                                 d1d7a2a6_moveml_d1d7a2a6_a1.as_ptr() as
                                     *const cty::c_void,
                                 6i32 as cty::c_ulong) {
                let mut a0_0: uint32_t =
                    *regs.regs.as_mut_ptr().offset(8isize).offset(0isize);
                let mut a1_0: uint32_t =
                    *regs.regs.as_mut_ptr().offset(8isize).offset(1isize);
                if a1_0 != 0x4c00i32 as cty::c_uint { return }
                process_address(a0_0);
                dead_cnt = 0i32
            }
        }
    };
}
// opcode signature:
// TIGCC grayscale lib and most others:
//		movem.l  (%a0)+,%d0-%d7/%a2-%a6 ; (%a1)==0x4c00+0xa00 => plane:=-0xa00(%a0)
// graphlib-titanik and graphlib-iceberg:
//		movem.l  (%a0)+,%d0-%d7/%a2-%a6 ; (%a1)==0x4c00+0xa00+12 => plane:=-0xa00-12(%a0)
// Patrick Davidson's gray.asm:
//		movem.l  (%a0)+,%d1-%d7/%a2-%a6 ; (%a1)==0x4c00 => plane:=(%a0)
// Grib:
//		movem.l  (%a0)+,%d0-%d7/%a2-%a7 ; (%a1)==0x4c00+0xa00 => plane:=-0xa00(%a0)
