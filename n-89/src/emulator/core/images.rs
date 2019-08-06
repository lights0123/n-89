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
    fn __ctype_b_loc() -> *mut *const cty::c_ushort;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> cty::c_int;
    #[no_mangle]
    fn fopen(_: *const cty::c_char, _: *const cty::c_char) -> *mut FILE;
    #[no_mangle]
    fn fread(_: *mut cty::c_void, _: cty::c_ulong, _: cty::c_ulong,
             _: *mut FILE) -> cty::c_ulong;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: cty::c_long, __whence: cty::c_int)
     -> cty::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> cty::c_long;
    #[no_mangle]
    fn malloc(_: cty::c_ulong) -> *mut cty::c_void;
    #[no_mangle]
    fn free(__ptr: *mut cty::c_void);
    #[no_mangle]
    fn memcpy(_: *mut cty::c_void, _: *const cty::c_void, _: cty::c_ulong)
     -> *mut cty::c_void;
    #[no_mangle]
    fn memset(_: *mut cty::c_void, _: cty::c_int, _: cty::c_ulong)
     -> *mut cty::c_void;
    #[no_mangle]
    fn strcpy(_: *mut cty::c_char, _: *const cty::c_char)
     -> *mut cty::c_char;
    #[no_mangle]
    fn strcmp(_: *const cty::c_char, _: *const cty::c_char) -> cty::c_int;
    #[no_mangle]
    fn strrchr(_: *const cty::c_char, _: cty::c_int) -> *mut cty::c_char;
    #[no_mangle]
    fn strcasecmp(_: *const cty::c_char, _: *const cty::c_char)
     -> cty::c_int;
    #[no_mangle]
    fn tiemu_info(format: *const cty::c_char, _: ...);
    #[no_mangle]
    fn tiemu_warning(format: *const cty::c_char, _: ...);
    /*
    Functions
*/
    #[no_mangle]
    fn ti68k_display_hw_param_block(s: *mut HW_PARM_BLOCK);
    #[no_mangle]
    fn ti68k_get_hw_param_block(rom_data: *mut uint8_t, rom_base: uint8_t,
                                block: *mut HW_PARM_BLOCK) -> cty::c_int;
    #[no_mangle]
    fn g_basename(file_name: *const cty::c_char) -> *const cty::c_char;
    #[no_mangle]
    fn ti68k_calctype_to_string(type_0: cty::c_int) -> *const cty::c_char;
    #[no_mangle]
    fn ti68k_romtype_to_string(type_0: cty::c_int) -> *const cty::c_char;
}
pub type __uint8_t = cty::c_uchar;
pub type __uint16_t = cty::c_ushort;
pub type __int32_t = cty::c_int;
pub type __uint32_t = cty::c_uint;
pub type __int64_t = cty::c_long;
pub type __off_t = cty::c_long;
pub type __off64_t = cty::c_long;
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
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct HW_PARM_BLOCK {
    pub len: uint16_t,
    pub hardwareID: uint32_t,
    pub hardwareRevision: uint32_t,
    pub bootMajor: uint32_t,
    pub bootRevision: uint32_t,
    pub bootBuild: uint32_t,
    pub gateArray: uint32_t,
    pub physDisplayBitsWide: uint32_t,
    pub physDisplayBitsTall: uint32_t,
    pub LCDBitsWide: uint32_t,
    pub LCDBitsTall: uint32_t,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct IMG_INFO32 {
    pub signature: [cty::c_char; 16],
    pub revision: int32_t,
    pub header_size: int32_t,
    pub calc_type: cty::c_char,
    pub version: [cty::c_char; 5],
    pub flash: cty::c_char,
    pub has_boot: cty::c_char,
    pub size: int32_t,
    pub hw_type: cty::c_char,
    pub rom_base: uint8_t,
    pub fill: [cty::c_char; 22],
    pub data: *mut cty::c_char,
}
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct IMG_INFO64 {
    pub signature: [cty::c_char; 16],
    pub revision: int64_t,
    pub header_size: int64_t,
    pub calc_type: cty::c_char,
    pub version: [cty::c_char; 5],
    pub flash: cty::c_char,
    pub has_boot: cty::c_char,
    pub size: int64_t,
    pub hw_type: cty::c_char,
    pub rom_base: uint8_t,
    pub fill: [cty::c_char; 22],
    pub data: *mut cty::c_char,
}
// system privileged part
// offset from SPP to boot
#[no_mangle]
pub static mut img_infos: IMG_INFO32 =
    IMG_INFO32{signature: [0; 16],
               revision: 0,
               header_size: 0,
               calc_type: 0,
               version: [0; 5],
               flash: 0,
               has_boot: 0,
               size: 0,
               hw_type: 0,
               rom_base: 0,
               fill: [0; 22],
               data: 0 as *const cty::c_char as *mut cty::c_char,};
#[no_mangle]
pub static mut img_loaded: cty::c_int = 0i32;
#[no_mangle]
pub static mut img_changed: cty::c_int = 0i32;
/*
    Utility functions
*/
#[no_mangle]
pub unsafe extern "C" fn ti68k_is_a_rom_file(mut filename:
                                                 *const cty::c_char)
 -> cty::c_int {
    let mut ext: *mut cty::c_char = 0 as *mut cty::c_char;
    ext = strrchr(filename, '.' as i32);
    if ext.is_null() {
        return 0i32
    } else {
        if 0 ==
               strcasecmp(ext,
                          b".rom\x00" as *const u8 as *const cty::c_char) {
            return (0 == 0i32) as cty::c_int
        }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_is_a_tib_file(mut filename:
                                                 *const cty::c_char)
 -> cty::c_int {
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_is_a_img_file(mut filename:
                                                 *const cty::c_char)
 -> cty::c_int {
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_is_a_sav_file(mut filename:
                                                 *const cty::c_char)
 -> cty::c_int {
    let mut ext: *mut cty::c_char = 0 as *mut cty::c_char;
    ext = strrchr(filename, '.' as i32);
    if ext.is_null() {
        return 0i32
    } else {
        if 0 ==
               strcasecmp(ext,
                          b".sav\x00" as *const u8 as *const cty::c_char) {
            return (0 == 0i32) as cty::c_int
        }
    }
    return 0i32;
}
/*
    Display information
*/
#[no_mangle]
pub unsafe extern "C" fn ti68k_display_rom_infos(mut s: *mut IMG_INFO32) {
    tiemu_info(b"ROM information:\x00" as *const u8 as *const cty::c_char);
    tiemu_info(b"  Calculator  : %s\x00" as *const u8 as *const cty::c_char,
               ti68k_calctype_to_string((*s).calc_type as cty::c_int));
    tiemu_info(b"  Firmware    : %s\x00" as *const u8 as *const cty::c_char,
               (*s).version.as_mut_ptr());
    tiemu_info(b"  Memory type : %s\x00" as *const u8 as *const cty::c_char,
               ti68k_romtype_to_string((*s).flash as cty::c_int));
    tiemu_info(b"  Memory size : %iMB (%i bytes)\x00" as *const u8 as
                   *const cty::c_char, (*s).size >> 20i32, (*s).size);
    tiemu_info(b"  ROM base    : %02x\x00" as *const u8 as
                   *const cty::c_char,
               (*s).rom_base as cty::c_int & 0xffi32);
    tiemu_info(b"  Hardware    : %i\x00" as *const u8 as *const cty::c_char,
               (*s).hw_type as cty::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_display_tib_infos(mut s: *mut IMG_INFO32) {
    tiemu_info(b"TIB information:\x00" as *const u8 as *const cty::c_char);
    tiemu_info(b"  Calculator  : %s\x00" as *const u8 as *const cty::c_char,
               ti68k_calctype_to_string((*s).calc_type as cty::c_int));
    tiemu_info(b"  Firmware    : %s\x00" as *const u8 as *const cty::c_char,
               (*s).version.as_mut_ptr());
    tiemu_info(b"  Memory type : %s\x00" as *const u8 as *const cty::c_char,
               ti68k_romtype_to_string((*s).flash as cty::c_int));
    tiemu_info(b"  Memory size : %iMB (%i bytes)\x00" as *const u8 as
                   *const cty::c_char, (*s).size >> 20i32, (*s).size);
    tiemu_info(b"  ROM base    : %02x\x00" as *const u8 as
                   *const cty::c_char,
               (*s).rom_base as cty::c_int & 0xffi32);
}
#[no_mangle]
pub unsafe extern "C" fn ti68k_display_img_infos(mut s: *mut IMG_INFO32) {
    tiemu_info(b"Image information:\x00" as *const u8 as *const cty::c_char);
    tiemu_info(b"  Calculator  : %s\x00" as *const u8 as *const cty::c_char,
               ti68k_calctype_to_string((*s).calc_type as cty::c_int));
    tiemu_info(b"  Firmware    : %s\x00" as *const u8 as *const cty::c_char,
               (*s).version.as_mut_ptr());
    tiemu_info(b"  Memory type : %s\x00" as *const u8 as *const cty::c_char,
               ti68k_romtype_to_string((*s).flash as cty::c_int));
    tiemu_info(b"  Memory size : %iMB (%i bytes)\x00" as *const u8 as
                   *const cty::c_char, (*s).size >> 20i32, (*s).size);
    tiemu_info(b"  ROM base    : %02x\x00" as *const u8 as
                   *const cty::c_char,
               (*s).rom_base as cty::c_int & 0xffi32);
    tiemu_info(b"  Hardware    : %i\x00" as *const u8 as *const cty::c_char,
               (*s).hw_type as cty::c_int);
    tiemu_info(b"  Has boot    : %s\x00" as *const u8 as *const cty::c_char,
               if 0 != (*s).has_boot as cty::c_int {
                   b"yes\x00" as *const u8 as *const cty::c_char
               } else { b"no\x00" as *const u8 as *const cty::c_char });
}
/*
    Try to get some information on the ROM dump:
    - size
    - ROM base address
    - FLASH/EPROM
    - soft version
    - calc type
*/
#[no_mangle]
pub unsafe extern "C" fn ti68k_get_img_infos(mut filename:
                                                 *const cty::c_char,
                                             mut ri: *mut IMG_INFO32)
 -> cty::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut ri32: IMG_INFO32 =
        IMG_INFO32{signature: [0; 16],
                   revision: 0,
                   header_size: 0,
                   calc_type: 0,
                   version: [0; 5],
                   flash: 0,
                   has_boot: 0,
                   size: 0,
                   hw_type: 0,
                   rom_base: 0,
                   fill: [0; 22],
                   data: 0 as *const cty::c_char as *mut cty::c_char,};
    let mut ri64: IMG_INFO64 =
        IMG_INFO64{signature: [0; 16],
                   revision: 0,
                   header_size: 0,
                   calc_type: 0,
                   version: [0; 5],
                   flash: 0,
                   has_boot: 0,
                   size: 0,
                   hw_type: 0,
                   rom_base: 0,
                   fill: [0; 22],
                   data: 0 as *mut cty::c_char,};
    // No filename, exits
    if 0 ==
           strcmp(g_basename(filename),
                  b"\x00" as *const u8 as *const cty::c_char) {
        return 768i32
    }
    // Check file
    if 0 == ti68k_is_a_img_file(filename) {
        tiemu_warning(b"Images must have \'.img\' extension (%s).\n\x00" as
                          *const u8 as *const cty::c_char, filename);
        return 768i32
    }
    // Open dest file
    f = fopen(filename, b"rb\x00" as *const u8 as *const cty::c_char);
    if f.is_null() {
        tiemu_warning(b"Unable to open this file: <%s>\n\x00" as *const u8 as
                          *const cty::c_char, filename);
        return 768i32
    }
    // Read header
    if fread(&mut ri32 as *mut IMG_INFO32 as *mut cty::c_void,
             ::core::mem::size_of::<IMG_INFO32>() as cty::c_ulong,
             1i32 as cty::c_ulong, f) < 1i32 as cty::c_ulong {
        tiemu_warning(b"Failed to read from file: <%s>\n\x00" as *const u8 as
                          *const cty::c_char, filename);
        fclose(f);
        return 768i32
    }
    *ri = ri32;
    // below is patch from Lionel
    if 0 !=
           strcmp((*ri).signature.as_mut_ptr(),
                  b"TiEmu img v2.00\x00" as *const u8 as *const cty::c_char)
           || (*ri).size > 4i32 * (1024i32 * 1024i32) ||
           (*ri).calc_type as cty::c_int > 1i32 << 4i32 ||
           (*ri).header_size == 0i32 || (*ri).hw_type as cty::c_int > 4i32 ||
           (*ri).rom_base as cty::c_int == 0i32 {
        // In addition to plain invalid files, this may happen if the image was
		// created on a 64-bit platform with TIEmu <= 3.03.
		// Try to read an IMG_INFO structure as it used to be written by those
		// 64-bit platforms.
        fseek(f, 0i32 as cty::c_long, 0i32);
        if fread(&mut ri64 as *mut IMG_INFO64 as *mut cty::c_void,
                 ::core::mem::size_of::<IMG_INFO64>() as cty::c_ulong,
                 1i32 as cty::c_ulong, f) < 1i32 as cty::c_ulong {
            tiemu_warning(b"Failed to read from file: <%s>\n\x00" as *const u8
                              as *const cty::c_char, filename);
            fclose(f);
            return 768i32
        } else {
            memcpy((*ri).signature.as_mut_ptr() as *mut cty::c_void,
                   &mut ri64.signature as *mut [cty::c_char; 16] as
                       *const cty::c_void,
                   ::core::mem::size_of::<[cty::c_char; 16]>() as
                       cty::c_ulong);
            (*ri).revision = ri64.revision as int32_t;
            (*ri).header_size = ri64.header_size as int32_t;
            (*ri).calc_type = ri64.calc_type;
            memcpy((*ri).version.as_mut_ptr() as *mut cty::c_void,
                   &mut ri64.version as *mut [cty::c_char; 5] as
                       *const cty::c_void,
                   ::core::mem::size_of::<[cty::c_char; 5]>() as
                       cty::c_ulong);
            (*ri).flash = ri64.flash;
            (*ri).has_boot = ri64.has_boot;
            (*ri).size = ri64.size as int32_t;
            (*ri).hw_type = ri64.hw_type;
            (*ri).rom_base = ri64.rom_base;
            if 0 !=
                   strcmp((*ri).signature.as_mut_ptr(),
                          b"TiEmu img v2.00\x00" as *const u8 as
                              *const cty::c_char) ||
                   (*ri).size > 4i32 * (1024i32 * 1024i32) ||
                   (*ri).calc_type as cty::c_int > 1i32 << 4i32 ||
                   (*ri).header_size == 0i32 ||
                   (*ri).hw_type as cty::c_int > 4i32 ||
                   (*ri).rom_base as cty::c_int == 0i32 {
                // Nope, it still doesn't seem to be a TIEmu image.
                tiemu_warning(b"Bad image: <%s>\n\x00" as *const u8 as
                                  *const cty::c_char, filename);
                return 771i32
            } else {
                tiemu_info(b"Found a reasonably valid 64-bit IMG_INFO in <%s>\n\x00"
                               as *const u8 as *const cty::c_char, filename);
            }
        }
    }
    // Close file
    if 0 != fclose(f) {
        tiemu_warning(b"Failed to close file: <%s>\n\x00" as *const u8 as
                          *const cty::c_char, filename);
        return 768i32
    }
    return 0i32;
}
/*
    This function loads an image.
*/
#[no_mangle]
pub unsafe extern "C" fn ti68k_load_image(mut filename: *const cty::c_char)
 -> cty::c_int {
    let mut img: *mut IMG_INFO32 = &mut img_infos;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut err: cty::c_int = 0;
    // Clear infos
    memset(img as *mut cty::c_void, 0i32,
           ::core::mem::size_of::<IMG_INFO32>() as cty::c_ulong);
    // No filename, exits
    if 0 ==
           strcmp(g_basename(filename),
                  b"\x00" as *const u8 as *const cty::c_char) {
        return 768i32
    }
    // Load infos
    err = ti68k_get_img_infos(filename, img);
    if 0 != err {
        tiemu_info(b"Unable to get information on image: %s\x00" as *const u8
                       as *const cty::c_char, filename);
        return err
    }
    ti68k_display_img_infos(img);
    // Open file
    f = fopen(filename, b"rb\x00" as *const u8 as *const cty::c_char);
    if f.is_null() {
        tiemu_warning(b"Unable to open this file: <%s>\n\x00" as *const u8 as
                          *const cty::c_char, filename);
        return 768i32
    }
    // Read pure data
    if 0 != fseek(f, (*img).header_size as cty::c_long, 0i32) {
        tiemu_warning(b"Failed to read from file: <%s>\n\x00" as *const u8 as
                          *const cty::c_char, filename);
        fclose(f);
        return 768i32
    }
    (*img).data =
        malloc(((*img).size + 4i32) as cty::c_ulong) as *mut cty::c_char;
    if (*img).data.is_null() { return 776i32 }
    if fread((*img).data as *mut cty::c_void, 1i32 as cty::c_ulong,
             (*img).size as cty::c_ulong, f) < (*img).size as size_t {
        tiemu_warning(b"Failed to read from file: <%s>\n\x00" as *const u8 as
                          *const cty::c_char, filename);
        fclose(f);
        return 768i32
    }
    let mut hwblock: HW_PARM_BLOCK =
        HW_PARM_BLOCK{len: 0,
                      hardwareID: 0,
                      hardwareRevision: 0,
                      bootMajor: 0,
                      bootRevision: 0,
                      bootBuild: 0,
                      gateArray: 0,
                      physDisplayBitsWide: 0,
                      physDisplayBitsTall: 0,
                      LCDBitsWide: 0,
                      LCDBitsTall: 0,};
    ti68k_get_hw_param_block((*img).data as *mut uint8_t, (*img).rom_base,
                             &mut hwblock);
    ti68k_display_hw_param_block(&mut hwblock);
    if 0 != fclose(f) {
        tiemu_warning(b"Failed to close file: <%s>\n\x00" as *const u8 as
                          *const cty::c_char, filename);
        return 768i32
    }
    img_loaded = 1i32;
    img_changed = 1i32;
    return 0i32;
}
/*
    Unload an image (free memory).
*/
#[no_mangle]
pub unsafe extern "C" fn ti68k_unload_image_or_upgrade() -> cty::c_int {
    let mut img: *mut IMG_INFO32 = &mut img_infos;
    if 0 == img_loaded { return -1i32 }
    (*img).data = 0 as *mut cty::c_char;
    img_loaded = 0i32;
    return 0i32;
}
