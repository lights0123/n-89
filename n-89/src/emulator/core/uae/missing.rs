#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]
use crate::libc::*;
extern "C" {
    // Make sure that the character types take exactly 1 byte.
    /* UCHAR_MAX != 0xFF */
    /* UCHAR_MAX != 0xFF */
    // This part coded by Kevin Kofler. Copyright (C) Kevin Kofler, 2005.
// This is hard to do right because too long numbers might cause a parse error.
    #[no_mangle]
    fn xmalloc(_: size_t) -> *mut cty::c_void;
    #[no_mangle]
    fn strcpy(_: *mut cty::c_char, _: *const cty::c_char)
     -> *mut cty::c_char;
    #[no_mangle]
    fn strlen(_: *const cty::c_char) -> cty::c_ulong;
}
pub type size_t = cty::c_ulong;
/* Hey EMACS -*- linux-c -*- */
/* $Id: missing.c 1693 2005-08-25 14:29:36Z roms $ */
/*
 * UAE - The Un*x Amiga Emulator
 *
 * Various stuff missing in some OSes.
 *
 * Copyright 1997 Bernd Schmidt
 */
#[no_mangle]
pub unsafe extern "C" fn my_strdup(mut s: *const cty::c_char)
 -> *mut cty::c_char {
    /* The casts to char * are there to shut up the compiler on HPUX */
    let mut x: *mut cty::c_char =
        xmalloc(strlen(s as
                           *mut cty::c_char).wrapping_add(1i32 as
                                                               cty::c_ulong))
            as *mut cty::c_char;
    strcpy(x, s as *mut cty::c_char);
    return x;
}
