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
    fn sprintf(_: *mut cty::c_char, _: *const cty::c_char, _: ...)
     -> cty::c_int;
}
pub type __uint16_t = cty::c_ushort;
pub type uint16_t = __uint16_t;
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct TUPLE {
    pub code: uint16_t,
    pub name: *mut cty::c_char,
}
// 6 chars max
#[no_mangle]
pub static mut operators: [TUPLE; 9] =
    [TUPLE{code: 0i32 as uint16_t,
           name:
               b"FCMP\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x1000i32 as uint16_t,
           name:
               b"FADD\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x2000i32 as uint16_t,
           name:
               b"FDIV\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x3000i32 as uint16_t,
           name:
               b"FMUL\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x4000i32 as uint16_t,
           name:
               b"FSUB\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x5000i32 as uint16_t,
           name:
               b"FINTRZ\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x6000i32 as uint16_t,
           name:
               b"FMOVE\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x7000i32 as uint16_t,
           name:
               b"FNEG\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x8000i32 as uint16_t,
           name:
               b"FTST\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,}];
// 7 chars max
#[no_mangle]
pub static mut sizes: [TUPLE; 6] =
    [TUPLE{code: 0i32 as uint16_t,
           name:
               b"BYTE\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x200i32 as uint16_t,
           name:
               b"WORD\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x400i32 as uint16_t,
           name:
               b"LONG\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x600i32 as uint16_t,
           name:
               b"SINGLE\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x800i32 as uint16_t,
           name:
               b"DOUBLE\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0xa00i32 as uint16_t,
           name:
               b"UNSGNED\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,}];
// 11 chars max
#[no_mangle]
pub static mut srcs: [TUPLE; 21] =
    [TUPLE{code: 0i32 as uint16_t,
           name:
               b"FP0\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x10i32 as uint16_t,
           name:
               b"FP1\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x20i32 as uint16_t,
           name:
               b"FP2\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x30i32 as uint16_t,
           name:
               b"FP3\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x40i32 as uint16_t,
           name:
               b"FP4\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x50i32 as uint16_t,
           name:
               b"FP5\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x60i32 as uint16_t,
           name:
               b"FP6\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x70i32 as uint16_t,
           name:
               b"FP7\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x80i32 as uint16_t,
           name:
               b"D0\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x90i32 as uint16_t,
           name:
               b"D1\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0xa0i32 as uint16_t,
           name:
               b"D2\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0xb0i32 as uint16_t,
           name:
               b"D3\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0xc0i32 as uint16_t,
           name:
               b"D4\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0xd0i32 as uint16_t,
           name:
               b"D5\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0xe0i32 as uint16_t,
           name:
               b"D6\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0xf0i32 as uint16_t,
           name:
               b"D7\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x100i32 as uint16_t,
           name:
               b"IMMED_LONG\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x110i32 as uint16_t,
           name:
               b"IMMED_SHORT\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x120i32 as uint16_t,
           name:
               b"FRAME_OFF\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x130i32 as uint16_t,
           name:
               b"EFFECT_ADDR\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x140i32 as uint16_t,
           name:
               b"IMMED_ZERO\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,}];
// 11 chars max
#[no_mangle]
pub static mut dsts: [TUPLE; 11] =
    [TUPLE{code: 0i32 as uint16_t,
           name:
               b"R0\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x1i32 as uint16_t,
           name:
               b"R1\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x2i32 as uint16_t,
           name:
               b"R2\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x3i32 as uint16_t,
           name:
               b"R3\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x4i32 as uint16_t,
           name:
               b"R4\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x5i32 as uint16_t,
           name:
               b"R5\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x6i32 as uint16_t,
           name:
               b"R6\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x7i32 as uint16_t,
           name:
               b"R7\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x8i32 as uint16_t,
           name:
               b"FRAME_OFF\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0x9i32 as uint16_t,
           name:
               b"EFFECT_ADDR\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,},
     TUPLE{code: 0xai32 as uint16_t,
           name:
               b"RETURN_REG\x00" as *const u8 as *const cty::c_char as
                   *mut cty::c_char,}];
/*
; BCD arithmetic package

***************************************************************************
*                        OPERATOR / OPERAND WORD                          *
*                                                                         *
*    | 15| 14| 13| 12| 11| 10| 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |    *
*    +---------------+-----------+-------------------+---------------+    *
*    |   OPERATOR    |   SIZE    |    SRC OPERAND    |  DEST OPERAND |    *
*    +---------------+-----------+-------------------+---------------+    *
*        FCMP   0      BYTE    0     FP0          0    R0 (FP or D) 0     *
*        FADD   1      WORD    1      |                 |                 *
*        FDIV   2      LONG    2     FP7          7    R7 (FP or D) 7     *
*        FMUL   3      SINGLE  3     D0           8    FRAME_OFF    8     *
*        FSUB   4      DOUBLE  4      |                EFFECT_ADDR  9     *
*        FINTRZ 5      UNSGNED 5     D7          15    RETURN_REG  10     *
*        FMOVE  6                    IMMED_LONG  16                       *
*        FNEG   7                    IMMED_SHORT 17                       *
*        FTST   8                    FRAME_OFF   18                       *
*                                    EFFECT_ADDR 19                       *
*                                    IMMED_ZERO  20                       *
***************************************************************************
bcdCmp          = 0x0000
bcdAdd          = 0x1000
bcdDiv          = 0x2000
bcdMul          = 0x3000
bcdSub          = 0x4000
bcdIntz         = 0x5000
bcdMove         = 0x6000
bcdNeg          = 0x7000
bcdTst          = 0x8000

bcdByte         = 0x0000
bcdWord         = 0x0200
bcdLong         = 0x0400
bcdSingle       = 0x0600
bcdDouble       = 0x0800
bcdUnsigned     = 0x0A00

; Source operand
bcdFP0          = 0x0000
bcdFP1          = 0x0010
bcdFP2          = 0x0020
bcdFP3          = 0x0030
bcdFP4          = 0x0040
bcdFP5          = 0x0050
bcdFP6          = 0x0060
bcdFP7          = 0x0070
bcdD0           = 0x0080
bcdD1           = 0x0090
bcdD2           = 0x00A0
bcdD3           = 0x00B0
bcdD4           = 0x00C0
bcdD5           = 0x00D0
bcdD6           = 0x00E0
bcdD7           = 0x00F0
bcdLongImm      = 0x0100
bcdShortImm     = 0x0110
bcdFrameSrc     = 0x0120
bcdAbsSrc       = 0x0130
bcdZeroImm      = 0x0140

; Destination operand
bcdR0           = 0x0000
bcdR1           = 0x0001
bcdR2           = 0x0002
bcdR3           = 0x0003
bcdR4           = 0x0004
bcdR5           = 0x0005
bcdR6           = 0x0006
bcdR7           = 0x0007
bcdFrameDest    = 0x0008
bcdAbsDest      = 0x0009
bcdRetReg       = 0x000A
*/
/*
    Input: FPU opcode in 'code'
    Output: FPU disassembled in 'buf'. sizeof(buf) >= 6+1+7+1+11+1+11 = 38
*/
#[no_mangle]
pub unsafe extern "C" fn DasmFPU(mut code: uint16_t,
                                 mut buf: *mut cty::c_char) -> cty::c_int {
    let mut operator: cty::c_int = code as cty::c_int & 0xf000i32;
    let mut size: cty::c_int = code as cty::c_int & 0xe00i32;
    let mut src: cty::c_int = code as cty::c_int & 0x1f0i32;
    let mut dst: cty::c_int = code as cty::c_int & 0xfi32;
    let mut idx: [cty::c_int; 4] = [0i32, 0, 0, 0];
    let mut i: cty::c_int = 0;
    let mut j: cty::c_int = 0i32;
    i = 0i32;
    while (i as cty::c_ulong) <
              (::core::mem::size_of::<[TUPLE; 9]>() as
                   cty::c_ulong).wrapping_div(::core::mem::size_of::<TUPLE>()
                                                   as cty::c_ulong) {
        if operators[i as usize].code as cty::c_int == operator {
            let fresh0 = j;
            j = j + 1;
            idx[fresh0 as usize] = i;
            break ;
        } else { i += 1 }
    }
    i = 0i32;
    while (i as cty::c_ulong) <
              (::core::mem::size_of::<[TUPLE; 6]>() as
                   cty::c_ulong).wrapping_div(::core::mem::size_of::<TUPLE>()
                                                   as cty::c_ulong) {
        if sizes[i as usize].code as cty::c_int == size {
            let fresh1 = j;
            j = j + 1;
            idx[fresh1 as usize] = i;
            break ;
        } else { i += 1 }
    }
    i = 0i32;
    while (i as cty::c_ulong) <
              (::core::mem::size_of::<[TUPLE; 21]>() as
                   cty::c_ulong).wrapping_div(::core::mem::size_of::<TUPLE>()
                                                   as cty::c_ulong) {
        if srcs[i as usize].code as cty::c_int == src {
            let fresh2 = j;
            j = j + 1;
            idx[fresh2 as usize] = i;
            break ;
        } else { i += 1 }
    }
    i = 0i32;
    while (i as cty::c_ulong) <
              (::core::mem::size_of::<[TUPLE; 11]>() as
                   cty::c_ulong).wrapping_div(::core::mem::size_of::<TUPLE>()
                                                   as cty::c_ulong) {
        if dsts[i as usize].code as cty::c_int == dst {
            let fresh3 = j;
            j = j + 1;
            idx[fresh3 as usize] = i;
            break ;
        } else { i += 1 }
    }
    sprintf(buf, b"%s.%s %s,%s\x00" as *const u8 as *const cty::c_char,
            operators[idx[0usize] as usize].name,
            sizes[idx[1usize] as usize].name, srcs[idx[2usize] as usize].name,
            dsts[idx[3usize] as usize].name);
    return 0i32;
}
/*

Et sinon, il y a un autre cas particulier bizarre, je ne sais pas
s'il vaut le coup de le tra�ter, mais _bcd_math est un ROM_CALL qui s'appelle comme �a:
- n'importe quelle m�thode d'appel d'un ROM_CALL (jsr, F-Line etc.)
- 2 octets: opcode FPU
et le ROM_CALL saute ces 2 octets en retournant, donc retourne � next_pc+2. Si tu veux
g�rer �a, tu devrais regarder le target du jsr ou F-Line et comparer avec l'adresse de
_bcd_math.
Pr�cisions:
- la "FPU" est purement �mul�e en logiciel par _bcd_math, et les opcodes ne correspondent
� aucune FPU r�elle
- Pour le F-Line, tu peux comparer l'opcode avec 0xF8B5, mais pour les appels par jsr,
tu n'as pas d'autre choix que de tester le target du saut pour savoir si on saute vers
_bcd_math ou aillers

Il y en a un dans TIGCCLIB:

__floatunssibf:
 link.w %a6,#-36
 pea.l 8(%a6)
 move.l 0xC8,%a0
 move.l 0xB5*4(%a0),%a0 // _bcd_math
 jsr (%a0)
 .word 0x6B30 // bcdMove | bcdUnsigned | bcdAbsSrc | bcdR0
 move.l -10(%a6),%d0
 move.l -6(%a6),%d1
 move.w -2(%a6),%d2
 unlk %a6
 rts

Le .word 0x6B30 est le code de la pseudo-FPU dont je parle. Et ce n'est pas
ex�cut� par le processeur, _bcd_math rajoute 2 octets � l'adresse de retour.

*/
