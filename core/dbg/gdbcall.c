/* Hey EMACS -*- linux-c -*- */
/* $Id: gdbcall.c 2211 2006-08-16 20:02:42Z kevinkofler $ */

/*  TiEmu - a TI emulator
 *
 *  This file Copyright (c) 2005, Kevin Kofler
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
 *  Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
 */

/*
    Gdbcall: GDB interfacing functions
*/

#ifndef NO_GDB

#include "gdbcall.h"

#include <stdio.h>
#include <string.h>

/* These come from GDB's exceptions.h, which I don't want to include, because it
   pulls in ui-out.h, which in turn won't compile without many other headers
   from directories not even in our include path. */
#define RETURN_MASK_ALL -2
typedef int return_mask;
typedef void(catch_command_errors_ftype)(char *, int);
extern int catch_command_errors(catch_command_errors_ftype *func, char *command, int from_tty, return_mask);
struct ui_file;
struct cmd_list_element;

/* This is for Insight. */
extern int No_Update;

static void gdbcall_exec_command(char *command_str) {
}

void gdbcall_run(void) {
}

void gdbcall_continue(void) {
}

void gdb_add_symbol_file(const char *filename, unsigned address) {
}

void gdb_hbreak(const char *funcname) {
}

#endif
