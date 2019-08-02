/* Hey EMACS -*- linux-c -*- */
/* $Id: romcalls.c 2268 2006-11-06 17:18:51Z roms $ */

/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin
 *  Copyright (c) 2006, Kevin Kofler
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
    Symbols (ROM calls address and names)
*/

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <vector>

#include "../images.h"
#include "../ti68k_def.h"
#include "romcalls.h"
#include "timem.h"
typedef const void *gconstpointer;
typedef int gint;
extern int img_changed; // set if image modified
static int loaded = 0;  // loaded

static ROM_CALL table[NMAX_ROMCALLS]; // list by id
// TODO: GList
static std::vector<void *> list; // sorted list (by id, addr or name)


/* =========== */

/*
    Retrieve base address of ROM calls table and size.
*/
void romcalls_get_table_infos(uint32_t *base, uint32_t *size) {
	*base = *size = 0;

	if(tihw.calc_type == TI92) return;

	*base = rd_long(&tihw.rom[0x12000 + 0x88 + 0xC8]);
	*size = rd_long(&tihw.rom[((*base - 4) & 0x0fffff)]);
}

/*
    Given a ROM call ID, retrieve address of ROM call.
*/
void romcalls_get_symbol_address(int id, uint32_t *addr) {
	uint32_t base;

	base = rd_long(&tihw.rom[0x12000 + 0x88 + 0xC8]);
	*addr = rd_long(&tihw.rom[(base & 0x0fffff) + 4 * id]);
}

/* =========== */

int romcalls_is_loaded(void) {
	return loaded;
}

/* =========== */

// cache last search (disasm)
static int last_id = 0;
extern "C" {
// returns id or -1
int romcalls_is_addr(uint32_t addr) {
	int i;

	if(!loaded) return -1;

	for(i = 0; i < (int)list.size(); i++) {
		if(addr == table[i].addr) return last_id = i;
	}

	return -1;
}

// returns id or -1
int romcalls_is_name(const char *name) {
	int i;

	if(!loaded) return -1;

	for(i = 0; i < (int)list.size(); i++) {
		if(!strcmp(name, table[i].name)) return i;
	}

	return -1;
}

const char *romcalls_get_name(int id) {
	if(!loaded) return "not loaded";
	return table[id].name;
}

uint32_t romcalls_get_addr(int id) {
	return table[id].addr;
}
}
