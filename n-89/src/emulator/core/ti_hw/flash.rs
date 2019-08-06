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
    fn calloc(_: cty::c_ulong, _: cty::c_ulong) -> *mut cty::c_void;
    #[no_mangle]
    fn free(__ptr: *mut cty::c_void);
    #[no_mangle]
    fn memset(_: *mut cty::c_void, _: cty::c_int, _: cty::c_ulong)
     -> *mut cty::c_void;
    #[no_mangle]
    fn tiemu_info(format: *const cty::c_char, _: ...);
    #[no_mangle]
    static mut tihw: Ti68kHardware;
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __uint32_t = cty::c_uint;
pub type __time_t = cty::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type time_t = __time_t;
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
/* $Id: flash.c 2372 2007-02-25 21:43:23Z roms $ */
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
/*
    FLASH algorithm management:
    - Sharp's LH28F160S3T: TI89/TI92+
    - Sharp's LH28F320BF: V200/TI89 Titanium
*/
//memset
#[no_mangle]
pub static mut wsm: FLASH_WSM =
    FLASH_WSM{cmd: 0,
              ret_or: 0,
              write: 0,
              erase: 0,
              changed: 0 as *const cty::c_int as *mut cty::c_int,
              nblocks: 0,
              write_ready: 0,
              write_phase: 0,
              erase_phase: 0,};
/* Functions */
#[no_mangle]
pub unsafe extern "C" fn hw_flash_init() -> cty::c_int {
    memset(&mut wsm as *mut FLASH_WSM as *mut cty::c_void, 0i32,
           ::core::mem::size_of::<FLASH_WSM>() as cty::c_ulong);
    wsm.nblocks = tihw.rom_size >> 16i32;
    wsm.changed =
        calloc(wsm.nblocks as cty::c_ulong,
               ::core::mem::size_of::<cty::c_int>() as cty::c_ulong) as
            *mut cty::c_int;
    wsm.write_phase = 0x50i32;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn hw_flash_reset() -> cty::c_int { return 0i32; }
#[no_mangle]
pub unsafe extern "C" fn hw_flash_exit() -> cty::c_int {
    if !wsm.changed.is_null() { free(wsm.changed as *mut cty::c_void); }
    wsm.changed = 0 as *mut cty::c_int;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn hw_flash_nblocks() -> cty::c_int {
    let mut i: cty::c_int = 0;
    let mut n: cty::c_int = 0i32;
    i = 0i32;
    while i < wsm.nblocks {
        if 0 != *wsm.changed.offset(i as isize) { n += 1 }
        i += 1
    }
    return n;
}
/*
    Read a byte from a Sharp FLASH memory.
*/
#[no_mangle]
pub unsafe extern "C" fn FlashReadByte(mut addr: uint32_t) -> uint8_t {
    if wsm.write_phase == 0x90i32 {
        match addr & 0xffffi32 as cty::c_uint {
            0 => {
                return (if tihw.calc_type == 1i32 << 3i32 ||
                               tihw.calc_type == 1i32 << 4i32 {
                            0xb0i32
                        } else { 0x89i32 }) as uint8_t
            }
            1 => { return 0i32 as uint8_t }
            2 => { // manufacturer code
                return 0xb5i32 as uint8_t
            }
            3 => { return 0i32 as uint8_t }
            _ => { return 0xffi32 as uint8_t }
        }
    } // device code
    return (*tihw.rom.offset((addr & (tihw.rom_size - 1i32) as cty::c_uint)
                                 as isize) as cty::c_int | wsm.ret_or) as
               uint8_t; // manufacturer code
}
#[no_mangle]
pub unsafe extern "C" fn FlashReadWord(mut addr: uint32_t) -> uint16_t {
    if wsm.write_phase == 0x90i32 {
        match addr & 0xffffi32 as cty::c_uint {
            0 => {
                return (if tihw.calc_type == 1i32 << 3i32 ||
                               tihw.calc_type == 1i32 << 4i32 {
                            0xb0i32
                        } else { 0x89i32 }) as uint16_t
            }
            2 => { return 0xb5i32 as uint16_t }
            _ => { return 0xffffi32 as uint16_t }
        }
    } // device code
    return (((*tihw.rom.offset((addr & (tihw.rom_size - 1i32) as cty::c_uint)
                                   as isize) as cty::c_int) << 8i32 |
                 *tihw.rom.offset((addr.wrapping_add(1i32 as cty::c_uint) &
                                       (tihw.rom_size - 1i32) as cty::c_uint)
                                      as isize) as cty::c_int) as uint16_t as
                cty::c_int | wsm.ret_or) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn FlashReadLong(mut addr: uint32_t) -> uint32_t {
    return ((FlashReadWord(addr) as cty::c_int) << 16i32 |
                FlashReadWord(addr.wrapping_add(2i32 as cty::c_uint)) as
                    cty::c_int) as uint32_t;
}
/*
    Write a byte to a Sharp FLASH memory
*/
#[no_mangle]
pub unsafe extern "C" fn FlashWriteByte(mut addr: uint32_t, mut v: uint8_t) {
    // int i;
    let mut rom: *mut uint8_t = tihw.rom;
    if tihw.calc_type == 1i32 << 0i32 { return }
    if 0 != tihw.protect { return }
    addr =
        (addr as cty::c_uint).wrapping_sub(tihw.rom_base) as uint32_t as
            uint32_t;
    addr &= (tihw.rom_size - 1i32) as cty::c_uint;
    // Write State Machine (WSM, Sharp's data sheet)
    if 0 != wsm.write_ready {
        if *rom.offset(addr as isize) as cty::c_int != v as cty::c_int {
            *wsm.changed.offset((addr >> 16i32) as isize) =
                (0 == 0i32) as cty::c_int
        } /* can't set bits from 0 to 1 with a write! */
        let ref mut fresh0 = *rom.offset(addr as isize);
        *fresh0 = (*fresh0 as cty::c_int & v as cty::c_int) as uint8_t;
        wsm.write_ready -= 1;
        wsm.ret_or = 0xffffffffu32 as cty::c_int
    } else if v as cty::c_int == 0x50i32 {
        // clear status register
        wsm.write_phase = 0x50i32
    } else if v as cty::c_int == 0x10i32 {
        // byte write setup/confirm
        if wsm.write_phase == 0x50i32 {
            wsm.write_phase = 0x51i32
        } else if wsm.write_phase == 0x51i32 {
            wsm.write_ready = 2i32;
            wsm.write_phase = 0x50i32
        }
    } else if v as cty::c_int == 0x20i32 {
        // block erase setup/confirm
        if wsm.write_phase == 0x50i32 { wsm.write_phase = 0x20i32 }
    } else if v as cty::c_int == 0xd0i32 {
        // confirm and block erase
        if wsm.write_phase == 0x20i32 {
            wsm.write_phase = 0xd0i32;
            wsm.ret_or = 0xffffffffu32 as cty::c_int;
            wsm.erase = 0xffffffffu32 as cty::c_int;
            wsm.erase_phase = 0i32;
            memset(&mut *rom.offset((addr & 0xff0000i32 as cty::c_uint) as
                                        isize) as *mut uint8_t as
                       *mut cty::c_void, 0xffi32,
                   (64i32 * 1024i32) as cty::c_ulong);
            *wsm.changed.offset((addr >> 16i32) as isize) =
                (0 == 0i32) as cty::c_int
            // printf("%i erased\n", addr>>16);
        }
    } else if v as cty::c_int == 0xffi32 {
        // read array/reset
        if wsm.write_phase == 0x50i32 {
            wsm.write_ready = 0i32;
            wsm.ret_or = 0i32
        }
    } else if v as cty::c_int == 0x90i32 {
        // read identifier codes
        wsm.write_phase = 0x90i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn FlashWriteWord(mut addr: uint32_t,
                                        mut data: uint16_t) {
    // roms: fix me...
    FlashWriteByte(addr.wrapping_add(0i32 as cty::c_uint),
                   ((data as cty::c_int & 0xff00i32) >> 8i32) as uint8_t);
    FlashWriteByte(addr.wrapping_add(1i32 as cty::c_uint),
                   (data as cty::c_int & 0xffi32) as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn FlashWriteLong(mut addr: uint32_t,
                                        mut data: uint32_t) {
    FlashWriteWord(addr.wrapping_add(0i32 as cty::c_uint),
                   (data >> 16i32 & 0xffffi32 as cty::c_uint) as uint16_t);
    FlashWriteWord(addr.wrapping_add(2i32 as cty::c_uint),
                   (data >> 0i32 & 0xffffi32 as cty::c_uint) as uint16_t);
}
/*
    Search for a vector table and get SSP & PC values.
    This is needed by m68k_reset() for booting.
*/
#[no_mangle]
pub unsafe extern "C" fn find_ssp_and_pc(mut ssp: *mut uint32_t,
                                         mut pc: *mut uint32_t) {
    let mut vt: cty::c_int = 0i32; // vector table
    // find PC reset vector
    if 0 != tihw.rom_flash {
        // FLASH (TI89, TI92+, V200, ...)
        vt = 0x12000i32; // skip SP
        while vt < tihw.rom_size {
            let mut rom: *mut uint8_t = tihw.rom.offset(vt as isize);
            if *rom.offset(0isize) as cty::c_int == 0xcci32 &&
                   *rom.offset(1isize) as cty::c_int == 0xcci32 &&
                   *rom.offset(2isize) as cty::c_int == 0xcci32 &&
                   *rom.offset(3isize) as cty::c_int == 0xcci32 {
                vt += 4i32;
                break ;
            } else { vt += 1 }
        }
        *ssp =
            (*tihw.rom.offset((vt + 3i32) as isize) as cty::c_int |
                 (*tihw.rom.offset((vt + 2i32) as isize) as cty::c_int) <<
                     8i32 |
                 (*tihw.rom.offset((vt + 1i32) as isize) as cty::c_int) <<
                     16i32 |
                 (*tihw.rom.offset(vt as isize) as cty::c_int) << 24i32) as
                uint32_t;
        vt += 4i32;
        *pc =
            (*tihw.rom.offset((vt + 3i32) as isize) as cty::c_int |
                 (*tihw.rom.offset((vt + 2i32) as isize) as cty::c_int) <<
                     8i32 |
                 (*tihw.rom.offset((vt + 1i32) as isize) as cty::c_int) <<
                     16i32 |
                 (*tihw.rom.offset(vt as isize) as cty::c_int) << 24i32) as
                uint32_t
    } else {
        // EPROM (TI92)
        vt = 0i32; // skip SP
        *ssp =
            (*tihw.rom.offset((vt + 3i32) as isize) as cty::c_int |
                 (*tihw.rom.offset((vt + 2i32) as isize) as cty::c_int) <<
                     8i32 |
                 (*tihw.rom.offset((vt + 1i32) as isize) as cty::c_int) <<
                     16i32 |
                 (*tihw.rom.offset(vt as isize) as cty::c_int) << 24i32) as
                uint32_t;
        vt += 4i32;
        *pc =
            (*tihw.rom.offset((vt + 3i32) as isize) as cty::c_int |
                 (*tihw.rom.offset((vt + 2i32) as isize) as cty::c_int) <<
                     8i32 |
                 (*tihw.rom.offset((vt + 1i32) as isize) as cty::c_int) <<
                     16i32 |
                 (*tihw.rom.offset(vt as isize) as cty::c_int) << 24i32) as
                uint32_t
    }
    tiemu_info(b"found SSP=$%06x and PC=$%06x at offset 0x%x\x00" as *const u8
                   as *const cty::c_char, *ssp, *pc, vt - 0x12000i32);
}
/*
#if 0
#define	printd	printf
#else
#define printd
#endif

void FlashWriteWord(uint32_t addr, uint16_t data)
{
    uint8_t *rom = tihw.rom;

    if(tihw.calc_type == TI92)
        return;

    if(tihw.protect)
        return;

    addr -= tihw.rom_base;
    addr &= tihw.rom_size - 1;

    if((addr & 0x1fff) == 0x1000)
        printf("WW: $%06x: %04x ($%06x)\n", addr+ti_rom_base[log_b2(tihw.calc_type)], data, m68k_getpc());

    // Write State Machine (WSM, Sharp's data sheet)
    switch(data & 0xff)
    {
    case FCD_SETUP_BYTE_WRITE:	//0x10: byte write setup/confirm
        printd("FCD_SETUP_BYTE_WRITE: $%06x\n", m68k_getpc());
        wsm.cmd = 0x10;
        wsm.write = 1;
        break;
    default:
        printd("FCD_BYTE_WRITE: %04x at $%06x\n", data, m68k_getpc());
        if(wsm.write)
        {
            wsm.cmd = 0xff;
            rom[addr+0] = MSB(data);
            rom[addr+1] = LSB(data);
            wsm.write = 0;
            wsm.ret_or = 0xffffffff;
        }
        break;
    case FCD_SETUP_BLCK_ERASE:	//0x20: block erase setup/confirm
        printd("FCD_SETUP_BLCK_ERASE: $%06x\n", m68k_getpc());
        wsm.cmd = 0x20;
        wsm.erase = 1;
        break;
    case FCD_CONFIRM_BLK_ERASE:	//0xd0: confirm and block erase
        if(wsm.cmd == 0x20)
        {
            printd("FCD_CONFIRM_BLK_ERASE: $%06x\n", m68k_getpc());
            wsm.cmd = 0xd0;
            memset(&rom[addr & 0xff0000], 0xff, 64*KB);
            wsm.erase = 0;
            wsm.ret_or = 0xffffffff;
        }
        break;
    case FCD_CLEAR_STATUS:		//0x50: clear status register
        printd("FCD_CLEAR_STATUS: $%06x\n", m68k_getpc());
        wsm.cmd = 0x50;
        wsm.write = 0;
        wsm.erase = 0;
        wsm.ret_or = 0;
        break;
    case FCD_READ_ID_CODES:		//0x90: read identifier codes
        printd("FCD_READ_ID_CODES: $%06x\n", m68k_getpc());
        wsm.cmd = 0x90;
        break;
    case FCD_READ_OR_RESET:		//0xff: read array/reset
        printd("FCD_READ_OR_RESET: $%06x\n", m68k_getpc());
        if(wsm.erase || wsm.write)
            break;

        wsm.cmd = 0xff;
        wsm.write = 0;
        wsm.erase = 0;
        wsm.ret_or = 0;
        break;
    }
}
  */
