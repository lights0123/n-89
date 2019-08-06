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
    fn abort() -> !;
}
pub type __uint16_t = cty::c_ushort;
pub type __uint32_t = cty::c_uint;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = cty::c_ulong;
/*
 * UAE - The Un*x Amiga Emulator
 *
 * MC68881 emulation
 *
 * Copyright 1996 Herman ten Brugge
 * stubbed out for TiEmu - Copyright (C) 2005 Kevin Kofler
 */
#[no_mangle]
pub unsafe extern "C" fn fdbcc_opp(mut opcode: uint32_t,
                                   mut extra: uint16_t) {
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn fscc_opp(mut opcode: uint32_t, mut extra: uint16_t) {
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn ftrapcc_opp(mut opcode: uint32_t,
                                     mut oldpc: uintptr_t) {
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn fbcc_opp(mut opcode: uint32_t, mut pc: uintptr_t,
                                  mut extra: uint32_t) {
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn fsave_opp(mut opcode: uint32_t) { abort(); }
#[no_mangle]
pub unsafe extern "C" fn frestore_opp(mut opcode: uint32_t) { abort(); }
#[no_mangle]
pub unsafe extern "C" fn fpp_opp(mut opcode: uint32_t, mut extra: uint16_t) {
    abort();
}
