#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
use crate::libc::*;
extern crate c2rust_bitfields;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn hw_get_word(addr: uint32_t) -> uint16_t;
    #[no_mangle]
    fn hw_get_long(addr: uint32_t) -> uint32_t;
    #[no_mangle]
    fn hw_put_word(addr: uint32_t, arg: uint16_t);
    #[no_mangle]
    fn hw_put_long(addr: uint32_t, arg: uint32_t);
    #[no_mangle]
    fn hw_get_real_address(addr: uint32_t) -> *mut uint8_t;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const cty::c_char, _: ...) -> cty::c_int;
    #[no_mangle]
    fn printf(_: *const cty::c_char, _: ...) -> cty::c_int;
    #[no_mangle]
    fn sprintf(_: *mut cty::c_char, _: *const cty::c_char, _: ...)
     -> cty::c_int;
    #[no_mangle]
    fn snprintf(_: *mut cty::c_char, _: cty::c_ulong,
                _: *const cty::c_char, _: ...) -> cty::c_int;
    #[no_mangle]
    static mut lookuptab: [mnemolookup; 0];
    #[no_mangle]
    static mut table68k: *mut instr;
    #[no_mangle]
    fn read_table68k();
    #[no_mangle]
    fn do_merges();
    #[no_mangle]
    static mut nr_cpuop_funcs: cty::c_int;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    static mut op_smalltbl_0_ff: [cputbl; 0];
    #[no_mangle]
    static mut op_smalltbl_1_ff: [cputbl; 0];
    #[no_mangle]
    static mut op_smalltbl_2_ff: [cputbl; 0];
    #[no_mangle]
    static mut op_smalltbl_3_ff: [cputbl; 0];
    /* 68000 slow but compatible.  */
    #[no_mangle]
    static mut op_smalltbl_5_ff: [cputbl; 0];
    #[no_mangle]
    fn __assert_fail(__assertion: *const cty::c_char,
                     __file: *const cty::c_char, __line: cty::c_uint,
                     __function: *const cty::c_char) -> !;
    #[no_mangle]
    fn strcpy(_: *mut cty::c_char, _: *const cty::c_char)
     -> *mut cty::c_char;
    #[no_mangle]
    fn strncpy(_: *mut cty::c_char, _: *const cty::c_char, _: cty::c_ulong)
     -> *mut cty::c_char;
    #[no_mangle]
    fn strcat(_: *mut cty::c_char, _: *const cty::c_char)
     -> *mut cty::c_char;
    #[no_mangle]
    fn strstr(_: *const cty::c_char, _: *const cty::c_char)
     -> *mut cty::c_char;
    #[no_mangle]
    static mut tihw: Ti68kHardware;
    /* 0 */
    #[no_mangle]
    fn DasmFPU(code: uint16_t, buf: *mut cty::c_char) -> cty::c_int;
}
pub type __int8_t = cty::c_schar;
pub type __uint8_t = cty::c_uchar;
pub type __int16_t = cty::c_short;
pub type __uint16_t = cty::c_ushort;
pub type __int32_t = cty::c_int;
pub type __uint32_t = cty::c_uint;
pub type __off_t = cty::c_long;
pub type __off64_t = cty::c_long;
pub type __time_t = cty::c_long;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = cty::c_ulong;
pub type size_t = cty::c_ulong;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: cty::c_int,
    pub _IO_read_ptr: *mut cty::c_char,
    pub _IO_read_end: *mut cty::c_char,
    pub _IO_read_base: *mut cty::c_char,
    pub _IO_write_base: *mut cty::c_char,
    pub _IO_write_ptr: *mut cty::c_char,
    pub _IO_write_end: *mut cty::c_char,
    pub _IO_buf_base: *mut cty::c_char,
    pub _IO_buf_end: *mut cty::c_char,
    pub _IO_save_base: *mut cty::c_char,
    pub _IO_backup_base: *mut cty::c_char,
    pub _IO_save_end: *mut cty::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: cty::c_int,
    pub _flags2: cty::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: cty::c_ushort,
    pub _vtable_offset: cty::c_schar,
    pub _shortbuf: [cty::c_char; 1],
    pub _lock: *mut cty::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut cty::c_void,
    pub __pad5: size_t,
    pub _mode: cty::c_int,
    pub _unused2: [cty::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
/* Hey EMACS -*- linux-c -*- */
/* $Id: readcpu.h 2681 2007-11-20 18:01:02Z roms $ */
pub type amodes = cty::c_uint;
pub const am_illg: amodes = 17;
pub const am_unknown: amodes = 16;
pub const immi: amodes = 15;
pub const imm2: amodes = 14;
pub const imm1: amodes = 13;
pub const imm0: amodes = 12;
pub const imm: amodes = 11;
pub const PC8r: amodes = 10;
pub const PC16: amodes = 9;
pub const absl: amodes = 8;
pub const absw: amodes = 7;
pub const Ad8r: amodes = 6;
pub const Ad16: amodes = 5;
pub const Apdi: amodes = 4;
pub const Aipi: amodes = 3;
pub const Aind: amodes = 2;
pub const Areg: amodes = 1;
pub const Dreg: amodes = 0;
pub type instrmnem = cty::c_uint;
pub const i_MMUOP: instrmnem = 127;
pub const i_MOVE16: instrmnem = 126;
pub const i_CPUSHA: instrmnem = 125;
pub const i_CPUSHP: instrmnem = 124;
pub const i_CPUSHL: instrmnem = 123;
pub const i_CINVA: instrmnem = 122;
pub const i_CINVP: instrmnem = 121;
pub const i_CINVL: instrmnem = 120;
pub const i_FRESTORE: instrmnem = 119;
pub const i_FSAVE: instrmnem = 118;
pub const i_FBcc: instrmnem = 117;
pub const i_FTRAPcc: instrmnem = 116;
pub const i_FScc: instrmnem = 115;
pub const i_FDBcc: instrmnem = 114;
pub const i_FPP: instrmnem = 113;
pub const i_MOVES: instrmnem = 112;
pub const i_TRAPcc: instrmnem = 111;
pub const i_RTM: instrmnem = 110;
pub const i_CALLM: instrmnem = 109;
pub const i_BKPT: instrmnem = 108;
pub const i_TAS: instrmnem = 107;
pub const i_UNPK: instrmnem = 106;
pub const i_PACK: instrmnem = 105;
pub const i_BFINS: instrmnem = 104;
pub const i_BFSET: instrmnem = 103;
pub const i_BFFFO: instrmnem = 102;
pub const i_BFCLR: instrmnem = 101;
pub const i_BFEXTS: instrmnem = 100;
pub const i_BFCHG: instrmnem = 99;
pub const i_BFEXTU: instrmnem = 98;
pub const i_BFTST: instrmnem = 97;
pub const i_MULL: instrmnem = 96;
pub const i_DIVL: instrmnem = 95;
pub const i_CAS2: instrmnem = 94;
pub const i_CAS: instrmnem = 93;
pub const i_MOVE2C: instrmnem = 92;
pub const i_MOVEC2: instrmnem = 91;
pub const i_CHK2: instrmnem = 90;
pub const i_CHK: instrmnem = 89;
pub const i_ROXRW: instrmnem = 88;
pub const i_ROXLW: instrmnem = 87;
pub const i_RORW: instrmnem = 86;
pub const i_ROLW: instrmnem = 85;
pub const i_LSLW: instrmnem = 84;
pub const i_LSRW: instrmnem = 83;
pub const i_ASLW: instrmnem = 82;
pub const i_ASRW: instrmnem = 81;
pub const i_ROXR: instrmnem = 80;
pub const i_ROXL: instrmnem = 79;
pub const i_ROR: instrmnem = 78;
pub const i_ROL: instrmnem = 77;
pub const i_LSL: instrmnem = 76;
pub const i_LSR: instrmnem = 75;
pub const i_ASL: instrmnem = 74;
pub const i_ASR: instrmnem = 73;
pub const i_MULS: instrmnem = 72;
pub const i_MULU: instrmnem = 71;
pub const i_DIVS: instrmnem = 70;
pub const i_DIVU: instrmnem = 69;
pub const i_Scc: instrmnem = 68;
pub const i_DBcc: instrmnem = 67;
pub const i_PEA: instrmnem = 66;
pub const i_LEA: instrmnem = 65;
pub const i_Bcc: instrmnem = 64;
pub const i_BSR: instrmnem = 63;
pub const i_JMP: instrmnem = 62;
pub const i_JSR: instrmnem = 61;
pub const i_RTR: instrmnem = 60;
pub const i_TRAPV: instrmnem = 59;
pub const i_RTS: instrmnem = 58;
pub const i_UNLK: instrmnem = 57;
pub const i_LINK: instrmnem = 56;
pub const i_RTD: instrmnem = 55;
pub const i_RTE: instrmnem = 54;
pub const i_STOP: instrmnem = 53;
pub const i_NOP: instrmnem = 52;
pub const i_RESET: instrmnem = 51;
pub const i_MVUSP2R: instrmnem = 50;
pub const i_MVR2USP: instrmnem = 49;
pub const i_TRAP: instrmnem = 48;
pub const i_MVMLE: instrmnem = 47;
pub const i_MVMEL: instrmnem = 46;
pub const i_EXT: instrmnem = 45;
pub const i_EXG: instrmnem = 44;
pub const i_SWAP: instrmnem = 43;
pub const i_MV2SR: instrmnem = 42;
pub const i_MVSR2: instrmnem = 41;
pub const i_MOVEA: instrmnem = 40;
pub const i_MOVE: instrmnem = 39;
pub const i_MVPMR: instrmnem = 38;
pub const i_MVPRM: instrmnem = 37;
pub const i_CMPA: instrmnem = 36;
pub const i_CMPM: instrmnem = 35;
pub const i_CMP: instrmnem = 34;
pub const i_BSET: instrmnem = 33;
pub const i_BCLR: instrmnem = 32;
pub const i_BCHG: instrmnem = 31;
pub const i_BTST: instrmnem = 30;
pub const i_TST: instrmnem = 29;
pub const i_NOT: instrmnem = 28;
pub const i_CLR: instrmnem = 27;
pub const i_NBCD: instrmnem = 26;
pub const i_NEGX: instrmnem = 25;
pub const i_NEG: instrmnem = 24;
pub const i_ABCD: instrmnem = 23;
pub const i_ADDX: instrmnem = 22;
pub const i_ADDA: instrmnem = 21;
pub const i_ADD: instrmnem = 20;
pub const i_SBCD: instrmnem = 19;
pub const i_SUBX: instrmnem = 18;
pub const i_SUBA: instrmnem = 17;
pub const i_SUB: instrmnem = 16;
pub const i_EORSR: instrmnem = 15;
pub const i_ANDSR: instrmnem = 14;
pub const i_ORSR: instrmnem = 13;
pub const i_EOR: instrmnem = 12;
pub const i_AND: instrmnem = 11;
pub const i_OR: instrmnem = 10;
pub const i_ILLG: instrmnem = 9;
pub const i_EORI: instrmnem = 8;
pub const i_ANDI: instrmnem = 7;
pub const i_ORI: instrmnem = 6;
pub const i_SUBI: instrmnem = 5;
pub const i_ADDI: instrmnem = 4;
pub const i_CMPI: instrmnem = 3;
pub const i_SUBQ: instrmnem = 2;
pub const i_ADDQ: instrmnem = 1;
pub const i_MOVEQ: instrmnem = 0;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct mnemolookup {
    pub mnemo: instrmnem,
    pub name: *const cty::c_char,
}
pub type wordsizes = cty::c_uint;
pub const sz_long: wordsizes = 2;
pub const sz_word: wordsizes = 1;
pub const sz_byte: wordsizes = 0;
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct instr {
    pub handler: cty::c_long,
    pub dreg: cty::c_uchar,
    pub sreg: cty::c_uchar,
    pub dpos: cty::c_schar,
    pub spos: cty::c_schar,
    pub sduse: cty::c_uchar,
    #[bitfield(name = "flagdead", ty = "cty::c_int", bits = "0..=7")]
    #[bitfield(name = "flaglive", ty = "cty::c_int", bits = "8..=15")]
    #[bitfield(name = "mnemo", ty = "cty::c_uint", bits = "16..=23")]
    #[bitfield(name = "cc", ty = "cty::c_uint", bits = "24..=27")]
    #[bitfield(name = "plev", ty = "cty::c_uint", bits = "28..=29")]
    #[bitfield(name = "size", ty = "cty::c_uint", bits = "30..=31")]
    #[bitfield(name = "smode", ty = "cty::c_uint", bits = "32..=36")]
    #[bitfield(name = "stype", ty = "cty::c_uint", bits = "37..=39")]
    #[bitfield(name = "dmode", ty = "cty::c_uint", bits = "40..=44")]
    #[bitfield(name = "suse", ty = "cty::c_uint", bits = "45..=45")]
    #[bitfield(name = "duse", ty = "cty::c_uint", bits = "46..=46")]
    #[bitfield(name = "unused1", ty = "cty::c_uint", bits = "47..=47")]
    #[bitfield(name = "clev", ty = "cty::c_uint", bits = "48..=50")]
    #[bitfield(name = "isjmp", ty = "cty::c_uint", bits = "51..=51")]
    #[bitfield(name = "unused2", ty = "cty::c_uint", bits = "52..=55")]
    pub flagdead_flaglive_mnemo_cc_plev_size_smode_stype_dmode_suse_duse_unused1_clev_isjmp_unused2: [u8; 7],
    #[bitfield(padding)]
    pub _pad: [u8; 4],
}
pub type time_t = __time_t;
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
pub type flagtype = cty::c_char;
pub type cpuop_func = unsafe extern "C" fn(_: uint32_t) -> cty::c_ulong;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct cputbl {
    pub handler: Option<unsafe extern "C" fn(_: uint32_t) -> cty::c_ulong>,
    pub specific: cty::c_int,
    pub opcode: uint16_t,
}
/*
 * UAE - The Un*x Amiga Emulator
 *
 * Stuff
 *
 * Copyright 1995, 1996 Ed Hanway
 * Copyright 1995-2001 Bernd Schmidt
 * $Id: options.h 2009 2006-02-25 06:33:50Z kevinkofler $
 */
// FILE
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct uae_prefs {
    pub cpu_level: cty::c_int,
    pub cpu_compatible: cty::c_int,
    pub address_space_24: cty::c_int,
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
pub struct TTIME {
    pub s: time_t,
    pub ms: cty::c_int,
}
/*
 * UAE - The Un*x Amiga Emulator
 *
 * Memory access functions
 *
 * Copyright 1996 Bernd Schmidt
 * $Id: maccess.h 1695 2005-08-26 21:05:37Z kevinkofler $
 */
#[inline]
unsafe extern "C" fn do_get_mem_long(mut a: *mut uint32_t) -> uint32_t {
    let mut b: *mut uint8_t = a as *mut uint8_t;
    return ((*b as cty::c_int) << 24i32 |
                (*b.offset(1isize) as cty::c_int) << 16i32 |
                (*b.offset(2isize) as cty::c_int) << 8i32 |
                *b.offset(3isize) as cty::c_int) as uint32_t;
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
#[inline(always)]
unsafe extern "C" fn m68k_setpc(mut newpc: uintptr_t) {
    regs.pc_oldp = hw_get_real_address(newpc as uint32_t);
    regs.pc_p = regs.pc_oldp;
    regs.pc = (newpc & 0xffffffi32 as cty::c_ulong) as uint32_t;
}
#[inline(always)]
unsafe extern "C" fn next_ilong() -> uint32_t {
    let mut r: uint32_t =
        do_get_mem_long(regs.pc_p.offset(0isize) as *mut uint32_t);
    regs.pc_p = regs.pc_p.offset(4isize);
    return r;
}
#[inline(always)]
unsafe extern "C" fn next_iword() -> uint32_t {
    let mut r: uint32_t =
        do_get_mem_word(regs.pc_p.offset(0isize) as *mut uint16_t) as
            uint32_t;
    regs.pc_p = regs.pc_p.offset(2isize);
    return r;
}
/*
 * UAE - The Un*x Amiga Emulator
 *
 * MC68000 emulation - machine dependent bits
 *
 * Copyright 1996 Bernd Schmidt
 * $Id: m68k.h 1695 2005-08-26 21:05:37Z kevinkofler $
 */
#[inline]
unsafe extern "C" fn cctrue(cc: cty::c_int) -> cty::c_int {
    match cc {
        0 => {
            return 1i32
            /* LE */
        }
        1 => { return 0i32 }
        2 => { /* F */
            return (0 == regs.flags.c && 0 == regs.flags.z) as cty::c_int
        }
        3 => { /* HI */
            return (0 != regs.flags.c || 0 != regs.flags.z) as cty::c_int
        }
        4 => { /* LS */
            return (0 == regs.flags.c) as cty::c_int
        }
        5 => { /* CC */
            return regs.flags.c as cty::c_int
        }
        6 => { /* CS */
            return (0 == regs.flags.z) as cty::c_int
        }
        7 => { /* NE */
            return regs.flags.z as cty::c_int
        }
        8 => { /* EQ */
            return (0 == regs.flags.v) as cty::c_int
        }
        9 => { /* VC */
            return regs.flags.v as cty::c_int
        }
        10 => { /* VS */
            return (0 == regs.flags.n) as cty::c_int
        }
        11 => { /* PL */
            return regs.flags.n as cty::c_int
        }
        12 => { /* MI */
            return (regs.flags.n == regs.flags.v) as cty::c_int
        }
        13 => { /* GE */
            return (regs.flags.n != regs.flags.v) as cty::c_int
        }
        14 => { /* LT */
            return (0 == regs.flags.z && regs.flags.n == regs.flags.v) as
                       cty::c_int
        }
        15 => {
            return (0 != regs.flags.z || regs.flags.n != regs.flags.v) as
                       cty::c_int
        }
        _ => { }
    } /* GT */
    //    abort();
    return 0i32;
}
static mut currprefs: uae_prefs =
    uae_prefs{cpu_level: 0i32, cpu_compatible: 1i32, address_space_24: 1i32,};
/* CYGNUS_SIM */
/* don't include glib.h in romcalls.h */
// tiemu end
/* Opcode of faulting instruction */
static mut last_op_for_exception_3: uint16_t = 0;
/* PC at fault time */
static mut last_addr_for_exception_3: uintptr_t = 0;
/* Address that generated the exception */
static mut last_fault_for_exception_3: uintptr_t = 0;
/* read (0) or write (1) access */
static mut last_writeaccess_for_exception_3: cty::c_int = 0;
/* instruction (1) or data (0) access */
static mut last_instructionaccess_for_exception_3: cty::c_int = 0;
#[no_mangle]
pub static mut areg_byteinc: [cty::c_int; 8] =
    [1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 1i32, 2i32];
#[no_mangle]
pub static mut imm8_table: [cty::c_int; 8] =
    [8i32, 1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32];
#[no_mangle]
pub static mut movem_index1: [cty::c_int; 256] = [0; 256];
#[no_mangle]
pub static mut movem_index2: [cty::c_int; 256] = [0; 256];
#[no_mangle]
pub static mut movem_next: [cty::c_int; 256] = [0; 256];
#[no_mangle]
pub static mut fpp_movem_index1: [cty::c_int; 256] = [0; 256];
#[no_mangle]
pub static mut fpp_movem_index2: [cty::c_int; 256] = [0; 256];
#[no_mangle]
pub static mut fpp_movem_next: [cty::c_int; 256] = [0; 256];
#[no_mangle]
pub static mut cpufunctbl:
           [Option<unsafe extern "C" fn(_: uint32_t) -> cty::c_ulong>; 65536]
           =
    [None; 65536];
#[no_mangle]
pub unsafe extern "C" fn dump_counts() { }
#[no_mangle]
pub static mut currIntLev: cty::c_int = -1i32;
#[no_mangle]
pub static mut broken_in: cty::c_int = 0;
#[no_mangle]
pub static mut delayTime: cty::c_int = 0i32;
unsafe extern "C" fn op_illg_1(mut opcode: uint32_t) -> cty::c_ulong {
    op_illg(opcode);
    return 4i32 as cty::c_ulong;
}
unsafe extern "C" fn build_cpufunctbl() {
    let mut i: cty::c_int = 0;
    let mut opcode: cty::c_ulong = 0;
    let mut tbl: *mut cputbl =
        if currprefs.cpu_level == 4i32 {
            op_smalltbl_0_ff.as_mut_ptr()
        } else if currprefs.cpu_level == 3i32 {
            op_smalltbl_1_ff.as_mut_ptr()
        } else if currprefs.cpu_level == 2i32 {
            op_smalltbl_2_ff.as_mut_ptr()
        } else if currprefs.cpu_level == 1i32 {
            op_smalltbl_3_ff.as_mut_ptr()
        } else { op_smalltbl_5_ff.as_mut_ptr() };
    printf(b"UAE: Building CPU function table (%d %d).\n\x00" as *const u8 as
               *const cty::c_char, currprefs.cpu_level,
           currprefs.address_space_24);
    opcode = 0i32 as cty::c_ulong;
    while opcode < 65536i32 as cty::c_ulong {
        cpufunctbl[opcode as usize] =
            Some(op_illg_1 as
                     unsafe extern "C" fn(_: uint32_t) -> cty::c_ulong);
        opcode = opcode.wrapping_add(1)
    }
    i = 0i32;
    while (*tbl.offset(i as isize)).handler.is_some() {
        if 0 == (*tbl.offset(i as isize)).specific {
            cpufunctbl[(*tbl.offset(i as isize)).opcode as usize] =
                (*tbl.offset(i as isize)).handler
        }
        i += 1
    }
    opcode = 0i32 as cty::c_ulong;
    while opcode < 65536i32 as cty::c_ulong {
        let mut f:
                Option<unsafe extern "C" fn(_: uint32_t) -> cty::c_ulong> =
            None;
        if !((*table68k.offset(opcode as isize)).mnemo() as cty::c_int ==
                 i_ILLG as cty::c_int ||
                 (*table68k.offset(opcode as isize)).clev() as cty::c_int >
                     currprefs.cpu_level) {
            if (*table68k.offset(opcode as isize)).handler !=
                   -1i32 as cty::c_long {
                f =
                    cpufunctbl[(*table68k.offset(opcode as isize)).handler as
                                   usize];
                if f ==
                       Some(op_illg_1 as
                                unsafe extern "C" fn(_: uint32_t)
                                    -> cty::c_ulong) {
                    abort();
                }
                cpufunctbl[opcode as usize] = f
            }
        }
        opcode = opcode.wrapping_add(1)
    }
    i = 0i32;
    while (*tbl.offset(i as isize)).handler.is_some() {
        if 0 != (*tbl.offset(i as isize)).specific {
            cpufunctbl[(*tbl.offset(i as isize)).opcode as usize] =
                (*tbl.offset(i as isize)).handler
        }
        i += 1
    };
}
/* 0 */
#[no_mangle]
pub unsafe extern "C" fn fill_prefetch_slow() {
    regs.ir = hw_get_word(m68k_getpc() as uint32_t);
    regs.irc =
        hw_get_word(m68k_getpc().wrapping_add(2i32 as cty::c_ulong) as
                        uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn init_m68k() {
    let mut i: cty::c_int = 0;
    printf(b"UAE: version 0.8.25\n\x00" as *const u8 as *const cty::c_char);
    //    update_68k_cycles ();
    i = 0i32;
    while i < 256i32 {
        let mut j: cty::c_int = 0;
        j = 0i32;
        while j < 8i32 { if 0 != i & 1i32 << j { break ; } j += 1 }
        movem_index1[i as usize] = j;
        movem_index2[i as usize] = 7i32 - j;
        movem_next[i as usize] = i & !(1i32 << j);
        i += 1
    }
    i = 0i32;
    while i < 256i32 {
        let mut j_0: cty::c_int = 0;
        j_0 = 7i32;
        while j_0 >= 0i32 { if 0 != i & 1i32 << j_0 { break ; } j_0 -= 1 }
        fpp_movem_index1[i as usize] = 7i32 - j_0;
        fpp_movem_index2[i as usize] = j_0;
        fpp_movem_next[i as usize] = i & !(1i32 << j_0);
        i += 1
    }
    printf(b"UAE: Building CPU table for configuration: 68\x00" as *const u8
               as *const cty::c_char);
    regs.address_space_mask = 0xffffffffu32;
    if 0 != currprefs.address_space_24 && currprefs.cpu_level > 1i32 {
        printf(b"EC\x00" as *const u8 as *const cty::c_char);
    }
    match currprefs.cpu_level {
        1 => { printf(b"010\x00" as *const u8 as *const cty::c_char); }
        2 => { printf(b"020\x00" as *const u8 as *const cty::c_char); }
        3 => { printf(b"020/881\x00" as *const u8 as *const cty::c_char); }
        4 => {
            /* Who is going to miss the MMU anyway...? :-)  */
            printf(b"040\x00" as *const u8 as *const cty::c_char);
        }
        _ => { printf(b"000\x00" as *const u8 as *const cty::c_char); }
    }
    if 0 != currprefs.address_space_24 {
        regs.address_space_mask = 0xffffffi32 as uint32_t;
        printf(b" 24-bit addressing\x00" as *const u8 as *const cty::c_char);
    }
    printf(b"\n\x00" as *const u8 as *const cty::c_char);
    read_table68k();
    do_merges();
    printf(b"UAE: %d CPU functions\n\x00" as *const u8 as *const cty::c_char,
           nr_cpuop_funcs);
    build_cpufunctbl();
}
#[no_mangle]
pub static mut regs: regstruct =
    regstruct{regs: [0; 16],
              usp: 0,
              isp: 0,
              sr: 0,
              t1: 0,
              s: 0,
              x: 0,
              stopped: 0,
              flags: flag_struct{c: 0, z: 0, n: 0, v: 0, x: 0,},
              intmask: 0,
              pc: 0,
              pc_p: 0 as *const uint8_t as *mut uint8_t,
              pc_oldp: 0 as *const uint8_t as *mut uint8_t,
              vbr: 0,
              sfc: 0,
              dfc: 0,
              fp: [0.; 8],
              fp_result: 0.,
              fpcr: 0,
              fpsr: 0,
              fpiar: 0,
              fpsr_highbyte: 0,
              spcflags: 0,
              kick_mask: 0,
              address_space_mask: 0,
              irc: 0,
              ir: 0,
              panic: 0,
              panic_pc: 0,
              panic_addr: 0,
              insn_end: 0 as *const cty::c_uchar as *mut cty::c_uchar,
              prevlock: 0,
              thislock: 0,
              exception: 0,
              end_of_registers: 0,
              msize: 0,
              profile: 0,
              profile_hist: 0 as *const cty::c_ushort as *mut cty::c_ushort,
              memory: 0 as *const cty::c_uchar as *mut cty::c_uchar,
              xyram_select: 0,
              xram_start: 0,
              yram_start: 0,
              xmem: 0 as *const cty::c_uchar as *mut cty::c_uchar,
              ymem: 0 as *const cty::c_uchar as *mut cty::c_uchar,
              xmem_offset: 0 as *const cty::c_uchar as *mut cty::c_uchar,
              ymem_offset: 0 as *const cty::c_uchar as *mut cty::c_uchar,};
#[no_mangle]
pub static mut lastint_regs: regstruct =
    regstruct{regs: [0; 16],
              usp: 0,
              isp: 0,
              sr: 0,
              t1: 0,
              s: 0,
              x: 0,
              stopped: 0,
              flags: flag_struct{c: 0, z: 0, n: 0, v: 0, x: 0,},
              intmask: 0,
              pc: 0,
              pc_p: 0 as *const uint8_t as *mut uint8_t,
              pc_oldp: 0 as *const uint8_t as *mut uint8_t,
              vbr: 0,
              sfc: 0,
              dfc: 0,
              fp: [0.; 8],
              fp_result: 0.,
              fpcr: 0,
              fpsr: 0,
              fpiar: 0,
              fpsr_highbyte: 0,
              spcflags: 0,
              kick_mask: 0,
              address_space_mask: 0,
              irc: 0,
              ir: 0,
              panic: 0,
              panic_pc: 0,
              panic_addr: 0,
              insn_end: 0 as *const cty::c_uchar as *mut cty::c_uchar,
              prevlock: 0,
              thislock: 0,
              exception: 0,
              end_of_registers: 0,
              msize: 0,
              profile: 0,
              profile_hist: 0 as *const cty::c_ushort as *mut cty::c_ushort,
              memory: 0 as *const cty::c_uchar as *mut cty::c_uchar,
              xyram_select: 0,
              xram_start: 0,
              yram_start: 0,
              xmem: 0 as *const cty::c_uchar as *mut cty::c_uchar,
              ymem: 0 as *const cty::c_uchar as *mut cty::c_uchar,
              xmem_offset: 0 as *const cty::c_uchar as *mut cty::c_uchar,
              ymem_offset: 0 as *const cty::c_uchar as *mut cty::c_uchar,};
/* 0 */
static mut m68kpc_offset: cty::c_long = 0;
/* NO_GDB */
#[no_mangle]
pub static mut lastint_no: cty::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn sym_addr(mut addr: uint32_t) -> *mut cty::c_char {
    static mut buf: [cty::c_char; 256] = [0; 256];
    let mut rcid: cty::c_int = romcalls_is_addr(addr);
    if rcid == -1i32 {
        snprintf(buf.as_mut_ptr(),
                 ::core::mem::size_of::<[cty::c_char; 256]>() as
                     cty::c_ulong,
                 b"$%06lX\x00" as *const u8 as *const cty::c_char,
                 addr as cty::c_ulong);
    } else {
        snprintf(buf.as_mut_ptr(),
                 ::core::mem::size_of::<[cty::c_char; 256]>() as
                     cty::c_ulong,
                 b"$%06lX -> tios::%s\x00" as *const u8 as
                     *const cty::c_char, addr as cty::c_ulong,
                 romcalls_get_name(rcid));
    }
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn ShowEA(mut f: *mut FILE, mut reg: cty::c_int,
                                mut mode: amodes, mut size: wordsizes,
                                mut buf: *mut cty::c_char) -> int32_t {
    let mut dp: uint16_t = 0;
    let mut disp8: int8_t = 0;
    let mut disp16: int16_t = 0;
    let mut r: cty::c_int = 0;
    let mut dispreg: uint32_t = 0;
    let mut addr: uintptr_t = 0;
    let mut offset: int32_t = 0i32;
    let mut buffer: [cty::c_char; 296] = [0; 296];
    match mode as cty::c_uint {
        0 => {
            sprintf(buffer.as_mut_ptr(),
                    b"D%d\x00" as *const u8 as *const cty::c_char, reg);
        }
        1 => {
            sprintf(buffer.as_mut_ptr(),
                    b"A%d\x00" as *const u8 as *const cty::c_char, reg);
        }
        2 => {
            sprintf(buffer.as_mut_ptr(),
                    b"(A%d)\x00" as *const u8 as *const cty::c_char, reg);
        }
        3 => {
            sprintf(buffer.as_mut_ptr(),
                    b"(A%d)+\x00" as *const u8 as *const cty::c_char, reg);
        }
        4 => {
            sprintf(buffer.as_mut_ptr(),
                    b"-(A%d)\x00" as *const u8 as *const cty::c_char, reg);
        }
        5 => {
            disp16 =
                hw_get_word((regs.pc as cty::c_long +
                                 regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                     as cty::c_long + m68kpc_offset) as
                                uint32_t) as int16_t;
            m68kpc_offset += 2i32 as cty::c_long;
            addr =
                (*regs.regs.as_mut_ptr().offset(8isize).offset(reg as
                                                                   isize)).wrapping_add(disp16
                                                                                            as
                                                                                            cty::c_uint)
                    as uintptr_t;
            if 0 != disp16 as cty::c_int & 0x8000i32 {
                sprintf(buffer.as_mut_ptr(),
                        b"(-$%X,A%d) [%s]\x00" as *const u8 as
                            *const cty::c_char,
                        -(disp16 as cty::c_int) & 0xffffi32, reg,
                        sym_addr(addr as uint32_t));
            } else {
                sprintf(buffer.as_mut_ptr(),
                        b"(+$%X,A%d) [%s]\x00" as *const u8 as
                            *const cty::c_char,
                        disp16 as cty::c_int & 0xffffi32, reg,
                        sym_addr(addr as uint32_t));
            }
        }
        6 => {
            dp =
                hw_get_word((regs.pc as cty::c_long +
                                 regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                     as cty::c_long + m68kpc_offset) as
                                uint32_t);
            m68kpc_offset += 2i32 as cty::c_long;
            disp8 = (dp as cty::c_int & 0xffi32) as int8_t;
            r = (dp as cty::c_int & 0x7000i32) >> 12i32;
            dispreg =
                if 0 != dp as cty::c_int & 0x8000i32 {
                    *regs.regs.as_mut_ptr().offset(8isize).offset(r as isize)
                } else { regs.regs[r as usize] };
            if 0 == dp as cty::c_int & 0x800i32 {
                dispreg = dispreg as int16_t as int32_t as uint32_t
            }
            dispreg <<= dp as cty::c_int >> 9i32 & 3i32;
            if 0 != dp as cty::c_int & 0x100i32 {
                let mut outer: int32_t = 0i32;
                let mut disp: int32_t = 0i32;
                let mut base: int32_t =
                    *regs.regs.as_mut_ptr().offset(8isize).offset(reg as
                                                                      isize)
                        as int32_t;
                let mut name: [cty::c_char; 10] = [0; 10];
                sprintf(name.as_mut_ptr(),
                        b"A%d, \x00" as *const u8 as *const cty::c_char,
                        reg);
                if 0 != dp as cty::c_int & 0x80i32 {
                    base = 0i32;
                    name[0usize] = 0i32 as cty::c_char
                }
                if 0 != dp as cty::c_int & 0x40i32 {
                    dispreg = 0i32 as uint32_t
                }
                if dp as cty::c_int & 0x30i32 == 0x20i32 {
                    disp =
                        hw_get_word((regs.pc as cty::c_long +
                                         regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                             as cty::c_long + m68kpc_offset)
                                        as uint32_t) as int16_t as int32_t;
                    m68kpc_offset += 2i32 as cty::c_long
                }
                if dp as cty::c_int & 0x30i32 == 0x30i32 {
                    disp =
                        hw_get_long((regs.pc as cty::c_long +
                                         regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                             as cty::c_long + m68kpc_offset)
                                        as uint32_t) as int32_t;
                    m68kpc_offset += 4i32 as cty::c_long
                }
                base += disp;
                if dp as cty::c_int & 0x3i32 == 0x2i32 {
                    outer =
                        hw_get_word((regs.pc as cty::c_long +
                                         regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                             as cty::c_long + m68kpc_offset)
                                        as uint32_t) as int16_t as int32_t;
                    m68kpc_offset += 2i32 as cty::c_long
                }
                if dp as cty::c_int & 0x3i32 == 0x3i32 {
                    outer =
                        hw_get_long((regs.pc as cty::c_long +
                                         regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                             as cty::c_long + m68kpc_offset)
                                        as uint32_t) as int32_t;
                    m68kpc_offset += 4i32 as cty::c_long
                }
                if 0 == dp as cty::c_int & 4i32 {
                    base =
                        (base as cty::c_uint).wrapping_add(dispreg) as
                            int32_t as int32_t
                }
                if 0 != dp as cty::c_int & 3i32 {
                    base = hw_get_long(base as uint32_t) as int32_t
                }
                if 0 != dp as cty::c_int & 4i32 {
                    base =
                        (base as cty::c_uint).wrapping_add(dispreg) as
                            int32_t as int32_t
                }
                addr = (base + outer) as uintptr_t;
                sprintf(buffer.as_mut_ptr(),
                        b"(%s%c%d.%c+%ld)+%ld [$%06lX]\x00" as *const u8 as
                            *const cty::c_char, name.as_mut_ptr(),
                        if 0 != dp as cty::c_int & 0x8000i32 {
                            'A' as i32
                        } else { 'D' as i32 }, r,
                        if 0 != dp as cty::c_int & 0x800i32 {
                            'L' as i32
                        } else { 'W' as i32 }, disp, outer, addr);
            } else {
                addr =
                    (*regs.regs.as_mut_ptr().offset(8isize).offset(reg as
                                                                       isize)).wrapping_add(disp8
                                                                                                as
                                                                                                int32_t
                                                                                                as
                                                                                                cty::c_uint).wrapping_add(dispreg)
                        as uintptr_t;
                sprintf(buffer.as_mut_ptr(),
                        b"(A%d, %c%d.%c, $%02X) [$%06lX]\x00" as *const u8 as
                            *const cty::c_char, reg,
                        if 0 != dp as cty::c_int & 0x8000i32 {
                            'A' as i32
                        } else { 'D' as i32 }, r,
                        if 0 != dp as cty::c_int & 0x800i32 {
                            'L' as i32
                        } else { 'W' as i32 }, disp8 as cty::c_int, addr);
            }
        }
        9 => {
            addr = m68k_getpc().wrapping_add(m68kpc_offset as cty::c_ulong);
            disp16 =
                hw_get_word((regs.pc as cty::c_long +
                                 regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                     as cty::c_long + m68kpc_offset) as
                                uint32_t) as int16_t;
            m68kpc_offset += 2i32 as cty::c_long;
            addr =
                (addr as cty::c_ulong).wrapping_add(disp16 as cty::c_ulong)
                    as uintptr_t as uintptr_t;
            if 0 != disp16 as cty::c_int & 0x8000i32 {
                sprintf(buffer.as_mut_ptr(),
                        b"(-$%X,PC) [%s]\x00" as *const u8 as
                            *const cty::c_char,
                        -(disp16 as cty::c_int) & 0xffffi32,
                        sym_addr(addr as uint32_t));
            } else {
                sprintf(buffer.as_mut_ptr(),
                        b"(+$%X,PC) [%s]\x00" as *const u8 as
                            *const cty::c_char,
                        disp16 as cty::c_int & 0xffffi32,
                        sym_addr(addr as uint32_t));
            }
        }
        10 => {
            addr = m68k_getpc().wrapping_add(m68kpc_offset as cty::c_ulong);
            dp =
                hw_get_word((regs.pc as cty::c_long +
                                 regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                     as cty::c_long + m68kpc_offset) as
                                uint32_t);
            m68kpc_offset += 2i32 as cty::c_long;
            disp8 = (dp as cty::c_int & 0xffi32) as int8_t;
            r = (dp as cty::c_int & 0x7000i32) >> 12i32;
            dispreg =
                if 0 != dp as cty::c_int & 0x8000i32 {
                    *regs.regs.as_mut_ptr().offset(8isize).offset(r as isize)
                } else { regs.regs[r as usize] };
            if 0 == dp as cty::c_int & 0x800i32 {
                dispreg = dispreg as int16_t as int32_t as uint32_t
            }
            dispreg <<= dp as cty::c_int >> 9i32 & 3i32;
            if 0 != dp as cty::c_int & 0x100i32 {
                let mut outer_0: int32_t = 0i32;
                let mut disp_0: int32_t = 0i32;
                let mut base_0: int32_t = addr as int32_t;
                let mut name_0: [cty::c_char; 10] = [0; 10];
                sprintf(name_0.as_mut_ptr(),
                        b"PC, \x00" as *const u8 as *const cty::c_char);
                if 0 != dp as cty::c_int & 0x80i32 {
                    base_0 = 0i32;
                    name_0[0usize] = 0i32 as cty::c_char
                }
                if 0 != dp as cty::c_int & 0x40i32 {
                    dispreg = 0i32 as uint32_t
                }
                if dp as cty::c_int & 0x30i32 == 0x20i32 {
                    disp_0 =
                        hw_get_word((regs.pc as cty::c_long +
                                         regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                             as cty::c_long + m68kpc_offset)
                                        as uint32_t) as int16_t as int32_t;
                    m68kpc_offset += 2i32 as cty::c_long
                }
                if dp as cty::c_int & 0x30i32 == 0x30i32 {
                    disp_0 =
                        hw_get_long((regs.pc as cty::c_long +
                                         regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                             as cty::c_long + m68kpc_offset)
                                        as uint32_t) as int32_t;
                    m68kpc_offset += 4i32 as cty::c_long
                }
                base_0 += disp_0;
                if dp as cty::c_int & 0x3i32 == 0x2i32 {
                    outer_0 =
                        hw_get_word((regs.pc as cty::c_long +
                                         regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                             as cty::c_long + m68kpc_offset)
                                        as uint32_t) as int16_t as int32_t;
                    m68kpc_offset += 2i32 as cty::c_long
                }
                if dp as cty::c_int & 0x3i32 == 0x3i32 {
                    outer_0 =
                        hw_get_long((regs.pc as cty::c_long +
                                         regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                             as cty::c_long + m68kpc_offset)
                                        as uint32_t) as int32_t;
                    m68kpc_offset += 4i32 as cty::c_long
                }
                if 0 == dp as cty::c_int & 4i32 {
                    base_0 =
                        (base_0 as cty::c_uint).wrapping_add(dispreg) as
                            int32_t as int32_t
                }
                if 0 != dp as cty::c_int & 3i32 {
                    base_0 = hw_get_long(base_0 as uint32_t) as int32_t
                }
                if 0 != dp as cty::c_int & 4i32 {
                    base_0 =
                        (base_0 as cty::c_uint).wrapping_add(dispreg) as
                            int32_t as int32_t
                }
                addr = (base_0 + outer_0) as uintptr_t;
                sprintf(buffer.as_mut_ptr(),
                        b"(%s%c%d.%c+%ld)+%ld [%s]\x00" as *const u8 as
                            *const cty::c_char, name_0.as_mut_ptr(),
                        if 0 != dp as cty::c_int & 0x8000i32 {
                            'A' as i32
                        } else { 'D' as i32 }, r,
                        if 0 != dp as cty::c_int & 0x800i32 {
                            'L' as i32
                        } else { 'W' as i32 }, disp_0, outer_0,
                        sym_addr(addr as uint32_t));
            } else {
                addr =
                    (addr as
                         cty::c_ulong).wrapping_add((disp8 as int32_t as
                                                          cty::c_uint).wrapping_add(dispreg)
                                                         as cty::c_ulong) as
                        uintptr_t as uintptr_t;
                sprintf(buffer.as_mut_ptr(),
                        b"(PC, %c%d.%c, $%02X) [%s]\x00" as *const u8 as
                            *const cty::c_char,
                        if 0 != dp as cty::c_int & 0x8000i32 {
                            'A' as i32
                        } else { 'D' as i32 }, r,
                        if 0 != dp as cty::c_int & 0x800i32 {
                            'L' as i32
                        } else { 'W' as i32 }, disp8 as cty::c_int,
                        sym_addr(addr as uint32_t));
            }
        }
        7 => {
            sprintf(buffer.as_mut_ptr(),
                    b"$%lX\x00" as *const u8 as *const cty::c_char,
                    hw_get_word((regs.pc as cty::c_long +
                                     regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                         as cty::c_long + m68kpc_offset) as
                                    uint32_t) as int16_t as int32_t as
                        cty::c_ulong);
            m68kpc_offset += 2i32 as cty::c_long
        }
        8 => {
            sprintf(buffer.as_mut_ptr(),
                    b"%s\x00" as *const u8 as *const cty::c_char,
                    sym_addr(hw_get_long((regs.pc as cty::c_long +
                                              regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                                  as cty::c_long +
                                              m68kpc_offset) as uint32_t)));
            m68kpc_offset += 4i32 as cty::c_long
        }
        11 => {
            match size as cty::c_uint {
                0 => {
                    sprintf(buffer.as_mut_ptr(),
                            b"#$%X\x00" as *const u8 as *const cty::c_char,
                            (hw_get_word((regs.pc as cty::c_long +
                                              regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                                  as cty::c_long +
                                              m68kpc_offset) as uint32_t) as
                                 cty::c_int & 0xffi32) as cty::c_uint);
                    m68kpc_offset += 2i32 as cty::c_long
                }
                1 => {
                    sprintf(buffer.as_mut_ptr(),
                            b"#$%X\x00" as *const u8 as *const cty::c_char,
                            (hw_get_word((regs.pc as cty::c_long +
                                              regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                                  as cty::c_long +
                                              m68kpc_offset) as uint32_t) as
                                 cty::c_int & 0xffffi32) as cty::c_uint);
                    m68kpc_offset += 2i32 as cty::c_long
                }
                2 => {
                    sprintf(buffer.as_mut_ptr(),
                            b"#$%lX\x00" as *const u8 as *const cty::c_char,
                            hw_get_long((regs.pc as cty::c_long +
                                             regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                                 as cty::c_long +
                                             m68kpc_offset) as uint32_t) as
                                cty::c_ulong);
                    m68kpc_offset += 4i32 as cty::c_long
                }
                _ => { }
            }
        }
        12 => {
            offset =
                hw_get_word((regs.pc as cty::c_long +
                                 regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                     as cty::c_long + m68kpc_offset) as
                                uint32_t) as int8_t as int32_t;
            m68kpc_offset += 2i32 as cty::c_long;
            sprintf(buffer.as_mut_ptr(),
                    b"#$%X\x00" as *const u8 as *const cty::c_char,
                    (offset & 0xffi32) as cty::c_uint);
        }
        13 => {
            offset =
                hw_get_word((regs.pc as cty::c_long +
                                 regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                     as cty::c_long + m68kpc_offset) as
                                uint32_t) as int16_t as int32_t;
            m68kpc_offset += 2i32 as cty::c_long;
            sprintf(buffer.as_mut_ptr(),
                    b"#$%X\x00" as *const u8 as *const cty::c_char,
                    (offset & 0xffffi32) as cty::c_uint);
        }
        14 => {
            offset =
                hw_get_long((regs.pc as cty::c_long +
                                 regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                     as cty::c_long + m68kpc_offset) as
                                uint32_t) as int32_t;
            m68kpc_offset += 4i32 as cty::c_long;
            sprintf(buffer.as_mut_ptr(),
                    b"#$%lX\x00" as *const u8 as *const cty::c_char,
                    offset as cty::c_ulong);
        }
        15 => {
            offset = (reg & 0xffi32) as int8_t as int32_t;
            sprintf(buffer.as_mut_ptr(),
                    b"#$%lX\x00" as *const u8 as *const cty::c_char,
                    offset as cty::c_ulong);
        }
        _ => { }
    }
    if buf.is_null() {
        fprintf(f, b"%s\x00" as *const u8 as *const cty::c_char,
                buffer.as_mut_ptr());
    } else { strcat(buf, buffer.as_mut_ptr()); }
    return offset;
}
/* NO_GDB */
/* 0 */
#[no_mangle]
pub unsafe extern "C" fn get_disp_ea_020(mut base: uint32_t, mut dp: uint32_t)
 -> uint32_t {
    let mut reg: cty::c_int =
        (dp >> 12i32 & 15i32 as cty::c_uint) as cty::c_int;
    let mut regd: int32_t = regs.regs[reg as usize] as int32_t;
    if dp & 0x800i32 as cty::c_uint == 0i32 as cty::c_uint {
        regd = regd as int16_t as int32_t
    }
    regd <<= dp >> 9i32 & 3i32 as cty::c_uint;
    if 0 != dp & 0x100i32 as cty::c_uint {
        let mut outer: int32_t = 0i32;
        if 0 != dp & 0x80i32 as cty::c_uint { base = 0i32 as uint32_t }
        if 0 != dp & 0x40i32 as cty::c_uint { regd = 0i32 }
        if dp & 0x30i32 as cty::c_uint == 0x20i32 as cty::c_uint {
            base =
                (base as
                     cty::c_uint).wrapping_add(next_iword() as int16_t as
                                                    int32_t as cty::c_uint)
                    as uint32_t as uint32_t
        }
        if dp & 0x30i32 as cty::c_uint == 0x30i32 as cty::c_uint {
            base =
                (base as cty::c_uint).wrapping_add(next_ilong()) as uint32_t
                    as uint32_t
        }
        if dp & 0x3i32 as cty::c_uint == 0x2i32 as cty::c_uint {
            outer = next_iword() as int16_t as int32_t
        }
        if dp & 0x3i32 as cty::c_uint == 0x3i32 as cty::c_uint {
            outer = next_ilong() as int32_t
        }
        if dp & 0x4i32 as cty::c_uint == 0i32 as cty::c_uint {
            base =
                (base as cty::c_uint).wrapping_add(regd as cty::c_uint) as
                    uint32_t as uint32_t
        }
        if 0 != dp & 0x3i32 as cty::c_uint { base = hw_get_long(base) }
        if 0 != dp & 0x4i32 as cty::c_uint {
            base =
                (base as cty::c_uint).wrapping_add(regd as cty::c_uint) as
                    uint32_t as uint32_t
        }
        return base.wrapping_add(outer as cty::c_uint)
    } else {
        return base.wrapping_add(dp as int8_t as int32_t as
                                     cty::c_uint).wrapping_add(regd as
                                                                    cty::c_uint)
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_disp_ea_000(mut base: uint32_t, mut dp: uint32_t)
 -> uint32_t {
    let mut reg: cty::c_int =
        (dp >> 12i32 & 15i32 as cty::c_uint) as cty::c_int;
    let mut regd: int32_t = regs.regs[reg as usize] as int32_t;
    if dp & 0x800i32 as cty::c_uint == 0i32 as cty::c_uint {
        regd = regd as int16_t as int32_t
    }
    return base.wrapping_add(dp as int8_t as
                                 cty::c_uint).wrapping_add(regd as
                                                                cty::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn MakeSR() {
    regs.sr =
        (((regs.t1 as cty::c_int) << 15i32 | (regs.s as cty::c_int) << 13i32
              | regs.intmask << 8i32) as cty::c_uint | regs.flags.x << 4i32 |
             regs.flags.n << 3i32 | regs.flags.z << 2i32 |
             regs.flags.v << 1i32 | regs.flags.c) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn MakeFromSR() {
    //    int oldm = regs.m;
    let mut olds: cty::c_int = regs.s as cty::c_int;
    regs.t1 = (regs.sr as cty::c_int >> 15i32 & 1i32) as flagtype;
    //    regs.t0 = (regs.sr >> 14) & 1;
    regs.s = (regs.sr as cty::c_int >> 13i32 & 1i32) as flagtype;
    //    regs.m = (regs.sr >> 12) & 1;
    regs.intmask = regs.sr as cty::c_int >> 8i32 & 7i32;
    regs.flags.x = (regs.sr as cty::c_int >> 4i32 & 1i32) as cty::c_uint;
    regs.flags.n = (regs.sr as cty::c_int >> 3i32 & 1i32) as cty::c_uint;
    regs.flags.z = (regs.sr as cty::c_int >> 2i32 & 1i32) as cty::c_uint;
    regs.flags.v = (regs.sr as cty::c_int >> 1i32 & 1i32) as cty::c_uint;
    regs.flags.c = (regs.sr as cty::c_int & 1i32) as cty::c_uint;
    if olds != regs.s as cty::c_int {
        if 0 != olds {
            regs.isp =
                *regs.regs.as_mut_ptr().offset(8isize).offset(7isize) as
                    uintptr_t;
            *regs.regs.as_mut_ptr().offset(8isize).offset(7isize) =
                regs.usp as uint32_t
        } else {
            regs.usp =
                *regs.regs.as_mut_ptr().offset(8isize).offset(7isize) as
                    uintptr_t;
            *regs.regs.as_mut_ptr().offset(8isize).offset(7isize) =
                regs.isp as uint32_t
        }
    }
    //    }
    set_special(8i32 as uint32_t);
    if 0 != regs.t1 {
        /*|| regs.t0*/
        set_special(32i32 as uint32_t);
    } else {
        /* Keep SPCFLAG_DOTRACE, we still want a trace exception for
	   SR-modifying instructions (including STOP).  */
        unset_special(32i32 as uint32_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn intlev() -> cty::c_int {
    let mut rc: cty::c_int = currIntLev;
    currIntLev = -1i32;
    return rc;
}
unsafe extern "C" fn exception_trace(mut nr: cty::c_int) {
    unset_special((32i32 | 64i32) as uint32_t);
    if 0 != regs.t1 {
        /*&& !regs.t0*/
        /* trace stays pending if exception is div by zero, chk,
		 * trapv or trap #x
		 */
        if nr == 5i32 || nr == 6i32 || nr == 7i32 ||
               nr >= 32i32 && nr <= 47i32 {
            set_special(64i32 as uint32_t);
        }
    }
    regs.t1 = 0i32 as flagtype;
}
/* 0 */
#[no_mangle]
pub unsafe extern "C" fn Exception_normal(mut nr: cty::c_int,
                                          mut oldpc: uintptr_t) {
    let mut currpc: uint32_t = m68k_getpc() as uint32_t;
    let mut newpc: uint32_t = 0;
    let mut sv: cty::c_int = regs.s as cty::c_int;
    //    exception_debug (nr);
    MakeSR();
    if 0 == regs.s {
        regs.usp =
            *regs.regs.as_mut_ptr().offset(8isize).offset(7isize) as
                uintptr_t;
        *regs.regs.as_mut_ptr().offset(8isize).offset(7isize) =
            regs.isp as uint32_t;
        regs.s = 1i32 as flagtype
    }
    if nr == 2i32 || nr == 3i32 {
        let mut mode: uint16_t =
            ((if 0 != sv { 4i32 } else { 0i32 }) |
                 (if 0 != last_instructionaccess_for_exception_3 {
                      2i32
                  } else { 1i32 })) as uint16_t;
        mode =
            (mode as cty::c_int |
                 if 0 != last_writeaccess_for_exception_3 {
                     0i32
                 } else { 16i32 }) as uint16_t;
        let ref mut fresh0 =
            *regs.regs.as_mut_ptr().offset(8isize).offset(7isize);
        *fresh0 =
            (*fresh0 as cty::c_uint).wrapping_sub(14i32 as cty::c_uint) as
                uint32_t as uint32_t;
        /* fixme: bit3=I/N */
        hw_put_word((*regs.regs.as_mut_ptr().offset(8isize).offset(7isize)).wrapping_add(0i32
                                                                                             as
                                                                                             cty::c_uint),
                    mode);
        hw_put_long((*regs.regs.as_mut_ptr().offset(8isize).offset(7isize)).wrapping_add(2i32
                                                                                             as
                                                                                             cty::c_uint),
                    last_fault_for_exception_3 as uint32_t);
        hw_put_word((*regs.regs.as_mut_ptr().offset(8isize).offset(7isize)).wrapping_add(6i32
                                                                                             as
                                                                                             cty::c_uint),
                    last_op_for_exception_3);
        hw_put_word((*regs.regs.as_mut_ptr().offset(8isize).offset(7isize)).wrapping_add(8i32
                                                                                             as
                                                                                             cty::c_uint),
                    regs.sr);
        hw_put_long((*regs.regs.as_mut_ptr().offset(8isize).offset(7isize)).wrapping_add(10i32
                                                                                             as
                                                                                             cty::c_uint),
                    last_addr_for_exception_3 as uint32_t);
    } else {
        let ref mut fresh1 =
            *regs.regs.as_mut_ptr().offset(8isize).offset(7isize);
        *fresh1 =
            (*fresh1 as cty::c_uint).wrapping_sub(4i32 as cty::c_uint) as
                uint32_t as uint32_t;
        hw_put_long(*regs.regs.as_mut_ptr().offset(8isize).offset(7isize),
                    currpc);
        let ref mut fresh2 =
            *regs.regs.as_mut_ptr().offset(8isize).offset(7isize);
        *fresh2 =
            (*fresh2 as cty::c_uint).wrapping_sub(2i32 as cty::c_uint) as
                uint32_t as uint32_t;
        hw_put_word(*regs.regs.as_mut_ptr().offset(8isize).offset(7isize),
                    regs.sr);
    }
    //        write_log ("Exception %d (%x) at %x -> %x!\n", nr, oldpc, currpc, get_long (regs.vbr + 4*nr));
    newpc = hw_get_long(regs.vbr.wrapping_add((4i32 * nr) as cty::c_uint));
    if 0 != newpc & 1i32 as cty::c_uint {
        exception3(regs.ir as uint32_t, m68k_getpc(), newpc as uintptr_t);
        return
    }
    m68k_setpc(newpc as uintptr_t);
    fill_prefetch_slow();
    exception_trace(nr);
}
#[no_mangle]
pub unsafe extern "C" fn Exception(mut nr: cty::c_int,
                                   mut oldpc: uintptr_t) {
    Exception_normal(nr, oldpc);
}
#[no_mangle]
pub static mut uscycle: cty::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn Interrupt(mut nr: cty::c_int) {
    regs.stopped = 0i32 as flagtype;
    unset_special(4i32 as uint32_t);
    assert!(nr < 8i32 && nr >= 0i32);
    lastint_regs = regs;
    lastint_no = nr;
    Exception(nr + 24i32, 0i32 as uintptr_t);
    regs.intmask = nr;
    set_special(8i32 as uint32_t);
}
static mut caar: uint32_t = 0;
static mut cacr: uint32_t = 0;
static mut itt0: uint32_t = 0;
static mut itt1: uint32_t = 0;
static mut dtt0: uint32_t = 0;
static mut dtt1: uint32_t = 0;
static mut tc: uint32_t = 0;
static mut mmusr: uint32_t = 0;
static mut urp: uint32_t = 0;
static mut srp: uint32_t = 0;
#[no_mangle]
pub unsafe extern "C" fn m68k_move2c(mut regno: cty::c_int,
                                     mut regp: *mut uint32_t) -> cty::c_int {
    if currprefs.cpu_level == 1i32 && regno & 0x7ffi32 > 1i32 ||
           currprefs.cpu_level < 4i32 && regno & 0x7ffi32 > 2i32 ||
           currprefs.cpu_level == 4i32 && regno == 0x802i32 {
        op_illg(0x4e7bi32 as uint32_t);
        return 0i32
    } else {
        match regno {
            0 => { regs.sfc = *regp & 7i32 as cty::c_uint }
            1 => { regs.dfc = *regp & 7i32 as cty::c_uint }
            2 => {
                cacr =
                    *regp &
                        (if currprefs.cpu_level < 4i32 {
                             0x3i32 as cty::c_uint
                         } else { 0x80008000u32 })
            }
            3 => { tc = *regp & 0xc000i32 as cty::c_uint }
            4 => {
                /* Mask out fields that should be zero.  */
                itt0 = *regp & 0xffffe364u32
            }
            5 => { itt1 = *regp & 0xffffe364u32 }
            6 => { dtt0 = *regp & 0xffffe364u32 }
            7 => { dtt1 = *regp & 0xffffe364u32 }
            2048 => { regs.usp = *regp as uintptr_t }
            2049 => { regs.vbr = *regp }
            2050 => { caar = *regp & 0xfci32 as cty::c_uint }
            2053 => {
                //	case 0x803: regs.msp = *regp; if (regs.m == 1) m68k_areg(regs, 7) = regs.msp; break;
			//	case 0x804: regs.isp = *regp; if (regs.m == 0) m68k_areg(regs, 7) = regs.isp; break;
                mmusr = *regp
            }
            2054 => { urp = *regp }
            2055 => { srp = *regp }
            _ => { op_illg(0x4e7bi32 as uint32_t); return 0i32 }
        }
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn m68k_movec2(mut regno: cty::c_int,
                                     mut regp: *mut uint32_t) -> cty::c_int {
    if currprefs.cpu_level == 1i32 && regno & 0x7ffi32 > 1i32 ||
           currprefs.cpu_level < 4i32 && regno & 0x7ffi32 > 2i32 ||
           currprefs.cpu_level == 4i32 && regno == 0x802i32 {
        op_illg(0x4e7ai32 as uint32_t);
        return 0i32
    } else {
        match regno {
            0 => { *regp = regs.sfc }
            1 => { *regp = regs.dfc }
            2 => { *regp = cacr }
            3 => { *regp = tc }
            4 => { *regp = itt0 }
            5 => { *regp = itt1 }
            6 => { *regp = dtt0 }
            7 => { *regp = dtt1 }
            2048 => { *regp = regs.usp as uint32_t }
            2049 => { *regp = regs.vbr }
            2050 => { *regp = caar }
            2053 => {
                //	case 0x803: *regp = regs.m == 1 ? m68k_areg(regs, 7) : regs.msp; break;
			//	case 0x804: *regp = regs.m == 0 ? m68k_areg(regs, 7) : regs.isp; break;
                *regp = mmusr
            }
            2054 => { *regp = urp }
            2055 => { *regp = srp }
            _ => { op_illg(0x4e7ai32 as uint32_t); return 0i32 }
        }
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn m68k_divl(mut opcode: uint32_t, mut src: uint32_t,
                                   mut extra: uint16_t,
                                   mut oldpc: uintptr_t) {
    if src == 0i32 as cty::c_uint { Exception(5i32, oldpc); return }
    if 0 != extra as cty::c_int & 0x800i32 {
        /* signed variant */
        let mut a: cty::c_longlong =
            regs.regs[(extra as cty::c_int >> 12i32 & 7i32) as usize] as
                int32_t as cty::c_longlong;
        let mut quot: cty::c_longlong = 0;
        let mut rem: cty::c_longlong = 0;
        if 0 != extra as cty::c_int & 0x400i32 {
            a &= 0xffffffffu32 as cty::c_longlong;
            a |=
                (regs.regs[(extra as cty::c_int & 7i32) as usize] as
                     cty::c_longlong) << 32i32
        }
        rem = a % src as int32_t as cty::c_longlong;
        quot = a / src as int32_t as cty::c_longlong;
        if quot as cty::c_ulonglong & 0xffffffff80000000u64 !=
               0i32 as cty::c_ulonglong &&
               quot as cty::c_ulonglong & 0xffffffff80000000u64 !=
                   0xffffffff80000000u64 {
            regs.flags.v = 1i32 as cty::c_uint;
            regs.flags.n = 1i32 as cty::c_uint;
            regs.flags.c = 0i32 as cty::c_uint
        } else {
            if ((rem as int32_t) < 0i32) as cty::c_int !=
                   (a < 0i32 as cty::c_longlong) as cty::c_int {
                rem = -rem
            }
            regs.flags.v = 0i32 as cty::c_uint;
            regs.flags.c = 0i32 as cty::c_uint;
            regs.flags.z =
                (quot as int32_t == 0i32) as cty::c_int as cty::c_uint;
            regs.flags.n =
                ((quot as int32_t) < 0i32) as cty::c_int as cty::c_uint;
            regs.regs[(extra as cty::c_int & 7i32) as usize] =
                rem as uint32_t;
            regs.regs[(extra as cty::c_int >> 12i32 & 7i32) as usize] =
                quot as uint32_t
        }
    } else {
        /* unsigned */
        let mut a_0: cty::c_ulonglong =
            regs.regs[(extra as cty::c_int >> 12i32 & 7i32) as usize] as
                cty::c_ulonglong;
        let mut quot_0: cty::c_ulonglong = 0;
        let mut rem_0: cty::c_ulonglong = 0;
        if 0 != extra as cty::c_int & 0x400i32 {
            a_0 &= 0xffffffffu32 as cty::c_ulonglong;
            a_0 |=
                (regs.regs[(extra as cty::c_int & 7i32) as usize] as
                     cty::c_ulonglong) << 32i32
        }
        rem_0 = a_0.wrapping_rem(src as cty::c_ulonglong);
        quot_0 = a_0.wrapping_div(src as cty::c_ulonglong);
        if quot_0 > 0xffffffffu32 as cty::c_ulonglong {
            regs.flags.v = 1i32 as cty::c_uint;
            regs.flags.n = 1i32 as cty::c_uint;
            regs.flags.c = 0i32 as cty::c_uint
        } else {
            regs.flags.v = 0i32 as cty::c_uint;
            regs.flags.c = 0i32 as cty::c_uint;
            regs.flags.z =
                (quot_0 as int32_t == 0i32) as cty::c_int as cty::c_uint;
            regs.flags.n =
                ((quot_0 as int32_t) < 0i32) as cty::c_int as cty::c_uint;
            regs.regs[(extra as cty::c_int & 7i32) as usize] =
                rem_0 as uint32_t;
            regs.regs[(extra as cty::c_int >> 12i32 & 7i32) as usize] =
                quot_0 as uint32_t
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn m68k_mull(mut opcode: uint32_t, mut src: uint32_t,
                                   mut extra: uint16_t) {
    if 0 != extra as cty::c_int & 0x800i32 {
        /* signed variant */
        let mut a: cty::c_longlong =
            regs.regs[(extra as cty::c_int >> 12i32 & 7i32) as usize] as
                int32_t as cty::c_longlong;
        a *= src as int32_t as cty::c_longlong;
        regs.flags.v = 0i32 as cty::c_uint;
        regs.flags.c = 0i32 as cty::c_uint;
        regs.flags.z =
            (a == 0i32 as cty::c_longlong) as cty::c_int as cty::c_uint;
        regs.flags.n =
            (a < 0i32 as cty::c_longlong) as cty::c_int as cty::c_uint;
        if 0 != extra as cty::c_int & 0x400i32 {
            regs.regs[(extra as cty::c_int & 7i32) as usize] =
                (a >> 32i32) as uint32_t
        } else if a as cty::c_ulonglong & 0xffffffff80000000u64 !=
                      0i32 as cty::c_ulonglong &&
                      a as cty::c_ulonglong & 0xffffffff80000000u64 !=
                          0xffffffff80000000u64 {
            regs.flags.v = 1i32 as cty::c_uint
        }
        regs.regs[(extra as cty::c_int >> 12i32 & 7i32) as usize] =
            a as uint32_t
    } else {
        /* unsigned */
        let mut a_0: cty::c_ulonglong =
            regs.regs[(extra as cty::c_int >> 12i32 & 7i32) as usize] as
                cty::c_ulonglong;
        a_0 = a_0.wrapping_mul(src as cty::c_ulonglong);
        regs.flags.v = 0i32 as cty::c_uint;
        regs.flags.c = 0i32 as cty::c_uint;
        regs.flags.z =
            (a_0 == 0i32 as cty::c_ulonglong) as cty::c_int as cty::c_uint;
        regs.flags.n =
            ((a_0 as cty::c_longlong) < 0i32 as cty::c_longlong) as
                cty::c_int as cty::c_uint;
        if 0 != extra as cty::c_int & 0x400i32 {
            regs.regs[(extra as cty::c_int & 7i32) as usize] =
                (a_0 >> 32i32) as uint32_t
        } else if a_0 & 0xffffffff00000000u64 != 0i32 as cty::c_ulonglong {
            regs.flags.v = 1i32 as cty::c_uint
        }
        regs.regs[(extra as cty::c_int >> 12i32 & 7i32) as usize] =
            a_0 as uint32_t
    };
}
static mut ccnames: [*mut cty::c_char; 16] =
    [b"T\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"F\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"HI\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"LS\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"CC\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"CS\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"NE\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"EQ\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"VC\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"VS\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"PL\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"MI\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"GE\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"LT\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"GT\x00" as *const u8 as *const cty::c_char as *mut cty::c_char,
     b"LE\x00" as *const u8 as *const cty::c_char as *mut cty::c_char];
/* NO_GDB */
#[no_mangle]
pub unsafe extern "C" fn m68k_reset() {
    /* The CPU is always the same in TiEmu. */
    /* 0 */
    //    regs.kick_mask = 0x00F80000;
    regs.spcflags = 0i32 as uint32_t;
    /* 0 */
    *regs.regs.as_mut_ptr().offset(8isize).offset(7isize) = tihw.initial_ssp;
    m68k_setpc(tihw.initial_pc as uintptr_t);
    regs.s = 1i32 as flagtype;
    //    regs.m = 0;
    regs.stopped = 0i32 as flagtype;
    regs.t1 = 0i32 as flagtype;
    //    regs.t0 = 0;
    regs.flags.z = 0i32 as cty::c_uint;
    regs.flags.x = 0i32 as cty::c_uint;
    regs.flags.c = 0i32 as cty::c_uint;
    regs.flags.v = 0i32 as cty::c_uint;
    regs.flags.n = 0i32 as cty::c_uint;
    regs.intmask = 7i32;
    regs.dfc = 0i32 as uint32_t;
    regs.sfc = regs.dfc;
    regs.vbr = regs.sfc;
    regs.fpiar = 0i32 as uint32_t;
    regs.fpsr = regs.fpiar;
    regs.fpcr = regs.fpsr;
    regs.irc = 0xffffi32 as uint16_t;
    fill_prefetch_slow();
}
#[no_mangle]
pub unsafe extern "C" fn op_illg(mut opcode: uint32_t) -> cty::c_ulong {
    /* 0 */
    if opcode & 0xf000i32 as cty::c_uint == 0xf000i32 as cty::c_uint {
        Exception(0xbi32, 0i32 as uintptr_t);
        return 4i32 as cty::c_ulong
    }
    if opcode & 0xf000i32 as cty::c_uint == 0xa000i32 as cty::c_uint {
        /* 0 */
        /* CYGNUS_SIM */
        Exception(0xai32, 0i32 as uintptr_t);
        return 4i32 as cty::c_ulong
    }
    Exception(4i32, 0i32 as uintptr_t);
    return 4i32 as cty::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn mmu_op(mut opcode: uint32_t, mut extra: uint16_t) {
    if opcode & 0xfe0i32 as cty::c_uint == 0x500i32 as cty::c_uint {
        /* PFLUSH */
        mmusr = 0i32 as uint32_t;
        printf(b"PFLUSH\n\x00" as *const u8 as *const cty::c_char);
    } else if opcode & 0xfd8i32 as cty::c_uint == 0x548i32 as cty::c_uint {
        /* PTEST */
        printf(b"PTEST\n\x00" as *const u8 as *const cty::c_char);
    } else { op_illg(opcode); };
}
#[no_mangle]
pub unsafe extern "C" fn m68k_disasm(mut output: *mut cty::c_char,
                                     mut addr: uintptr_t) -> cty::c_int {
    let mut buf: [cty::c_char; 273] = [0; 273];
    let mut newpc: uintptr_t = 0i32 as uintptr_t;
    let mut instrname: [cty::c_char; 20] = [0; 20];
    let mut ccpt: *mut cty::c_char = 0 as *mut cty::c_char;
    let mut opcode: uint32_t = 0;
    let mut lookup: *mut mnemolookup = 0 as *mut mnemolookup;
    let mut dp: *mut instr = 0 as *mut instr;
    let mut orig_opcode: uint32_t = 0;
    let mut nextpc: uintptr_t = 0;
    m68kpc_offset = addr.wrapping_sub(m68k_getpc()) as cty::c_long;
    *output.offset(0isize) = '\u{0}' as i32 as cty::c_char;
    sprintf(buf.as_mut_ptr(),
            b"%06lx: \x00" as *const u8 as *const cty::c_char,
            m68k_getpc().wrapping_add(m68kpc_offset as cty::c_ulong));
    strcat(output, buf.as_mut_ptr());
    opcode =
        hw_get_word((regs.pc as cty::c_long +
                         regs.pc_p.wrapping_offset_from(regs.pc_oldp) as
                             cty::c_long + m68kpc_offset) as uint32_t) as
            uint32_t;
    orig_opcode = opcode;
    m68kpc_offset += 2i32 as cty::c_long;
    if cpufunctbl[opcode as usize] ==
           Some(op_illg_1 as
                    unsafe extern "C" fn(_: uint32_t) -> cty::c_ulong) ||
           orig_opcode & 0xf000i32 as cty::c_uint ==
               0xa000i32 as cty::c_uint ||
           orig_opcode & 0xf000i32 as cty::c_uint ==
               0xf000i32 as cty::c_uint {
        if !(orig_opcode >= 0xf800i32 as cty::c_uint &&
                 orig_opcode <= 0xfff2i32 as cty::c_uint) {
            opcode = 0x4afci32 as uint32_t
        }
    }
    dp = table68k.offset(opcode as isize);
    lookup = lookuptab.as_mut_ptr();
    while (*lookup).mnemo as cty::c_uint != (*dp).mnemo() {
        lookup = lookup.offset(1isize)
    }
    strcpy(instrname.as_mut_ptr(), (*lookup).name);
    ccpt =
        strstr(instrname.as_mut_ptr(),
               b"cc\x00" as *const u8 as *const cty::c_char);
    if !ccpt.is_null() {
        strncpy(ccpt, ccnames[(*dp).cc() as usize], 2i32 as cty::c_ulong);
    }
    strcat(output, instrname.as_mut_ptr());
    match (*dp).size() as cty::c_int {
        0 => {
            strcat(output, b".B \x00" as *const u8 as *const cty::c_char);
        }
        1 => {
            strcat(output, b".W \x00" as *const u8 as *const cty::c_char);
        }
        2 => {
            strcat(output, b".L \x00" as *const u8 as *const cty::c_char);
        }
        _ => {
            strcat(output, b"   \x00" as *const u8 as *const cty::c_char);
        }
    }
    if 0 != (*dp).suse() {
        newpc = m68k_getpc().wrapping_add(m68kpc_offset as cty::c_ulong);
        newpc =
            (newpc as
                 cty::c_ulong).wrapping_add(ShowEA(0 as *mut FILE,
                                                    (*dp).sreg as cty::c_int,
                                                    (*dp).smode() as amodes,
                                                    (*dp).size() as wordsizes,
                                                    output) as cty::c_ulong)
                as uintptr_t as uintptr_t
    }
    if 0 != (*dp).suse() as cty::c_int && 0 != (*dp).duse() as cty::c_int {
        strcat(output, b",\x00" as *const u8 as *const cty::c_char);
    }
    if 0 != (*dp).duse() {
        newpc = m68k_getpc().wrapping_add(m68kpc_offset as cty::c_ulong);
        newpc =
            (newpc as
                 cty::c_ulong).wrapping_add(ShowEA(0 as *mut FILE,
                                                    (*dp).dreg as cty::c_int,
                                                    (*dp).dmode() as amodes,
                                                    (*dp).size() as wordsizes,
                                                    output) as cty::c_ulong)
                as uintptr_t as uintptr_t
    }
    if !ccpt.is_null() {
        if 0 != cctrue((*dp).cc() as cty::c_int) {
            sprintf(buf.as_mut_ptr(),
                    b" [%06lX] (TRUE)\x00" as *const u8 as
                        *const cty::c_char, newpc);
            strcat(output, buf.as_mut_ptr());
        } else {
            sprintf(buf.as_mut_ptr(),
                    b" [%06lX] (FALSE)\x00" as *const u8 as
                        *const cty::c_char, newpc);
            strcat(output, buf.as_mut_ptr());
        }
    } else if opcode & 0xff00i32 as cty::c_uint == 0x6100i32 as cty::c_uint
     {
        /* BSR */
        sprintf(buf.as_mut_ptr(),
                b" [%s]\x00" as *const u8 as *const cty::c_char,
                sym_addr(newpc as uint32_t)); /* addr */
        strcat(output, buf.as_mut_ptr());
    } else if orig_opcode >= 0xf800i32 as cty::c_uint &&
                  orig_opcode <= 0xfff2i32 as cty::c_uint {
        let mut buffer: *mut cty::c_char =
            &mut *output.offset(8isize) as *mut cty::c_char;
        let mut pm: cty::c_ulong = 0;
        let mut pc: uint32_t = m68k_getpc() as uint32_t;
        /* F-Line ROM calls (see KerNO doc and thanks to Lionel Debroux) */
        match orig_opcode {
            65520 => {
                /* 6 byte bsr w/long word displacement */
                pm =
                    hw_get_long((regs.pc as cty::c_long +
                                     regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                         as cty::c_long + m68kpc_offset) as
                                    uint32_t) as cty::c_ulong;
                m68kpc_offset += (6i32 - 2i32) as cty::c_long;
                if 0 != pm & 0x8000i32 as cty::c_ulong {
                    sprintf(buffer,
                            b"FLINE bsr.l *-$%lX [%lX]\x00" as *const u8 as
                                *const cty::c_char,
                            -(pm as int32_t as cty::c_long) -
                                2i32 as cty::c_long,
                            pc as cty::c_long + pm as int32_t as cty::c_long
                                + 2i32 as cty::c_long);
                } else {
                    sprintf(buffer,
                            b"FLINE bsr.l *+$%lX [%lX]\x00" as *const u8 as
                                *const cty::c_char,
                            pm.wrapping_add(2i32 as cty::c_ulong),
                            (pc as
                                 cty::c_ulong).wrapping_add(pm).wrapping_add(2i32
                                                                                  as
                                                                                  cty::c_ulong));
                }
            }
            65521 => {
                /* 6 byte bra w/long word displacement */
                pm =
                    hw_get_long((regs.pc as cty::c_long +
                                     regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                         as cty::c_long + m68kpc_offset) as
                                    uint32_t) as cty::c_ulong;
                m68kpc_offset += (6i32 - 2i32) as cty::c_long;
                if 0 != pm & 0x8000i32 as cty::c_ulong {
                    sprintf(buffer,
                            b"FLINE bra.l *-$%lX [%lX]\x00" as *const u8 as
                                *const cty::c_char,
                            -(pm as int32_t as cty::c_long) -
                                2i32 as cty::c_long,
                            pc as cty::c_long + pm as int32_t as cty::c_long
                                + 2i32 as cty::c_long);
                } else {
                    sprintf(buffer,
                            b"FLINE bra.l *+$%lX [%lX]\x00" as *const u8 as
                                *const cty::c_char,
                            pm.wrapping_add(2i32 as cty::c_ulong),
                            (pc as
                                 cty::c_ulong).wrapping_add(pm).wrapping_add(2i32
                                                                                  as
                                                                                  cty::c_ulong));
                }
            }
            65522 => {
                /* 4 byte ROM CALL */
                pm =
                    hw_get_word((regs.pc as cty::c_long +
                                     regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                         as cty::c_long + m68kpc_offset) as
                                    uint32_t) as cty::c_ulong;
                m68kpc_offset += (4i32 - 2i32) as cty::c_long;
                sprintf(buffer,
                        b"FLINE $%04x.l [%s]\x00" as *const u8 as
                            *const cty::c_char,
                        pm.wrapping_div(4i32 as cty::c_ulong),
                        romcalls_get_name(pm.wrapping_div(4i32 as
                                                              cty::c_ulong) as _));
            }
            65518 => {
                /* jmp __ld_entry_point_plus_0x8000+word */
                pm =
                    hw_get_word((regs.pc as cty::c_long +
                                     regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                         as cty::c_long + m68kpc_offset) as
                                    uint32_t) as cty::c_ulong;
                m68kpc_offset += (4i32 - 2i32) as cty::c_long;
                let mut handle: cty::c_int = 0;
                let mut addr_0: uint32_t = 0;
                heap_search_for_address(pc.wrapping_add(2i32 as cty::c_uint),
                                        &mut handle);
                if handle > 0i32 {
                    heap_get_block_addr(handle, &mut addr_0);
                } else { addr_0 = 0i32 as uint32_t }
                sprintf(buffer,
                        b"FLINE jmp.w *+$%lX [%lX]\x00" as *const u8 as
                            *const cty::c_char,
                        pm as cty::c_short as cty::c_long +
                            0x8000i32 as cty::c_long,
                        addr_0 as cty::c_long +
                            pm as cty::c_short as cty::c_long +
                            0x8000i32 as cty::c_long);
            }
            65519 => {
                /* jsr __ld_entry_point_plus_0x8000+word */
                pm =
                    hw_get_word((regs.pc as cty::c_long +
                                     regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                         as cty::c_long + m68kpc_offset) as
                                    uint32_t) as cty::c_ulong;
                m68kpc_offset += (4i32 - 2i32) as cty::c_long;
                let mut handle_0: cty::c_int = 0;
                let mut addr_1: uint32_t = 0;
                heap_search_for_address(pc.wrapping_add(2i32 as cty::c_uint),
                                        &mut handle_0);
                if handle_0 > 0i32 {
                    heap_get_block_addr(handle_0, &mut addr_1);
                } else { addr_1 = 0i32 as uint32_t }
                sprintf(buffer,
                        b"FLINE jsr.w *+$%lX [%lX]\x00" as *const u8 as
                            *const cty::c_char,
                        pm as cty::c_short as cty::c_long +
                            0x8000i32 as cty::c_long,
                        addr_1 as cty::c_long +
                            pm as cty::c_short as cty::c_long +
                            0x8000i32 as cty::c_long);
            }
            63669 => {
                /* 2 byte ROM call followed by an FPU opcode (special case: _bcd_math) */
                let mut tmp: [cty::c_char; 64] = [0; 64];
                pm =
                    hw_get_word((regs.pc as cty::c_long +
                                     regs.pc_p.wrapping_offset_from(regs.pc_oldp)
                                         as cty::c_long + m68kpc_offset) as
                                    uint32_t) as cty::c_ulong;
                m68kpc_offset += (4i32 - 2i32) as cty::c_long;
                DasmFPU(pm as uint16_t, tmp.as_mut_ptr());
                sprintf(buffer,
                        b"FLINE _bcd_math (FPU: %s)\x00" as *const u8 as
                            *const cty::c_char, tmp.as_mut_ptr());
            }
            _ => {
                /* 2 byte ROM CALL */
                sprintf(buffer,
                        b"FLINE $%03x.w [%s]\x00" as *const u8 as
                            *const cty::c_char,
                        opcode & 0x7ffi32 as cty::c_uint,
                        romcalls_get_name(opcode as i32 & 0x7ffi32));
            }
        }
    } else if orig_opcode & 0xf000i32 as cty::c_uint ==
                  0xa000i32 as cty::c_uint {
        /* ER_throw */
        let mut buffer_0: *mut cty::c_char =
            &mut *output.offset(8isize) as *mut cty::c_char;
        sprintf(buffer_0,
                b"ER_throw %d [%s]\x00" as *const u8 as *const cty::c_char,
                opcode & 0xfffi32 as cty::c_uint,
                ercodes_get_name(opcode & 0xfffi32 as cty::c_uint));
    } else if opcode == 0x4afci32 as cty::c_uint &&
                  orig_opcode != 0x4afci32 as cty::c_uint {
        /* illegal instruction, but not ILLEGAL */
        sprintf(output,
                b"%06lx: DC.W $%04X\x00" as *const u8 as *const cty::c_char,
                addr, orig_opcode);
    }
    nextpc = m68k_getpc().wrapping_add(m68kpc_offset as cty::c_ulong);
    return nextpc.wrapping_sub(addr) as cty::c_int;
}
/* NO_GDB */
/* 0 */
unsafe extern "C" fn exception3f(mut opcode: uint32_t, mut addr: uintptr_t,
                                 mut fault: uintptr_t,
                                 mut writeaccess: cty::c_int,
                                 mut instructionaccess: cty::c_int) {
    last_addr_for_exception_3 = addr;
    last_fault_for_exception_3 = fault;
    last_op_for_exception_3 = opcode as uint16_t;
    last_writeaccess_for_exception_3 = writeaccess;
    last_instructionaccess_for_exception_3 = instructionaccess;
    Exception(3i32, fault);
}
#[no_mangle]
pub unsafe extern "C" fn exception3(mut opcode: uint32_t, mut addr: uintptr_t,
                                    mut fault: uintptr_t) {
    exception3f(opcode, addr, fault, 0i32, 0i32);
}
/*
 * UAE - The Un*x Amiga Emulator
 *
 * MC68000 emulation
 *
 * Copyright 1995 Bernd Schmidt
 * $Id: newcpu.h 2085 2006-05-16 19:28:18Z roms $
 */
/* You can set this to long double to be more accurate. However, the
   resulting alignment issues will cost a lot of performance in some
   apps */
/*,msp*/
//    flagtype t0;
//    flagtype m;
/* NOTE stuff related to simulator framework */
/* NOTE need to have m68k performance information here */
/* NOTE control information */
/* NOTE simulator information */
/* CYGNUS_SIM */
/* CYGNUS_SIM */
/* These are only used by the 68020/68881 code, and therefore don't
 * need to handle prefetch.  */
/* A traced STOP instruction drops through immediately without
	   actually stopping.  */
/* 0 */
/* NO_GDB */
#[no_mangle]
pub unsafe extern "C" fn exception3i(mut opcode: uint32_t,
                                     mut addr: uintptr_t,
                                     mut fault: uintptr_t) {
    exception3f(opcode, addr, fault, 0i32, 1i32);
}
