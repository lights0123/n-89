/* Hey EMACS -*- linux-c -*- */
/* $Id: m68k.c 2733 2007-12-21 05:33:19Z kevinkofler $ */

/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005-2007, Romain Li�vin, Kevin Kofler
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
    M68K management (wrapper for the UAE engine)
*/

#include <stdlib.h>

#include "../dbg/bkpts.h"
#include "../images.h"
#include "../ti68k_def.h"
#include "../ti68k_err.h"
#include "../ti_sw/handles.h"
#include "../uae/libuae.h"
#include "flash.h"
#include "gscales.h"
#include "hw.h"
#include "hwprot.h"
#include "mem.h"

int pending_ints;

int hw_m68k_init(void) {
	// init breakpoints

	// init instruction logging
	logger.pclog_size = 11;
	logger.pclog_buf = (uint32_t *)malloc(logger.pclog_size * sizeof(uint32_t));
	if(logger.pclog_buf == NULL) return ERR_MALLOC;
	logger.pclog_ptr = 0;

	init_m68k();

	return 0;
}

int hw_m68k_reset(void) {
	// retrieve SSP & PC values for boot
	find_ssp_and_pc(&tihw.initial_ssp, &tihw.initial_pc);

	// and reset
	m68k_reset();

	pending_ints = 0;

	return 0;
}

int hw_m68k_exit(void) {
	ti68k_bkpt_clear_address();
	ti68k_bkpt_clear_exception();

	if(logger.pclog_buf) free(logger.pclog_buf);
	logger.pclog_buf = NULL;

	return 0;
}

// UAE does not implement interrupt priority and pending interrupts
// I re-implement it as replacement of Interrupt()
void Interrupt2(int nr) {
	if((nr > regs.intmask) || (nr == 7)) {
		Interrupt(nr);
		pending_ints &= ~(1 << nr); // clr pending int
	}
}

void hw_m68k_irq(int n) {
	set_special(SPCFLAG_INT);
	currIntLev = n; // unused, used for compat with old Interrupt()

	pending_ints |= (1 << n); // set pending int
}

// parse pending ints for one to raise (up to low prority order)
#define GET_INT_LVL(level)                         \
	{                                              \
		int mask = 0x80;                           \
		for(level = 7; level; level--, mask >>= 1) \
			if(pending_ints & mask) break;         \
	}

/* Replace UAE's M68000_run() */

/*
    Returns cycle count.
*/
static unsigned int cycles = 0;
unsigned int hw_m68k_get_cycle_count(int reset) {
	if(reset) cycles = 0;
	return cycles;
}

/*
  Do 'n' instructions (up to 'maxcycles', unless set to 0).
  Returned value:
  - 1 if breakpoint has been encountered,
  - 2 if trace,
  - 0 otherwise.
*/
int hw_m68k_run(int n, unsigned maxcycles) {
	int i = n;
	unsigned int cycles_at_start = cycles;

	for(i = 0; i < n && (!maxcycles || cycles - cycles_at_start < maxcycles); i++) {
		uint32_t opcode;
		unsigned int insn_cycles;

		// refresh hardware
		do_cycles();

		// OSC1 stopped ? Refresh hardware and wake-up on interrupt. No opcode execution.
		if((regs.spcflags & SPCFLAG_STOP)) {
			if(pending_ints) {
				int level;
				GET_INT_LVL(level);

				// wake-up on int level 6 (ON key) or level 1..5
				if((pending_ints & (tihw.io[5] << 1)) || (level == 6)) {
					Interrupt2(level);
					regs.stopped = 0;
					regs.spcflags &= ~SPCFLAG_STOP;
				}
			}

			cycles += 4;        // cycle count for hw_m68k_run loop
			cycle_count += 4;   // cycle count for hw.c timers
			tihw.lcd_tick += 4; // used by grayscale for time plane exposure

			continue;
		}

		// store PC in the log buffer
		if(logger.pclog_size > 1) {
			logger.pclog_buf[logger.pclog_ptr++] = m68k_getpc();
			if(logger.pclog_ptr >= logger.pclog_size) logger.pclog_ptr = 0;
		}

		// search for next opcode and execute it
		opcode = get_iword_prefetch(0);
		insn_cycles = (*cpufunctbl[opcode])(opcode)*2; // increments PC automatically now
		cycles += insn_cycles;                         // cycle count for hw_m68k_run loop
		cycle_count += insn_cycles;                    // cycle count for hw.c timers
		tihw.lcd_tick += insn_cycles;                  // used by grayscale for time plane exposure

		// HW2/3 grayscales management
		lcd_hook_hw2(0);

		// process (pending) interrupts
		if(pending_ints) {
			int level;
			GET_INT_LVL(level);

			Interrupt2(level);
			regs.stopped = 0;
			// regs.spcflags &= ~SPCFLAG_STOP;
		}

		// management of special flags
		if(regs.spcflags) {
			if(regs.spcflags & SPCFLAG_ADRERR) {
				Exception(3, 0);
				unset_special(SPCFLAG_ADRERR);
			}

			if(regs.spcflags & SPCFLAG_DOTRACE) {
				Exception(9, 0);
			}

			if(regs.spcflags & SPCFLAG_TRACE) {
				unset_special(SPCFLAG_TRACE);
			}

			if(regs.spcflags & SPCFLAG_BRK) {
				unset_special(SPCFLAG_BRK);
				return DBG_BREAK;
			}

			if(regs.spcflags & SPCFLAG_DBTRACE) {
				unset_special(SPCFLAG_DBTRACE);
				return DBG_TRACE;
			}

			if(regs.spcflags & SPCFLAG_DBSKIP) {
				unset_special(SPCFLAG_DBSKIP);
			}
		}
	}

	return 0;
}
