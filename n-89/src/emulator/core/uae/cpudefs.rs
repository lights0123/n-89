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
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct instr_def {
    pub bits: cty::c_uint,
    pub n_variable: cty::c_int,
    pub bitpos: [cty::c_char; 16],
    pub mask: cty::c_uint,
    pub cpulevel: cty::c_int,
    pub plevel: cty::c_int,
    pub flaginfo: [C2RustUnnamed; 5],
    pub sduse: cty::c_uchar,
    pub opcstr: *const cty::c_char,
}
#[derive ( BitfieldStruct , Clone , Copy )]
#[repr(C)]
pub struct C2RustUnnamed {
    #[bitfield(name = "flaguse", ty = "cty::c_uint", bits = "0..=2")]
    #[bitfield(name = "flagset", ty = "cty::c_uint", bits = "3..=5")]
    pub flaguse_flagset: [u8; 1],
    #[bitfield(padding)]
    pub _pad: [u8; 3],
}
// Initialized in run_static_initializers
#[no_mangle]
pub static mut defs68k: [instr_def; 179] =
    [instr_def{bits: 0,
               n_variable: 0,
               bitpos: [0; 16],
               mask: 0,
               cpulevel: 0,
               plevel: 0,
               flaginfo:
                   [C2RustUnnamed{flaguse_flagset: [0; 1], _pad: [0; 3],}; 5],
               sduse: 0,
               opcstr: 0 as *const cty::c_char,}; 179];
#[no_mangle]
pub static mut n_defs68k: cty::c_int = 179i32;
pub unsafe extern "C" fn run_static_initializers() {
    defs68k =
        [instr_def{bits: 60i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"ORSR.B  #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 124i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"ORSR.W  #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 192i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [17i32 as cty::c_char, 17i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 63936i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"CHK2.z  #1,s[!Dreg,Areg,Aipi,Apdi,Immd]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 0i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [17i32 as cty::c_char, 17i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ORI.z   #z,d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 572i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"ANDSR.B #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 636i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"ANDSR.W #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 512i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [17i32 as cty::c_char, 17i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ANDI.z  #z,d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 1024i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [17i32 as cty::c_char, 17i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"SUBI.z  #z,d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 1536i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [17i32 as cty::c_char, 17i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ADDI.z  #z,d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 1728i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"CALLM   s[!Dreg,Areg,Aipi,Apdi,Immd]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 1728i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"RTM     s[Dreg,Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 2048i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"BTST    #1,s[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 2112i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"BCHG    #1,s[!Areg,Immd]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 2176i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"BCLR    #1,s[!Areg,Immd]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 2240i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"BSET    #1,s[!Areg,Immd]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 2620i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"EORSR.B #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 2684i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"EORSR.W #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 2560i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [17i32 as cty::c_char, 17i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"EORI.z  #z,d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 3072i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [17i32 as cty::c_char, 17i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"CMPI.z  #z,s[!Areg,Immd]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 2752i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"CAS.B   #1,s[!Dreg,Areg,Immd,PC8r,PC16]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 3264i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"CAS.W   #1,s[!Dreg,Areg,Immd,PC8r,PC16]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 3324i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"CAS2.W  #2\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 3584i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [17i32 as cty::c_char, 17i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"MOVES.z #1,s[!Dreg,Areg,Immd,PC8r,PC16]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 3776i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"CAS.L   #1,s[!Dreg,Areg,Immd,PC8r,PC16]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 3836i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"CAS2.L  #2\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 256i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MVPMR.W d[Areg-Ad16],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 320i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MVPMR.L d[Areg-Ad16],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 384i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MVPRM.W Dr,d[Areg-Ad16]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 448i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MVPRM.L Dr,d[Areg-Ad16]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 256i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"BTST    Dr,s[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 320i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"BCHG    Dr,s[!Areg,Immd]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 384i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"BCLR    Dr,s[!Areg,Immd]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 448i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"BSET    Dr,s[!Areg,Immd]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 4096i32 as cty::c_uint,
                   n_variable: 12i32,
                   bitpos:
                       [14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61440i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MOVE.B  s,d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 8192i32 as cty::c_uint,
                   n_variable: 12i32,
                   bitpos:
                       [14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61440i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MOVEA.L s,d[Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 8192i32 as cty::c_uint,
                   n_variable: 12i32,
                   bitpos:
                       [14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61440i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MOVE.L  s,d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 12288i32 as cty::c_uint,
                   n_variable: 12i32,
                   bitpos:
                       [14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61440i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MOVEA.W s,d[Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 12288i32 as cty::c_uint,
                   n_variable: 12i32,
                   bitpos:
                       [14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61440i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MOVE.W  s,d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 16384i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [17i32 as cty::c_char, 17i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(4i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(4i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 48i32 as cty::c_uchar,
                   opcstr:
                       b"NEGX.z  d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 16576i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 1i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"MVSR2.W d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 16896i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [17i32 as cty::c_char, 17i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(3i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 32i32 as cty::c_uchar,
                   opcstr:
                       b"CLR.z   d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 17088i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 1i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"MVSR2.B d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 17408i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [17i32 as cty::c_char, 17i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 48i32 as cty::c_uchar,
                   opcstr:
                       b"NEG.z   d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 17600i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"MV2SR.B s[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 17920i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [17i32 as cty::c_char, 17i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 48i32 as cty::c_uchar,
                   opcstr:
                       b"NOT.z   d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 18112i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"MV2SR.W s[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 18440i32 as cty::c_uint,
                   n_variable: 3i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65528i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 49i32 as cty::c_uchar,
                   opcstr:
                       b"LINK.L  Ar,#2\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 18432i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 48i32 as cty::c_uchar,
                   opcstr:
                       b"NBCD.B  d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 18504i32 as cty::c_uint,
                   n_variable: 3i32,
                   bitpos:
                       [9i32 as cty::c_char, 9i32 as cty::c_char,
                        9i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65528i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"BKPT    #k\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 18496i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 48i32 as cty::c_uchar,
                   opcstr:
                       b"SWAP.W  s[Dreg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 18496i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 0i32 as cty::c_uchar,
                   opcstr:
                       b"PEA.L   s[!Dreg,Areg,Aipi,Apdi,Immd]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 18560i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 48i32 as cty::c_uchar,
                   opcstr:
                       b"EXT.W   d[Dreg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 18560i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 2i32 as cty::c_uchar,
                   opcstr:
                       b"MVMLE.W #1,d[!Dreg,Areg,Aipi]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 18624i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 48i32 as cty::c_uchar,
                   opcstr:
                       b"EXT.L   d[Dreg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 18624i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 2i32 as cty::c_uchar,
                   opcstr:
                       b"MVMLE.L #1,d[!Dreg,Areg,Aipi]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 18880i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 48i32 as cty::c_uchar,
                   opcstr:
                       b"EXT.B   d[Dreg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 18944i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [17i32 as cty::c_char, 17i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"TST.z   s\x00" as *const u8 as *const cty::c_char,},
         instr_def{bits: 19136i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 48i32 as cty::c_uchar,
                   opcstr:
                       b"TAS.B   d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 19196i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 0i32 as cty::c_uchar,
                   opcstr:
                       b"ILLEGAL\x00" as *const u8 as *const cty::c_char,},
         instr_def{bits: 19456i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"MULL.L  #1,s[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 19520i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"DIVL.L  #1,s[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 19584i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 1i32 as cty::c_uchar,
                   opcstr:
                       b"MVMEL.W #1,s[!Dreg,Areg,Apdi,Immd]\x00" as *const u8
                           as *const cty::c_char,},
         instr_def{bits: 19648i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 1i32 as cty::c_uchar,
                   opcstr:
                       b"MVMEL.L #1,s[!Dreg,Areg,Apdi,Immd]\x00" as *const u8
                           as *const cty::c_char,},
         instr_def{bits: 20032i32 as cty::c_uint,
                   n_variable: 4i32,
                   bitpos:
                       [8i32 as cty::c_char, 8i32 as cty::c_char,
                        8i32 as cty::c_char, 8i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65520i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"TRAP    #J\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20048i32 as cty::c_uint,
                   n_variable: 3i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65528i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 49i32 as cty::c_uchar,
                   opcstr:
                       b"LINK.W  Ar,#1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20056i32 as cty::c_uint,
                   n_variable: 3i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65528i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 48i32 as cty::c_uchar,
                   opcstr:
                       b"UNLK.L  Ar\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20064i32 as cty::c_uint,
                   n_variable: 3i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65528i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"MVR2USP.L Ar\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20072i32 as cty::c_uint,
                   n_variable: 3i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65528i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 32i32 as cty::c_uchar,
                   opcstr:
                       b"MVUSP2R.L Ar\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20080i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 0i32 as cty::c_uchar,
                   opcstr: b"RESET\x00" as *const u8 as *const cty::c_char,},
         instr_def{bits: 20081i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 0i32 as cty::c_uchar,
                   opcstr: b"NOP\x00" as *const u8 as *const cty::c_char,},
         instr_def{bits: 20082i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"STOP    #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20083i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 0i32 as cty::c_uchar,
                   opcstr: b"RTE\x00" as *const u8 as *const cty::c_char,},
         instr_def{bits: 20084i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"RTD     #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20085i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 0i32 as cty::c_uchar,
                   opcstr: b"RTS\x00" as *const u8 as *const cty::c_char,},
         instr_def{bits: 20086i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 0i32 as cty::c_uchar,
                   opcstr: b"TRAPV\x00" as *const u8 as *const cty::c_char,},
         instr_def{bits: 20087i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 0i32 as cty::c_uchar,
                   opcstr: b"RTR\x00" as *const u8 as *const cty::c_char,},
         instr_def{bits: 20090i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 1i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"MOVEC2  #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20091i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 1i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"MOVE2C  #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20096i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        }],
                   sduse: 128i32 as cty::c_uchar,
                   opcstr:
                       b"JSR.L   s[!Dreg,Areg,Aipi,Apdi,Immd]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 16640i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"CHK.L   s[!Areg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 16768i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"CHK.W   s[!Areg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20160i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        }],
                   sduse: 128i32 as cty::c_uchar,
                   opcstr:
                       b"JMP.L   s[!Dreg,Areg,Aipi,Apdi,Immd]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 16832i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 2i32 as cty::c_uchar,
                   opcstr:
                       b"LEA.L   s[!Dreg,Areg,Aipi,Apdi,Immd],Ar\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 20544i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [7i32 as cty::c_char, 7i32 as cty::c_char,
                        7i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ADDA.W  #j,d[Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20608i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [7i32 as cty::c_char, 7i32 as cty::c_char,
                        7i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ADDA.L  #j,d[Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20480i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [7i32 as cty::c_char, 7i32 as cty::c_char,
                        7i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ADDQ.z  #j,d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20800i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [7i32 as cty::c_char, 7i32 as cty::c_char,
                        7i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"SUBA.W  #j,d[Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20864i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [7i32 as cty::c_char, 7i32 as cty::c_char,
                        7i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"SUBA.L  #j,d[Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20736i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [7i32 as cty::c_char, 7i32 as cty::c_char,
                        7i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"SUBQ.z  #j,d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20680i32 as cty::c_uint,
                   n_variable: 7i32,
                   bitpos:
                       [2i32 as cty::c_char, 2i32 as cty::c_char,
                        2i32 as cty::c_char, 2i32 as cty::c_char,
                        15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61688i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 49i32 as cty::c_uchar,
                   opcstr:
                       b"DBcc.W  Dr,#1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20672i32 as cty::c_uint,
                   n_variable: 10i32,
                   bitpos:
                       [2i32 as cty::c_char, 2i32 as cty::c_char,
                        2i32 as cty::c_char, 2i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61632i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 32i32 as cty::c_uchar,
                   opcstr:
                       b"Scc.B   d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20730i32 as cty::c_uint,
                   n_variable: 4i32,
                   bitpos:
                       [2i32 as cty::c_char, 2i32 as cty::c_char,
                        2i32 as cty::c_char, 2i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61695i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"TRAPcc  #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20731i32 as cty::c_uint,
                   n_variable: 4i32,
                   bitpos:
                       [2i32 as cty::c_char, 2i32 as cty::c_char,
                        2i32 as cty::c_char, 2i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61695i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"TRAPcc  #2\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 20732i32 as cty::c_uint,
                   n_variable: 4i32,
                   bitpos:
                       [2i32 as cty::c_char, 2i32 as cty::c_char,
                        2i32 as cty::c_char, 2i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61695i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 0i32 as cty::c_uchar,
                   opcstr:
                       b"TRAPcc\x00" as *const u8 as *const cty::c_char,},
         instr_def{bits: 24832i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        }],
                   sduse: 64i32 as cty::c_uchar,
                   opcstr:
                       b"BSR.W   #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 24832i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [6i32 as cty::c_char, 6i32 as cty::c_char,
                        6i32 as cty::c_char, 6i32 as cty::c_char,
                        6i32 as cty::c_char, 6i32 as cty::c_char,
                        6i32 as cty::c_char, 6i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        }],
                   sduse: 64i32 as cty::c_uchar,
                   opcstr:
                       b"BSR.B   #i\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 25087i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(4i32 as cty::c_uint);
                            init.set_flagset(6i32 as cty::c_uint);
                            init
                        }],
                   sduse: 64i32 as cty::c_uchar,
                   opcstr:
                       b"BSR.L   #2\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 24576i32 as cty::c_uint,
                   n_variable: 4i32,
                   bitpos:
                       [3i32 as cty::c_char, 3i32 as cty::c_char,
                        3i32 as cty::c_char, 3i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61695i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 64i32 as cty::c_uchar,
                   opcstr:
                       b"Bcc.W   #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 24576i32 as cty::c_uint,
                   n_variable: 12i32,
                   bitpos:
                       [3i32 as cty::c_char, 3i32 as cty::c_char,
                        3i32 as cty::c_char, 3i32 as cty::c_char,
                        6i32 as cty::c_char, 6i32 as cty::c_char,
                        6i32 as cty::c_char, 6i32 as cty::c_char,
                        6i32 as cty::c_char, 6i32 as cty::c_char,
                        6i32 as cty::c_char, 6i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61440i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 64i32 as cty::c_uchar,
                   opcstr:
                       b"Bcc.B   #i\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 24831i32 as cty::c_uint,
                   n_variable: 4i32,
                   bitpos:
                       [3i32 as cty::c_char, 3i32 as cty::c_char,
                        3i32 as cty::c_char, 3i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61695i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(2i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 64i32 as cty::c_uchar,
                   opcstr:
                       b"Bcc.L   #2\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 28672i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 5i32 as cty::c_char,
                        5i32 as cty::c_char, 5i32 as cty::c_char,
                        5i32 as cty::c_char, 5i32 as cty::c_char,
                        5i32 as cty::c_char, 5i32 as cty::c_char,
                        5i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MOVEQ   #i,Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 32768i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"OR.z    s[!Areg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 32960i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"DIVU.W  s[!Areg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 33024i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(4i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(4i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"SBCD.B  d[Dreg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 33024i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(4i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(4i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"SBCD.B  d[Areg-Apdi],Arp\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 33024i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"OR.z    Dr,d[!Areg,Dreg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 33088i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"PACK    d[Dreg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 33088i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"PACK    d[Areg-Apdi],Arp\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 33152i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"UNPK    d[Dreg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 33152i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"UNPK    d[Areg-Apdi],Arp\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 33216i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"DIVS.W  s[!Areg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 36864i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"SUB.z   s,Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 37056i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"SUBA.W  s,Ar\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 37120i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"SUBX.z  d[Dreg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 37120i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"SUBX.z  d[Areg-Apdi],Arp\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 37120i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"SUB.z   Dr,d[!Areg,Dreg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 37312i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"SUBA.L  s,Ar\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 45056i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"CMP.z   s,Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 45248i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"CMPA.W  s,Ar\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 45504i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"CMPA.L  s,Ar\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 45312i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"CMPM.z  d[Areg-Aipi],ArP\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 45312i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"EOR.z   Dr,d[!Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 49152i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"AND.z   s[!Areg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 49344i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"MULU.W  s[!Areg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 49408i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(4i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(4i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ABCD.B  d[Dreg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 49408i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(4i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(4i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ABCD.B  d[Areg-Apdi],Arp\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 49408i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"AND.z   Dr,d[!Areg,Dreg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 49472i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 51i32 as cty::c_uchar,
                   opcstr:
                       b"EXG.L   Dr,d[Dreg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 49472i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 51i32 as cty::c_uchar,
                   opcstr:
                       b"EXG.L   Ar,d[Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 49536i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 51i32 as cty::c_uchar,
                   opcstr:
                       b"EXG.L   Dr,d[Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 49600i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"MULS.W  s[!Areg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 53248i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ADD.z   s,Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 53440i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ADDA.W  s,Ar\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 53504i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ADDX.z  d[Dreg],Dr\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 53504i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ADDX.z  d[Areg-Apdi],Arp\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 53504i32 as cty::c_uint,
                   n_variable: 11i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 17i32 as cty::c_char,
                        17i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61696i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ADD.z   Dr,d[!Areg,Dreg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 53696i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 11i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61888i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ADDA.L  s,Ar\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 57344i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [7i32 as cty::c_char, 7i32 as cty::c_char,
                        7i32 as cty::c_char, 4i32 as cty::c_char,
                        17i32 as cty::c_char, 17i32 as cty::c_char,
                        16i32 as cty::c_char, 16i32 as cty::c_char,
                        16i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61496i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ASf.z   #j,DR\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 57352i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [7i32 as cty::c_char, 7i32 as cty::c_char,
                        7i32 as cty::c_char, 4i32 as cty::c_char,
                        17i32 as cty::c_char, 17i32 as cty::c_char,
                        16i32 as cty::c_char, 16i32 as cty::c_char,
                        16i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61496i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"LSf.z   #j,DR\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 57360i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [7i32 as cty::c_char, 7i32 as cty::c_char,
                        7i32 as cty::c_char, 4i32 as cty::c_char,
                        17i32 as cty::c_char, 17i32 as cty::c_char,
                        16i32 as cty::c_char, 16i32 as cty::c_char,
                        16i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61496i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ROXf.z  #j,DR\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 57368i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [7i32 as cty::c_char, 7i32 as cty::c_char,
                        7i32 as cty::c_char, 4i32 as cty::c_char,
                        17i32 as cty::c_char, 17i32 as cty::c_char,
                        16i32 as cty::c_char, 16i32 as cty::c_char,
                        16i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61496i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ROf.z   #j,DR\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 57376i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 4i32 as cty::c_char,
                        17i32 as cty::c_char, 17i32 as cty::c_char,
                        16i32 as cty::c_char, 16i32 as cty::c_char,
                        16i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61496i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ASf.z   Dr,DR\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 57384i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 4i32 as cty::c_char,
                        17i32 as cty::c_char, 17i32 as cty::c_char,
                        16i32 as cty::c_char, 16i32 as cty::c_char,
                        16i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61496i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"LSf.z   Dr,DR\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 57392i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 4i32 as cty::c_char,
                        17i32 as cty::c_char, 17i32 as cty::c_char,
                        16i32 as cty::c_char, 16i32 as cty::c_char,
                        16i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61496i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ROXf.z  Dr,DR\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 57400i32 as cty::c_uint,
                   n_variable: 9i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 4i32 as cty::c_char,
                        17i32 as cty::c_char, 17i32 as cty::c_char,
                        16i32 as cty::c_char, 16i32 as cty::c_char,
                        16i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 61496i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ROf.z   Dr,DR\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 57536i32 as cty::c_uint,
                   n_variable: 7i32,
                   bitpos:
                       [4i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65216i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ASfW.W  d[!Dreg,Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 58048i32 as cty::c_uint,
                   n_variable: 7i32,
                   bitpos:
                       [4i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65216i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"LSfW.W  d[!Dreg,Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 58560i32 as cty::c_uint,
                   n_variable: 7i32,
                   bitpos:
                       [4i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65216i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(0i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ROXfW.W d[!Dreg,Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 59072i32 as cty::c_uint,
                   n_variable: 7i32,
                   bitpos:
                       [4i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 13i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65216i32 as cty::c_uint,
                   cpulevel: 0i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(2i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(0i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"ROfW.W  d[!Dreg,Areg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 59584i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"BFTST   #1,s[!Areg,Apdi,Aipi,Immd]\x00" as *const u8
                           as *const cty::c_char,},
         instr_def{bits: 59840i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"BFEXTU  #1,s[!Areg,Apdi,Aipi,Immd]\x00" as *const u8
                           as *const cty::c_char,},
         instr_def{bits: 60096i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"BFCHG   #1,s[!Areg,Apdi,Aipi,Immd,PC8r,PC16]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 60352i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"BFEXTS  #1,s[!Areg,Apdi,Aipi,Immd]\x00" as *const u8
                           as *const cty::c_char,},
         instr_def{bits: 60608i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"BFCLR   #1,s[!Areg,Apdi,Aipi,Immd,PC8r,PC16]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 60864i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"BFFFO   #1,s[!Areg,Apdi,Aipi,Immd]\x00" as *const u8
                           as *const cty::c_char,},
         instr_def{bits: 61120i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"BFSET   #1,s[!Areg,Apdi,Aipi,Immd,PC8r,PC16]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 61376i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 2i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 19i32 as cty::c_uchar,
                   opcstr:
                       b"BFINS   #1,s[!Areg,Apdi,Aipi,Immd,PC8r,PC16]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 61952i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 3i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"FPP      #1,s\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62016i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 3i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"FDBcc    #1,s[Areg-Dreg]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62016i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 3i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"FScc     #1,s[!Areg,Immd,PC8r,PC16]\x00" as *const u8
                           as *const cty::c_char,},
         instr_def{bits: 62074i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 3i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"FTRAPcc  #1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62075i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 3i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"FTRAPcc  #2\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62076i32 as cty::c_uint,
                   n_variable: 0i32,
                   bitpos:
                       [0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65535i32 as cty::c_uint,
                   cpulevel: 3i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 0i32 as cty::c_uchar,
                   opcstr:
                       b"FTRAPcc\x00" as *const u8 as *const cty::c_char,},
         instr_def{bits: 62080i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [10i32 as cty::c_char, 10i32 as cty::c_char,
                        10i32 as cty::c_char, 10i32 as cty::c_char,
                        10i32 as cty::c_char, 10i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 3i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"FBcc     #K,#1\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62144i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [10i32 as cty::c_char, 10i32 as cty::c_char,
                        10i32 as cty::c_char, 10i32 as cty::c_char,
                        10i32 as cty::c_char, 10i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 3i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"FBcc     #K,#2\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62208i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 3i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 32i32 as cty::c_uchar,
                   opcstr:
                       b"FSAVE    s[!Dreg,Areg,Aipi,Immd,PC8r,PC16]\x00" as
                           *const u8 as *const cty::c_char,},
         instr_def{bits: 62272i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 3i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 16i32 as cty::c_uchar,
                   opcstr:
                       b"FRESTORE s[!Dreg,Areg,Apdi,Immd]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62720i32 as cty::c_uint,
                   n_variable: 8i32,
                   bitpos:
                       [5i32 as cty::c_char, 5i32 as cty::c_char,
                        5i32 as cty::c_char, 5i32 as cty::c_char,
                        5i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65280i32 as cty::c_uint,
                   cpulevel: 4i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(3i32 as cty::c_uint);
                            init.set_flagset(5i32 as cty::c_uint);
                            init
                        }],
                   sduse: 17i32 as cty::c_uchar,
                   opcstr:
                       b"MMUOP    #i,s\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62472i32 as cty::c_uint,
                   n_variable: 5i32,
                   bitpos:
                       [18i32 as cty::c_char, 18i32 as cty::c_char,
                        15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65336i32 as cty::c_uint,
                   cpulevel: 4i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 2i32 as cty::c_uchar,
                   opcstr:
                       b"CINVL    #p,Ar\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62480i32 as cty::c_uint,
                   n_variable: 5i32,
                   bitpos:
                       [18i32 as cty::c_char, 18i32 as cty::c_char,
                        15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65336i32 as cty::c_uint,
                   cpulevel: 4i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 2i32 as cty::c_uchar,
                   opcstr:
                       b"CINVP    #p,Ar\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62488i32 as cty::c_uint,
                   n_variable: 5i32,
                   bitpos:
                       [18i32 as cty::c_char, 18i32 as cty::c_char,
                        15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65336i32 as cty::c_uint,
                   cpulevel: 4i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 0i32 as cty::c_uchar,
                   opcstr:
                       b"CINVA    #p\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62504i32 as cty::c_uint,
                   n_variable: 5i32,
                   bitpos:
                       [18i32 as cty::c_char, 18i32 as cty::c_char,
                        15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65336i32 as cty::c_uint,
                   cpulevel: 4i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 2i32 as cty::c_uchar,
                   opcstr:
                       b"CPUSHL   #p,Ar\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62512i32 as cty::c_uint,
                   n_variable: 5i32,
                   bitpos:
                       [18i32 as cty::c_char, 18i32 as cty::c_char,
                        15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65336i32 as cty::c_uint,
                   cpulevel: 4i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 2i32 as cty::c_uchar,
                   opcstr:
                       b"CPUSHP   #p,Ar\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62520i32 as cty::c_uint,
                   n_variable: 5i32,
                   bitpos:
                       [18i32 as cty::c_char, 18i32 as cty::c_char,
                        15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65336i32 as cty::c_uint,
                   cpulevel: 4i32,
                   plevel: 2i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 0i32 as cty::c_uchar,
                   opcstr:
                       b"CPUSHA   #p\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 63008i32 as cty::c_uint,
                   n_variable: 3i32,
                   bitpos:
                       [15i32 as cty::c_char, 15i32 as cty::c_char,
                        15i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65528i32 as cty::c_uint,
                   cpulevel: 4i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MOVE16   ArP,AxP\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62976i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 4i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MOVE16   s[Dreg-Aipi],L\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62976i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 4i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MOVE16   L,d[Areg-Aipi]\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62976i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [11i32 as cty::c_char, 11i32 as cty::c_char,
                        11i32 as cty::c_char, 12i32 as cty::c_char,
                        12i32 as cty::c_char, 12i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 4i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MOVE16   s[Aind],L\x00" as *const u8 as
                           *const cty::c_char,},
         instr_def{bits: 62976i32 as cty::c_uint,
                   n_variable: 6i32,
                   bitpos:
                       [13i32 as cty::c_char, 13i32 as cty::c_char,
                        13i32 as cty::c_char, 14i32 as cty::c_char,
                        14i32 as cty::c_char, 14i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char,
                        0i32 as cty::c_char, 0i32 as cty::c_char],
                   mask: 65472i32 as cty::c_uint,
                   cpulevel: 4i32,
                   plevel: 0i32,
                   flaginfo:
                       [{
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        },
                        {
                            let mut init =
                                C2RustUnnamed{flaguse_flagset: [0; 1],
                                              _pad: [0; 3],};
                            init.set_flaguse(1i32 as cty::c_uint);
                            init.set_flagset(1i32 as cty::c_uint);
                            init
                        }],
                   sduse: 18i32 as cty::c_uchar,
                   opcstr:
                       b"MOVE16   L,d[Aipi-Aind]\x00" as *const u8 as
                           *const cty::c_char,}]
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
