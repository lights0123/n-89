/* Hey EMACS -*- linux-c -*- */
/* $Id: disasm.c 2841 2009-05-15 15:26:47Z roms $ */

/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005-2006, Romain Li�vin, Kevin Kofler
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

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <inttypes.h>
#include <stdio.h>
#include <string.h>

#include "../ti68k_int.h"
#include "../ti68k_def.h"

/* ti68k_debug_disassemble is a wrapper around the GDB or UAE disassembler, so
   we can use the GDB disassembler in GDB-enabled builds and the UAE one in
   non-GDB builds. The abstraction also allows plugging in any other
   disassembler instead at any moment. */


extern int m68k_disasm(char *output, uint32_t addr);

static const char *instr[] = { "ORSR",      /* ORI  #<data>,SR		*/
	                           "ANDSR",     /* ANDI #<data>,SR		*/
	                           "EORSR",     /* EORI #<data>,SR		*/
	                           "MVSR2",     /* MOVE SR,<ea>			*/
	                           "MV2SR",     /* MOVE <ea>,SR			*/
	                           "MVR2USP.L", /* MOVE An,USP			*/
	                           "MVUSP2R.L", /* MOVE USP,An			*/
	                           "MVMLE",     /* MOVEM <ea>,<list>  	*/
	                           "MVMEL",     /* MOVEM <list>,<ea>	*/
	                           "MVPMR",     /* MOVEP <Dx>,<(d16,Ay)>*/
	                           "MVPRM",     /* MOVEP <d16,Ay>,<Dx>	*/
	                           "TRAP.L",    /* TRAP	#<vector>		*/
	                           "RESET.L",   "NOP.L", "STOP.L", "RTE.L",
	                           "RTS.L",     "JMP.L", "LEA.L",  "BTST", /* do nothing			*/
	                           "BT",                                   /* BRA					*/

	                           NULL };

static int match_opcode(const char *opcode) {
	int i;

	if(opcode == NULL) return -1;

	for(i = 0; instr[i] != NULL; i++) {
		if(!strncmp(opcode, (char *)instr[i], strlen(instr[i]))) return i;
	}

	return -1;
}

// testing patterns: 0x55, 0xAA, 0x02, 0xF5, 0x5F, 0xFA, 0xAF
static char *create_reg_list(uint8_t value, char name) {
	char *str = g_strdup("");
	char *str2;
	int i;
	int pre_bit = 0;
	int cur_bit;
	int start = -1;
	int end = -1;

	for(i = 0; i < 8; i++, value >>= 1) {
		cur_bit = value & 1;

		if(pre_bit == 0 && cur_bit == 1) start = i;
		if(pre_bit == 1 && cur_bit == 0) end = i - 1;

		if(start == end && start != -1) {
			str2 = g_strdup_printf("%s%c%i/", str, name, i - 1);
			free(str);
			str = str2;
			end = start = -1;
		} else if(end > start) {
			str2 = g_strdup_printf("%s%c%i-%c%i/", str, name, start, name, end);
			free(str);
			str = str2;
			end = start = -1;
		}

		pre_bit = cur_bit;
	}

	if((end = i - 1) > (start + 1) && start != -1) {
		str2 = g_strdup_printf("%s%c%i-%i", str, name, start, end);
		free(str);
		str = str2;
	} else if(start > 0 && end > 0) {
		str2 = g_strdup_printf("%s%c%i", str, name, start);
		free(str);
		str = str2;
	} else
		str[strlen(str) - 1] = '\0';

	return str;
}

static char *create_rev_reg_list(uint8_t value, char name) {
	char *str = g_strdup("");
	char *str2;
	int i;
	int pre_bit = 0;
	int cur_bit;
	int start = -1;
	int end = -1;

	for(i = 0; i < 8; i++, value <<= 1) {
		cur_bit = (value & (1 << 7)) >> 7;

		if(pre_bit == 0 && cur_bit == 1) start = i;
		if(pre_bit == 1 && cur_bit == 0) end = i - 1;

		if(start == end && start != -1) {
			str2 = g_strdup_printf("%s%c%i/", str, name, i - 1);
			free(str);
			str = str2;
			end = start = -1;
		} else if(end > start) {
			str2 = g_strdup_printf("%s%c%i-%c%i/", str, name, start, name, end);
			free(str);
			str = str2;
			end = start = -1;
		}

		pre_bit = cur_bit;
	}

	if((end = i - 1) > (start + 1) && start != -1) {
		str2 = g_strdup_printf("%s%c%i-%i", str, name, start, end);
		free(str);
		str = str2;
	} else if(start > 0 && end > 0) {
		str2 = g_strdup_printf("%s%c%i", str, name, start);
		free(str);
		str = str2;
	} else
		str[strlen(str) - 1] = '\0';

	return str;
}

static char *create_reg_lists(uint16_t value) {
	char *a, *d;
	char *str;

	a = create_reg_list(MSB(value), 'A');
	d = create_reg_list(LSB(value), 'D');

	return str = g_strconcat(a, "/", d, NULL);
}

static char *create_rev_reg_lists(uint16_t value) {
	char *a, *d;
	char *str;

	d = create_rev_reg_list(MSB(value), 'D');
	a = create_rev_reg_list(LSB(value), 'A');

	return str = g_strconcat(d, "/", a, NULL);
}
