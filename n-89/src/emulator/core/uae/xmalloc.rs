#![allow(dead_code,
mutable_transmutes,
non_camel_case_types,
non_snake_case,
non_upper_case_globals,
unused_assignments,
unused_mut)]

use crate::libc::*;

extern "C" {
	pub type _IO_wide_data;
	pub type _IO_codecvt;
	pub type _IO_marker;
	#[no_mangle]
	static mut stderr: *mut FILE;
	#[no_mangle]
	fn fprintf(_: *mut FILE, _: *const cty::c_char, _: ...) -> cty::c_int;
	#[no_mangle]
	fn malloc(_: cty::c_ulong) -> *mut cty::c_void;
	#[no_mangle]
	fn calloc(_: cty::c_ulong, _: cty::c_ulong) -> *mut cty::c_void;
	#[no_mangle]
	fn abort() -> !;
}

pub type size_t = cty::c_ulong;
pub type __off_t = cty::c_long;
pub type __off64_t = cty::c_long;

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
/*
 * UAE - The Un*x Amiga Emulator
 *
 * Various stuff missing in some OSes.
 *
 * Copyright 1997 Bernd Schmidt
 */
#[no_mangle]
pub unsafe extern "C" fn xmalloc(mut n: size_t) -> *mut cty::c_void {
	let mut a: *mut cty::c_void = malloc(n);
	if a.is_null() {
		printf(
			b"virtual memory exhausted\n\x00" as *const u8 as
				*const cty::c_char);
		abort();
	}
	return a;
}

#[no_mangle]
pub unsafe extern "C" fn xcalloc(mut n: size_t, mut size: size_t)
                                 -> *mut cty::c_void {
	let mut a: *mut cty::c_void = calloc(n, size);
	if a.is_null() {
		printf(b"virtual memory exhausted\n\x00" as *const u8 as
			*const cty::c_char);
		abort();
	}
	return a;
}
