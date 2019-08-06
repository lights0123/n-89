#![allow(dead_code,
mutable_transmutes,
non_camel_case_types,
non_snake_case,
non_upper_case_globals,
unused_assignments,
unused_mut)]
extern crate c2rust_bitfields;

use c2rust_bitfields::BitfieldStruct;
use ndless::prelude::*;

use crate::libc::*;

extern "C" {
	pub type _IO_wide_data;
	pub type _IO_codecvt;
	pub type _IO_marker;
	#[no_mangle]
	fn xmalloc(_: size_t) -> *mut cty::c_void;
	#[no_mangle]
	fn __ctype_b_loc() -> *mut *const cty::c_ushort;
	#[no_mangle]
	static mut stderr: *mut FILE;
	#[no_mangle]
	fn fprintf(_: *mut FILE, _: *const cty::c_char, _: ...) -> cty::c_int;
	#[no_mangle]
	fn strcmp(_: *const cty::c_char, _: *const cty::c_char) -> cty::c_int;
	#[no_mangle]
	fn strncmp(_: *const cty::c_char, _: *const cty::c_char,
	           _: cty::c_ulong) -> cty::c_int;
	#[no_mangle]
	fn strlen(_: *const cty::c_char) -> cty::c_ulong;
	#[no_mangle]
	static mut defs68k: [instr_def; 0];
	#[no_mangle]
	static mut n_defs68k: cty::c_int;
}

pub type __int8_t = cty::c_schar;
pub type __uint16_t = cty::c_ushort;
pub type __int32_t = cty::c_int;
pub type __off_t = cty::c_long;
pub type __off64_t = cty::c_long;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type uint16_t = __uint16_t;
pub type size_t = cty::c_ulong;
pub type C2RustUnnamed = cty::c_uint;

pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mnemolookup {
	pub mnemo: instrmnem,
	pub name: *const cty::c_char,
}

pub type wordsizes = cty::c_uint;

pub const sz_long: wordsizes = 2;
pub const sz_word: wordsizes = 1;
pub const sz_byte: wordsizes = 0;

pub type C2RustUnnamed_0 = cty::c_uint;

pub const fa_isbranch: C2RustUnnamed_0 = 7;
pub const fa_isjmp: C2RustUnnamed_0 = 6;
pub const fa_unknown: C2RustUnnamed_0 = 5;
pub const fa_dontcare: C2RustUnnamed_0 = 4;
pub const fa_one: C2RustUnnamed_0 = 3;
pub const fa_zero: C2RustUnnamed_0 = 2;
pub const fa_unset: C2RustUnnamed_0 = 1;
pub const fa_set: C2RustUnnamed_0 = 0;

pub type C2RustUnnamed_1 = cty::c_uint;

pub const fu_isjmp: C2RustUnnamed_1 = 4;
pub const fu_unknown: C2RustUnnamed_1 = 3;
pub const fu_maybecc: C2RustUnnamed_1 = 2;
pub const fu_unused: C2RustUnnamed_1 = 1;
pub const fu_used: C2RustUnnamed_1 = 0;

pub type C2RustUnnamed_2 = cty::c_uint;

pub const lastbit: C2RustUnnamed_2 = 19;
pub const bitp: C2RustUnnamed_2 = 18;
pub const bitz: C2RustUnnamed_2 = 17;
pub const bitR: C2RustUnnamed_2 = 16;
pub const bitr: C2RustUnnamed_2 = 15;
pub const bitD: C2RustUnnamed_2 = 14;
pub const bitd: C2RustUnnamed_2 = 13;
pub const bitS: C2RustUnnamed_2 = 12;
pub const bits: C2RustUnnamed_2 = 11;
pub const bitK: C2RustUnnamed_2 = 10;
pub const bitk: C2RustUnnamed_2 = 9;
pub const bitJ: C2RustUnnamed_2 = 8;
pub const bitj: C2RustUnnamed_2 = 7;
pub const bitI: C2RustUnnamed_2 = 6;
pub const biti: C2RustUnnamed_2 = 5;
pub const bitf: C2RustUnnamed_2 = 4;
pub const bitC: C2RustUnnamed_2 = 3;
pub const bitc: C2RustUnnamed_2 = 2;
pub const bit1: C2RustUnnamed_2 = 1;
pub const bit0: C2RustUnnamed_2 = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct instr_def {
	pub bits: cty::c_uint,
	pub n_variable: cty::c_int,
	pub bitpos: [cty::c_char; 16],
	pub mask: cty::c_uint,
	pub cpulevel: cty::c_int,
	pub plevel: cty::c_int,
	pub flaginfo: [C2RustUnnamed_3; 5],
	pub sduse: cty::c_uchar,
	pub opcstr: *const cty::c_char,
}

#[derive(BitfieldStruct, Clone, Copy)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
	#[bitfield(name = "flaguse", ty = "cty::c_uint", bits = "0..=2")]
	#[bitfield(name = "flagset", ty = "cty::c_uint", bits = "3..=5")]
	pub flaguse_flagset: [u8; 1],
	#[bitfield(padding)]
	pub _pad: [u8; 3],
}

#[derive(BitfieldStruct, Clone, Copy)]
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
/* Hey EMACS -*- linux-c -*- */
/* $Id: readcpu.c 2681 2007-11-20 18:01:02Z roms $ */
/*
 * UAE - The Un*x Amiga Emulator
 *
 * Read 68000 CPU specs from file "table68k"
 *
 * Copyright 1995,1996 Bernd Schmidt
 */
#[no_mangle]
pub static mut nr_cpuop_funcs: cty::c_int = 0;
#[no_mangle]
pub static mut lookuptab: [mnemolookup; 130] =
	[mnemolookup {
		mnemo: i_MOVEQ,
		name: b"MOVEQ\x00" as *const u8 as *const cty::c_char,
	},
		mnemolookup {
			mnemo: i_ADDQ,
			name: b"ADDQ\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_SUBQ,
			name: b"SUBQ\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CMPI,
			name: b"CMPI\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ADDI,
			name: b"ADDI\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_SUBI,
			name: b"SUBI\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ORI,
			name: b"ORI\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ANDI,
			name: b"ANDI\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_EORI,
			name: b"EORI\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ILLG,
			name: b"ILLEGAL\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_OR,
			name: b"OR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CHK,
			name: b"CHK\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CHK2,
			name: b"CHK2\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_AND,
			name: b"AND\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_EOR,
			name: b"EOR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ORSR,
			name: b"ORSR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ANDSR,
			name: b"ANDSR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_EORSR,
			name: b"EORSR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_SUB,
			name: b"SUB\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_SUBA,
			name: b"SUBA\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_SUBX,
			name: b"SUBX\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_SBCD,
			name: b"SBCD\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ADD,
			name: b"ADD\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ADDA,
			name: b"ADDA\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ADDX,
			name: b"ADDX\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ABCD,
			name: b"ABCD\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_NEG,
			name: b"NEG\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_NEGX,
			name: b"NEGX\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_NBCD,
			name: b"NBCD\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CLR,
			name: b"CLR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_NOT,
			name: b"NOT\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_TST,
			name: b"TST\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BTST,
			name: b"BTST\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BCHG,
			name: b"BCHG\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BCLR,
			name: b"BCLR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BSET,
			name: b"BSET\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CMP,
			name: b"CMP\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CMPM,
			name: b"CMPM\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CMPA,
			name: b"CMPA\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MVPRM,
			name: b"MVPRM\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MVPMR,
			name: b"MVPMR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MOVE,
			name: b"MOVE\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MOVEA,
			name: b"MOVEA\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MVSR2,
			name: b"MVSR2\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MV2SR,
			name: b"MV2SR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_SWAP,
			name: b"SWAP\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_EXG,
			name: b"EXG\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_EXT,
			name: b"EXT\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MVMEL,
			name: b"MVMEL\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MVMLE,
			name: b"MVMLE\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_TRAP,
			name: b"TRAP\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MVR2USP,
			name: b"MVR2USP\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MVUSP2R,
			name: b"MVUSP2R\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_NOP,
			name: b"NOP\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_RESET,
			name: b"RESET\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_RTE,
			name: b"RTE\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_RTD,
			name: b"RTD\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_LINK,
			name: b"LINK\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_UNLK,
			name: b"UNLK\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_RTS,
			name: b"RTS\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_STOP,
			name: b"STOP\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_TRAPV,
			name: b"TRAPV\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_RTR,
			name: b"RTR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_JSR,
			name: b"JSR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_JMP,
			name: b"JMP\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BSR,
			name: b"BSR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_Bcc,
			name: b"Bcc\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_LEA,
			name: b"LEA\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_PEA,
			name: b"PEA\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_DBcc,
			name: b"DBcc\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_Scc,
			name: b"Scc\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_DIVU,
			name: b"DIVU\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_DIVS,
			name: b"DIVS\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MULU,
			name: b"MULU\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MULS,
			name: b"MULS\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ASR,
			name: b"ASR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ASL,
			name: b"ASL\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_LSR,
			name: b"LSR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_LSL,
			name: b"LSL\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ROL,
			name: b"ROL\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ROR,
			name: b"ROR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ROXL,
			name: b"ROXL\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ROXR,
			name: b"ROXR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ASRW,
			name: b"ASRW\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ASLW,
			name: b"ASLW\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_LSRW,
			name: b"LSRW\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_LSLW,
			name: b"LSLW\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ROLW,
			name: b"ROLW\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_RORW,
			name: b"RORW\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ROXLW,
			name: b"ROXLW\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ROXRW,
			name: b"ROXRW\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MOVE2C,
			name: b"MOVE2C\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MOVEC2,
			name: b"MOVEC2\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CAS,
			name: b"CAS\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CAS2,
			name: b"CAS2\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MULL,
			name: b"MULL\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_DIVL,
			name: b"DIVL\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BFTST,
			name: b"BFTST\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BFEXTU,
			name: b"BFEXTU\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BFCHG,
			name: b"BFCHG\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BFEXTS,
			name: b"BFEXTS\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BFCLR,
			name: b"BFCLR\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BFFFO,
			name: b"BFFFO\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BFSET,
			name: b"BFSET\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BFINS,
			name: b"BFINS\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_PACK,
			name: b"PACK\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_UNPK,
			name: b"UNPK\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_TAS,
			name: b"TAS\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_BKPT,
			name: b"BKPT\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CALLM,
			name: b"CALLM\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_RTM,
			name: b"RTM\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_TRAPcc,
			name: b"TRAPcc\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MOVES,
			name: b"MOVES\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_FPP,
			name: b"FPP\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_FDBcc,
			name: b"FDBcc\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_FScc,
			name: b"FScc\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_FTRAPcc,
			name: b"FTRAPcc\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_FBcc,
			name: b"FBcc\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_FBcc,
			name: b"FBcc\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_FSAVE,
			name: b"FSAVE\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_FRESTORE,
			name: b"FRESTORE\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CINVL,
			name: b"CINVL\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CINVP,
			name: b"CINVP\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CINVA,
			name: b"CINVA\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CPUSHL,
			name: b"CPUSHL\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CPUSHP,
			name: b"CPUSHP\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_CPUSHA,
			name: b"CPUSHA\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MOVE16,
			name: b"MOVE16\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_MMUOP,
			name: b"MMUOP\x00" as *const u8 as *const cty::c_char,
		},
		mnemolookup {
			mnemo: i_ILLG,
			name: b"\x00" as *const u8 as *const cty::c_char,
		}];
#[no_mangle]
pub static mut table68k: *mut instr = 0 as *const instr as *mut instr;

#[inline(always)]
unsafe extern "C" fn mode_from_str(mut str: *const cty::c_char) -> amodes {
	if strncmp(str, b"Dreg\x00" as *const u8 as *const cty::c_char,
	           4i32 as cty::c_ulong) == 0i32 {
		return Dreg
	}
	if strncmp(str, b"Areg\x00" as *const u8 as *const cty::c_char,
	           4i32 as cty::c_ulong) == 0i32 {
		return Areg
	}
	if strncmp(str, b"Aind\x00" as *const u8 as *const cty::c_char,
	           4i32 as cty::c_ulong) == 0i32 {
		return Aind
	}
	if strncmp(str, b"Apdi\x00" as *const u8 as *const cty::c_char,
	           4i32 as cty::c_ulong) == 0i32 {
		return Apdi
	}
	if strncmp(str, b"Aipi\x00" as *const u8 as *const cty::c_char,
	           4i32 as cty::c_ulong) == 0i32 {
		return Aipi
	}
	if strncmp(str, b"Ad16\x00" as *const u8 as *const cty::c_char,
	           4i32 as cty::c_ulong) == 0i32 {
		return Ad16
	}
	if strncmp(str, b"Ad8r\x00" as *const u8 as *const cty::c_char,
	           4i32 as cty::c_ulong) == 0i32 {
		return Ad8r
	}
	if strncmp(str, b"absw\x00" as *const u8 as *const cty::c_char,
	           4i32 as cty::c_ulong) == 0i32 {
		return absw
	}
	if strncmp(str, b"absl\x00" as *const u8 as *const cty::c_char,
	           4i32 as cty::c_ulong) == 0i32 {
		return absl
	}
	if strncmp(str, b"PC16\x00" as *const u8 as *const cty::c_char,
	           4i32 as cty::c_ulong) == 0i32 {
		return PC16
	}
	if strncmp(str, b"PC8r\x00" as *const u8 as *const cty::c_char,
	           4i32 as cty::c_ulong) == 0i32 {
		return PC8r
	}
	if strncmp(str, b"Immd\x00" as *const u8 as *const cty::c_char,
	           4i32 as cty::c_ulong) == 0i32 {
		return imm
	}
	panic!();
}

#[inline(always)]
unsafe extern "C" fn mode_from_mr(mut mode: cty::c_int, mut reg: cty::c_int)
                                  -> amodes {
	match mode {
		0 => { return Dreg }
		1 => { return Areg }
		2 => { return Aind }
		3 => { return Aipi }
		4 => { return Apdi }
		5 => { return Ad16 }
		6 => { return Ad8r }
		7 => {
			match reg {
				0 => { return absw }
				1 => { return absl }
				2 => { return PC16 }
				3 => { return PC8r }
				4 => { return imm }
				5 | 6 | 7 => { return am_illg }
				_ => {}
			}
		}
		_ => {}
	}
	panic!();
}

fn isspace(ch: u8) -> bool {
	b" \t\n0x0b0x0c\r".contains(&ch)
}

unsafe extern "C" fn build_insn(mut insn: cty::c_int) {
	let mut current_block: u64;
	let mut find: cty::c_int = -1i32;
	let mut variants: cty::c_int = 0;
	let mut isjmp: cty::c_int = 0i32;
	let mut id: instr_def =
		instr_def {
			bits: 0,
			n_variable: 0,
			bitpos: [0; 16],
			mask: 0,
			cpulevel: 0,
			plevel: 0,
			flaginfo:
			[C2RustUnnamed_3 {
				flaguse_flagset: [0; 1],
				_pad: [0; 3],
			}; 5],
			sduse: 0,
			opcstr: 0 as *const cty::c_char,
		};
	let mut opcstr: *const cty::c_char = 0 as *const cty::c_char;
	let mut i: cty::c_int = 0;
	let mut flaglive: cty::c_int = 0i32;
	let mut flagdead: cty::c_int = 0i32;
	id = *defs68k.as_mut_ptr().offset(insn as isize);
	/* Note: We treat anything with unknown flags as a jump. That
	   is overkill, but "the programmer" was lazy quite often, and
	   *this* programmer can't be bothered to work out what can and
	   can't trap. Usually, this will be overwritten with the gencomp
	   based information, anyway. */
	for i in 0..5 {
		match id.flaginfo[i as usize].flagset() as cty::c_int {
			6 => { isjmp = 1i32 }
			7 => { isjmp = 1i32 }
			2 => { flagdead |= 1i32 << i }
			3 => { flagdead |= 1i32 << i }
			4 => { flagdead |= 1i32 << i }
			5 => {
				isjmp = 1i32;
				flagdead = -1i32;
				break;
			}
			0 => { flagdead |= 1i32 << i }
			1 | _ => {}
		}
	}
	for i in 0..5 {
		match id.flaginfo[i as usize].flaguse() as cty::c_int {
			4 => {
				isjmp = 1i32;
				flaglive |= 1i32 << i
			}
			2 => {
				isjmp = 1i32;
				flaglive |= 1i32 << i
			}
			3 => {
				isjmp = 1i32;
				flaglive |= 1i32 << i
			}
			0 => { flaglive |= 1i32 << i }
			1 | _ => {}
		}
	}
	opcstr = id.opcstr;
	variants = 0i32;
	while variants < 1i32 << id.n_variable {
		let mut bitcnt: [cty::c_int; 19] = [0; 19];
		let mut bitval: [cty::c_int; 19] = [0; 19];
		let mut bitpos: [cty::c_int; 19] = [0; 19];
		let mut i_0: cty::c_int = 0;
		let mut opc: uint16_t = id.bits as uint16_t;
		let mut msk: uint16_t = 0;
		let mut vmsk: uint16_t = 0;
		let mut pos: cty::c_int = 0i32;
		let mut mnp: cty::c_int = 0i32;
		let mut bitno: cty::c_int = 0i32;
		let mut mnemonic: [cty::c_char; 10] = [0; 10];
		let mut sz: wordsizes = sz_long;
		let mut srcgather: cty::c_int = 0i32;
		let mut dstgather: cty::c_int = 0i32;
		let mut usesrc: cty::c_int = 0i32;
		let mut usedst: cty::c_int = 0i32;
		let mut srctype: cty::c_int = 0i32;
		let mut srcpos: cty::c_int = -1i32;
		let mut dstpos: cty::c_int = -1i32;
		let mut srcmode: amodes = am_unknown;
		let mut destmode: amodes = am_unknown;
		let mut srcreg: cty::c_int = -1i32;
		let mut destreg: cty::c_int = -1i32;
		for i in 0..lastbit as usize {
			bitcnt[i] = 0;
			bitval[i] = 0;
		}
		vmsk = (1i32 << id.n_variable) as uint16_t;
		i_0 = 0i32;
		msk = 0x8000i32 as uint16_t;
		while i_0 < 16i32 {
			if 0 == msk as cty::c_uint & id.mask {
				let fresh0 = bitno;
				bitno = bitno + 1;
				let mut currbit: cty::c_int =
					id.bitpos[fresh0 as usize] as cty::c_int;
				let mut bit_set: cty::c_int = 0;
				vmsk = (vmsk as cty::c_int >> 1i32) as uint16_t;
				bit_set =
					if 0 != variants & vmsk as cty::c_int {
						1i32
					} else { 0i32 };
				if 0 != bit_set {
					opc =
						(opc as cty::c_int | msk as cty::c_int) as uint16_t
				}
				bitpos[currbit as usize] = 15i32 - i_0;
				bitcnt[currbit as usize] += 1;
				bitval[currbit as usize] <<= 1i32;
				bitval[currbit as usize] |= bit_set
			}
			i_0 += 1;
			msk = (msk as cty::c_int >> 1i32) as uint16_t
		}
		if bitval[bitj as usize] == 0i32 {
			bitval[bitj as usize] = 8i32
		}
		/* first check whether this one does not match after all */
		if !(bitval[bitz as usize] == 3i32 || bitval[bitC as usize] == 1i32) && !(0 != bitcnt[bitI as usize] && (bitval[bitI as usize] == 0i32 || bitval[bitI as usize] == 0xffi32)) {
			/* bitI and bitC get copied to biti and bitc */
			if 0 != bitcnt[bitI as usize] {
				bitval[biti as usize] = bitval[bitI as usize];
				bitpos[biti as usize] = bitpos[bitI as usize]
			}
			if 0 != bitcnt[bitC as usize] {
				bitval[bitc as usize] = bitval[bitC as usize]
			}
			pos = 0i32;
			while 0 != *opcstr.offset(pos as isize) && !isspace(*opcstr.offset(pos as isize)) {
				if *opcstr.offset(pos as isize) == b'.' {
					pos += 1;
					sz = match *opcstr.offset(pos as isize) {
						b'B' => sz_byte,
						b'W' => sz_word,
						b'L' => sz_long,
						b'z' => {
							match bitval[bitz as usize] {
								0 => sz_byte,
								1 => sz_word,
								2 => sz_long,
								_ => unreachable!(),
							}
						}
						_ => unreachable!()
					}
				} else {
					//0 0 79
					//1 1 82
					//2 2 83
					//3 3 82
					//0 0 79
					//1 1 82
					//2 2 83
					//3 3 82
					//0 0 67
					//1 1 72
					//2 2 75
					//3 3 50
					//0 0 67
					//1 1 72
					//2 2 75
					//3 3 50
//						dbg!((mnp, pos, *opcstr.offset(pos as isize)));

					mnemonic[mnp as usize] = *opcstr.offset(pos as isize);
					if mnemonic[mnp as usize] == b'f'
					{
						find = -1i32;
						mnemonic[mnp as usize] = match bitval[bitf as usize] {
							0 => b'R',
							1 => b'L',
							_ => panic!(),
						}
					}
					mnp += 1
				}
				pos += 1
			}
			mnemonic[mnp as usize] = 0;
			/* now, we have read the mnemonic and the size */
			while 0 != *opcstr.offset(pos as isize) && isspace(*opcstr.offset(pos as isize)) {
				pos += 1
			}
			/* A goto a day keeps the D******a away. */
			if *opcstr.offset(pos as isize) as cty::c_int == 0i32 {
				current_block = 11251136543644381707;
			} else {
				/* parse the source address */
				usesrc = 1i32;
				let fresh1 = pos;
				pos = pos + 1;
				match *opcstr.offset(fresh1 as isize) {
					b'D' => {
						srcmode = Dreg;
						let fresh2 = pos;
						pos = pos + 1;
						match *opcstr.offset(fresh2 as isize) as
							cty::c_int {
							114 => {
								srcreg =
									bitval[bitr as cty::c_int as usize];
								srcgather = 1i32;
								srcpos =
									bitpos[bitr as cty::c_int as usize]
							}
							82 => {
								srcreg =
									bitval[bitR as cty::c_int as usize];
								srcgather = 1i32;
								srcpos =
									bitpos[bitR as cty::c_int as usize]
							}
							_ => { panic!(); }
						}
						current_block = 17478354070635831526;
					}
					b'A' => {
						srcmode = Areg;
						let fresh3 = pos;
						pos = pos + 1;
						match *opcstr.offset(fresh3 as isize) as
							cty::c_int {
							114 => {
								srcreg =
									bitval[bitr as cty::c_int as usize];
								srcgather = 1i32;
								srcpos =
									bitpos[bitr as cty::c_int as usize]
							}
							82 => {
								srcreg =
									bitval[bitR as cty::c_int as usize];
								srcgather = 1i32;
								srcpos =
									bitpos[bitR as cty::c_int as usize]
							}
							_ => { panic!(); }
						}
						match *opcstr.offset(pos as isize) as cty::c_int
							{
								112 => {
									srcmode = Apdi;
									pos += 1
								}
								80 => {
									srcmode = Aipi;
									pos += 1
								}
								_ => {}
							}
						current_block = 17478354070635831526;
					}
					b'L' => {
						srcmode = absl;
						current_block = 17478354070635831526;
					}
					b'#' => {
						let fresh4 = pos;
						pos = pos + 1;
						match *opcstr.offset(fresh4 as isize) as
							cty::c_int {
							122 => { srcmode = imm }
							48 => { srcmode = imm0 }
							49 => { srcmode = imm1 }
							50 => { srcmode = imm2 }
							105 => {
								srcmode = immi;
								srcreg =
									bitval[biti as cty::c_int as usize]
										as int8_t as int32_t;
								if 0i32 < 4i32 {
									/* Used for branch instructions */
									srctype = 1i32;
									srcgather = 1i32;
									srcpos =
										bitpos[biti as cty::c_int as
											usize]
								}
							}
							106 => {
								srcmode = immi;
								srcreg =
									bitval[bitj as cty::c_int as usize];
								if 0i32 < 3i32 {
									/* 1..8 for ADDQ/SUBQ and rotshi insns */
									srcgather = 1i32;
									srctype = 3i32;
									srcpos =
										bitpos[bitj as cty::c_int as
											usize]
								}
							}
							74 => {
								srcmode = immi;
								srcreg =
									bitval[bitJ as cty::c_int as usize];
								if 0i32 < 5i32 {
									/* 0..15 */
									srcgather = 1i32;
									srctype = 2i32;
									srcpos =
										bitpos[bitJ as cty::c_int as
											usize]
								}
							}
							107 => {
								srcmode = immi;
								srcreg =
									bitval[bitk as cty::c_int as usize];
								if 0i32 < 3i32 {
									srcgather = 1i32;
									srctype = 4i32;
									srcpos =
										bitpos[bitk as cty::c_int as
											usize]
								}
							}
							75 => {
								srcmode = immi;
								srcreg =
									bitval[bitK as cty::c_int as usize];
								if 0i32 < 5i32 {
									/* 0..15 */
									srcgather = 1i32;
									srctype = 5i32;
									srcpos =
										bitpos[bitK as cty::c_int as
											usize]
								}
							}
							112 => {
								srcmode = immi;
								srcreg =
									bitval[bitK as cty::c_int as usize];
								if 0i32 < 5i32 {
									/* 0..3 */
									srcgather = 1i32;
									srctype = 7i32;
									srcpos =
										bitpos[bitp as cty::c_int as
											usize]
								}
							}
							_ => { panic!(); }
						}
						current_block = 17478354070635831526;
					}
					b'd' => {
						srcreg = bitval[bitD as cty::c_int as usize];
						srcmode =
							mode_from_mr(bitval[bitd as cty::c_int as
								usize],
							             bitval[bitD as cty::c_int as
								             usize]);
						if srcmode as cty::c_uint ==
							am_illg as cty::c_int as cty::c_uint {
							current_block = 7659304154607701039;
						} else {
							if 0i32 < 2i32 &&
								(srcmode as cty::c_uint ==
									Areg as cty::c_int as
										cty::c_uint ||
									srcmode as cty::c_uint ==
										Dreg as cty::c_int as
											cty::c_uint ||
									srcmode as cty::c_uint ==
										Aind as cty::c_int as
											cty::c_uint ||
									srcmode as cty::c_uint ==
										Ad16 as cty::c_int as
											cty::c_uint ||
									srcmode as cty::c_uint ==
										Ad8r as cty::c_int as
											cty::c_uint ||
									srcmode as cty::c_uint ==
										Aipi as cty::c_int as
											cty::c_uint ||
									srcmode as cty::c_uint ==
										Apdi as cty::c_int as
											cty::c_uint) {
								srcgather = 1i32;
								srcpos =
									bitpos[bitD as cty::c_int as usize]
							}
							if *opcstr.offset(pos as isize) as cty::c_int
								== '[' as i32 {
								pos += 1;
								if *opcstr.offset(pos as isize) as
									cty::c_int == '!' as i32 {
									loop
										/* exclusion */
										{
											pos += 1;
											if mode_from_str(opcstr.offset(pos
												as
												isize))
												as cty::c_uint ==
												srcmode as cty::c_uint {
												current_block =
													7659304154607701039;
												break;
											}
											pos += 4i32;
											if !(*opcstr.offset(pos as isize)
												as cty::c_int ==
												',' as i32) {
												current_block =
													16718638665978159145;
												break;
											}
										}
									match current_block {
										7659304154607701039 => {}
										_ => {
											pos += 1;
											current_block =
												17664728594743454682;
										}
									}
								} else if *opcstr.offset((pos + 4i32) as
									isize) as
									cty::c_int == '-' as i32 {
									/* replacement */
									if mode_from_str(opcstr.offset(pos as
										isize))
										as cty::c_uint ==
										srcmode as cty::c_uint {
										srcmode =
											mode_from_str(opcstr.offset(pos
												as
												isize).offset(5isize));
										pos += 10i32;
										current_block =
											17664728594743454682;
									} else {
										current_block =
											7659304154607701039;
									}
								} else {
									loop
										/* normal */
										{
											if !(mode_from_str(opcstr.offset(pos
												as
												isize))
												as cty::c_uint !=
												srcmode as cty::c_uint)
											{
												current_block =
													10818670573987138623;
												break;
											}
											pos += 4i32;
											if *opcstr.offset(pos as isize) as
												cty::c_int == ']' as i32 {
												current_block =
													7659304154607701039;
												break;
											}
											pos += 1
										}
									match current_block {
										7659304154607701039 => {}
										_ => {
											while *opcstr.offset(pos as
												isize)
												as cty::c_int !=
												']' as i32 {
												pos += 1
											}
											pos += 1;
											current_block =
												17478354070635831526;
										}
									}
								}
							} else {
								current_block = 17664728594743454682;
							}
							match current_block {
								17478354070635831526 => {}
								7659304154607701039 => {}
								_ =>
								/* Some addressing modes are invalid as destination */
									{
										if srcmode as cty::c_uint ==
											imm as cty::c_int as
												cty::c_uint ||
											srcmode as cty::c_uint ==
												PC16 as cty::c_int as
													cty::c_uint ||
											srcmode as cty::c_uint ==
												PC8r as cty::c_int as
													cty::c_uint {
											current_block =
												7659304154607701039;
										} else {
											current_block =
												17478354070635831526;
										}
									}
							}
						}
					}
					b's' => {
						srcreg = bitval[bitS as cty::c_int as usize];
						srcmode =
							mode_from_mr(bitval[bits as cty::c_int as
								usize],
							             bitval[bitS as cty::c_int as
								             usize]);
						if srcmode as cty::c_uint ==
							am_illg as cty::c_int as cty::c_uint {
							current_block = 7659304154607701039;
						} else {
							if 0i32 < 2i32 &&
								(srcmode as cty::c_uint ==
									Areg as cty::c_int as
										cty::c_uint ||
									srcmode as cty::c_uint ==
										Dreg as cty::c_int as
											cty::c_uint ||
									srcmode as cty::c_uint ==
										Aind as cty::c_int as
											cty::c_uint ||
									srcmode as cty::c_uint ==
										Ad16 as cty::c_int as
											cty::c_uint ||
									srcmode as cty::c_uint ==
										Ad8r as cty::c_int as
											cty::c_uint ||
									srcmode as cty::c_uint ==
										Aipi as cty::c_int as
											cty::c_uint ||
									srcmode as cty::c_uint ==
										Apdi as cty::c_int as
											cty::c_uint) {
								srcgather = 1i32;
								srcpos =
									bitpos[bitS as cty::c_int as usize]
							}
							if *opcstr.offset(pos as isize) as cty::c_int
								== '[' as i32 {
								pos += 1;
								if *opcstr.offset(pos as isize) as
									cty::c_int == '!' as i32 {
									loop
										/* exclusion */
										{
											pos += 1;
											if mode_from_str(opcstr.offset(pos
												as
												isize))
												as cty::c_uint ==
												srcmode as cty::c_uint {
												current_block =
													7659304154607701039;
												break;
											}
											pos += 4i32;
											if !(*opcstr.offset(pos as isize)
												as cty::c_int ==
												',' as i32) {
												current_block =
													15115612870595956036;
												break;
											}
										}
									match current_block {
										7659304154607701039 => {}
										_ => {
											pos += 1;
											current_block =
												17478354070635831526;
										}
									}
								} else if *opcstr.offset((pos + 4i32) as
									isize) as
									cty::c_int == '-' as i32 {
									/* replacement */
									if mode_from_str(opcstr.offset(pos as
										isize))
										as cty::c_uint ==
										srcmode as cty::c_uint {
										srcmode =
											mode_from_str(opcstr.offset(pos
												as
												isize).offset(5isize));
										pos += 10i32;
										current_block =
											17478354070635831526;
									} else {
										current_block =
											7659304154607701039;
									}
								} else {
									loop
										/* normal */
										{
											if !(mode_from_str(opcstr.offset(pos
												as
												isize))
												as cty::c_uint !=
												srcmode as cty::c_uint)
											{
												current_block =
													11940120049376251063;
												break;
											}
											pos += 4i32;
											if *opcstr.offset(pos as isize) as
												cty::c_int == ']' as i32 {
												current_block =
													7659304154607701039;
												break;
											}
											pos += 1
										}
									match current_block {
										7659304154607701039 => {}
										_ => {
											while *opcstr.offset(pos as
												isize)
												as cty::c_int !=
												']' as i32 {
												pos += 1
											}
											pos += 1;
											current_block =
												17478354070635831526;
										}
									}
								}
							} else {
								current_block = 17478354070635831526;
							}
						}
					}
					_ => { panic!(); }
				}
				match current_block {
					7659304154607701039 => {}
					_ => {
						/* safety check - might have changed */
						if srcmode as cty::c_uint !=
							Areg as cty::c_int as cty::c_uint &&
							srcmode as cty::c_uint !=
								Dreg as cty::c_int as cty::c_uint &&
							srcmode as cty::c_uint !=
								Aind as cty::c_int as cty::c_uint &&
							srcmode as cty::c_uint !=
								Ad16 as cty::c_int as cty::c_uint &&
							srcmode as cty::c_uint !=
								Ad8r as cty::c_int as cty::c_uint &&
							srcmode as cty::c_uint !=
								Aipi as cty::c_int as cty::c_uint &&
							srcmode as cty::c_uint !=
								Apdi as cty::c_int as cty::c_uint &&
							srcmode as cty::c_uint !=
								immi as cty::c_int as cty::c_uint {
							srcgather = 0i32
						}
						if srcmode as cty::c_uint ==
							Areg as cty::c_int as cty::c_uint &&
							sz as cty::c_uint ==
								sz_byte as cty::c_int as cty::c_uint
						{
							current_block = 7659304154607701039;
						} else if *opcstr.offset(pos as isize) as
							cty::c_int != ',' as i32 {
							current_block = 11251136543644381707;
						} else {
							pos += 1;
							/* parse the destination address */
							usedst = 1i32;
							let fresh5 = pos;
							pos = pos + 1;
							match *opcstr.offset(fresh5 as isize) as
								cty::c_int {
								68 => {
									destmode = Dreg;
									let fresh6 = pos;
									pos = pos + 1;
									match *opcstr.offset(fresh6 as isize)
										as cty::c_int {
										114 => {
											destreg =
												bitval[bitr as cty::c_int
													as usize];
											dstgather = 1i32;
											dstpos =
												bitpos[bitr as cty::c_int
													as usize]
										}
										82 => {
											destreg =
												bitval[bitR as cty::c_int
													as usize];
											dstgather = 1i32;
											dstpos =
												bitpos[bitR as cty::c_int
													as usize]
										}
										_ => { panic!(); }
									}
									if dstpos < 0i32 || dstpos >= 32i32 {
										panic!();
									}
									current_block = 10914975420504029141;
								}
								65 => {
									destmode = Areg;
									let fresh7 = pos;
									pos = pos + 1;
									match *opcstr.offset(fresh7 as isize)
										as cty::c_int {
										114 => {
											destreg =
												bitval[bitr as cty::c_int
													as usize];
											dstgather = 1i32;
											dstpos =
												bitpos[bitr as cty::c_int
													as usize]
										}
										82 => {
											destreg =
												bitval[bitR as cty::c_int
													as usize];
											dstgather = 1i32;
											dstpos =
												bitpos[bitR as cty::c_int
													as usize]
										}
										120 => {
											destreg = 0i32;
											dstgather = 0i32;
											dstpos = 0i32
										}
										_ => { panic!(); }
									}
									if dstpos < 0i32 || dstpos >= 32i32 {
										panic!();
									}
									match *opcstr.offset(pos as isize) as
										cty::c_int {
										112 => {
											destmode = Apdi;
											pos += 1
										}
										80 => {
											destmode = Aipi;
											pos += 1
										}
										_ => {}
									}
									current_block = 10914975420504029141;
								}
								76 => {
									destmode = absl;
									current_block = 10914975420504029141;
								}
								35 => {
									let fresh8 = pos;
									pos = pos + 1;
									match *opcstr.offset(fresh8 as isize)
										as cty::c_int {
										122 => { destmode = imm }
										48 => { destmode = imm0 }
										49 => { destmode = imm1 }
										50 => { destmode = imm2 }
										105 => {
											destmode = immi;
											destreg =
												bitval[biti as cty::c_int
													as usize] as
													int8_t as int32_t
										}
										106 => {
											destmode = immi;
											destreg =
												bitval[bitj as cty::c_int
													as usize]
										}
										74 => {
											destmode = immi;
											destreg =
												bitval[bitJ as cty::c_int
													as usize]
										}
										107 => {
											destmode = immi;
											destreg =
												bitval[bitk as cty::c_int
													as usize]
										}
										75 => {
											destmode = immi;
											destreg =
												bitval[bitK as cty::c_int
													as usize]
										}
										_ => { panic!(); }
									}
									current_block = 10914975420504029141;
								}
								100 => {
									destreg =
										bitval[bitD as cty::c_int as
											usize];
									destmode =
										mode_from_mr(bitval[bitd as
											cty::c_int
											as usize],
										             bitval[bitD as
											             cty::c_int
											             as
											             usize]);
									if destmode as cty::c_uint ==
										am_illg as cty::c_int as
											cty::c_uint {
										current_block =
											7659304154607701039;
									} else {
										if 0i32 < 1i32 &&
											(destmode as cty::c_uint
												==
												Areg as cty::c_int as
													cty::c_uint ||
												destmode as
													cty::c_uint ==
													Dreg as
														cty::c_int as
														cty::c_uint
												||
												destmode as
													cty::c_uint ==
													Aind as
														cty::c_int as
														cty::c_uint
												||
												destmode as
													cty::c_uint ==
													Ad16 as
														cty::c_int as
														cty::c_uint
												||
												destmode as
													cty::c_uint ==
													Ad8r as
														cty::c_int as
														cty::c_uint
												||
												destmode as
													cty::c_uint ==
													Aipi as
														cty::c_int as
														cty::c_uint
												||
												destmode as
													cty::c_uint ==
													Apdi as
														cty::c_int as
														cty::c_uint)
										{
											dstgather = 1i32;
											dstpos =
												bitpos[bitD as cty::c_int
													as usize]
										}
										if *opcstr.offset(pos as isize) as
											cty::c_int == '[' as i32 {
											pos += 1;
											if *opcstr.offset(pos as
												isize)
												as cty::c_int ==
												'!' as i32 {
												loop
													/* exclusion */
													{
														pos += 1;
														if mode_from_str(opcstr.offset(pos
															as
															isize))
															as cty::c_uint
															==
															destmode as
																cty::c_uint
														{
															current_block =
																7659304154607701039;
															break;
														}
														pos += 4i32;
														if !(*opcstr.offset(pos
															as
															isize)
															as
															cty::c_int
															==
															',' as i32) {
															current_block =
																11579725430234891816;
															break;
														}
													}
												match current_block {
													7659304154607701039 =>
														{}
													_ => {
														pos += 1;
														current_block =
															18078681159150601890;
													}
												}
											} else if *opcstr.offset((pos
												+
												4i32)
												as
												isize)
												as cty::c_int
												== '-' as i32 {
												/* replacement */
												if mode_from_str(opcstr.offset(pos
													as
													isize))
													as cty::c_uint ==
													destmode as
														cty::c_uint {
													destmode =
														mode_from_str(opcstr.offset(pos
															as
															isize).offset(5isize));
													pos += 10i32;
													current_block =
														18078681159150601890;
												} else {
													current_block =
														7659304154607701039;
												}
											} else {
												loop
													/* normal */
													{
														if !(mode_from_str(opcstr.offset(pos
															as
															isize))
															as
															cty::c_uint
															!=
															destmode as
																cty::c_uint)
														{
															current_block =
																13837311639792312879;
															break;
														}
														pos += 4i32;
														if *opcstr.offset(pos
															as
															isize)
															as cty::c_int
															== ']' as i32 {
															current_block =
																7659304154607701039;
															break;
														}
														pos += 1
													}
												match current_block {
													7659304154607701039 =>
														{}
													_ => {
														while *opcstr.offset(pos
															as
															isize)
															as
															cty::c_int
															!=
															']' as
																i32
															{
																pos += 1
															}
														pos += 1;
														current_block =
															10914975420504029141;
													}
												}
											}
										} else {
											current_block =
												18078681159150601890;
										}
										match current_block {
											10914975420504029141 => {}
											7659304154607701039 => {}
											_ =>
											/* Some addressing modes are invalid as destination */
												{
													if destmode as
														cty::c_uint ==
														imm as cty::c_int
															as cty::c_uint
														||
														destmode as
															cty::c_uint ==
															PC16 as
																cty::c_int
																as
																cty::c_uint
														||
														destmode as
															cty::c_uint ==
															PC8r as
																cty::c_int
																as
																cty::c_uint
													{
														current_block =
															7659304154607701039;
													} else {
														current_block =
															10914975420504029141;
													}
												}
										}
									}
								}
								115 => {
									destreg =
										bitval[bitS as cty::c_int as
											usize];
									destmode =
										mode_from_mr(bitval[bits as
											cty::c_int
											as usize],
										             bitval[bitS as
											             cty::c_int
											             as
											             usize]);
									if destmode as cty::c_uint ==
										am_illg as cty::c_int as
											cty::c_uint {
										current_block =
											7659304154607701039;
									} else {
										if 0i32 < 1i32 &&
											(destmode as cty::c_uint
												==
												Areg as cty::c_int as
													cty::c_uint ||
												destmode as
													cty::c_uint ==
													Dreg as
														cty::c_int as
														cty::c_uint
												||
												destmode as
													cty::c_uint ==
													Aind as
														cty::c_int as
														cty::c_uint
												||
												destmode as
													cty::c_uint ==
													Ad16 as
														cty::c_int as
														cty::c_uint
												||
												destmode as
													cty::c_uint ==
													Ad8r as
														cty::c_int as
														cty::c_uint
												||
												destmode as
													cty::c_uint ==
													Aipi as
														cty::c_int as
														cty::c_uint
												||
												destmode as
													cty::c_uint ==
													Apdi as
														cty::c_int as
														cty::c_uint)
										{
											dstgather = 1i32;
											dstpos =
												bitpos[bitS as cty::c_int
													as usize]
										}
										if *opcstr.offset(pos as isize) as
											cty::c_int == '[' as i32 {
											pos += 1;
											if *opcstr.offset(pos as
												isize)
												as cty::c_int ==
												'!' as i32 {
												loop
													/* exclusion */
													{
														pos += 1;
														if mode_from_str(opcstr.offset(pos
															as
															isize))
															as cty::c_uint
															==
															destmode as
																cty::c_uint
														{
															current_block =
																7659304154607701039;
															break;
														}
														pos += 4i32;
														if !(*opcstr.offset(pos
															as
															isize)
															as
															cty::c_int
															==
															',' as i32) {
															current_block =
																10447688102407413348;
															break;
														}
													}
												match current_block {
													7659304154607701039 =>
														{}
													_ => {
														pos += 1;
														current_block =
															10914975420504029141;
													}
												}
											} else if *opcstr.offset((pos
												+
												4i32)
												as
												isize)
												as cty::c_int
												== '-' as i32 {
												/* replacement */
												if mode_from_str(opcstr.offset(pos
													as
													isize))
													as cty::c_uint ==
													destmode as
														cty::c_uint {
													destmode =
														mode_from_str(opcstr.offset(pos
															as
															isize).offset(5isize));
													pos += 10i32;
													current_block =
														10914975420504029141;
												} else {
													current_block =
														7659304154607701039;
												}
											} else {
												loop
													/* normal */
													{
														if !(mode_from_str(opcstr.offset(pos
															as
															isize))
															as
															cty::c_uint
															!=
															destmode as
																cty::c_uint)
														{
															current_block =
																9032728402242067558;
															break;
														}
														pos += 4i32;
														if *opcstr.offset(pos
															as
															isize)
															as cty::c_int
															== ']' as i32 {
															current_block =
																7659304154607701039;
															break;
														}
														pos += 1
													}
												match current_block {
													7659304154607701039 =>
														{}
													_ => {
														while *opcstr.offset(pos
															as
															isize)
															as
															cty::c_int
															!=
															']' as
																i32
															{
																pos += 1
															}
														pos += 1;
														current_block =
															10914975420504029141;
													}
												}
											}
										} else {
											current_block =
												10914975420504029141;
										}
									}
								}
								_ => { panic!(); }
							}
							match current_block {
								7659304154607701039 => {}
								_ => {
									/* safety check - might have changed */
									if destmode as cty::c_uint !=
										Areg as cty::c_int as
											cty::c_uint &&
										destmode as cty::c_uint !=
											Dreg as cty::c_int as
												cty::c_uint &&
										destmode as cty::c_uint !=
											Aind as cty::c_int as
												cty::c_uint &&
										destmode as cty::c_uint !=
											Ad16 as cty::c_int as
												cty::c_uint &&
										destmode as cty::c_uint !=
											Ad8r as cty::c_int as
												cty::c_uint &&
										destmode as cty::c_uint !=
											Aipi as cty::c_int as
												cty::c_uint &&
										destmode as cty::c_uint !=
											Apdi as cty::c_int as
												cty::c_uint {
										dstgather = 0i32
									}
									if destmode as cty::c_uint ==
										Areg as cty::c_int as
											cty::c_uint &&
										sz as cty::c_uint ==
											sz_byte as cty::c_int as
												cty::c_uint {
										current_block =
											7659304154607701039;
									} else {
										current_block =
											11251136543644381707;
									}
								}
							}
						}
					}
				}
			}
			match current_block {
				7659304154607701039 => {}
				_ => {
					/* now, we have a match */
					if (*table68k.offset(opc as isize)).mnemo() as
						cty::c_int != i_ILLG as cty::c_int {
						printf(
							b"UAE: Double match: %x: %s\n\x00" as
								*const u8 as *const cty::c_char,
							opc as cty::c_int, opcstr);
					}
					if find == -1i32 {
						find = 0i32;
						loop {
							if strcmp(mnemonic.as_mut_ptr(),
							          lookuptab[find as usize].name) ==
								0i32 {
								let ref mut fresh9 =
									*table68k.offset(opc as isize);
								(*fresh9).set_mnemo(lookuptab[find as
									usize].mnemo
									as cty::c_uint);
								break;
							} else {
								if strlen(lookuptab[find as usize].name)
									== 0i32 as cty::c_ulong {
									panic!();
								}
								find += 1
							}
						}
					} else {
						let ref mut fresh10 =
							*table68k.offset(opc as isize);
						(*fresh10).set_mnemo(lookuptab[find as
							usize].mnemo as
							cty::c_uint)
					}
					let ref mut fresh11 = *table68k.offset(opc as isize);
					(*fresh11).set_cc(bitval[bitc as cty::c_int as usize]
						as cty::c_uint);
					if (*table68k.offset(opc as isize)).mnemo() as
						cty::c_int == i_BTST as cty::c_int ||
						(*table68k.offset(opc as isize)).mnemo() as
							cty::c_int == i_BSET as cty::c_int ||
						(*table68k.offset(opc as isize)).mnemo() as
							cty::c_int == i_BCLR as cty::c_int ||
						(*table68k.offset(opc as isize)).mnemo() as
							cty::c_int == i_BCHG as cty::c_int {
						sz =
							(if destmode as cty::c_uint ==
								Dreg as cty::c_int as cty::c_uint {
								sz_long as cty::c_int
							} else { sz_byte as cty::c_int }) as
								wordsizes
					}
					let ref mut fresh12 = *table68k.offset(opc as isize);
					(*fresh12).set_size(sz as cty::c_uint);
					(*table68k.offset(opc as isize)).sreg =
						srcreg as cty::c_uchar;
					(*table68k.offset(opc as isize)).dreg =
						destreg as cty::c_uchar;
					let ref mut fresh13 = *table68k.offset(opc as isize);
					(*fresh13).set_smode(srcmode as cty::c_uint);
					let ref mut fresh14 = *table68k.offset(opc as isize);
					(*fresh14).set_dmode(destmode as cty::c_uint);
					(*table68k.offset(opc as isize)).spos =
						(if 0 != srcgather { srcpos } else { -1i32 }) as
							cty::c_schar;
					(*table68k.offset(opc as isize)).dpos =
						(if 0 != dstgather { dstpos } else { -1i32 }) as
							cty::c_schar;
					let ref mut fresh15 = *table68k.offset(opc as isize);
					(*fresh15).set_suse(usesrc as cty::c_uint);
					let ref mut fresh16 = *table68k.offset(opc as isize);
					(*fresh16).set_duse(usedst as cty::c_uint);
					let ref mut fresh17 = *table68k.offset(opc as isize);
					(*fresh17).set_stype(srctype as cty::c_uint);
					let ref mut fresh18 = *table68k.offset(opc as isize);
					(*fresh18).set_plev(id.plevel as cty::c_uint);
					let ref mut fresh19 = *table68k.offset(opc as isize);
					(*fresh19).set_clev(id.cpulevel as cty::c_uint);
					let ref mut fresh20 = *table68k.offset(opc as isize);
					(*fresh20).set_flagdead(flagdead);
					let ref mut fresh21 = *table68k.offset(opc as isize);
					(*fresh21).set_flaglive(flaglive);
					let ref mut fresh22 = *table68k.offset(opc as isize);
					(*fresh22).set_isjmp(isjmp as cty::c_uint)
				}
			}
		}

		/* FOO! */
		variants += 1
	};
}

#[no_mangle]
pub unsafe extern "C" fn read_table68k() {
	let mut i: cty::c_int = 0;
	if table68k.is_null() {
		table68k =
			xmalloc((65536i32 as
				cty::c_ulong).wrapping_mul(::core::mem::size_of::<instr>()
				as cty::c_ulong)) as
				*mut instr
	}
	i = 0i32;
	while i < 65536i32 {
		let ref mut fresh23 = *table68k.offset(i as isize);
		(*fresh23).set_mnemo(i_ILLG as cty::c_int as cty::c_uint);
		(*table68k.offset(i as isize)).handler = -1i32 as cty::c_long;
		i += 1
	}
	i = 0i32;
	while i < n_defs68k {
		build_insn(i);
		i += 1
	};
}

static mut mismatch: cty::c_int = 0;

unsafe extern "C" fn handle_merges(mut opcode: cty::c_long) {
	let mut smsk: uint16_t = 0i32 as uint16_t;
	let mut dmsk: uint16_t = 0;
	let mut sbitdst: cty::c_int = 0i32;
	let mut dstend: cty::c_int = 0;
	let mut srcreg: cty::c_int = 0;
	let mut dstreg: cty::c_int = 0;
	if (*table68k.offset(opcode as isize)).spos as cty::c_int == -1i32 {
		sbitdst = 1i32;
		smsk = 0i32 as uint16_t
	} else {
		match (*table68k.offset(opcode as isize)).stype() as cty::c_int {
			0 => {
				smsk = 7i32 as uint16_t;
				sbitdst = 8i32
			}
			1 => {
				smsk = 255i32 as uint16_t;
				sbitdst = 256i32
			}
			2 => {
				smsk = 15i32 as uint16_t;
				sbitdst = 16i32
			}
			3 => {
				smsk = 7i32 as uint16_t;
				sbitdst = 8i32
			}
			4 => {
				smsk = 7i32 as uint16_t;
				sbitdst = 8i32
			}
			5 => {
				smsk = 63i32 as uint16_t;
				sbitdst = 64i32
			}
			7 => {
				smsk = 3i32 as uint16_t;
				sbitdst = 4i32
			}
			_ => {
				smsk = 0i32 as uint16_t;
				sbitdst = 0i32;
				panic!();
			}
		}
		smsk =
			((smsk as cty::c_int) <<
				(*table68k.offset(opcode as isize)).spos as cty::c_int) as
				uint16_t
	}
	if (*table68k.offset(opcode as isize)).dpos as cty::c_int == -1i32 {
		dstend = 1i32;
		dmsk = 0i32 as uint16_t
	} else {
		dmsk =
			(7i32 << (*table68k.offset(opcode as isize)).dpos as cty::c_int)
				as uint16_t;
		dstend = 8i32
	}
	srcreg = 0i32;
	while srcreg < sbitdst {
		dstreg = 0i32;
		while dstreg < dstend {
			let mut code: uint16_t = opcode as uint16_t;
			code =
				(code as cty::c_int & !(smsk as cty::c_int) |
					srcreg <<
						(*table68k.offset(opcode as isize)).spos as
							cty::c_int) as uint16_t;
			code =
				(code as cty::c_int & !(dmsk as cty::c_int) |
					dstreg <<
						(*table68k.offset(opcode as isize)).dpos as
							cty::c_int) as uint16_t;
			/* Check whether this is in fact the same instruction.
			 * The instructions should never differ, except for the
			 * Bcc.(BW) case. */
			if (*table68k.offset(code as isize)).mnemo() as cty::c_int !=
				(*table68k.offset(opcode as isize)).mnemo() as cty::c_int
				||
				(*table68k.offset(code as isize)).size() as cty::c_int !=
					(*table68k.offset(opcode as isize)).size() as
						cty::c_int ||
				(*table68k.offset(code as isize)).suse() as cty::c_int !=
					(*table68k.offset(opcode as isize)).suse() as
						cty::c_int ||
				(*table68k.offset(code as isize)).duse() as cty::c_int !=
					(*table68k.offset(opcode as isize)).duse() as
						cty::c_int {
				mismatch += 1
			} else if 0 !=
				(*table68k.offset(opcode as isize)).suse() as
					cty::c_int &&
				((*table68k.offset(opcode as isize)).spos as
					cty::c_int !=
					(*table68k.offset(code as isize)).spos as
						cty::c_int ||
					(*table68k.offset(opcode as isize)).smode() as
						cty::c_int !=
						(*table68k.offset(code as isize)).smode()
							as cty::c_int ||
					(*table68k.offset(opcode as isize)).stype() as
						cty::c_int !=
						(*table68k.offset(code as isize)).stype()
							as cty::c_int) {
				mismatch += 1
			} else if 0 !=
				(*table68k.offset(opcode as isize)).duse() as
					cty::c_int &&
				((*table68k.offset(opcode as isize)).dpos as
					cty::c_int !=
					(*table68k.offset(code as isize)).dpos as
						cty::c_int ||
					(*table68k.offset(opcode as isize)).dmode() as
						cty::c_int !=
						(*table68k.offset(code as isize)).dmode()
							as cty::c_int) {
				mismatch += 1
			} else if code as cty::c_long != opcode {
				(*table68k.offset(code as isize)).handler = opcode
			}
			dstreg += 1
		}
		srcreg += 1
	};
}

#[no_mangle]
pub unsafe extern "C" fn do_merges() {
	let mut opcode: cty::c_long = 0;
	let mut nr: cty::c_int = 0i32;
	mismatch = 0i32;
	opcode = 0i32 as cty::c_long;
	while opcode < 65536i32 as cty::c_long {
		if !((*table68k.offset(opcode as isize)).handler !=
			-1i32 as cty::c_long ||
			(*table68k.offset(opcode as isize)).mnemo() as cty::c_int ==
				i_ILLG as cty::c_int) {
			nr += 1;
			handle_merges(opcode);
		}
		opcode += 1
	}
	nr_cpuop_funcs = nr;
}

#[no_mangle]
pub unsafe extern "C" fn get_no_mismatches() -> cty::c_int {
	return mismatch;
}
