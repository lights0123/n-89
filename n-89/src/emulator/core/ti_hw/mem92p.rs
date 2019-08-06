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
    static mut tihw: Ti68kHardware;
    #[no_mangle]
    static mut mem_get_byte_ptr: GETBYTE_FUNC;
    #[no_mangle]
    static mut mem_get_word_ptr: GETWORD_FUNC;
    #[no_mangle]
    static mut mem_get_long_ptr: GETLONG_FUNC;
    #[no_mangle]
    static mut mem_put_byte_ptr: PUTBYTE_FUNC;
    #[no_mangle]
    static mut mem_put_word_ptr: PUTWORD_FUNC;
    #[no_mangle]
    static mut mem_put_long_ptr: PUTLONG_FUNC;
    #[no_mangle]
    static mut mem_get_real_addr_ptr: REALADR_FUNC;
    #[no_mangle]
    static mut wsm: FLASH_WSM;
    #[no_mangle]
    fn FlashWriteByte(addr: uint32_t, v: uint8_t);
    #[no_mangle]
    fn FlashWriteWord(addr: uint32_t, v: uint16_t);
    #[no_mangle]
    fn FlashWriteLong(addr: uint32_t, v: uint32_t);
    #[no_mangle]
    fn io_get_byte(addr: uint32_t) -> uint8_t;
    #[no_mangle]
    fn io_get_word(addr: uint32_t) -> uint16_t;
    #[no_mangle]
    fn io_get_long(addr: uint32_t) -> uint32_t;
    #[no_mangle]
    fn io_put_long(addr: uint32_t, arg: uint32_t);
    #[no_mangle]
    fn io_put_word(addr: uint32_t, arg: uint16_t);
    #[no_mangle]
    fn io_put_byte(addr: uint32_t, arg: uint8_t);
    // ---
    #[no_mangle]
    fn io2_get_byte(addr: uint32_t) -> uint8_t;
    #[no_mangle]
    fn io2_get_word(addr: uint32_t) -> uint16_t;
    #[no_mangle]
    fn io2_get_long(addr: uint32_t) -> uint32_t;
    #[no_mangle]
    fn io2_put_long(addr: uint32_t, arg: uint32_t);
    #[no_mangle]
    fn io2_put_word(addr: uint32_t, arg: uint16_t);
    #[no_mangle]
    fn io2_put_byte(addr: uint32_t, arg: uint8_t);
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __uint32_t = cty::c_uint;
pub type __time_t = cty::c_long;
pub type time_t = __time_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
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
/* Typedefs */
pub type GETBYTE_FUNC = Option<unsafe extern "C" fn(_: uint32_t) -> uint8_t>;
pub type GETWORD_FUNC = Option<unsafe extern "C" fn(_: uint32_t) -> uint16_t>;
pub type GETLONG_FUNC = Option<unsafe extern "C" fn(_: uint32_t) -> uint32_t>;
pub type PUTBYTE_FUNC
    =
    Option<unsafe extern "C" fn(_: uint32_t, _: uint8_t) -> ()>;
pub type PUTWORD_FUNC
    =
    Option<unsafe extern "C" fn(_: uint32_t, _: uint16_t) -> ()>;
pub type PUTLONG_FUNC
    =
    Option<unsafe extern "C" fn(_: uint32_t, _: uint32_t) -> ()>;
pub type REALADR_FUNC
    =
    Option<unsafe extern "C" fn(_: uint32_t) -> *mut uint8_t>;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct FLASH_WSM {
    pub cmd: cty::c_int,
    pub ret_or: cty::c_int,
    pub write: cty::c_int,
    pub erase: cty::c_int,
    pub changed: *mut cty::c_int,
    pub nblocks: cty::c_int,
    pub write_ready: cty::c_int,
    pub write_phase: cty::c_int,
    pub erase_phase: cty::c_int,
}
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
/* Hey EMACS -*- linux-c -*- */
/* $Id: mem92p.c 2428 2007-04-04 17:05:38Z roms $ */
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
    Memory management: TI92+ FLASH without Hardware Protection
    Some values may be hard-coded for performance reasons !
*/
// 000000-0fffff : RAM (256 KB)
// 100000-1fffff :
// 200000-2fffff : mirror of FLASH (HW2)
// 300000-3fffff :
// 400000-4fffff : external FLASH (2 MB)
// 500000-5fffff :
// 600000-6fffff : memory mapped I/O (all HW)
// 700000-7fffff : memory mapped I/O (HW2)
// 800000-8fffff : unused
// 900000-9fffff :	 ...
// a00000-afffff :
// b00000-bfffff :
// c00000-cfffff :
// d00000-dfffff :
// e00000-efffff :   ...
// d00000-ffffff : unused
#[no_mangle]
pub unsafe extern "C" fn ti92p_mem_init() -> cty::c_int {
    // set mappers
    mem_get_byte_ptr =
        Some(ti92p_get_byte as unsafe extern "C" fn(_: uint32_t) -> uint8_t);
    mem_get_word_ptr =
        Some(ti92p_get_word as unsafe extern "C" fn(_: uint32_t) -> uint16_t);
    mem_get_long_ptr =
        Some(ti92p_get_long as unsafe extern "C" fn(_: uint32_t) -> uint32_t);
    mem_put_byte_ptr =
        Some(ti92p_put_byte as
                 unsafe extern "C" fn(_: uint32_t, _: uint8_t) -> ());
    mem_put_word_ptr =
        Some(ti92p_put_word as
                 unsafe extern "C" fn(_: uint32_t, _: uint16_t) -> ());
    mem_put_long_ptr =
        Some(ti92p_put_long as
                 unsafe extern "C" fn(_: uint32_t, _: uint32_t) -> ());
    mem_get_real_addr_ptr =
        Some(ti92p_get_real_addr as
                 unsafe extern "C" fn(_: uint32_t) -> *mut uint8_t);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ti92p_get_real_addr(mut adr: uint32_t)
 -> *mut uint8_t {
    // RAM access
    if adr >= 0i32 as cty::c_uint && adr <= 0x1fffffi32 as cty::c_uint {
        return tihw.ram.offset((adr &
                                    (256i32 * 1024i32 - 1i32) as cty::c_uint)
                                   as isize)
    } else {
        // FLASH access
        if adr >= 0x200000i32 as cty::c_uint &&
               adr <= 0x5fffffi32 as cty::c_uint {
            return tihw.rom.offset((adr &
                                        (2i32 * (1024i32 * 1024i32) - 1i32) as
                                            cty::c_uint) as isize)
        } else {
            // memory-mapped I/O
            if adr >= 0x600000i32 as cty::c_uint &&
                   adr <= 0x6fffffi32 as cty::c_uint {
                return tihw.io.offset((adr & (32i32 - 1i32) as cty::c_uint)
                                          as isize)
            } else {
                // memory-mapped I/O (hw2)
                if adr >= 0x700000i32 as cty::c_uint &&
                       adr <= (0x700000i32 + (32i32 - 1i32)) as cty::c_uint {
                    return tihw.io2.offset((adr &
                                                (32i32 - 1i32) as
                                                    cty::c_uint) as isize)
                }
            }
        }
    }
    return tihw.unused;
}
#[no_mangle]
pub unsafe extern "C" fn ti92p_get_long(mut adr: uint32_t) -> uint32_t {
    // RAM access
    if adr >= 0i32 as cty::c_uint && adr <= 0x1fffffi32 as cty::c_uint {
        return ((((*tihw.ram.offset((adr &
                                         (256i32 * 1024i32 - 1i32) as
                                             cty::c_uint) as isize) as
                       cty::c_int) << 8i32 |
                      *tihw.ram.offset((adr.wrapping_add(1i32 as cty::c_uint)
                                            &
                                            (256i32 * 1024i32 - 1i32) as
                                                cty::c_uint) as isize) as
                          cty::c_int) as uint16_t as cty::c_int) << 16i32 |
                    ((*tihw.ram.offset((adr.wrapping_add(2i32 as cty::c_uint)
                                            &
                                            (256i32 * 1024i32 - 1i32) as
                                                cty::c_uint) as isize) as
                          cty::c_int) << 8i32 |
                         *tihw.ram.offset((adr.wrapping_add(2i32 as
                                                                cty::c_uint).wrapping_add(1i32
                                                                                               as
                                                                                               cty::c_uint)
                                               &
                                               (256i32 * 1024i32 - 1i32) as
                                                   cty::c_uint) as isize) as
                             cty::c_int) as uint16_t as cty::c_int) as
                   uint32_t
    } else {
        // FLASH access
        if adr >= 0x200000i32 as cty::c_uint &&
               adr <= 0x5fffffi32 as cty::c_uint {
            return ((((*tihw.rom.offset((adr &
                                             (2i32 * (1024i32 * 1024i32) -
                                                  1i32) as cty::c_uint) as
                                            isize) as cty::c_int) << 8i32 |
                          *tihw.rom.offset((adr.wrapping_add(1i32 as
                                                                 cty::c_uint)
                                                &
                                                (2i32 * (1024i32 * 1024i32) -
                                                     1i32) as cty::c_uint) as
                                               isize) as cty::c_int) as
                         uint16_t as cty::c_int) << 16i32 |
                        ((*tihw.rom.offset((adr.wrapping_add(2i32 as
                                                                 cty::c_uint)
                                                &
                                                (2i32 * (1024i32 * 1024i32) -
                                                     1i32) as cty::c_uint) as
                                               isize) as cty::c_int) << 8i32
                             |
                             *tihw.rom.offset((adr.wrapping_add(2i32 as
                                                                    cty::c_uint).wrapping_add(1i32
                                                                                                   as
                                                                                                   cty::c_uint)
                                                   &
                                                   (2i32 * (1024i32 * 1024i32)
                                                        - 1i32) as
                                                       cty::c_uint) as isize)
                                 as cty::c_int) as uint16_t as cty::c_int)
                       as uint32_t | wsm.ret_or as cty::c_uint
        } else {
            // memory-mapped I/O
            if adr >= 0x600000i32 as cty::c_uint &&
                   adr <= 0x6fffffi32 as cty::c_uint {
                return io_get_long(adr)
            } else {
                // memory-mapped I/O (hw2)
                if adr >= 0x700000i32 as cty::c_uint &&
                       adr <= (0x700000i32 + (32i32 - 1i32)) as cty::c_uint {
                    return io2_get_long(adr)
                }
            }
        }
    }
    return 0x14141414i32 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn ti92p_get_word(mut adr: uint32_t) -> uint16_t {
    // RAM access
    if adr >= 0i32 as cty::c_uint && adr <= 0x1fffffi32 as cty::c_uint {
        return ((*tihw.ram.offset((adr &
                                       (256i32 * 1024i32 - 1i32) as
                                           cty::c_uint) as isize) as
                     cty::c_int) << 8i32 |
                    *tihw.ram.offset((adr.wrapping_add(1i32 as cty::c_uint) &
                                          (256i32 * 1024i32 - 1i32) as
                                              cty::c_uint) as isize) as
                        cty::c_int) as uint16_t
    } else {
        // FLASH access
        if adr >= 0x200000i32 as cty::c_uint &&
               adr <= 0x5fffffi32 as cty::c_uint {
            return (((*tihw.rom.offset((adr &
                                            (2i32 * (1024i32 * 1024i32) -
                                                 1i32) as cty::c_uint) as
                                           isize) as cty::c_int) << 8i32 |
                         *tihw.rom.offset((adr.wrapping_add(1i32 as
                                                                cty::c_uint)
                                               &
                                               (2i32 * (1024i32 * 1024i32) -
                                                    1i32) as cty::c_uint) as
                                              isize) as cty::c_int) as
                        uint16_t as cty::c_int | wsm.ret_or) as uint16_t
        } else {
            // memory-mapped I/O
            if adr >= 0x600000i32 as cty::c_uint &&
                   adr <= 0x6fffffi32 as cty::c_uint {
                return io_get_word(adr)
            } else {
                // memory-mapped I/O (hw2)
                if adr >= 0x700000i32 as cty::c_uint &&
                       adr <= (0x700000i32 + (32i32 - 1i32)) as cty::c_uint {
                    return io2_get_word(adr)
                }
            }
        }
    }
    return 0x1414i32 as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn ti92p_get_byte(mut adr: uint32_t) -> uint8_t {
    // RAM access
    if adr >= 0i32 as cty::c_uint && adr <= 0x1fffffi32 as cty::c_uint {
        return *tihw.ram.offset((adr &
                                     (256i32 * 1024i32 - 1i32) as
                                         cty::c_uint) as isize)
    } else {
        // FLASH access
        if adr >= 0x200000i32 as cty::c_uint &&
               adr <= 0x5fffffi32 as cty::c_uint {
            return (*tihw.rom.offset((adr &
                                          (2i32 * (1024i32 * 1024i32) - 1i32)
                                              as cty::c_uint) as isize) as
                        cty::c_int | wsm.ret_or) as uint8_t
        } else {
            // memory-mapped I/O
            if adr >= 0x600000i32 as cty::c_uint &&
                   adr <= 0x6fffffi32 as cty::c_uint {
                return io_get_byte(adr)
            } else {
                // memory-mapped I/O (hw2)
                if adr >= 0x700000i32 as cty::c_uint &&
                       adr <= (0x700000i32 + (32i32 - 1i32)) as cty::c_uint {
                    return io2_get_byte(adr)
                }
            }
        }
    }
    return 0x14i32 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn ti92p_put_long(mut adr: uint32_t,
                                        mut arg: uint32_t) {
    // RAM access
    if adr >= 0i32 as cty::c_uint && adr <= 0x1fffffi32 as cty::c_uint {
        *tihw.ram.offset((adr & (256i32 * 1024i32 - 1i32) as cty::c_uint) as
                             isize) =
            ((arg >> 16i32) as uint16_t as cty::c_int >> 8i32) as uint8_t;
        *tihw.ram.offset((adr.wrapping_add(1i32 as cty::c_uint) &
                              (256i32 * 1024i32 - 1i32) as cty::c_uint) as
                             isize) =
            ((arg >> 16i32) as uint16_t as cty::c_int & 0xffi32) as uint8_t;
        *tihw.ram.offset((adr.wrapping_add(2i32 as cty::c_uint) &
                              (256i32 * 1024i32 - 1i32) as cty::c_uint) as
                             isize) =
            ((arg & 0xffffi32 as cty::c_uint) as uint16_t as cty::c_int >>
                 8i32) as uint8_t;
        *tihw.ram.offset((adr.wrapping_add(2i32 as
                                               cty::c_uint).wrapping_add(1i32
                                                                              as
                                                                              cty::c_uint)
                              & (256i32 * 1024i32 - 1i32) as cty::c_uint) as
                             isize) =
            ((arg & 0xffffi32 as cty::c_uint) as uint16_t as cty::c_int &
                 0xffi32) as uint8_t
    } else if adr >= 0x200000i32 as cty::c_uint &&
                  adr <= 0x5fffffi32 as cty::c_uint {
        FlashWriteLong(adr, arg);
    } else if adr >= 0x600000i32 as cty::c_uint &&
                  adr <= 0x6fffffi32 as cty::c_uint {
        io_put_long(adr, arg);
    } else if adr >= 0x700000i32 as cty::c_uint &&
                  adr <= (0x700000i32 + (32i32 - 1i32)) as cty::c_uint {
        io2_put_long(adr, arg);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ti92p_put_word(mut adr: uint32_t,
                                        mut arg: uint16_t) {
    // FLASH access
    // memory-mapped I/O
    // memory-mapped I/O (hw2)
    // RAM access
    if adr >= 0i32 as cty::c_uint && adr <= 0x1fffffi32 as cty::c_uint {
        *tihw.ram.offset((adr & (256i32 * 1024i32 - 1i32) as cty::c_uint) as
                             isize) = (arg as cty::c_int >> 8i32) as uint8_t;
        *tihw.ram.offset((adr.wrapping_add(1i32 as cty::c_uint) &
                              (256i32 * 1024i32 - 1i32) as cty::c_uint) as
                             isize) =
            (arg as cty::c_int & 0xffi32) as uint8_t
    } else if adr >= 0x200000i32 as cty::c_uint &&
                  adr <= 0x5fffffi32 as cty::c_uint {
        FlashWriteWord(adr, arg);
    } else if adr >= 0x600000i32 as cty::c_uint &&
                  adr <= 0x6fffffi32 as cty::c_uint {
        io_put_word(adr, arg);
    } else if adr >= 0x700000i32 as cty::c_uint &&
                  adr <= (0x700000i32 + (32i32 - 1i32)) as cty::c_uint {
        io2_put_word(adr, arg);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ti92p_put_byte(mut adr: uint32_t, mut arg: uint8_t) {
    // FLASH access
    // memory-mapped I/O
    // memory-mapped I/O (hw2)
    // RAM access
    if adr >= 0i32 as cty::c_uint && adr <= 0x1fffffi32 as cty::c_uint {
        *tihw.ram.offset((adr & (256i32 * 1024i32 - 1i32) as cty::c_uint) as
                             isize) = arg
    } else if adr >= 0x200000i32 as cty::c_uint &&
                  adr <= 0x5fffffi32 as cty::c_uint {
        FlashWriteByte(adr, arg);
    } else if adr >= 0x600000i32 as cty::c_uint &&
                  adr <= 0x6fffffi32 as cty::c_uint {
        io_put_byte(adr, arg);
    } else if adr >= 0x700000i32 as cty::c_uint &&
                  adr <= (0x700000i32 + (32i32 - 1i32)) as cty::c_uint {
        io2_put_byte(adr, arg);
    };
}
// FLASH access
// memory-mapped I/O
// memory-mapped I/O (hw2)
