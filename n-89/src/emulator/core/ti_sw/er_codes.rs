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
/* Hey EMACS -*- linux-c -*- */
/* $Id: er_codes.c 2268 2006-11-06 17:18:51Z roms $ */
/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2005, Kevin Kofler
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
    ER_throw support
*/
/* This table is converted and hand-edited from the TIGCC error.h. */
#[derive ( Copy , Clone )]
#[repr(C)]
pub struct C2RustUnnamed {
    pub id: cty::c_uint,
    pub name: *const cty::c_char,
}
static mut erthrow_table: [C2RustUnnamed; 126] =
    [C2RustUnnamed{id: 0i32 as cty::c_uint,
                   name: b"ER_OK\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 1i32 as cty::c_uint,
                   name: b"ER_EXIT\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 2i32 as cty::c_uint,
                   name: b"ER_STOP\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 3i32 as cty::c_uint,
                   name: b"ER_OFF\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 4i32 as cty::c_uint,
                   name:
                       b"ER_PRGM_STOP\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 9i32 as cty::c_uint,
                   name:
                       b"ER_NO_MSG\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 10i32 as cty::c_uint,
                   name:
                       b"ER_FUNC_DID_NOT_RETURN_VALUE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 20i32 as cty::c_uint,
                   name:
                       b"ER_TEST_NOT_TRUE_OR_FALSE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 30i32 as cty::c_uint,
                   name:
                       b"ER_ARG_CANNOT_BE_FOLDER\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 40i32 as cty::c_uint,
                   name:
                       b"ER_ARGUMENT\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 50i32 as cty::c_uint,
                   name:
                       b"ER_ARG_MISMATCH\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 60i32 as cty::c_uint,
                   name:
                       b"ER_EXPECTED_BOOL_OR_AGG\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 70i32 as cty::c_uint,
                   name:
                       b"ER_ARG_MUST_BE_DECIMAL\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 80i32 as cty::c_uint,
                   name:
                       b"ER_ARG_MUST_BE_LABEL\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 90i32 as cty::c_uint,
                   name:
                       b"ER_ARGUMENT_MUST_BE_LIST\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 100i32 as cty::c_uint,
                   name:
                       b"ER_ARG_MUST_BE_MATRIX\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 110i32 as cty::c_uint,
                   name:
                       b"ER_ARG_MUST_BE_PIC\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 120i32 as cty::c_uint,
                   name:
                       b"ER_ARG_MUST_BE_PIC_OR_STR\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 130i32 as cty::c_uint,
                   name:
                       b"ER_ARG_MUST_BE_STRING\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 140i32 as cty::c_uint,
                   name:
                       b"ER_EXPECTED_VAR\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 150i32 as cty::c_uint,
                   name:
                       b"ER_ARG_MUST_BE_EMPTY_FOLDER\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 160i32 as cty::c_uint,
                   name:
                       b"ER_EXPECTED_ALGEBRAIC\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 161i32 as cty::c_uint,
                   name:
                       b"ER_ASAP_TOO_LONG\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 163i32 as cty::c_uint,
                   name:
                       b"ER_ATTRIBUTE_NOT_FOUND\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 165i32 as cty::c_uint,
                   name:
                       b"ER_BATT_LOW\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 170i32 as cty::c_uint,
                   name:
                       b"ER_BOUND\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 180i32 as cty::c_uint,
                   name:
                       b"ER_BREAK\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 185i32 as cty::c_uint,
                   name:
                       b"ER_CHECKSUM\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 190i32 as cty::c_uint,
                   name:
                       b"ER_CIRCULAR_DEFINITION\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 200i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_SUCH_THAT\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 210i32 as cty::c_uint,
                   name:
                       b"ER_DATATYPE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 220i32 as cty::c_uint,
                   name:
                       b"ER_DEPENDENT_LIMIT\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 225i32 as cty::c_uint,
                   name:
                       b"ER_DIFF_EQ_SETUP\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 230i32 as cty::c_uint,
                   name:
                       b"ER_DIMENSION\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 240i32 as cty::c_uint,
                   name:
                       b"ER_NON_CONFORMING_LISTS\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 250i32 as cty::c_uint,
                   name:
                       b"ER_DIVBY0\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 260i32 as cty::c_uint,
                   name:
                       b"ER_DOMAIN\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 270i32 as cty::c_uint,
                   name:
                       b"ER_DUPLICATE_VAR_NAME\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 280i32 as cty::c_uint,
                   name:
                       b"ER_ELSE_WITHOUT_IF\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 290i32 as cty::c_uint,
                   name:
                       b"ER_ENDTRY_WITHOUT_ELSE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 295i32 as cty::c_uint,
                   name:
                       b"ER_EXCESSIVE_ITERATION\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 300i32 as cty::c_uint,
                   name:
                       b"ER_EXPECTED_2OR3_ELEMENTS\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 305i32 as cty::c_uint,
                   name:
                       b"ER_EXPIRED\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 307i32 as cty::c_uint,
                   name:
                       b"ER_APP_EXT_NOT_FOUND\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 308i32 as cty::c_uint,
                   name:
                       b"ER_APP_NOT_FOUND\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 310i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_NSOLVE_ARG1\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 320i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_SOLVE_ARG1\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 330i32 as cty::c_uint,
                   name:
                       b"ER_FOLDER\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 335i32 as cty::c_uint,
                   name:
                       b"ER_FUNCS_IN_DIFF_EQ\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 345i32 as cty::c_uint,
                   name:
                       b"ER_INCONSISTENT_UNITS\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 350i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_SUBSCRIPT\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 360i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_INDIR_STRING\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 380i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_ANS\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 390i32 as cty::c_uint,
                   name:
                       b"ER_ILLEGAL_ASSIGNMENT\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 400i32 as cty::c_uint,
                   name:
                       b"ER_ILLEGAL_ASSIGNMENT_VALUE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 405i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_AXES\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 410i32 as cty::c_uint,
                   name:
                       b"ER_ILLEGAL_COMMAND\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 420i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_FOLDER_NAME\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 430i32 as cty::c_uint,
                   name:
                       b"ER_GRAPH_MODE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 435i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_GUESS\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 440i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_IMPLIED_MULT\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 450i32 as cty::c_uint,
                   name:
                       b"ER_ILLEGAL_IN_FUNC\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 460i32 as cty::c_uint,
                   name:
                       b"ER_ILLEGAL_IN_CUSTOM\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 470i32 as cty::c_uint,
                   name:
                       b"ER_ILLEGAL_IN_DIALOG\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 480i32 as cty::c_uint,
                   name:
                       b"ER_ILLEGAL_IN_TOOLBAR\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 490i32 as cty::c_uint,
                   name:
                       b"ER_CANNOT_EXIT_FROM_TRY\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 500i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_LABEL\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 510i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_LIST_OR_MATRIX\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 520i32 as cty::c_uint,
                   name:
                       b"ER_INVAL_OUTSIDE_TB_CM\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 530i32 as cty::c_uint,
                   name:
                       b"ER_INVAL_OUTSIDE_DG_TB_CM\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 540i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_OUTSIDE_DIALOG\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 550i32 as cty::c_uint,
                   name:
                       b"ER_MUST_BE_IN_PRGM_OR_FUNC\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 560i32 as cty::c_uint,
                   name:
                       b"ER_EXIT_NOT_IN_LOOP\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 570i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_PATHNAME\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 575i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_POLAR_COMPLEX\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 580i32 as cty::c_uint,
                   name:
                       b"ER_ILLEGAL_PRGM_REF\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 590i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_SYNTAX_BLOCK\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 600i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_TABLE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 605i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_USE_OF_UNITS\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 610i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_LOCAL_DECLARATION\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 620i32 as cty::c_uint,
                   name:
                       b"ER_EXPECTED_VAR_OR_FUNC\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 630i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_VAR_REF\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 640i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_VECTOR_SYNTAX\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 650i32 as cty::c_uint,
                   name:
                       b"ER_LINK_IO\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 665i32 as cty::c_uint,
                   name:
                       b"ER_MAT_NOT_DIAGONALIZABLE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 670i32 as cty::c_uint,
                   name:
                       b"ER_MEMORY\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 673i32 as cty::c_uint,
                   name:
                       b"ER_STACK_VIO\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 680i32 as cty::c_uint,
                   name:
                       b"ER_EXPECTED_LPAR\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 690i32 as cty::c_uint,
                   name:
                       b"ER_EXPECTED_RPAR\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 700i32 as cty::c_uint,
                   name:
                       b"ER_EXPECTED_DOUBLE_QUOTE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 710i32 as cty::c_uint,
                   name:
                       b"ER_EXPECTED_RIGHT_BRACKET\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 720i32 as cty::c_uint,
                   name:
                       b"ER_EXPECTED_RIGHT_BRACE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 730i32 as cty::c_uint,
                   name:
                       b"ER_INVALID_BLOCK_STRUCTURE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 740i32 as cty::c_uint,
                   name:
                       b"ER_MISSING_THEN\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 750i32 as cty::c_uint,
                   name:
                       b"ER_NOT_FUNC_OR_PRGM\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 765i32 as cty::c_uint,
                   name:
                       b"ER_NO_FUNCS_SEL\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 780i32 as cty::c_uint,
                   name:
                       b"ER_NO_SOLUTION\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 790i32 as cty::c_uint,
                   name:
                       b"ER_NON_ALGEBRAIC_VARIABLE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 800i32 as cty::c_uint,
                   name:
                       b"ER_UNREAL_RESULT\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 810i32 as cty::c_uint,
                   name:
                       b"ER_MEMORY_DML\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 830i32 as cty::c_uint,
                   name:
                       b"ER_OVERFLOW\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 840i32 as cty::c_uint,
                   name:
                       b"ER_STAT_PLOT\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 850i32 as cty::c_uint,
                   name:
                       b"ER_PRGM_NOT_FOUND\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 860i32 as cty::c_uint,
                   name:
                       b"ER_RECURSION_TOO_DEEP\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 870i32 as cty::c_uint,
                   name:
                       b"ER_RESERVED\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 875i32 as cty::c_uint,
                   name:
                       b"ER_ROM_ROUTINE_NOT_AVAILABLE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 880i32 as cty::c_uint,
                   name:
                       b"ER_SEQUENCE_SETUP\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 885i32 as cty::c_uint,
                   name:
                       b"ER_SIGNATURE_ERR\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 890i32 as cty::c_uint,
                   name:
                       b"ER_SINGULARMAT\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 895i32 as cty::c_uint,
                   name:
                       b"ER_SLOPE_FIELD_FUNCS\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 900i32 as cty::c_uint,
                   name:
                       b"ER_EMPTY_GROUP_NOT_VALID\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 910i32 as cty::c_uint,
                   name:
                       b"ER_SYNTAX\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 930i32 as cty::c_uint,
                   name:
                       b"ER_TOO_FEW_ARGS\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 940i32 as cty::c_uint,
                   name:
                       b"ER_TOO_MANY_ARGS\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 950i32 as cty::c_uint,
                   name:
                       b"ER_TOO_MANY_SUBSCRIPTS\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 955i32 as cty::c_uint,
                   name:
                       b"ER_TOO_MANY_UNDEFINED\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 960i32 as cty::c_uint,
                   name:
                       b"ER_UNDEFINED_VAR\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 965i32 as cty::c_uint,
                   name:
                       b"ER_UNLICENSED\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 970i32 as cty::c_uint,
                   name:
                       b"ER_VAR_IN_USE\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 980i32 as cty::c_uint,
                   name:
                       b"ER_PROTECTED\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 990i32 as cty::c_uint,
                   name:
                       b"ER_NAME_TOO_LONG\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 1000i32 as cty::c_uint,
                   name:
                       b"ER_RANGE\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 1010i32 as cty::c_uint,
                   name: b"ER_ZOOM\x00" as *const u8 as *const cty::c_char,},
     C2RustUnnamed{id: 1020i32 as cty::c_uint,
                   name:
                       b"ER_ILLEGAL_TAG\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 1030i32 as cty::c_uint,
                   name:
                       b"ER_MEM_VIO\x00" as *const u8 as
                           *const cty::c_char,},
     C2RustUnnamed{id: 2048i32 as cty::c_uint,
                   name: 0 as *const cty::c_char,}];
#[no_mangle]
pub unsafe extern "C" fn ercodes_get_name(mut id: cty::c_uint)
 -> *const cty::c_char {
    if id >= 2048i32 as cty::c_uint {
        return b"Unknown ERROR code\x00" as *const u8 as *const cty::c_char
    } else {
        let mut n: cty::c_uint =
            (::core::mem::size_of::<[C2RustUnnamed; 126]>() as
                 cty::c_ulong).wrapping_div(::core::mem::size_of::<C2RustUnnamed>()
                                                 as cty::c_ulong) as
                cty::c_uint;
        let mut lb: cty::c_uint = 0i32 as cty::c_uint;
        let mut ub: cty::c_uint = n.wrapping_sub(1i32 as cty::c_uint);
        while lb < ub.wrapping_sub(1i32 as cty::c_uint) {
            let mut mid: cty::c_uint = lb.wrapping_add(ub) >> 1i32;
            if erthrow_table[mid as usize].id <= id {
                lb = mid
            } else { ub = mid }
        }
        if id != erthrow_table[lb as usize].id {
            static mut buffer: [cty::c_char; 100] = [0; 100];
            sprintf(buffer.as_mut_ptr(),
                    b"%s + %u\x00" as *const u8 as *const cty::c_char,
                    erthrow_table[lb as usize].name,
                    id.wrapping_sub(erthrow_table[lb as usize].id));
            return buffer.as_mut_ptr()
        } else { return erthrow_table[lb as usize].name }
    };
}
