#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
use crate::libc::*;
use crate::emulator::core::dbg::bkpts::{ti68k_bkpt_clear_address, ti68k_bkpt_clear_exception};

extern "C" {
    #[no_mangle]
    fn malloc(_: cty::c_ulong) -> *mut cty::c_void;
    #[no_mangle]
    fn free(__ptr: *mut cty::c_void);
    #[no_mangle]
    static mut tihw: Ti68kHardware;
    #[no_mangle]
    static mut logger: Ti68kLogging;
    #[no_mangle]
    fn Exception(_: cty::c_int, _: uintptr_t);
    #[no_mangle]
    fn Interrupt(_: cty::c_int);
    #[no_mangle]
    static mut currIntLev: cty::c_int;
    #[no_mangle]
    static mut regs: regstruct;
    #[no_mangle]
    fn init_m68k();
    #[no_mangle]
    fn m68k_reset();
    #[no_mangle]
    static mut cpufunctbl:
           [Option<unsafe extern "C" fn(_: uint32_t)
                       -> cty::c_ulong>; 65536];
    #[no_mangle]
    fn find_ssp_and_pc(ssp: *mut uint32_t, pc: *mut uint32_t);
    #[no_mangle]
    fn lcd_hook_hw2(refresh: cty::c_int);
    #[no_mangle]
    fn hw_update() -> cty::c_int;
    #[no_mangle]
    static mut cycle_instr: cty::c_uint;
    #[no_mangle]
    static mut cycle_count: cty::c_uint;
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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct Ti68kLogging {
    pub pclog_size: cty::c_int,
    pub pclog_buf: *mut uint32_t,
    pub pclog_ptr: cty::c_int,
    pub link_size: cty::c_int,
    pub link_buf: *mut uint16_t,
    pub link_ptr: cty::c_int,
    pub link_mask: cty::c_int,
}
pub type cpuop_func = unsafe extern "C" fn(_: uint32_t) -> cty::c_ulong;
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
#[inline]
unsafe extern "C" fn do_get_mem_word(mut a: *mut uint16_t) -> uint16_t {
    let mut b: *mut uint8_t = a as *mut uint8_t;
    return ((*b as cty::c_int) << 8i32 | *b.offset(1isize) as cty::c_int) as
               uint16_t;
}
#[inline(always)]
unsafe extern "C" fn set_special(mut x: uint32_t) { regs.spcflags |= x; }
#[inline(always)]
unsafe extern "C" fn unset_special(mut x: uint32_t) { regs.spcflags &= !x; }
#[inline(always)]
unsafe extern "C" fn m68k_getpc() -> uintptr_t {
    return (regs.pc as cty::c_long +
                (regs.pc_p as
                     *mut cty::c_char).wrapping_offset_from(regs.pc_oldp as
                                                                 *mut cty::c_char)
                    as cty::c_long) as uintptr_t;
}
#[inline]
unsafe extern "C" fn do_cycles() {
    if cycle_count >= cycle_instr {
        hw_update();
        cycle_count = cycle_count.wrapping_sub(cycle_instr)
    };
}
/* Hey EMACS -*- linux-c -*- */
/* $Id: m68k.c 2733 2007-12-21 05:33:19Z kevinkofler $ */
/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
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
    M68K management (wrapper for the UAE engine)
*/
#[no_mangle]
pub static mut pending_ints: cty::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn hw_m68k_init() -> cty::c_int {
    // init breakpoints
    // init instruction logging
    logger.pclog_size = 11i32;
    logger.pclog_buf =
        malloc((logger.pclog_size as
                    cty::c_ulong).wrapping_mul(::core::mem::size_of::<uint32_t>()
                                                    as cty::c_ulong)) as
            *mut uint32_t;
    if logger.pclog_buf.is_null() { return 776i32 }
    logger.pclog_ptr = 0i32;
    init_m68k();
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn hw_m68k_reset() -> cty::c_int {
    // retrieve SSP & PC values for boot
    find_ssp_and_pc(&mut tihw.initial_ssp, &mut tihw.initial_pc);
    // and reset
    m68k_reset();
    pending_ints = 0i32;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn hw_m68k_exit() -> cty::c_int {
    ti68k_bkpt_clear_address();
    ti68k_bkpt_clear_exception();
    free(logger.pclog_buf as *mut cty::c_void);
    return 0i32;
}
// UAE does not implement interrupt priority and pending interrupts
// I re-implement it as replacement of Interrupt()
#[no_mangle]
pub unsafe extern "C" fn Interrupt2(mut nr: cty::c_int) {
    if nr > regs.intmask || nr == 7i32 {
        Interrupt(nr);
        pending_ints &= !(1i32 << nr)
        // clr pending int
    }; // unused, used for compat with old Interrupt()
}
#[no_mangle]
pub unsafe extern "C" fn hw_m68k_irq(mut n: cty::c_int) {
    set_special(8i32 as uint32_t);
    currIntLev = n;
    pending_ints |= 1i32 << n;
    // set pending int
}
// parse pending ints for one to raise (up to low prority order)
/* Replace UAE's M68000_run() */
/*
    Returns cycle count.
*/
static mut cycles: cty::c_uint = 0i32 as cty::c_uint;
#[no_mangle]
pub unsafe extern "C" fn hw_m68k_get_cycle_count(mut reset: cty::c_int)
 -> cty::c_uint {
    if 0 != reset { cycles = 0i32 as cty::c_uint }
    return cycles;
}
/*
  Do 'n' instructions (up to 'maxcycles', unless set to 0).
  Returned value:
  - 1 if breakpoint has been encountered,
  - 2 if trace,
  - 0 otherwise.
*/
#[no_mangle]
pub unsafe extern "C" fn hw_m68k_run(mut n: cty::c_int,
                                     mut maxcycles: cty::c_uint)
 -> cty::c_int {
    let mut i: cty::c_int = n;
    let mut cycles_at_start: cty::c_uint = cycles;
    i = 0i32;
    while i < n &&
              (0 == maxcycles ||
                   cycles.wrapping_sub(cycles_at_start) < maxcycles) {
        let mut opcode: uint32_t = 0;
        let mut insn_cycles: cty::c_uint = 0;
        // refresh hardware
        do_cycles();
        // OSC1 stopped ? Refresh hardware and wake-up on interrupt. No opcode execution.
        if 0 != regs.spcflags & 4i32 as cty::c_uint {
            if 0 != pending_ints {
                let mut level: cty::c_int = 0;
                let mut mask: cty::c_int = 0x80i32;
                level = 7i32;
                while 0 != level {
                    if 0 != pending_ints & mask { break ; }
                    level -= 1;
                    mask >>= 1i32
                }
                // wake-up on int level 6 (ON key) or level 1..5
                if 0 !=
                       pending_ints &
                           (*tihw.io.offset(5isize) as cty::c_int) << 1i32 ||
                       level == 6i32 {
                    Interrupt2(level); // cycle count for hw_m68k_run loop
                    regs.stopped =
                        0i32 as flagtype; // cycle count for hw.c timers
                    regs.spcflags &= !4i32 as cty::c_uint
                }
            } // used by grayscale for time plane exposure
            cycles = cycles.wrapping_add(4i32 as cty::c_uint);
            cycle_count = cycle_count.wrapping_add(4i32 as cty::c_uint);
            tihw.lcd_tick = tihw.lcd_tick.wrapping_add(4i32 as cty::c_ulong)
        } else {
            // store PC in the log buffer
            if logger.pclog_size > 1i32 {
                let fresh0 = logger.pclog_ptr;
                logger.pclog_ptr = logger.pclog_ptr + 1;
                *logger.pclog_buf.offset(fresh0 as isize) =
                    m68k_getpc() as uint32_t;
                if logger.pclog_ptr >= logger.pclog_size {
                    logger.pclog_ptr = 0i32
                }
            }
            // search for next opcode and execute it
            opcode =
                (if 0 != 0i32 {
                     do_get_mem_word(regs.pc_p.offset(0isize) as
                                         *mut uint16_t) as cty::c_int
                 } else { regs.ir as cty::c_int }) as
                    uint32_t; // increments PC automatically now
            insn_cycles =
                cpufunctbl[opcode as
                               usize].expect("non-null function pointer")(opcode).wrapping_mul(2i32
                                                                                                   as
                                                                                                   cty::c_ulong)
                    as cty::c_uint; // cycle count for hw_m68k_run loop
            cycles =
                cycles.wrapping_add(insn_cycles); // cycle count for hw.c timers
            cycle_count =
                cycle_count.wrapping_add(insn_cycles); // used by grayscale for time plane exposure
            tihw.lcd_tick =
                tihw.lcd_tick.wrapping_add(insn_cycles as cty::c_ulong);
            // HW2/3 grayscales management
            lcd_hook_hw2(0i32);
            // process (pending) interrupts
            if 0 != pending_ints {
                let mut level_0: cty::c_int = 0;
                let mut mask_0: cty::c_int = 0x80i32;
                level_0 = 7i32;
                while 0 != level_0 {
                    if 0 != pending_ints & mask_0 { break ; }
                    level_0 -= 1;
                    mask_0 >>= 1i32
                }
                Interrupt2(level_0);
                regs.stopped = 0i32 as flagtype
                // regs.spcflags &= ~SPCFLAG_STOP;
            }
            // management of special flags
            if 0 != regs.spcflags {
                if 0 != regs.spcflags & 256i32 as cty::c_uint {
                    Exception(3i32, 0i32 as uintptr_t);
                    unset_special(256i32 as uint32_t);
                }
                if 0 != regs.spcflags & 64i32 as cty::c_uint {
                    Exception(9i32, 0i32 as uintptr_t);
                }
                if 0 != regs.spcflags & 32i32 as cty::c_uint {
                    unset_special(32i32 as uint32_t);
                }
                if 0 != regs.spcflags & 16i32 as cty::c_uint {
                    unset_special(16i32 as uint32_t);
                    return 1i32
                }
                if 0 != regs.spcflags & 1i32 as cty::c_uint {
                    unset_special(1i32 as uint32_t);
                    return 2i32
                }
                if 0 != regs.spcflags & 2i32 as cty::c_uint {
                    unset_special(2i32 as uint32_t);
                }
            }
        }
        i += 1
    }
    return 0i32;
}
