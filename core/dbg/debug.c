/* Hey EMACS -*- linux-c -*- */
/* $Id: debug.c 2792 2008-05-26 16:48:30Z roms $ */

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
    Debug: debugging functions
*/

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <stdio.h>
#include <string.h>

#include "../ti68k_def.h"
#include "../ti68k_err.h"
#include "../ti_hw/m68k.h"
#include "../ti_sw/romcalls.h"
#include "../uae/libuae.h"
#include "debug.h"

int ti68k_debug_get_pc(void) {
	return m68k_getpc();
}

int ti68k_debug_get_old_pc(void) {
	return logger.pclog_buf[(logger.pclog_ptr + logger.pclog_size - 1) % logger.pclog_size];
}

int ti68k_debug_break(void) {
	regs.spcflags |= SPCFLAG_BRK;
	return 0;
}

int ti68k_debug_trace(void) {
	// Set up an internal trap (DBTRACE) which will
	// launch/refresh the debugger when encountered
	regs.spcflags |= SPCFLAG_DBTRACE;

	return 0;
}

static const uint16_t rets[] = {

	0x4e77, // RTR
	0x4e75, // RTS
	0x4e74, // RTD
	0x4e73, // RTE
	0x4e72, // STOP
};

static inline int is_ret_inst(uint16_t inst) {
	int i;
	for(i = 0; i < sizeof(rets) / sizeof(uint16_t); i++)
		if(curriword() == rets[i]) return !0;
	return 0;
}

static inline int is_bsr_inst(uint16_t ci) {
	int t1, t2, t3, t4, t5;

	t1 = ((ci >> 6) == (0x4e80 >> 6));   /* jsr */
	t2 = ((ci >> 8) == (0x61ff >> 8));   /* bsr */
	t3 = (ci >= 0xf800 && ci <= 0xffee); /* fline */
	t4 = (ci == 0xfff0) || (ci == 0xfff2) || ((ci & 0xf000) == 0x5000) && ((ci & 0x00f8) == 0x00c8); /* dbcc */
	t5 = ((ci >> 4) == (0x4e40 >> 4)); /* trap */

	// printf("<%i %i %i %i %i>\n", ret1, ret2, ret3, ret4, ret5);

	return t1 || t2 || t3 || t4 || t5;
}


// Used to read/modify/write memory directly from debugger
uint8_t *ti68k_get_real_address(uint32_t addr) {
	return hw_get_real_address(addr);
}