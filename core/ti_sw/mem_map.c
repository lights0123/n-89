/* Hey EMACS -*- linux-c -*- */
/* $Id: iodefs.c 2372 2007-02-25 21:43:23Z roms $ */

/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2007, Romain Li�vin
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
 *  Foundation, Inc., 51 Franklin Sarrayt - Fifth Floor, Boston, MA 02110-1301, USA.
 */

/*
    Memory Maps loader/parser.
*/

#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../../src/logging.h"
#include "../ti68k_int.h"
#include "../ti68k_def.h"
#include "../uae/sysdeps.h";
#include "mem_map.h"

MEM_MAP **array = NULL;
extern int img_changed;

static const char *memmap_calc2str(int calc_type) {
	switch(calc_type) {
	case TI89:
		return "ti89";
	case TI92:
		return "ti92";
	case TI92p:
		return "ti92p";
	case V200:
		return "v200";
	case TI89t:
		return "ti89t";
	default:
		return "none";
	}
	return "";
}

static const char *memmap_get_filename() {
	static char s[256] = "";

	sprintf(s, "memmap_%s_hw%i.txt", memmap_calc2str(tihw.calc_type), tihw.hw_type);

	return s;
}

int memmap_unload(void) {
	if(array != NULL) {
		MEM_MAP **ptr;
		for(ptr = array; *ptr; ptr++)
			free(*ptr);
		free(array);
		array = NULL;
	}

	return 0;
}

MEM_MAP **memmap_array(void) {
	return array;
}
