/* Hey EMACS -*- linux-c -*- */
/* $Id: gencpu.c 2681 2007-11-20 18:01:02Z roms $ */

/*
 * UAE - The Un*x Amiga Emulator
 *
 * MC68000 emulation generator
 *
 * This is a fairly stupid program that generates a lot of case labels that
 * can be #included in a switch statement.
 * As an alternative, it can generate functions that handle specific
 * MC68000 instructions, plus a prototype header file and a function pointer
 * array to look up the function for an opcode.
 * Error checking is bad, an illegal table68k file will cause the program to
 * call abort().
 * The generated code is sometimes sub-optimal, an optimizing compiler should
 * take care of this.
 *
 * The source for the insn timings is Markt & Technik's Amiga Magazin 8/1992.
 *
 * Copyright 1995, 1996, 1997, 1998, 1999, 2000 Bernd Schmidt
 */

#include "readcpu.h"
#include "sysdeps.h"
#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define BOOL_TYPE "int"

static FILE *headerfile;
static FILE *stblfile;

static int using_prefetch;
static int using_exception_3;
static int using_ce = 0;
static int cpu_level;

/* Flags for genamode.  */
#define GF_APDI 1
#define GF_AD8R 2
#define GF_PC8R 4
#define GF_AA 7
#define GF_NOREFILL 8
#define GF_PREFETCH 16

/* For the current opcode, the next lower level that will have different code.
 * Initialized to -1 for each opcode. If it remains unchanged, indicates we
 * are done with that opcode.  */
static int next_cpu_level;

static int *opcode_map;
static int *opcode_next_clev;
static int *opcode_last_postfix;
static unsigned long *counts;

static void read_counts(void) {
	FILE *file;
	unsigned long opcode, count, total;
	char name[20];
	int nr = 0;
	memset(counts, 0, 65536 * sizeof *counts);

	file = fopen("frequent.68k", "r");
	if(file) {
		fscanf(file, "Total: %lu\n", &total);
		while(fscanf(file, "%lx: %lu %s\n", &opcode, &count, name) == 3) {
			opcode_next_clev[nr] = 4;
			opcode_last_postfix[nr] = -1;
			opcode_map[nr++] = opcode;
			counts[opcode] = count;
		}
		fclose(file);
	}
	if(nr == nr_cpuop_funcs) return;
	for(opcode = 0; opcode < 0x10000; opcode++) {
		if(table68k[opcode].handler == -1 && table68k[opcode].mnemo != i_ILLG && counts[opcode] == 0) {
			opcode_next_clev[nr] = 4;
			opcode_last_postfix[nr] = -1;
			opcode_map[nr++] = opcode;
			counts[opcode] = count;
		}
	}
	if(nr != nr_cpuop_funcs) abort();
}

static char endlabelstr[80];
static int endlabelno = 0;
static int need_endlabel;

static int n_braces = 0;
static int m68k_pc_offset = 0;
static int insn_n_cycles;

static void returncycles(char *s, int cycles) {
	if(using_ce) return;
	printf("%sreturn %d * %d;\n", s, cycles, CYCLE_UNIT / 2);
}

static int isreg(amodes mode) {
	if(mode == Dreg || mode == Areg) return 1;
	return 0;
}

static void start_brace(void) {
	n_braces++;
	printf("{");
}

static void close_brace(void) {
	assert(n_braces > 0);
	n_braces--;
	printf("}");
}

static void finish_braces(void) {
	while(n_braces > 0)
		close_brace();
}

static void pop_braces(int to) {
	while(n_braces > to)
		close_brace();
}

static int bit_size(int size) {
	switch(size) {
	case sz_byte:
		return 8;
	case sz_word:
		return 16;
	case sz_long:
		return 32;
	default:
		abort();
	}
	return 0;
}

static const char *bit_mask(int size) {
	switch(size) {
	case sz_byte:
		return "0xff";
	case sz_word:
		return "0xffff";
	case sz_long:
		return "0xffffffff";
	default:
		abort();
	}
	return 0;
}

static void gen_nextilong(char *type, char *name, int norefill) {
	int r = m68k_pc_offset;
	m68k_pc_offset += 4;

	if(using_ce) {
		printf("\t%s %s;\n", type, name);
		/* we must do this because execution order of (something | something2) is not defined */
		if(norefill) {
			printf("\t%s = get_word_ce_prefetch (%d) << 16;\n", name, r + 2);
			printf("\t%s |= regs.irc;\n", name);
		} else {
			printf("\t%s = get_word_ce_prefetch (%d) << 16;\n", name, r + 2);
			printf("\t%s |= get_word_ce_prefetch (%d);\n", name, r + 4);
		}
	} else {
		if(using_prefetch) {
			if(norefill) {
				printf("\t%s %s;\n", type, name);
				printf("\t%s = get_word_prefetch (%d) << 16;\n", name, r + 2);
				printf("\t%s |= regs.irc;\n", name);
				insn_n_cycles += 4;
			} else
				printf("\t%s %s = get_long_prefetch (%d);\n", type, name, r + 2);
			insn_n_cycles += 8;
		} else {
			insn_n_cycles += 8;
			printf("\t%s %s = get_ilong (%d);\n", type, name, r);
		}
	}
}

static const char *gen_nextiword(int norefill) {
	static char buffer[80];
	int r = m68k_pc_offset;
	m68k_pc_offset += 2;

	if(using_ce) {
		if(norefill)
			strcpy(buffer, "regs.irc");
		else
			sprintf(buffer, "get_word_ce_prefetch (%d)", r + 2);
	} else {
		if(using_prefetch) {
			if(norefill) {
				sprintf(buffer, "regs.irc", r);
			} else {
				sprintf(buffer, "get_word_prefetch (%d)", r + 2);
				insn_n_cycles += 4;
			}
		} else {
			sprintf(buffer, "get_iword (%d)", r);
			insn_n_cycles += 4;
		}
	}
	return buffer;
}

static const char *gen_nextibyte(int norefill) {
	static char buffer[80];
	int r = m68k_pc_offset;
	m68k_pc_offset += 2;

	if(using_ce) {
		if(norefill)
			strcpy(buffer, "(uae_u8)regs.irc");
		else
			sprintf(buffer, "(uae_u8)get_word_ce_prefetch (%d)", r + 2);
	} else {
		insn_n_cycles += 4;
		if(using_prefetch) {
			if(norefill) {
				sprintf(buffer, "(uae_u8)regs.irc", r);
			} else {
				sprintf(buffer, "(uae_u8)get_word_prefetch (%d)", r + 2);
				insn_n_cycles += 4;
			}
		} else {
			sprintf(buffer, "get_ibyte (%d)", r);
			insn_n_cycles += 4;
		}
	}
	return buffer;
}

static void irc2ir(void) {
	if(!using_prefetch) return;
	printf("\tregs.ir = regs.irc;\n");
}

static int did_prefetch;

static void fill_prefetch_2(void) {
	if(!using_prefetch) return;
	if(using_ce)
		printf("\tget_word_ce_prefetch (%d);\n", m68k_pc_offset + 2);
	else
		printf("\tget_word_prefetch (%d);\n", m68k_pc_offset + 2);
	did_prefetch = 1;
	insn_n_cycles += 4;
}

static void fill_prefetch_1(int o, int needcycles) {
	if(!using_prefetch) return;
	if(using_ce) {
		if(needcycles) {
			printf("\tlostcycles = get_word_ce_prefetch_cycles (%d);\n", o);
		} else {
			printf("\tget_word_ce_prefetch (%d);\n", o);
		}
	} else {
		printf("\tget_word_prefetch (%d);\n", o);
	}
	did_prefetch = 1;
	insn_n_cycles += 4;
}

static void fill_prefetch_full(void) {
	fill_prefetch_1(0, 0);
	irc2ir();
	fill_prefetch_1(2, 0);
}

static void fill_prefetch_0(void) {
	if(!using_prefetch) return;
	if(using_ce)
		printf("\tget_word_ce_prefetch (0);\n");
	else
		printf("\tget_word_prefetch (0);\n");
	did_prefetch = 1;
	insn_n_cycles += 4;
}

static void fill_prefetch_next_1(needcycles) {
	if(needcycles && using_ce) {
		printf("\tint lostcycles;\n");
	}
	irc2ir();
	fill_prefetch_1(m68k_pc_offset + 2, needcycles);
}

static void fill_prefetch_next(void) {
	fill_prefetch_next_1(0);
}
static void fill_prefetch_next_cycles(void) {
	fill_prefetch_next_1(1);
}

static void fill_prefetch_next_delay(int extracycles) {
	if(!using_prefetch) return;
	if(using_ce) {
		if(extracycles > 0) {
			printf("\t{\n");
			fill_prefetch_next_cycles();
			printf("\tif ((%d + 4) * %d > lostcycles) do_cycles_ce ((%d + 4) * %d - lostcycles);\n",
			       extracycles, CYCLE_UNIT / 2, extracycles, CYCLE_UNIT / 2);
			printf("\t}\n");
		} else {
			fill_prefetch_next();
		}
	} else {
		fill_prefetch_next();
	}
}

static void fill_prefetch_finish(void) {
	if(did_prefetch || !using_prefetch) return;
	fill_prefetch_1(m68k_pc_offset, 0);
}

static void sync_m68k_pc(void) {
	if(m68k_pc_offset == 0) return;
	printf("\tm68k_incpc (%d);\n", m68k_pc_offset);
	m68k_pc_offset = 0;
}

/* getv == 1: fetch data; getv != 0: check for odd address. If movem != 0,
 * the calling routine handles Apdi and Aipi modes.
 * gb-- movem == 2 means the same thing but for a MOVE16 instruction */
static void genamode(amodes mode, char *reg, wordsizes size, char *name, int getv, int movem, int flags) {
	char namea[100];
	int m68k_pc_offset_last = m68k_pc_offset;

	sprintf(namea, "%sa", name);

	start_brace();
	switch(mode) {
	case Dreg:
		if(movem) abort();
		if(getv == 1) switch(size) {
			case sz_byte:
#if defined(AMIGA) && !defined(WARPUP)
				/* sam: I don't know why gcc.2.7.2.1 produces a code worse */
				/* if it is not done like that: */
				printf("\tuae_s8 %s = ((uae_u8*)&m68k_dreg(regs, %s))[3];\n", name, reg);
#else
				printf("\tuae_s8 %s = m68k_dreg(regs, %s);\n", name, reg);
#endif
				break;
			case sz_word:
#if defined(AMIGA) && !defined(WARPUP)
				printf("\tuae_s16 %s = ((uae_s16*)&m68k_dreg(regs, %s))[1];\n", name, reg);
#else
				printf("\tuae_s16 %s = m68k_dreg(regs, %s);\n", name, reg);
#endif
				break;
			case sz_long:
				printf("\tuae_s32 %s = m68k_dreg(regs, %s);\n", name, reg);
				break;
			default:
				abort();
			}
		return;
	case Areg:
		if(movem) abort();
		if(getv == 1) switch(size) {
			case sz_word:
				printf("\tuae_s16 %s = m68k_areg(regs, %s);\n", name, reg);
				break;
			case sz_long:
				printf("\tuae_s32 %s = m68k_areg(regs, %s);\n", name, reg);
				break;
			default:
				abort();
			}
		return;
	case Aind:
		printf("\tuaecptr %sa = m68k_areg(regs, %s);\n", name, reg);
		break;
	case Aipi:
		printf("\tuaecptr %sa = m68k_areg(regs, %s);\n", name, reg);
		break;
	case Apdi:
		insn_n_cycles += 2;
		switch(size) {
		case sz_byte:
			if(movem)
				printf("\tuaecptr %sa = m68k_areg(regs, %s);\n", name, reg);
			else
				printf("\tuaecptr %sa = m68k_areg(regs, %s) - areg_byteinc[%s];\n", name, reg, reg);
			break;
		case sz_word:
			printf("\tuaecptr %sa = m68k_areg(regs, %s) - %d;\n", name, reg, movem ? 0 : 2);
			break;
		case sz_long:
			printf("\tuaecptr %sa = m68k_areg(regs, %s) - %d;\n", name, reg, movem ? 0 : 4);
			break;
		default:
			abort();
		}
		break;
	case Ad16:
		printf("\tuaecptr %sa = m68k_areg(regs, %s) + (uae_s32)(uae_s16)%s;\n", name, reg,
		       gen_nextiword(flags & GF_NOREFILL));
		break;
	case Ad8r:
		insn_n_cycles += 2;
		if(cpu_level > 1) {
			if(next_cpu_level < 1) next_cpu_level = 1;
			sync_m68k_pc();
			start_brace();
			/* This would ordinarily be done in gen_nextiword, which we bypass.  */
			insn_n_cycles += 4;
			printf("\tuaecptr %sa = get_disp_ea_020(m68k_areg(regs, %s), next_iword());\n", name, reg);
		} else
			printf("\tuaecptr %sa = get_disp_ea_000(m68k_areg(regs, %s), %s);\n", name, reg,
			       gen_nextiword(flags & GF_NOREFILL));

		break;
	case PC16:
		printf("\tuaecptr %sa = m68k_getpc () + %d;\n", name, m68k_pc_offset);
		printf("\t%sa += (uae_s32)(uae_s16)%s;\n", name, gen_nextiword(flags & GF_NOREFILL));
		break;
	case PC8r:
		insn_n_cycles += 2;
		if(cpu_level > 1) {
			if(next_cpu_level < 1) next_cpu_level = 1;
			sync_m68k_pc();
			start_brace();
			/* This would ordinarily be done in gen_nextiword, which we bypass.  */
			insn_n_cycles += 4;
			printf("\tuaecptr tmppc = m68k_getpc();\n");
			printf("\tuaecptr %sa = get_disp_ea_020(tmppc, next_iword());\n", name);
		} else {
			printf("\tuaecptr tmppc = m68k_getpc() + %d;\n", m68k_pc_offset);
			printf("\tuaecptr %sa = get_disp_ea_000(tmppc, %s);\n", name, gen_nextiword(flags & GF_NOREFILL));
		}

		break;
	case absw:
		printf("\tuaecptr %sa = (uae_s32)(uae_s16)%s;\n", name, gen_nextiword(flags & GF_NOREFILL));
		break;
	case absl:
		gen_nextilong("uaecptr", namea, flags & GF_NOREFILL);
		break;
	case imm:
		if(getv != 1) abort();
		switch(size) {
		case sz_byte:
			printf("\tuae_s8 %s = %s;\n", name, gen_nextibyte(flags & GF_NOREFILL));
			break;
		case sz_word:
			printf("\tuae_s16 %s = %s;\n", name, gen_nextiword(flags & GF_NOREFILL));
			break;
		case sz_long:
			gen_nextilong("uae_s32", name, flags & GF_NOREFILL);
			break;
		default:
			abort();
		}
		return;
	case imm0:
		if(getv != 1) abort();
		printf("\tuae_s8 %s = %s;\n", name, gen_nextibyte(flags & GF_NOREFILL));
		return;
	case imm1:
		if(getv != 1) abort();
		printf("\tuae_s16 %s = %s;\n", name, gen_nextiword(flags & GF_NOREFILL));
		return;
	case imm2:
		if(getv != 1) abort();
		gen_nextilong("uae_s32", name, flags & GF_NOREFILL);
		return;
	case immi:
		if(getv != 1) abort();
		printf("\tuae_u32 %s = %s;\n", name, reg);
		return;
	default:
		abort();
	}

	/* We get here for all non-reg non-immediate addressing modes to
	 * actually fetch the value. */

	if(using_prefetch && using_exception_3 && getv != 0 && size != sz_byte) {
		printf("\tif (%sa & 1) {\n", name);
		if(using_prefetch)
			printf("\t\texception3 (opcode, m68k_getpc() + %d, %sa);\n", m68k_pc_offset + 2, name);
		else
			printf("\t\texception3 (opcode, m68k_getpc() + %d, %sa);\n", m68k_pc_offset_last, name);
		printf("\t\tgoto %s;\n", endlabelstr);
		printf("\t}\n");
		need_endlabel = 1;
		start_brace();
	}

	if(flags & GF_PREFETCH) fill_prefetch_next();

	if(getv == 1) {
		switch(size) {
		case sz_byte:
			insn_n_cycles += 4;
			break;
		case sz_word:
			insn_n_cycles += 4;
			break;
		case sz_long:
			insn_n_cycles += 8;
			break;
		default:
			abort();
		}
		start_brace();
		switch(size) {
		case sz_byte:
			printf("\tuae_s8 %s = get_byte(%sa);\n", name, name);
			break;
		case sz_word:
			printf("\tuae_s16 %s = get_word(%sa);\n", name, name);
			break;
		case sz_long:
			printf("\tuae_s32 %s = get_long(%sa);\n", name, name);
			break;
		default:
			abort();
		}
	}

	/* We now might have to fix up the register for pre-dec or post-inc
	 * addressing modes. */
	if(!movem) switch(mode) {
		case Aipi:
			switch(size) {
			case sz_byte:
				printf("\tm68k_areg(regs, %s) += areg_byteinc[%s];\n", reg, reg);
				break;
			case sz_word:
				printf("\tm68k_areg(regs, %s) += 2;\n", reg);
				break;
			case sz_long:
				printf("\tm68k_areg(regs, %s) += 4;\n", reg);
				break;
			default:
				abort();
			}
			break;
		case Apdi:
			printf("\tm68k_areg (regs, %s) = %sa;\n", reg, name);
			break;
		default:
			break;
		}
}

static void genastore(char *from, amodes mode, char *reg, wordsizes size, char *to) {
	switch(mode) {
	case Dreg:
		switch(size) {
		case sz_byte:
			printf("\tm68k_dreg(regs, %s) = (m68k_dreg(regs, %s) & ~0xff) | ((%s) & 0xff);\n", reg, reg, from);
			break;
		case sz_word:
			printf("\tm68k_dreg(regs, %s) = (m68k_dreg(regs, %s) & ~0xffff) | ((%s) & 0xffff);\n",
			       reg, reg, from);
			break;
		case sz_long:
			printf("\tm68k_dreg(regs, %s) = (%s);\n", reg, from);
			break;
		default:
			abort();
		}
		break;
	case Areg:
		switch(size) {
		case sz_word:
			printf("\tm68k_areg(regs, %s) = (uae_s32)(uae_s16)(%s);\n", reg, from);
			break;
		case sz_long:
			printf("\tm68k_areg(regs, %s) = (%s);\n", reg, from);
			break;
		default:
			abort();
		}
		break;
	case Aind:
	case Aipi:
	case Apdi:
	case Ad16:
	case Ad8r:
	case absw:
	case absl:
	case PC16:
	case PC8r:
		if(using_prefetch) sync_m68k_pc();
		switch(size) {
		case sz_byte:
			insn_n_cycles += 4;
			printf("\tput_byte(%sa,%s);\n", to, from);
			break;
		case sz_word:
			insn_n_cycles += 4;
			if(cpu_level < 2 && (mode == PC16 || mode == PC8r)) abort();
			printf("\tput_word(%sa,%s);\n", to, from);
			break;
		case sz_long:
			insn_n_cycles += 8;
			if(cpu_level < 2 && (mode == PC16 || mode == PC8r)) abort();
			printf("\tput_long(%sa,%s);\n", to, from);
			break;
		default:
			abort();
		}
		break;
	case imm:
	case imm0:
	case imm1:
	case imm2:
	case immi:
		abort();
		break;
	default:
		abort();
	}
}

static void genmovemel(uae_u16 opcode) {
	char getcode[100];
	int size = table68k[opcode].size == sz_long ? 4 : 2;

	if(table68k[opcode].size == sz_long) {
		strcpy(getcode, "get_long(srca)");
	} else {
		strcpy(getcode, "(uae_s32)(uae_s16)get_word(srca)");
	}

	printf("\tuae_u16 mask = %s;\n", gen_nextiword(0));
	printf("\tunsigned int dmask = mask & 0xff, amask = (mask >> 8) & 0xff;\n");
	genamode(table68k[opcode].dmode, "dstreg", table68k[opcode].size, "src", 2, 1, 0);
	start_brace();
	printf("\twhile (dmask) { m68k_dreg(regs, movem_index1[dmask]) = %s; srca += %d; dmask = "
	       "movem_next[dmask]; }\n",
	       getcode, size);
	printf("\twhile (amask) { m68k_areg(regs, movem_index1[amask]) = %s; srca += %d; amask = "
	       "movem_next[amask]; }\n",
	       getcode, size);

	if(table68k[opcode].dmode == Aipi) printf("\tm68k_areg(regs, dstreg) = srca;\n");
}

static void genmovemle(uae_u16 opcode) {
	char putcode[100];
	int size = table68k[opcode].size == sz_long ? 4 : 2;
	if(table68k[opcode].size == sz_long) {
		strcpy(putcode, "put_long(srca,");
	} else {
		strcpy(putcode, "put_word(srca,");
	}

	printf("\tuae_u16 mask = %s;\n", gen_nextiword(0));
	genamode(table68k[opcode].dmode, "dstreg", table68k[opcode].size, "src", 2, 1, 0);
	if(using_prefetch) sync_m68k_pc();

	start_brace();
	if(table68k[opcode].dmode == Apdi) {
		printf("\tuae_u16 amask = mask & 0xff, dmask = (mask >> 8) & 0xff;\n");
		printf("\twhile (amask) { srca -= %d; %s m68k_areg(regs, movem_index2[amask])); amask = "
		       "movem_next[amask]; }\n",
		       size, putcode);
		printf("\twhile (dmask) { srca -= %d; %s m68k_dreg(regs, movem_index2[dmask])); dmask = "
		       "movem_next[dmask]; }\n",
		       size, putcode);
		printf("\tm68k_areg(regs, dstreg) = srca;\n");
	} else {
		printf("\tuae_u16 dmask = mask & 0xff, amask = (mask >> 8) & 0xff;\n");
		printf("\twhile (dmask) { %s m68k_dreg(regs, movem_index1[dmask])); srca += %d; dmask = "
		       "movem_next[dmask]; }\n",
		       putcode, size);
		printf("\twhile (amask) { %s m68k_areg(regs, movem_index1[amask])); srca += %d; amask = "
		       "movem_next[amask]; }\n",
		       putcode, size);
	}
}

static void duplicate_carry(int n) {
	while(n-- != 0)
		printf("\t");
	printf("COPY_CARRY;\n");
}

typedef enum {
	flag_logical_noclobber,
	flag_logical,
	flag_add,
	flag_sub,
	flag_cmp,
	flag_addx,
	flag_subx,
	flag_zn,
	flag_av,
	flag_sv
} flagtypes;

static void genflags_normal(flagtypes type, wordsizes size, char *value, char *src, char *dst) {
	char vstr[100], sstr[100], dstr[100];
	char usstr[100], udstr[100];
	char unsstr[100], undstr[100];

	switch(size) {
	case sz_byte:
		strcpy(vstr, "((uae_s8)(");
		strcpy(usstr, "((uae_u8)(");
		break;
	case sz_word:
		strcpy(vstr, "((uae_s16)(");
		strcpy(usstr, "((uae_u16)(");
		break;
	case sz_long:
		strcpy(vstr, "((uae_s32)(");
		strcpy(usstr, "((uae_u32)(");
		break;
	default:
		abort();
	}
	strcpy(unsstr, usstr);

	strcpy(sstr, vstr);
	strcpy(dstr, vstr);
	strcat(vstr, value);
	strcat(vstr, "))");
	strcat(dstr, dst);
	strcat(dstr, "))");
	strcat(sstr, src);
	strcat(sstr, "))");

	strcpy(udstr, usstr);
	strcat(udstr, dst);
	strcat(udstr, "))");
	strcat(usstr, src);
	strcat(usstr, "))");

	strcpy(undstr, unsstr);
	strcat(unsstr, "-");
	strcat(undstr, "~");
	strcat(undstr, dst);
	strcat(undstr, "))");
	strcat(unsstr, src);
	strcat(unsstr, "))");

	switch(type) {
	case flag_logical_noclobber:
	case flag_logical:
	case flag_zn:
	case flag_av:
	case flag_sv:
	case flag_addx:
	case flag_subx:
		break;

	case flag_add:
		start_brace();
		printf("uae_u32 %s = %s + %s;\n", value, dstr, sstr);
		break;
	case flag_sub:
	case flag_cmp:
		start_brace();
		printf("uae_u32 %s = %s - %s;\n", value, dstr, sstr);
		break;
	}

	switch(type) {
	case flag_logical_noclobber:
	case flag_logical:
	case flag_zn:
		break;

	case flag_add:
	case flag_sub:
	case flag_addx:
	case flag_subx:
	case flag_cmp:
	case flag_av:
	case flag_sv:
		start_brace();
		printf("\t" BOOL_TYPE " flgs = %s < 0;\n", sstr);
		printf("\t" BOOL_TYPE " flgo = %s < 0;\n", dstr);
		printf("\t" BOOL_TYPE " flgn = %s < 0;\n", vstr);
		break;
	}

	switch(type) {
	case flag_logical:
		printf("\tCLEAR_CZNV;\n");
		printf("\tSET_ZFLG (%s == 0);\n", vstr);
		printf("\tSET_NFLG (%s < 0);\n", vstr);
		break;
	case flag_logical_noclobber:
		printf("\tSET_ZFLG (%s == 0);\n", vstr);
		printf("\tSET_NFLG (%s < 0);\n", vstr);
		break;
	case flag_av:
		printf("\tSET_VFLG ((flgs ^ flgn) & (flgo ^ flgn));\n");
		break;
	case flag_sv:
		printf("\tSET_VFLG ((flgs ^ flgo) & (flgn ^ flgo));\n");
		break;
	case flag_zn:
		printf("\tSET_ZFLG (GET_ZFLG & (%s == 0));\n", vstr);
		printf("\tSET_NFLG (%s < 0);\n", vstr);
		break;
	case flag_add:
		printf("\tSET_ZFLG (%s == 0);\n", vstr);
		printf("\tSET_VFLG ((flgs ^ flgn) & (flgo ^ flgn));\n");
		printf("\tSET_CFLG (%s < %s);\n", undstr, usstr);
		duplicate_carry(0);
		printf("\tSET_NFLG (flgn != 0);\n");
		break;
	case flag_sub:
		printf("\tSET_ZFLG (%s == 0);\n", vstr);
		printf("\tSET_VFLG ((flgs ^ flgo) & (flgn ^ flgo));\n");
		printf("\tSET_CFLG (%s > %s);\n", usstr, udstr);
		duplicate_carry(0);
		printf("\tSET_NFLG (flgn != 0);\n");
		break;
	case flag_addx:
		printf("\tSET_VFLG ((flgs ^ flgn) & (flgo ^ flgn));\n");          /* minterm SON: 0x42 */
		printf("\tSET_CFLG (flgs ^ ((flgs ^ flgo) & (flgo ^ flgn)));\n"); /* minterm SON: 0xD4 */
		duplicate_carry(0);
		break;
	case flag_subx:
		printf("\tSET_VFLG ((flgs ^ flgo) & (flgo ^ flgn));\n");          /* minterm SON: 0x24 */
		printf("\tSET_CFLG (flgs ^ ((flgs ^ flgn) & (flgo ^ flgn)));\n"); /* minterm SON: 0xB2 */
		duplicate_carry(0);
		break;
	case flag_cmp:
		printf("\tSET_ZFLG (%s == 0);\n", vstr);
		printf("\tSET_VFLG ((flgs != flgo) && (flgn != flgo));\n");
		printf("\tSET_CFLG (%s > %s);\n", usstr, udstr);
		printf("\tSET_NFLG (flgn != 0);\n");
		break;
	}
}

static void genflags(flagtypes type, wordsizes size, char *value, char *src, char *dst) {
	/* Temporarily deleted 68k/ARM flag optimizations.  I'd prefer to have
	   them in the appropriate m68k.h files and use just one copy of this
	   code here.  The API can be changed if necessary.  */
#ifdef OPTIMIZED_FLAGS
	switch(type) {
	case flag_add:
	case flag_sub:
		start_brace();
		printf("\tuae_u32 %s;\n", value);
		break;

	default:
		break;
	}

	/* At least some of those casts are fairly important! */
	switch(type) {
	case flag_logical_noclobber:
		printf("\t{uae_u32 oldcznv = GET_CZNV & ~(FLAGVAL_Z | FLAGVAL_N);\n");
		if(strcmp(value, "0") == 0) {
			printf("\tSET_CZNV (olcznv | FLAGVAL_Z);\n");
		} else {
			switch(size) {
			case sz_byte:
				printf("\toptflag_testb ((uae_s8)(%s));\n", value);
				break;
			case sz_word:
				printf("\toptflag_testw ((uae_s16)(%s));\n", value);
				break;
			case sz_long:
				printf("\toptflag_testl ((uae_s32)(%s));\n", value);
				break;
			}
			printf("\tIOR_CZNV (oldcznv);\n");
		}
		printf("\t}\n");
		return;
	case flag_logical:
		if(strcmp(value, "0") == 0) {
			printf("\tSET_CZNV (FLAGVAL_Z);\n");
		} else {
			switch(size) {
			case sz_byte:
				printf("\toptflag_testb ((uae_s8)(%s));\n", value);
				break;
			case sz_word:
				printf("\toptflag_testw ((uae_s16)(%s));\n", value);
				break;
			case sz_long:
				printf("\toptflag_testl ((uae_s32)(%s));\n", value);
				break;
			}
		}
		return;

	case flag_add:
		switch(size) {
		case sz_byte:
			printf("\toptflag_addb (%s, (uae_s8)(%s), (uae_s8)(%s));\n", value, src, dst);
			break;
		case sz_word:
			printf("\toptflag_addw (%s, (uae_s16)(%s), (uae_s16)(%s));\n", value, src, dst);
			break;
		case sz_long:
			printf("\toptflag_addl (%s, (uae_s32)(%s), (uae_s32)(%s));\n", value, src, dst);
			break;
		}
		return;

	case flag_sub:
		switch(size) {
		case sz_byte:
			printf("\toptflag_subb (%s, (uae_s8)(%s), (uae_s8)(%s));\n", value, src, dst);
			break;
		case sz_word:
			printf("\toptflag_subw (%s, (uae_s16)(%s), (uae_s16)(%s));\n", value, src, dst);
			break;
		case sz_long:
			printf("\toptflag_subl (%s, (uae_s32)(%s), (uae_s32)(%s));\n", value, src, dst);
			break;
		}
		return;

	case flag_cmp:
		switch(size) {
		case sz_byte:
			printf("\toptflag_cmpb ((uae_s8)(%s), (uae_s8)(%s));\n", src, dst);
			break;
		case sz_word:
			printf("\toptflag_cmpw ((uae_s16)(%s), (uae_s16)(%s));\n", src, dst);
			break;
		case sz_long:
			printf("\toptflag_cmpl ((uae_s32)(%s), (uae_s32)(%s));\n", src, dst);
			break;
		}
		return;

	default:
		break;
	}
#endif

	genflags_normal(type, size, value, src, dst);
}

static void force_range_for_rox(const char *var, wordsizes size) {
	/* Could do a modulo operation here... which one is faster? */
	switch(size) {
	case sz_long:
		printf("\tif (%s >= 33) %s -= 33;\n", var, var);
		break;
	case sz_word:
		printf("\tif (%s >= 34) %s -= 34;\n", var, var);
		printf("\tif (%s >= 17) %s -= 17;\n", var, var);
		break;
	case sz_byte:
		printf("\tif (%s >= 36) %s -= 36;\n", var, var);
		printf("\tif (%s >= 18) %s -= 18;\n", var, var);
		printf("\tif (%s >= 9) %s -= 9;\n", var, var);
		break;
	}
}

static const char *cmask(wordsizes size) {
	switch(size) {
	case sz_byte:
		return "0x80";
	case sz_word:
		return "0x8000";
	case sz_long:
		return "0x80000000";
	default:
		abort();
	}
}

static int source_is_imm1_8(struct instr *i) {
	return i->stype == 3;
}

static void gen_opcode(unsigned long int opcode) {
	struct instr *curi = table68k + opcode;
	int tmpc = 0;
	insn_n_cycles = 4;

	start_brace();
#if 0
    printf ("uae_u8 *m68k_pc = regs.pc_p;\n");
#endif
	m68k_pc_offset = 2;
	switch(curi->plev) {
	case 0: /* not privileged */
		break;
	case 1: /* unprivileged only on 68000 */
		if(cpu_level == 0) break;
		if(next_cpu_level < 0) next_cpu_level = 0;

	/* fall through */
	case 2: /* priviledged */
		printf("if (!regs.s) { Exception(8,0); goto %s; }\n", endlabelstr);
		need_endlabel = 1;
		start_brace();
		break;
	case 3: /* privileged if size == word */
		if(curi->size == sz_byte) break;
		printf("if (!regs.s) { Exception(8,0); goto %s; }\n", endlabelstr);
		need_endlabel = 1;
		start_brace();
		break;
	}
	switch(curi->mnemo) {
	case i_OR:
	case i_AND:
	case i_EOR:
	case i_ORI:
	case i_ANDI:
	case i_EORI:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, 0);
		printf("\tsrc %c= dst;\n", (curi->mnemo == i_OR || curi->mnemo == i_ORI) ?
		                           '|' :
		                           (curi->mnemo == i_AND || curi->mnemo == i_ANDI) ? '&' : '^');
		genflags(flag_logical, curi->size, "src", "", "");
		if(curi->size == sz_long && isreg(curi->dmode))
			tmpc += curi->mnemo == (i_AND || i_ANDI) ? 2 : 4;
		fill_prefetch_next_delay(tmpc);
		genastore("src", curi->dmode, "dstreg", curi->size, "dst");
		break;
	case i_ORSR:
	case i_EORSR:
		printf("\tMakeSR();\n");
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		if(curi->size == sz_byte) {
			printf("\tsrc &= 0xFF;\n");
		}
		fill_prefetch_next_delay(4);
		printf("\tregs.sr %c= src;\n", curi->mnemo == i_EORSR ? '^' : '|');
		printf("\tMakeFromSR();\n");
		break;
	case i_ANDSR:
		printf("\tMakeSR();\n");
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		if(curi->size == sz_byte) {
			printf("\tsrc |= 0xFF00;\n");
		}
		fill_prefetch_next_delay(4);
		printf("\tregs.sr &= src;\n");
		printf("\tMakeFromSR();\n");
		break;
	case i_SUB:
	case i_SUBQ:
	case i_SUBI:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, 0);
		if(isreg(curi->dmode)) {
			if(curi->dmode == Dreg && curi->size == sz_long)
				tmpc += (curi->smode == imm || curi->smode == immi) ? 4 : 2;
		}
		fill_prefetch_next_delay(tmpc);
		start_brace();
		genflags(flag_sub, curi->size, "newv", "src", "dst");
		genastore("newv", curi->dmode, "dstreg", curi->size, "dst");
		break;
	case i_SUBA:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", sz_long, "dst", 1, 0, 0);
		if(isreg(curi->dmode) && curi->dmode == Areg) {
			tmpc += curi->size == sz_long ? 2 : 4;
			if(curi->size == sz_long) tmpc += (isreg(curi->smode) || curi->smode == imm) ? 2 : 0;
		}
		fill_prefetch_next_delay(tmpc);
		start_brace();
		printf("\tuae_u32 newv = dst - src;\n");
		genastore("newv", curi->dmode, "dstreg", sz_long, "dst");
		break;
	case i_SUBX:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, GF_AA);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, GF_AA);
		if((isreg(curi->smode) && curi->size == sz_long) || !isreg(curi->smode)) tmpc += 2;
		fill_prefetch_next_delay(tmpc);
		start_brace();
		printf("\tuae_u32 newv = dst - src - (GET_XFLG ? 1 : 0);\n");
		genflags(flag_subx, curi->size, "newv", "src", "dst");
		genflags(flag_zn, curi->size, "newv", "", "");
		genastore("newv", curi->dmode, "dstreg", curi->size, "dst");
		break;
	case i_SBCD:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, GF_AA);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, GF_AA);
		fill_prefetch_next();
		start_brace();
		printf("\tuae_u16 newv_lo = (dst & 0xF) - (src & 0xF) - (GET_XFLG ? 1 : 0);\n");
		printf("\tuae_u16 newv_hi = (dst & 0xF0) - (src & 0xF0);\n");
		printf("\tuae_u16 newv, tmp_newv;\n");
		printf("\tint bcd = 0;\n");
		printf("\tnewv = tmp_newv = newv_hi + newv_lo;\n");
		printf("\tif (newv_lo & 0xF0) { newv -= 6; bcd = 6; };\n");
		printf("\tif ((((dst & 0xFF) - (src & 0xFF) - (GET_XFLG ? 1 : 0)) & 0x100) > 0xFF) { newv "
		       "-= 0x60; }\n");
		printf("\tSET_CFLG ((((dst & 0xFF) - (src & 0xFF) - bcd - (GET_XFLG ? 1 : 0)) & 0x300) > "
		       "0xFF);\n");
		duplicate_carry(0);
		genflags(flag_zn, curi->size, "newv", "", "");
		printf("\tSET_VFLG ((tmp_newv & 0x80) != 0 && (newv & 0x80) == 0);\n");
		genastore("newv", curi->dmode, "dstreg", curi->size, "dst");
		break;
	case i_ADD:
	case i_ADDQ:
	case i_ADDI:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, 0);
		if(isreg(curi->dmode)) {
			if(curi->dmode == Dreg && curi->size == sz_long)
				tmpc += (curi->smode == imm || curi->smode == immi) ? 4 : 2;
		}
		fill_prefetch_next_delay(tmpc);
		start_brace();
		genflags(flag_add, curi->size, "newv", "src", "dst");
		genastore("newv", curi->dmode, "dstreg", curi->size, "dst");
		break;
	case i_ADDA:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", sz_long, "dst", 1, 0, 0);
		if(isreg(curi->dmode) && curi->dmode == Areg) {
			tmpc += curi->size == sz_long ? 2 : 4;
			if(curi->size == sz_long) tmpc += (isreg(curi->smode) || curi->smode == imm) ? 2 : 0;
		}
		fill_prefetch_next_delay(tmpc);
		start_brace();
		printf("\tuae_u32 newv = dst + src;\n");
		genastore("newv", curi->dmode, "dstreg", sz_long, "dst");
		break;
	case i_ADDX:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, GF_AA);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, GF_AA);
		if((isreg(curi->smode) && curi->size == sz_long) || !isreg(curi->smode)) tmpc += 2;
		fill_prefetch_next_delay(tmpc);
		start_brace();
		printf("\tuae_u32 newv = dst + src + (GET_XFLG ? 1 : 0);\n");
		genflags(flag_addx, curi->size, "newv", "src", "dst");
		genflags(flag_zn, curi->size, "newv", "", "");
		genastore("newv", curi->dmode, "dstreg", curi->size, "dst");
		break;
	case i_ABCD:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, GF_AA);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, GF_AA);
		fill_prefetch_next();
		start_brace();
		printf("\tuae_u16 newv_lo = (src & 0xF) + (dst & 0xF) + (GET_XFLG ? 1 : 0);\n");
		printf("\tuae_u16 newv_hi = (src & 0xF0) + (dst & 0xF0);\n");
		printf("\tuae_u16 newv, tmp_newv;\n");
		printf("\tint cflg;\n");
		printf("\tnewv = tmp_newv = newv_hi + newv_lo;");
		printf("\tif (newv_lo > 9) { newv += 6; }\n");
		printf("\tcflg = (newv & 0x3F0) > 0x90;\n");
		printf("\tif (cflg) newv += 0x60;\n");
		printf("\tSET_CFLG (cflg);\n");
		duplicate_carry(0);
		genflags(flag_zn, curi->size, "newv", "", "");
		printf("\tSET_VFLG ((tmp_newv & 0x80) == 0 && (newv & 0x80) != 0);\n");
		genastore("newv", curi->dmode, "dstreg", curi->size, "dst");
		break;
	case i_NEG:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		if(isreg(curi->smode) && curi->size == sz_long) tmpc += 2;
		fill_prefetch_next_delay(tmpc);
		start_brace();
		genflags(flag_sub, curi->size, "dst", "src", "0");
		genastore("dst", curi->smode, "srcreg", curi->size, "src");
		break;
	case i_NEGX:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		if(isreg(curi->smode) && curi->size == sz_long) tmpc += 2;
		fill_prefetch_next_delay(tmpc);
		start_brace();
		printf("\tuae_u32 newv = 0 - src - (GET_XFLG ? 1 : 0);\n");
		genflags(flag_subx, curi->size, "newv", "src", "0");
		genflags(flag_zn, curi->size, "newv", "", "");
		genastore("newv", curi->smode, "srcreg", curi->size, "src");
		break;
	case i_NBCD:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		if(isreg(curi->smode)) tmpc += 2;
		fill_prefetch_next_delay(tmpc);
		start_brace();
		printf("\tuae_u16 newv_lo = - (src & 0xF) - (GET_XFLG ? 1 : 0);\n");
		printf("\tuae_u16 newv_hi = - (src & 0xF0);\n");
		printf("\tuae_u16 newv, tmp_newv;\n");
		printf("\tint bcd = 0;\n");
		printf("\tnewv = tmp_newv = newv_hi + newv_lo;");
		printf("\tif (newv_lo & 0xF0) { newv -= 6; bcd = 6; };\n");
		printf("\tif (((- (src & 0xFF) - (GET_XFLG ? 1 : 0)) & 0x100) > 0xFF) { newv -= 0x60; }\n");
		printf("\tSET_CFLG (((- (src & 0xFF) - bcd - (GET_XFLG ? 1 : 0)) & 0x300) > 0xFF);\n");
		duplicate_carry(0);
		genflags(flag_zn, curi->size, "newv", "", "");
		printf("\tSET_VFLG ((tmp_newv & 0x80) != 0 && (newv & 0x80) == 0);\n");
		genastore("newv", curi->smode, "srcreg", curi->size, "src");
		break;
	case i_CLR:
		genamode(curi->smode, "srcreg", curi->size, "src", cpu_level == 0 ? 1 : 2, 0, 0);
		if(isreg(curi->smode) && curi->size == sz_long) tmpc += 2;
		fill_prefetch_next_delay(tmpc);
		genflags(flag_logical, curi->size, "0", "", "");
		genastore("0", curi->smode, "srcreg", curi->size, "src");
		break;
	case i_NOT:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		if(isreg(curi->smode) && curi->size == sz_long) tmpc += 2;
		fill_prefetch_next_delay(tmpc);
		start_brace();
		printf("\tuae_u32 dst = ~src;\n");
		genflags(flag_logical, curi->size, "dst", "", "");
		genastore("dst", curi->smode, "srcreg", curi->size, "src");
		break;
	case i_TST:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		fill_prefetch_next();
		genflags(flag_logical, curi->size, "src", "", "");
		break;
	case i_BTST:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, 0);
		if(isreg(curi->dmode)) tmpc += 2;
		fill_prefetch_next_delay(tmpc);
		if(curi->size == sz_byte)
			printf("\tsrc &= 7;\n");
		else
			printf("\tsrc &= 31;\n");
		printf("\tSET_ZFLG (1 ^ ((dst >> src) & 1));\n");
		break;
	case i_BCHG:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, 0);
		if(isreg(curi->dmode)) tmpc += 4;
		fill_prefetch_next_delay(tmpc);
		if(curi->size == sz_byte)
			printf("\tsrc &= 7;\n");
		else
			printf("\tsrc &= 31;\n");
		printf("\tdst ^= (1 << src);\n");
		printf("\tSET_ZFLG (((uae_u32)dst & (1 << src)) >> src);\n");
		genastore("dst", curi->dmode, "dstreg", curi->size, "dst");
		break;
	case i_BCLR:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, 0);
		if(isreg(curi->dmode)) tmpc += 4;
		fill_prefetch_next_delay(tmpc);
		if(curi->size == sz_byte)
			printf("\tsrc &= 7;\n");
		else
			printf("\tsrc &= 31;\n");
		printf("\tSET_ZFLG (1 ^ ((dst >> src) & 1));\n");
		printf("\tdst &= ~(1 << src);\n");
		genastore("dst", curi->dmode, "dstreg", curi->size, "dst");
		break;
	case i_BSET:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, 0);
		if(isreg(curi->dmode)) tmpc += 4;
		fill_prefetch_next_delay(tmpc);
		if(curi->size == sz_byte)
			printf("\tsrc &= 7;\n");
		else
			printf("\tsrc &= 31;\n");
		printf("\tSET_ZFLG (1 ^ ((dst >> src) & 1));\n");
		printf("\tdst |= (1 << src);\n");
		genastore("dst", curi->dmode, "dstreg", curi->size, "dst");
		break;
	case i_CMPM:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, GF_AA);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, GF_AA);
		fill_prefetch_next();
		start_brace();
		genflags(flag_cmp, curi->size, "newv", "src", "dst");
		break;
	case i_CMP:
	case i_CMPI:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, 0);
		if(isreg(curi->dmode)) {
			if(curi->dmode == Areg || (curi->dmode == Dreg && curi->size == sz_long)) tmpc += 2;
		}
		fill_prefetch_next_delay(tmpc);
		start_brace();
		genflags(flag_cmp, curi->size, "newv", "src", "dst");
		break;
	case i_CMPA:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", sz_long, "dst", 1, 0, 0);
		if(isreg(curi->dmode)) {
			if(curi->dmode == Areg || (curi->dmode == Dreg && curi->size == sz_long)) tmpc += 2;
		}
		fill_prefetch_next_delay(tmpc);
		start_brace();
		genflags(flag_cmp, sz_long, "newv", "src", "dst");
		break;
	/* The next two are coded a little unconventional, but they are doing
	 * weird things... */
	case i_MVPRM:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		printf("\tuaecptr memp = m68k_areg(regs, dstreg) + (uae_s32)(uae_s16)%s;\n", gen_nextiword(0));
		fill_prefetch_next();
		if(curi->size == sz_word) {
			printf("\tput_byte(memp, src >> 8); put_byte(memp + 2, src);\n");
		} else {
			printf("\tput_byte(memp, src >> 24); put_byte(memp + 2, src >> 16);\n");
			printf("\tput_byte(memp + 4, src >> 8); put_byte(memp + 6, src);\n");
		}
		break;
	case i_MVPMR:
		printf("\tuaecptr memp = m68k_areg(regs, srcreg) + (uae_s32)(uae_s16)%s;\n", gen_nextiword(0));
		genamode(curi->dmode, "dstreg", curi->size, "dst", 2, 0, 0);
		if(curi->size == sz_word) {
			printf("\tuae_u16 val = (get_byte(memp) << 8) + get_byte(memp + 2);\n");
		} else {
			printf("\tuae_u32 val = (get_byte(memp) << 24) + (get_byte(memp + 2) << 16)\n");
			printf("              + (get_byte(memp + 4) << 8) + get_byte(memp + 6);\n");
		}
		fill_prefetch_next();
		genastore("val", curi->dmode, "dstreg", curi->size, "dst");
		break;
	case i_MOVE:
	case i_MOVEQ:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 2, 0, GF_APDI);
		genastore("src", curi->dmode, "dstreg", curi->size, "dst");
		genflags(flag_logical, curi->size, "src", "", "");
		sync_m68k_pc();
		fill_prefetch_next();
		break;
	case i_MOVEA:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 2, 0, GF_APDI);
		if(curi->size == sz_word) {
			printf("\tuae_u32 val = (uae_s32)(uae_s16)src;\n");
		} else {
			printf("\tuae_u32 val = src;\n");
		}
		fill_prefetch_next();
		genastore("val", curi->dmode, "dstreg", sz_long, "dst");
		break;
	case i_MVSR2:
		genamode(curi->smode, "srcreg", sz_word, "src", 2, 0, 0);
		if(isreg(curi->smode)) tmpc += 2;
		fill_prefetch_next_delay(tmpc);
		printf("\tMakeSR();\n");
		if(curi->size == sz_byte)
			genastore("regs.sr & 0xff", curi->smode, "srcreg", sz_word, "src");
		else
			genastore("regs.sr", curi->smode, "srcreg", sz_word, "src");
		break;
	case i_MV2SR:
		genamode(curi->smode, "srcreg", sz_word, "src", 1, 0, 0);
		if(curi->size == sz_byte) {
			tmpc += 8;
			printf("\tMakeSR();\n\tregs.sr &= 0xFF00;\n\tregs.sr |= src & 0xFF;\n");
		} else {
			tmpc += 4;
			printf("\tregs.sr = src;\n");
		}
		fill_prefetch_next_delay(tmpc);
		printf("\tMakeFromSR();\n");
		break;
	case i_SWAP:
		genamode(curi->smode, "srcreg", sz_long, "src", 1, 0, 0);
		fill_prefetch_next();
		start_brace();
		printf("\tuae_u32 dst = ((src >> 16)&0xFFFF) | ((src&0xFFFF)<<16);\n");
		genflags(flag_logical, sz_long, "dst", "", "");
		genastore("dst", curi->smode, "srcreg", sz_long, "src");
		break;
	case i_EXG:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, 0);
		fill_prefetch_next_delay(2);
		genastore("dst", curi->smode, "srcreg", curi->size, "src");
		genastore("src", curi->dmode, "dstreg", curi->size, "dst");
		break;
	case i_EXT:
		genamode(curi->smode, "srcreg", sz_long, "src", 1, 0, 0);
		fill_prefetch_next();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u32 dst = (uae_s32)(uae_s8)src;\n");
			break;
		case sz_word:
			printf("\tuae_u16 dst = (uae_s16)(uae_s8)src;\n");
			break;
		case sz_long:
			printf("\tuae_u32 dst = (uae_s32)(uae_s16)src;\n");
			break;
		default:
			abort();
		}
		genflags(flag_logical, curi->size == sz_word ? sz_word : sz_long, "dst", "", "");
		genastore("dst", curi->smode, "srcreg", curi->size == sz_word ? sz_word : sz_long, "src");
		break;
	case i_MVMEL:
		genmovemel(opcode);
		fill_prefetch_next();
		break;
	case i_MVMLE:
		genmovemle(opcode);
		fill_prefetch_next();
		break;
	case i_TRAP:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		sync_m68k_pc();
		printf("\tException(src+32,0);\n");
		did_prefetch = 1;
		m68k_pc_offset = 0;
		break;
	case i_MVR2USP:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		fill_prefetch_next();
		printf("\tregs.usp = src;\n");
		break;
	case i_MVUSP2R:
		genamode(curi->smode, "srcreg", curi->size, "src", 2, 0, 0);
		fill_prefetch_next();
		genastore("regs.usp", curi->smode, "srcreg", curi->size, "src");
		break;
	case i_RESET:
		fill_prefetch_next();
		/*printf ("\tcpureset();\n");*/
		break;
	case i_NOP:
		fill_prefetch_next();
		break;
	case i_STOP:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		printf("\tregs.sr = src;\n");
		printf("\tMakeFromSR();\n");
		printf("\tm68k_setstopped(1);\n");
		sync_m68k_pc();
		fill_prefetch_full();
		break;
	case i_RTE:
		if(cpu_level == 0) {
			genamode(Aipi, "7", sz_word, "sr", 1, 0, GF_NOREFILL);
			genamode(Aipi, "7", sz_long, "pc", 1, 0, GF_NOREFILL);
			printf("\tregs.sr = sr; m68k_setpc (pc);\n");
			printf("\tMakeFromSR();\n");
		} else {
			int old_brace_level = n_braces;
			if(next_cpu_level < 0) next_cpu_level = 0;
			printf("\tuae_u16 newsr; uae_u32 newpc; for (;;) {\n");
			genamode(Aipi, "7", sz_word, "sr", 1, 0, 0);
			genamode(Aipi, "7", sz_long, "pc", 1, 0, 0);
			genamode(Aipi, "7", sz_word, "format", 1, 0, 0);
			printf("\tnewsr = sr; newpc = pc;\n");
			printf("\tif ((format & 0xF000) == 0x0000) { break; }\n");
			printf("\telse if ((format & 0xF000) == 0x1000) { ; }\n");
			printf("\telse if ((format & 0xF000) == 0x2000) { m68k_areg(regs, 7) += 4; break; }\n");
			printf(
			"\telse if ((format & 0xF000) == 0x8000) { m68k_areg(regs, 7) += 50; break; }\n");
			printf(
			"\telse if ((format & 0xF000) == 0x9000) { m68k_areg(regs, 7) += 12; break; }\n");
			printf(
			"\telse if ((format & 0xF000) == 0xa000) { m68k_areg(regs, 7) += 24; break; }\n");
			printf(
			"\telse if ((format & 0xF000) == 0xb000) { m68k_areg(regs, 7) += 84; break; }\n");
			printf("\telse { Exception(14,0); goto %s; }\n", endlabelstr);
			printf("\tregs.sr = newsr; MakeFromSR();\n}\n");
			pop_braces(old_brace_level);
			printf("\tregs.sr = newsr; MakeFromSR();\n");
			printf("\tif (newpc & 1)\n");
			printf("\t\texception3 (0x%04.4X, m68k_getpc(), newpc);\n", opcode);
			printf("\telse\n");
			printf("\t\tm68k_setpc (newpc);\n");
			need_endlabel = 1;
		}
		/* PC is set and prefetch filled. */
		m68k_pc_offset = 0;
		fill_prefetch_full();
		break;
	case i_RTD:
		genamode(Aipi, "7", sz_long, "pc", 1, 0, 0);
		genamode(curi->smode, "srcreg", curi->size, "offs", 1, 0, 0);
		printf("\tm68k_areg(regs, 7) += offs;\n");
		printf("\tif (pc & 1)\n");
		printf("\t\texception3 (0x%04.4X, m68k_getpc(), pc);\n", opcode);
		printf("\telse\n");
		printf("\t\tm68k_setpc (pc);\n");
		/* PC is set and prefetch filled. */
		m68k_pc_offset = 0;
		fill_prefetch_full();
		break;
	case i_LINK:
		genamode(Apdi, "7", sz_long, "old", 2, 0, 0);
		genamode(curi->smode, "srcreg", sz_long, "src", 1, 0, 0);
		genastore("src", Apdi, "7", sz_long, "old");
		genastore("m68k_areg(regs, 7)", curi->smode, "srcreg", sz_long, "src");
		genamode(curi->dmode, "dstreg", curi->size, "offs", 1, 0, 0);
		printf("\tm68k_areg(regs, 7) += offs;\n");
		fill_prefetch_next();
		break;
	case i_UNLK:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		printf("\tm68k_areg(regs, 7) = src;\n");
		genamode(Aipi, "7", sz_long, "old", 1, 0, 0);
		fill_prefetch_next();
		genastore("old", curi->smode, "srcreg", curi->size, "src");
		break;
	case i_RTS:
		printf("\tm68k_do_rts();\n");
		m68k_pc_offset = 0;
		fill_prefetch_full();
		break;
	case i_TRAPV:
		sync_m68k_pc();
		printf("\tif (GET_VFLG) {\n");
		printf("\t\tException (7, m68k_getpc ());\n");
		printf("\t\tgoto %s;\n", endlabelstr);
		printf("\t}\n");
		fill_prefetch_next();
		need_endlabel = 1;
		break;
	case i_RTR:
		printf("\tMakeSR();\n");
		genamode(Aipi, "7", sz_word, "sr", 1, 0, 0);
		genamode(Aipi, "7", sz_long, "pc", 1, 0, 0);
		printf("\tregs.sr &= 0xFF00; sr &= 0xFF;\n");
		printf("\tregs.sr |= sr; m68k_setpc(pc);\n");
		printf("\tMakeFromSR();\n");
		m68k_pc_offset = 0;
		fill_prefetch_full();
		break;
	case i_JSR:
		genamode(curi->smode, "srcreg", curi->size, "src", 0, 0, GF_AA | GF_NOREFILL);
		start_brace();
		printf("\tuaecptr oldpc = m68k_getpc() + %d;\n", m68k_pc_offset);
		if(using_exception_3) {
			printf("\tif (srca & 1) {\n");
			printf("\t\texception3i (opcode, oldpc, srca);\n");
			printf("\t\tgoto %s;\n", endlabelstr);
			printf("\t}\n");
			need_endlabel = 1;
		}
		printf("\tm68k_setpc (srca);\n");
		m68k_pc_offset = 0;
		fill_prefetch_1(0, 0);
		printf("\tm68k_areg (regs, 7) -= 4;\n");
		printf("\tput_long (m68k_areg (regs, 7), oldpc);\n");
		fill_prefetch_next();
		break;
	case i_JMP:
		genamode(curi->smode, "srcreg", curi->size, "src", 0, 0, GF_AA | GF_NOREFILL);
		if(using_exception_3) {
			printf("\tif (srca & 1) {\n");
			printf("\t\texception3i (opcode, m68k_getpc() + 6, srca);\n");
			printf("\t\tgoto %s;\n", endlabelstr);
			printf("\t}\n");
			need_endlabel = 1;
		}
		printf("\tm68k_setpc(srca);\n");
		m68k_pc_offset = 0;
		fill_prefetch_full();
		break;
	case i_BSR:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, GF_AA | GF_NOREFILL);
		printf("{");
		printf("\tuae_s32 s = (uae_s32)src + 2;\n");
		if(using_exception_3) {
			printf("\tif (src & 1) {\n");
			printf("\t\texception3i (opcode, m68k_getpc() + 2, m68k_getpc() + s);\n");
			printf("\t\tgoto %s;\n", endlabelstr);
			printf("\t}\n");
			need_endlabel = 1;
		}
		printf("\tm68k_do_bsr(m68k_getpc() + %d, s);\n", m68k_pc_offset);
		printf("}");
		m68k_pc_offset = 0;
		fill_prefetch_full();
		break;
	case i_Bcc:
		if(curi->size == sz_long) {
			if(cpu_level < 2) {
				printf("\tif (cctrue(%d)) {\n", curi->cc, endlabelstr);
				printf("\t\texception3i (opcode, m68k_getpc() + 2, m68k_getpc() + 1);\n");
				printf("\t\tgoto %s;\n", endlabelstr);
				printf("\t}\n");
				sync_m68k_pc();
				irc2ir();
				fill_prefetch_2();
				printf("\tgoto %s;\n", endlabelstr);
				need_endlabel = 1;
			} else {
				if(next_cpu_level < 1) next_cpu_level = 1;
			}
		}
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, GF_AA | GF_NOREFILL);
		printf("\tif (!cctrue(%d)) goto didnt_jump;\n", curi->cc);
		if(using_exception_3) {
			printf("\tif (src & 1) {\n");
			printf(
			"\t\texception3i (opcode, m68k_getpc() + 2, m68k_getpc() + 2 + (uae_s32)src);\n");
			printf("\t\tgoto %s;\n", endlabelstr);
			printf("\t}\n");
			need_endlabel = 1;
		}
		if(using_prefetch) {
			if(curi->size == sz_byte) {
				printf("\tm68k_incpc ((uae_s32)src + 2);\n");
			} else {
				printf("\tm68k_incpc ((uae_s32)src + 2);\n");
			}
			fill_prefetch_full();
			if(using_ce)
				printf("\treturn;\n");
			else
				printf("\treturn 10 * %d;\n", CYCLE_UNIT / 2);
		} else {
			printf("\tm68k_incpc ((uae_s32)src + 2);\n");
			returncycles("\t", 10);
		}
		printf("didnt_jump:;\n");
		need_endlabel = 1;
		sync_m68k_pc();
		if(curi->size == sz_byte) {
			irc2ir();
			fill_prefetch_2();
		} else
			fill_prefetch_full();
		insn_n_cycles = curi->size == sz_byte ? 8 : 12;
		break;
	case i_LEA:
		genamode(curi->smode, "srcreg", curi->size, "src", 0, 0, GF_AA);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 2, 0, GF_AA);
		if(curi->smode == Ad8r || curi->smode == PC8r) tmpc += 4;
		fill_prefetch_next_delay(tmpc);
		genastore("srca", curi->dmode, "dstreg", curi->size, "dst");
		break;
	case i_PEA:
		genamode(curi->smode, "srcreg", curi->size, "src", 0, 0, GF_AA);
		genamode(Apdi, "7", sz_long, "dst", 2, 0, GF_AA);
		if(curi->smode == Ad8r || curi->smode == PC8r) tmpc += 4;
		fill_prefetch_next_delay(tmpc);
		genastore("srca", Apdi, "7", sz_long, "dst");
		break;
	case i_DBcc:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, GF_AA | GF_NOREFILL);
		genamode(curi->dmode, "dstreg", curi->size, "offs", 1, 0, GF_AA | GF_NOREFILL);

		printf("\tuaecptr oldpc = m68k_getpc();\n");
		printf("\tif (!cctrue(%d)) {\n", curi->cc);
		printf("\t\tm68k_incpc((uae_s32)offs + 2);\n");
		printf("\t");
		fill_prefetch_1(0, 0);
		printf("\t");
		genastore("(src-1)", curi->smode, "srcreg", curi->size, "src");

		printf("\t\tif (src) {\n");
		if(using_exception_3) {
			printf("\t\t\tif (offs & 1) {\n");
			printf("\t\t\t\texception3i (opcode, m68k_getpc() + 2, m68k_getpc() + 2 + "
			       "(uae_s32)offs + 2);\n");
			printf("\t\t\t\tgoto %s;\n", endlabelstr);
			printf("\t\t\t}\n");
			need_endlabel = 1;
		}
		irc2ir();
		fill_prefetch_1(2, 0);
		returncycles("\t\t\t", 12);
		printf("\t\t}\n");
		printf("\t} else {\n");
		printf("\t}\n");
		printf("\tm68k_setpc (oldpc + %d);\n", m68k_pc_offset);
		m68k_pc_offset = 0;
		fill_prefetch_full();
		insn_n_cycles = 12;
		need_endlabel = 1;
		break;
	case i_Scc:
		genamode(curi->smode, "srcreg", curi->size, "src", cpu_level == 0 ? 1 : 2, 0, 0);
		start_brace();
		fill_prefetch_next_cycles();
		printf("{");
		printf("\tint val = cctrue(%d) ? 0xff : 0;\n", curi->cc);
		genastore("val", curi->smode, "srcreg", curi->size, "src");
		printf("}");
		break;
	case i_DIVU:
		printf("\tuaecptr oldpc = m68k_getpc();\n");
		genamode(curi->smode, "srcreg", sz_word, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", sz_long, "dst", 1, 0, 0);
		fill_prefetch_next_cycles();
		sync_m68k_pc();
		printf("\tCLEAR_CZNV;\n");
		printf("\tif (src == 0) {\n");
		if(cpu_level > 0) {
			/* 68020 sets V when dividing by zero and N if dst is negative
			 * 68000 clears both
			 */
			printf("\t\tSET_VFLG (1);\n");
			printf("\t\tif (dst < 0) SET_NFLG (1);\n");
		}
		printf("\t\tException (5, oldpc);\n");
		printf("\t\tgoto %s;\n", endlabelstr);
		printf("\t} else {\n");
		printf("\t\tuae_u32 newv = (uae_u32)dst / (uae_u32)(uae_u16)src;\n");
		printf("\t\tuae_u32 rem = (uae_u32)dst %% (uae_u32)(uae_u16)src;\n");
		if(using_ce) {
			printf("\t\tint cycles = 138 * %d - lostcycles;\n", CYCLE_UNIT / 2);
		}
		/* The N flag appears to be set each time there is an overflow.
		 * Weird. but 68020 only sets N when dst is negative.. */
		printf("\t\tif (newv > 0xffff) {\n");
		printf("\t\t\tSET_VFLG (1);\n");
#ifdef UNDEF68020
		if(cpu_level >= 2)
			printf("\t\t\tif (currprefs.cpu_level == 0 || dst < 0) SET_NFLG (1);\n");
		else /* ??? some 68000 revisions may not set NFLG when overflow happens.. */
#endif
			printf("\t\t\tSET_NFLG (1);\n");
		printf("\t\t} else {\n");
		printf("\t\t");
		genflags(flag_logical, sz_word, "newv", "", "");
		printf("\t\t\tnewv = (newv & 0xffff) | ((uae_u32)rem << 16);\n");
		printf("\t\t");
		genastore("newv", curi->dmode, "dstreg", sz_long, "dst");
		printf("\t\t}\n");
		printf("\t}\n");
		insn_n_cycles += 136;
		need_endlabel = 1;
		break;
	case i_DIVS:
		printf("\tuaecptr oldpc = m68k_getpc();\n");
		genamode(curi->smode, "srcreg", sz_word, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", sz_long, "dst", 1, 0, 0);
		fill_prefetch_next_cycles();
		sync_m68k_pc();
		printf("\tCLEAR_CZNV;\n");
		printf("\tif (src == 0) {\n");
		if(cpu_level > 0) {
			/* 68020 sets V when dividing by zero. Z is also set.
			 * 68000 clears both
			 */
			printf("\t\tSET_VFLG (1);\n");
			printf("\t\tSET_ZFLG (1);\n");
		}
		printf("\t\tException (5, oldpc);\n");
		printf("\t\tgoto %s;\n", endlabelstr);
		printf("\t} else {\n");
		printf("\t\tuae_s32 newv = (uae_s32)dst / (uae_s32)(uae_s16)src;\n");
		printf("\t\tuae_u16 rem = (uae_s32)dst %% (uae_s32)(uae_s16)src;\n");
		if(using_ce) {
			printf("\t\tint cycles = 156 * %d - lostcycles;\n", CYCLE_UNIT / 2);
		}
		printf("\t\tif ((newv & 0xffff8000) != 0 && (newv & 0xffff8000) != 0xffff8000) {\n");
		printf("\t\t\tSET_VFLG (1);\n");
#ifdef UNDEF68020
		if(cpu_level > 0)
			printf("\t\t\tif (currprefs.cpu_level == 0) SET_NFLG (1);\n");
		else
#endif
			printf("\t\t\tSET_NFLG (1);\n");
		printf("\t\t} else {\n");
		printf("\t\t\tif (((uae_s16)rem < 0) != ((uae_s32)dst < 0)) rem = -rem;\n");
		genflags(flag_logical, sz_word, "newv", "", "");
		printf("\t\t\tnewv = (newv & 0xffff) | ((uae_u32)rem << 16);\n");
		printf("\t\t");
		genastore("newv", curi->dmode, "dstreg", sz_long, "dst");
		printf("\t\t}\n");
		printf("\t}\n");
		insn_n_cycles += 154;
		need_endlabel = 1;
		break;
	case i_MULU:
		genamode(curi->smode, "srcreg", sz_word, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", sz_word, "dst", 1, 0, 0);
		fill_prefetch_next_cycles();
		start_brace();
		printf("\tuae_u32 newv = (uae_u32)(uae_u16)dst * (uae_u32)(uae_u16)src;\n");
		genflags(flag_logical, sz_long, "newv", "", "");
		genastore("newv", curi->dmode, "dstreg", sz_long, "dst");
		sync_m68k_pc();
		insn_n_cycles += 66;
		break;
	case i_MULS:
		genamode(curi->smode, "srcreg", sz_word, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", sz_word, "dst", 1, 0, 0);
		fill_prefetch_next_cycles();
		start_brace();
		printf("\tuae_u32 newv = (uae_s32)(uae_s16)dst * (uae_s32)(uae_s16)src;\n");
		genflags(flag_logical, sz_long, "newv", "", "");
		genastore("newv", curi->dmode, "dstreg", sz_long, "dst");
		insn_n_cycles += 66;
		break;
	case i_CHK:
		printf("\tuaecptr oldpc = m68k_getpc();\n");
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, 0);
		sync_m68k_pc();
		fill_prefetch_next_delay(6);
		printf("\tSET_ZFLG (!dst);\n");
		printf("\tSET_CFLG (0);\n");
		printf("\tSET_VFLG (0);\n");
		printf("\tif ((uae_s32)dst < 0) {\n");
		printf("\t\tSET_NFLG (1);\n");
		printf("\t\tException (6, oldpc);\n");
		printf("\t\tgoto %s;\n", endlabelstr);
		printf("\t} else if (dst > src) {\n");
		printf("\t\tSET_NFLG (0);\n");
		printf("\t\tException (6, oldpc);\n");
		printf("\t\tgoto %s;\n", endlabelstr);
		printf("\t}\n");
		need_endlabel = 1;
		break;

	case i_CHK2:
		printf("\tuaecptr oldpc = m68k_getpc();\n");
		genamode(curi->smode, "srcreg", curi->size, "extra", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 2, 0, 0);
		fill_prefetch_0();
		printf("\t{uae_s32 upper,lower,reg = regs.regs[(extra >> 12) & 15];\n");
		switch(curi->size) {
		case sz_byte:
			printf("\tlower=(uae_s32)(uae_s8)get_byte(dsta); upper = "
			       "(uae_s32)(uae_s8)get_byte(dsta+1);\n");
			printf("\tif ((extra & 0x8000) == 0) reg = (uae_s32)(uae_s8)reg;\n");
			break;
		case sz_word:
			printf("\tlower=(uae_s32)(uae_s16)get_word(dsta); upper = "
			       "(uae_s32)(uae_s16)get_word(dsta+2);\n");
			printf("\tif ((extra & 0x8000) == 0) reg = (uae_s32)(uae_s16)reg;\n");
			break;
		case sz_long:
			printf("\tlower=get_long(dsta); upper = get_long(dsta+4);\n");
			break;
		default:
			abort();
		}
		printf("\tSET_ZFLG (upper == reg || lower == reg);\n");
		printf(
		"\tSET_CFLG (lower <= upper ? reg < lower || reg > upper : reg > upper || reg < lower);\n");
		printf("\tif ((extra & 0x800) && GET_CFLG) { Exception(6,oldpc); goto %s; }\n}\n", endlabelstr);
		need_endlabel = 1;
		break;

	case i_ASR:
		genamode(curi->smode, "srcreg", curi->size, "cnt", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next_cycles();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u32 val = (uae_u8)data;\n");
			break;
		case sz_word:
			printf("\tuae_u32 val = (uae_u16)data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tuae_u32 sign = (%s & val) >> %d;\n", cmask(curi->size), bit_size(curi->size) - 1);
		printf("\tcnt &= 63;\n");
		printf("\tCLEAR_CZNV;\n");
		printf("\tif (cnt >= %d) {\n", bit_size(curi->size));
		printf("\t\tval = %s & (uae_u32)-sign;\n", bit_mask(curi->size));
		printf("\t\tSET_CFLG (sign);\n");
		duplicate_carry(0);
		if(source_is_imm1_8(curi))
			printf("\t} else {\n");
		else
			printf("\t} else if (cnt > 0) {\n");
		printf("\t\tval >>= cnt - 1;\n");
		printf("\t\tSET_CFLG (val & 1);\n");
		duplicate_carry(0);
		printf("\t\tval >>= 1;\n");
		printf("\t\tval |= (%s << (%d - cnt)) & (uae_u32)-sign;\n", bit_mask(curi->size),
		       bit_size(curi->size));
		printf("\t\tval &= %s;\n", bit_mask(curi->size));
		printf("\t}\n");
		genflags(flag_logical_noclobber, curi->size, "val", "", "");
		genastore("val", curi->dmode, "dstreg", curi->size, "data");
		break;
	case i_ASL:
		genamode(curi->smode, "srcreg", curi->size, "cnt", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next_cycles();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u32 val = (uae_u8)data;\n");
			break;
		case sz_word:
			printf("\tuae_u32 val = (uae_u16)data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tcnt &= 63;\n");
		printf("\tCLEAR_CZNV;\n");
		printf("\tif (cnt >= %d) {\n", bit_size(curi->size));
		printf("\t\tSET_VFLG (val != 0);\n");
		printf("\t\tSET_CFLG (cnt == %d ? val & 1 : 0);\n", bit_size(curi->size));
		duplicate_carry(0);
		printf("\t\tval = 0;\n");
		if(source_is_imm1_8(curi))
			printf("\t} else {\n");
		else
			printf("\t} else if (cnt > 0) {\n");
		printf("\t\tuae_u32 mask = (%s << (%d - cnt)) & %s;\n", bit_mask(curi->size),
		       bit_size(curi->size) - 1, bit_mask(curi->size));
		printf("\t\tSET_VFLG ((val & mask) != mask && (val & mask) != 0);\n");
		printf("\t\tval <<= cnt - 1;\n");
		printf("\t\tSET_CFLG ((val & %s) >> %d);\n", cmask(curi->size), bit_size(curi->size) - 1);
		duplicate_carry(0);
		printf("\t\tval <<= 1;\n");
		printf("\t\tval &= %s;\n", bit_mask(curi->size));
		printf("\t}\n");
		genflags(flag_logical_noclobber, curi->size, "val", "", "");
		genastore("val", curi->dmode, "dstreg", curi->size, "data");
		break;
	case i_LSR:
		genamode(curi->smode, "srcreg", curi->size, "cnt", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next_cycles();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u32 val = (uae_u8)data;\n");
			break;
		case sz_word:
			printf("\tuae_u32 val = (uae_u16)data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tcnt &= 63;\n");
		printf("\tCLEAR_CZNV;\n");
		printf("\tif (cnt >= %d) {\n", bit_size(curi->size));
		printf("\t\tSET_CFLG ((cnt == %d) & (val >> %d));\n", bit_size(curi->size), bit_size(curi->size) - 1);
		duplicate_carry(0);
		printf("\t\tval = 0;\n");
		if(source_is_imm1_8(curi))
			printf("\t} else {\n");
		else
			printf("\t} else if (cnt > 0) {\n");
		printf("\t\tval >>= cnt - 1;\n");
		printf("\t\tSET_CFLG (val & 1);\n");
		duplicate_carry(0);
		printf("\t\tval >>= 1;\n");
		printf("\t}\n");
		genflags(flag_logical_noclobber, curi->size, "val", "", "");
		genastore("val", curi->dmode, "dstreg", curi->size, "data");
		break;
	case i_LSL:
		genamode(curi->smode, "srcreg", curi->size, "cnt", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next_cycles();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u32 val = (uae_u8)data;\n");
			break;
		case sz_word:
			printf("\tuae_u32 val = (uae_u16)data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tcnt &= 63;\n");
		printf("\tCLEAR_CZNV;\n");
		printf("\tif (cnt >= %d) {\n", bit_size(curi->size));
		printf("\t\tSET_CFLG (cnt == %d ? val & 1 : 0);\n", bit_size(curi->size));
		duplicate_carry(0);
		printf("\t\tval = 0;\n");
		if(source_is_imm1_8(curi))
			printf("\t} else {\n");
		else
			printf("\t} else if (cnt > 0) {\n");
		printf("\t\tval <<= (cnt - 1);\n");
		printf("\t\tSET_CFLG ((val & %s) >> %d);\n", cmask(curi->size), bit_size(curi->size) - 1);
		duplicate_carry(0);
		printf("\t\tval <<= 1;\n");
		printf("\tval &= %s;\n", bit_mask(curi->size));
		printf("\t}\n");
		genflags(flag_logical_noclobber, curi->size, "val", "", "");
		genastore("val", curi->dmode, "dstreg", curi->size, "data");
		break;
	case i_ROL:
		genamode(curi->smode, "srcreg", curi->size, "cnt", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next_cycles();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u32 val = (uae_u8)data;\n");
			break;
		case sz_word:
			printf("\tuae_u32 val = (uae_u16)data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tcnt &= 63;\n");
		printf("\tCLEAR_CZNV;\n");
		if(source_is_imm1_8(curi))
			printf("{");
		else
			printf("\tif (cnt > 0) {\n");
		printf("\tuae_u32 loval;\n");
		printf("\tcnt &= %d;\n", bit_size(curi->size) - 1);
		printf("\tloval = val >> (%d - cnt);\n", bit_size(curi->size));
		printf("\tval <<= cnt;\n");
		printf("\tval |= loval;\n");
		printf("\tval &= %s;\n", bit_mask(curi->size));
		printf("\tSET_CFLG (val & 1);\n");
		printf("}\n");
		genflags(flag_logical_noclobber, curi->size, "val", "", "");
		genastore("val", curi->dmode, "dstreg", curi->size, "data");
		break;
	case i_ROR:
		genamode(curi->smode, "srcreg", curi->size, "cnt", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next_cycles();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u32 val = (uae_u8)data;\n");
			break;
		case sz_word:
			printf("\tuae_u32 val = (uae_u16)data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tcnt &= 63;\n");
		printf("\tCLEAR_CZNV;\n");
		if(source_is_imm1_8(curi))
			printf("{");
		else
			printf("\tif (cnt > 0) {");
		printf("\tuae_u32 hival;\n");
		printf("\tcnt &= %d;\n", bit_size(curi->size) - 1);
		printf("\thival = val << (%d - cnt);\n", bit_size(curi->size));
		printf("\tval >>= cnt;\n");
		printf("\tval |= hival;\n");
		printf("\tval &= %s;\n", bit_mask(curi->size));
		printf("\tSET_CFLG ((val & %s) >> %d);\n", cmask(curi->size), bit_size(curi->size) - 1);
		printf("\t}\n");
		genflags(flag_logical_noclobber, curi->size, "val", "", "");
		genastore("val", curi->dmode, "dstreg", curi->size, "data");
		break;
	case i_ROXL:
		genamode(curi->smode, "srcreg", curi->size, "cnt", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next_cycles();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u32 val = (uae_u8)data;\n");
			break;
		case sz_word:
			printf("\tuae_u32 val = (uae_u16)data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tcnt &= 63;\n");
		printf("\tCLEAR_CZNV;\n");
		if(source_is_imm1_8(curi))
			printf("{");
		else {
			force_range_for_rox("cnt", curi->size);
			printf("\tif (cnt > 0) {\n");
		}
		printf("\tcnt--;\n");
		printf("\t{\n\tuae_u32 carry;\n");
		printf("\tuae_u32 loval = val >> (%d - cnt);\n", bit_size(curi->size) - 1);
		printf("\tcarry = loval & 1;\n");
		printf("\tval = (((val << 1) | GET_XFLG) << cnt) | (loval >> 1);\n");
		printf("\tSET_XFLG (carry);\n");
		printf("\tval &= %s;\n", bit_mask(curi->size));
		printf("\t} }\n");
		printf("\tSET_CFLG (GET_XFLG);\n");
		genflags(flag_logical_noclobber, curi->size, "val", "", "");
		genastore("val", curi->dmode, "dstreg", curi->size, "data");
		break;
	case i_ROXR:
		genamode(curi->smode, "srcreg", curi->size, "cnt", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next_cycles();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u32 val = (uae_u8)data;\n");
			break;
		case sz_word:
			printf("\tuae_u32 val = (uae_u16)data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tcnt &= 63;\n");
		printf("\tCLEAR_CZNV;\n");
		if(source_is_imm1_8(curi))
			printf("{");
		else {
			force_range_for_rox("cnt", curi->size);
			printf("\tif (cnt > 0) {\n");
		}
		printf("\tcnt--;\n");
		printf("\t{\n\tuae_u32 carry;\n");
		printf("\tuae_u32 hival = (val << 1) | GET_XFLG;\n");
		printf("\thival <<= (%d - cnt);\n", bit_size(curi->size) - 1);
		printf("\tval >>= cnt;\n");
		printf("\tcarry = val & 1;\n");
		printf("\tval >>= 1;\n");
		printf("\tval |= hival;\n");
		printf("\tSET_XFLG (carry);\n");
		printf("\tval &= %s;\n", bit_mask(curi->size));
		printf("\t} }\n");
		printf("\tSET_CFLG (GET_XFLG);\n");
		genflags(flag_logical_noclobber, curi->size, "val", "", "");
		genastore("val", curi->dmode, "dstreg", curi->size, "data");
		break;
	case i_ASRW:
		genamode(curi->smode, "srcreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u32 val = (uae_u8)data;\n");
			break;
		case sz_word:
			printf("\tuae_u32 val = (uae_u16)data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tuae_u32 sign = %s & val;\n", cmask(curi->size));
		printf("\tuae_u32 cflg = val & 1;\n");
		printf("\tval = (val >> 1) | sign;\n");
		genflags(flag_logical, curi->size, "val", "", "");
		printf("\tSET_CFLG (cflg);\n");
		duplicate_carry(0);
		genastore("val", curi->smode, "srcreg", curi->size, "data");
		break;
	case i_ASLW:
		genamode(curi->smode, "srcreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u32 val = (uae_u8)data;\n");
			break;
		case sz_word:
			printf("\tuae_u32 val = (uae_u16)data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tuae_u32 sign = %s & val;\n", cmask(curi->size));
		printf("\tuae_u32 sign2;\n");
		printf("\tval <<= 1;\n");
		genflags(flag_logical, curi->size, "val", "", "");
		printf("\tsign2 = %s & val;\n", cmask(curi->size));
		printf("\tSET_CFLG (sign != 0);\n");
		duplicate_carry(0);

		printf("\tSET_VFLG (GET_VFLG | (sign2 != sign));\n");
		genastore("val", curi->smode, "srcreg", curi->size, "data");
		break;
	case i_LSRW:
		genamode(curi->smode, "srcreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u32 val = (uae_u8)data;\n");
			break;
		case sz_word:
			printf("\tuae_u32 val = (uae_u16)data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tuae_u32 carry = val & 1;\n");
		printf("\tval >>= 1;\n");
		genflags(flag_logical, curi->size, "val", "", "");
		printf("SET_CFLG (carry);\n");
		duplicate_carry(0);
		genastore("val", curi->smode, "srcreg", curi->size, "data");
		break;
	case i_LSLW:
		genamode(curi->smode, "srcreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u8 val = data;\n");
			break;
		case sz_word:
			printf("\tuae_u16 val = data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tuae_u32 carry = val & %s;\n", cmask(curi->size));
		printf("\tval <<= 1;\n");
		genflags(flag_logical, curi->size, "val", "", "");
		printf("SET_CFLG (carry >> %d);\n", bit_size(curi->size) - 1);
		duplicate_carry(0);
		genastore("val", curi->smode, "srcreg", curi->size, "data");
		break;
	case i_ROLW:
		genamode(curi->smode, "srcreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u8 val = data;\n");
			break;
		case sz_word:
			printf("\tuae_u16 val = data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tuae_u32 carry = val & %s;\n", cmask(curi->size));
		printf("\tval <<= 1;\n");
		printf("\tif (carry)  val |= 1;\n");
		genflags(flag_logical, curi->size, "val", "", "");
		printf("SET_CFLG (carry >> %d);\n", bit_size(curi->size) - 1);
		genastore("val", curi->smode, "srcreg", curi->size, "data");
		break;
	case i_RORW:
		genamode(curi->smode, "srcreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u8 val = data;\n");
			break;
		case sz_word:
			printf("\tuae_u16 val = data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tuae_u32 carry = val & 1;\n");
		printf("\tval >>= 1;\n");
		printf("\tif (carry) val |= %s;\n", cmask(curi->size));
		genflags(flag_logical, curi->size, "val", "", "");
		printf("SET_CFLG (carry);\n");
		genastore("val", curi->smode, "srcreg", curi->size, "data");
		break;
	case i_ROXLW:
		genamode(curi->smode, "srcreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u8 val = data;\n");
			break;
		case sz_word:
			printf("\tuae_u16 val = data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tuae_u32 carry = val & %s;\n", cmask(curi->size));
		printf("\tval <<= 1;\n");
		printf("\tif (GET_XFLG) val |= 1;\n");
		genflags(flag_logical, curi->size, "val", "", "");
		printf("SET_CFLG (carry >> %d);\n", bit_size(curi->size) - 1);
		duplicate_carry(0);
		genastore("val", curi->smode, "srcreg", curi->size, "data");
		break;
	case i_ROXRW:
		genamode(curi->smode, "srcreg", curi->size, "data", 1, 0, 0);
		fill_prefetch_next();
		start_brace();
		switch(curi->size) {
		case sz_byte:
			printf("\tuae_u8 val = data;\n");
			break;
		case sz_word:
			printf("\tuae_u16 val = data;\n");
			break;
		case sz_long:
			printf("\tuae_u32 val = data;\n");
			break;
		default:
			abort();
		}
		printf("\tuae_u32 carry = val & 1;\n");
		printf("\tval >>= 1;\n");
		printf("\tif (GET_XFLG) val |= %s;\n", cmask(curi->size));
		genflags(flag_logical, curi->size, "val", "", "");
		printf("SET_CFLG (carry);\n");
		duplicate_carry(0);
		genastore("val", curi->smode, "srcreg", curi->size, "data");
		break;
	case i_MOVEC2:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		fill_prefetch_next();
		start_brace();
		printf("\tint regno = (src >> 12) & 15;\n");
		printf("\tuae_u32 *regp = regs.regs + regno;\n");
		printf("\tif (! m68k_movec2(src & 0xFFF, regp)) goto %s;\n", endlabelstr);
		break;
	case i_MOVE2C:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		fill_prefetch_next();
		start_brace();
		printf("\tint regno = (src >> 12) & 15;\n");
		printf("\tuae_u32 *regp = regs.regs + regno;\n");
		printf("\tif (! m68k_move2c(src & 0xFFF, regp)) goto %s;\n", endlabelstr);
		break;
	case i_CAS: {
		int old_brace_level;
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, 0);
		fill_prefetch_0();
		start_brace();
		printf("\tint ru = (src >> 6) & 7;\n");
		printf("\tint rc = src & 7;\n");
		genflags(flag_cmp, curi->size, "newv", "m68k_dreg(regs, rc)", "dst");
		printf("\tif (GET_ZFLG)");
		old_brace_level = n_braces;
		start_brace();
		genastore("(m68k_dreg(regs, ru))", curi->dmode, "dstreg", curi->size, "dst");
		pop_braces(old_brace_level);
		printf("else");
		start_brace();
		printf("m68k_dreg(regs, rc) = dst;\n");
		pop_braces(old_brace_level);
	} break;
	case i_CAS2:
		genamode(curi->smode, "srcreg", curi->size, "extra", 1, 0, 0);
		printf("\tuae_u32 rn1 = regs.regs[(extra >> 28) & 15];\n");
		printf("\tuae_u32 rn2 = regs.regs[(extra >> 12) & 15];\n");
		if(curi->size == sz_word) {
			int old_brace_level = n_braces;
			printf("\tuae_u16 dst1 = get_word(rn1), dst2 = get_word(rn2);\n");
			genflags(flag_cmp, curi->size, "newv", "m68k_dreg(regs, (extra >> 16) & 7)", "dst1");
			printf("\tif (GET_ZFLG) {\n");
			genflags(flag_cmp, curi->size, "newv", "m68k_dreg(regs, extra & 7)", "dst2");
			printf("\tif (GET_ZFLG) {\n");
			printf("\tput_word(rn1, m68k_dreg(regs, (extra >> 22) & 7));\n");
			printf("\tput_word(rn1, m68k_dreg(regs, (extra >> 6) & 7));\n");
			printf("\t}}\n");
			pop_braces(old_brace_level);
			printf("\tif (! GET_ZFLG) {\n");
			printf("\tm68k_dreg(regs, (extra >> 22) & 7) = (m68k_dreg(regs, (extra >> 22) & 7) & "
			       "~0xffff) | (dst1 & 0xffff);\n");
			printf("\tm68k_dreg(regs, (extra >> 6) & 7) = (m68k_dreg(regs, (extra >> 6) & 7) & "
			       "~0xffff) | (dst2 & 0xffff);\n");
			printf("\t}\n");
		} else {
			int old_brace_level = n_braces;
			printf("\tuae_u32 dst1 = get_long(rn1), dst2 = get_long(rn2);\n");
			genflags(flag_cmp, curi->size, "newv", "m68k_dreg(regs, (extra >> 16) & 7)", "dst1");
			printf("\tif (GET_ZFLG) {\n");
			genflags(flag_cmp, curi->size, "newv", "m68k_dreg(regs, extra & 7)", "dst2");
			printf("\tif (GET_ZFLG) {\n");
			printf("\tput_long(rn1, m68k_dreg(regs, (extra >> 22) & 7));\n");
			printf("\tput_long(rn1, m68k_dreg(regs, (extra >> 6) & 7));\n");
			printf("\t}}\n");
			pop_braces(old_brace_level);
			printf("\tif (! GET_ZFLG) {\n");
			printf("\tm68k_dreg(regs, (extra >> 22) & 7) = dst1;\n");
			printf("\tm68k_dreg(regs, (extra >> 6) & 7) = dst2;\n");
			printf("\t}\n");
		}
		break;
	case i_MOVES: /* ignore DFC and SFC because we have no MMU */
	{
		int old_brace_level;
		genamode(curi->smode, "srcreg", curi->size, "extra", 1, 0, 0);
		printf("\tif (extra & 0x800)\n");
		old_brace_level = n_braces;
		start_brace();
		printf("\tuae_u32 src = regs.regs[(extra >> 12) & 15];\n");
		genamode(curi->dmode, "dstreg", curi->size, "dst", 2, 0, 0);
		genastore("src", curi->dmode, "dstreg", curi->size, "dst");
		pop_braces(old_brace_level);
		printf("else");
		start_brace();
		genamode(curi->dmode, "dstreg", curi->size, "src", 1, 0, 0);
		printf("\tif (extra & 0x8000) {\n");
		switch(curi->size) {
		case sz_byte:
			printf("\tm68k_areg(regs, (extra >> 12) & 7) = (uae_s32)(uae_s8)src;\n");
			break;
		case sz_word:
			printf("\tm68k_areg(regs, (extra >> 12) & 7) = (uae_s32)(uae_s16)src;\n");
			break;
		case sz_long:
			printf("\tm68k_areg(regs, (extra >> 12) & 7) = src;\n");
			break;
		default:
			abort();
		}
		printf("\t} else {\n");
		genastore("src", Dreg, "(extra >> 12) & 7", curi->size, "");
		printf("\t}\n");
		pop_braces(old_brace_level);
	} break;
	case i_BKPT: /* only needed for hardware emulators */
		sync_m68k_pc();
		printf("\top_illg(opcode);\n");
		break;
	case i_CALLM: /* not present in 68030 */
		sync_m68k_pc();
		printf("\top_illg(opcode);\n");
		break;
	case i_RTM: /* not present in 68030 */
		sync_m68k_pc();
		printf("\top_illg(opcode);\n");
		break;
	case i_TRAPcc:
		if(curi->smode != am_unknown && curi->smode != am_illg)
			genamode(curi->smode, "srcreg", curi->size, "dummy", 1, 0, 0);
		fill_prefetch_0();
		printf("\tif (cctrue(%d)) { Exception(7,m68k_getpc()); goto %s; }\n", curi->cc, endlabelstr);
		need_endlabel = 1;
		break;
	case i_DIVL:
		sync_m68k_pc();
		start_brace();
		printf("\tuaecptr oldpc = m68k_getpc();\n");
		genamode(curi->smode, "srcreg", curi->size, "extra", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, 0);
		sync_m68k_pc();
		printf("\tm68k_divl(opcode, dst, extra, oldpc);\n");
		break;
	case i_MULL:
		genamode(curi->smode, "srcreg", curi->size, "extra", 1, 0, 0);
		genamode(curi->dmode, "dstreg", curi->size, "dst", 1, 0, 0);
		sync_m68k_pc();
		printf("\tm68k_mull(opcode, dst, extra);\n");
		break;
	case i_BFTST:
	case i_BFEXTU:
	case i_BFCHG:
	case i_BFEXTS:
	case i_BFCLR:
	case i_BFFFO:
	case i_BFSET:
	case i_BFINS:
		genamode(curi->smode, "srcreg", curi->size, "extra", 1, 0, 0);
		genamode(curi->dmode, "dstreg", sz_long, "dst", 2, 0, 0);
		start_brace();
		printf("\tuae_s32 offset = extra & 0x800 ? m68k_dreg(regs, (extra >> 6) & 7) : (extra >> "
		       "6) & 0x1f;\n");
		printf(
		"\tint width = (((extra & 0x20 ? m68k_dreg(regs, extra & 7) : extra) -1) & 0x1f) +1;\n");
		if(curi->dmode == Dreg) {
			printf("\tuae_u32 tmp = m68k_dreg(regs, dstreg) << (offset & 0x1f);\n");
		} else {
			printf("\tuae_u32 tmp,bf0,bf1;\n");
			printf("\tdsta += (offset >> 3) | (offset & 0x80000000 ? ~0x1fffffff : 0);\n");
			printf("\tbf0 = get_long(dsta);bf1 = get_byte(dsta+4) & 0xff;\n");
			printf("\ttmp = (bf0 << (offset & 7)) | (bf1 >> (8 - (offset & 7)));\n");
		}
		printf("\ttmp >>= (32 - width);\n");
		printf("\tSET_NFLG (tmp & (1 << (width-1)) ? 1 : 0);\n");
		printf("\tSET_ZFLG (tmp == 0); SET_VFLG (0); SET_CFLG (0);\n");
		switch(curi->mnemo) {
		case i_BFTST:
			break;
		case i_BFEXTU:
			printf("\tm68k_dreg(regs, (extra >> 12) & 7) = tmp;\n");
			break;
		case i_BFCHG:
			printf("\ttmp = ~tmp;\n");
			break;
		case i_BFEXTS:
			printf("\tif (GET_NFLG) tmp |= width == 32 ? 0 : (-1 << width);\n");
			printf("\tm68k_dreg(regs, (extra >> 12) & 7) = tmp;\n");
			break;
		case i_BFCLR:
			printf("\ttmp = 0;\n");
			break;
		case i_BFFFO:
			printf("\t{ uae_u32 mask = 1 << (width-1);\n");
			printf("\twhile (mask) { if (tmp & mask) break; mask >>= 1; offset++; }}\n");
			printf("\tm68k_dreg(regs, (extra >> 12) & 7) = offset;\n");
			break;
		case i_BFSET:
			printf("\ttmp = 0xffffffff;\n");
			break;
		case i_BFINS:
			printf("\ttmp = m68k_dreg(regs, (extra >> 12) & 7);\n");
			printf("\tSET_NFLG (tmp & (1 << (width - 1)) ? 1 : 0);\n");
			printf("\tSET_ZFLG (tmp == 0);\n");
			break;
		default:
			break;
		}
		if(curi->mnemo == i_BFCHG || curi->mnemo == i_BFCLR || curi->mnemo == i_BFSET || curi->mnemo == i_BFINS) {
			printf("\ttmp <<= (32 - width);\n");
			if(curi->dmode == Dreg) {
				printf("\tm68k_dreg(regs, dstreg) = (m68k_dreg(regs, dstreg) & ((offset & 0x1f) == "
				       "0 ? 0 :\n");
				printf("\t\t(0xffffffff << (32 - (offset & 0x1f))))) |\n");
				printf("\t\t(tmp >> (offset & 0x1f)) |\n");
				printf("\t\t(((offset & 0x1f) + width) >= 32 ? 0 :\n");
				printf(" (m68k_dreg(regs, dstreg) & ((uae_u32)0xffffffff >> ((offset & 0x1f) + "
				       "width))));\n");
			} else {
				printf("\tbf0 = (bf0 & (0xff000000 << (8 - (offset & 7)))) |\n");
				printf("\t\t(tmp >> (offset & 7)) |\n");
				printf("\t\t(((offset & 7) + width) >= 32 ? 0 :\n");
				printf("\t\t (bf0 & ((uae_u32)0xffffffff >> ((offset & 7) + width))));\n");
				printf("\tput_long(dsta,bf0 );\n");
				printf("\tif (((offset & 7) + width) > 32) {\n");
				printf("\t\tbf1 = (bf1 & (0xff >> (width - 32 + (offset & 7)))) |\n");
				printf("\t\t\t(tmp << (8 - (offset & 7)));\n");
				printf("\t\tput_byte(dsta+4,bf1);\n");
				printf("\t}\n");
			}
		}
		break;
	case i_PACK:
		if(curi->smode == Dreg) {
			printf("\tuae_u16 val = m68k_dreg(regs, srcreg) + %s;\n", gen_nextiword(0));
			printf("\tm68k_dreg(regs, dstreg) = (m68k_dreg(regs, dstreg) & 0xffffff00) | ((val >> "
			       "4) & 0xf0) | (val & 0xf);\n");
		} else {
			printf("\tuae_u16 val;\n");
			printf("\tm68k_areg(regs, srcreg) -= areg_byteinc[srcreg];\n");
			printf("\tval = (uae_u16)get_byte(m68k_areg(regs, srcreg));\n");
			printf("\tm68k_areg(regs, srcreg) -= areg_byteinc[srcreg];\n");
			printf("\tval = (val | ((uae_u16)get_byte(m68k_areg(regs, srcreg)) << 8)) + %s;\n",
			       gen_nextiword(0));
			printf("\tm68k_areg(regs, dstreg) -= areg_byteinc[dstreg];\n");
			printf("\tput_byte(m68k_areg(regs, dstreg),((val >> 4) & 0xf0) | (val & 0xf));\n");
		}
		break;
	case i_UNPK:
		if(curi->smode == Dreg) {
			printf("\tuae_u16 val = m68k_dreg(regs, srcreg);\n");
			printf("\tval = (((val << 4) & 0xf00) | (val & 0xf)) + %s;\n", gen_nextiword(0));
			printf("\tm68k_dreg(regs, dstreg) = (m68k_dreg(regs, dstreg) & 0xffff0000) | (val & "
			       "0xffff);\n");
		} else {
			printf("\tuae_u16 val;\n");
			printf("\tm68k_areg(regs, srcreg) -= areg_byteinc[srcreg];\n");
			printf("\tval = (uae_u16)get_byte(m68k_areg(regs, srcreg));\n");
			printf("\tval = (((val << 4) & 0xf00) | (val & 0xf)) + %s;\n", gen_nextiword(0));
			printf("\tm68k_areg(regs, dstreg) -= areg_byteinc[dstreg];\n");
			printf("\tput_byte(m68k_areg(regs, dstreg),val);\n");
			printf("\tm68k_areg(regs, dstreg) -= areg_byteinc[dstreg];\n");
			printf("\tput_byte(m68k_areg(regs, dstreg),val >> 8);\n");
		}
		break;
	case i_TAS:
		genamode(curi->smode, "srcreg", curi->size, "src", 1, 0, 0);
		genflags(flag_logical, curi->size, "src", "", "");
		if(!isreg(curi->smode)) tmpc += 2;
		fill_prefetch_next_delay(tmpc);
		printf("\tsrc |= 0x80;\n");
		genastore("src", curi->smode, "srcreg", curi->size, "src");
		break;
	case i_FPP:
		genamode(curi->smode, "srcreg", curi->size, "extra", 1, 0, 0);
		sync_m68k_pc();
		printf("\tfpp_opp(opcode,extra);\n");
		break;
	case i_FDBcc:
		genamode(curi->smode, "srcreg", curi->size, "extra", 1, 0, 0);
		sync_m68k_pc();
		printf("\tfdbcc_opp(opcode,extra);\n");
		break;
	case i_FScc:
		genamode(curi->smode, "srcreg", curi->size, "extra", 1, 0, 0);
		sync_m68k_pc();
		printf("\tfscc_opp(opcode,extra);\n");
		break;
	case i_FTRAPcc:
		sync_m68k_pc();
		start_brace();
		printf("\tuaecptr oldpc = m68k_getpc();\n");
		if(curi->smode != am_unknown && curi->smode != am_illg)
			genamode(curi->smode, "srcreg", curi->size, "dummy", 1, 0, 0);
		sync_m68k_pc();
		printf("\tftrapcc_opp(opcode,oldpc);\n");
		break;
	case i_FBcc:
		sync_m68k_pc();
		start_brace();
		printf("\tuaecptr pc = m68k_getpc();\n");
		genamode(curi->dmode, "srcreg", curi->size, "extra", 1, 0, 0);
		sync_m68k_pc();
		printf("\tfbcc_opp(opcode,pc,extra);\n");
		break;
	case i_FSAVE:
		sync_m68k_pc();
		printf("\tfsave_opp(opcode);\n");
		break;
	case i_FRESTORE:
		sync_m68k_pc();
		printf("\tfrestore_opp(opcode);\n");
		break;

	case i_CINVL:
	case i_CINVP:
	case i_CINVA:
	case i_CPUSHL:
	case i_CPUSHP:
	case i_CPUSHA:
		break;
	case i_MOVE16:
		if((opcode & 0xfff8) == 0xf620) {
			/* MOVE16 (Ax)+,(Ay)+ */
			printf("\tuaecptr mems = m68k_areg(regs, srcreg) & ~15, memd;\n");
			printf("\tdstreg = (%s >> 12) & 7;\n", gen_nextiword(0));
			printf("\tmemd = m68k_areg(regs, dstreg) & ~15;\n");
			printf("\tput_long(memd, get_long(mems));\n");
			printf("\tput_long(memd+4, get_long(mems+4));\n");
			printf("\tput_long(memd+8, get_long(mems+8));\n");
			printf("\tput_long(memd+12, get_long(mems+12));\n");
			printf("\tif (srcreg != dstreg)\n");
			printf("\tm68k_areg(regs, srcreg) += 16;\n");
			printf("\tm68k_areg(regs, dstreg) += 16;\n");
		} else {
			/* Other variants */
			genamode(curi->smode, "srcreg", curi->size, "mems", 0, 2, 0);
			genamode(curi->dmode, "dstreg", curi->size, "memd", 0, 2, 0);
			printf("\tmemsa &= ~15;\n");
			printf("\tmemda &= ~15;\n");
			printf("\tput_long(memda, get_long(memsa));\n");
			printf("\tput_long(memda+4, get_long(memsa+4));\n");
			printf("\tput_long(memda+8, get_long(memsa+8));\n");
			printf("\tput_long(memda+12, get_long(memsa+12));\n");
			if((opcode & 0xfff8) == 0xf600)
				printf("\tm68k_areg(regs, srcreg) += 16;\n");
			else if((opcode & 0xfff8) == 0xf608)
				printf("\tm68k_areg(regs, dstreg) += 16;\n");
		}
		break;

	case i_MMUOP:
		genamode(curi->smode, "srcreg", curi->size, "extra", 1, 0, 0);
		sync_m68k_pc();
		printf("\tmmu_op(opcode,extra);\n");
		break;
	default:
		abort();
		break;
	}
	finish_braces();
	fill_prefetch_finish();
	sync_m68k_pc();
	did_prefetch = 0;
}

static void generate_includes(FILE *f) {
	fprintf(f, "#include <stdlib.h>\n");
	fprintf(f, "#include \"sysdeps.h\"\n");
	fprintf(f, "#include \"uconfig.h\"\n");
	fprintf(f, "#include \"options.h\"\n");
	fprintf(f, "#include \"memory.h\"\n");
	/*    fprintf (f, "#include \"custom.h\"\n");*/
	fprintf(f, "#include \"newcpu.h\"\n");
	fprintf(f, "#include \"cpu_prefetch.h\"\n");
	fprintf(f, "#include \"cputbl.h\"\n");

	fprintf(f, "#define CPUFUNC(x) x##_ff\n"
	           "#ifdef NOFLAGS\n"
	           "#include \"noflags.h\"\n"
	           "#endif\n");
}

static int postfix;

static void generate_one_opcode(int rp) {
	int i;
	uae_u16 smsk, dmsk;
	long int opcode = opcode_map[rp];

	if(table68k[opcode].mnemo == i_ILLG || table68k[opcode].clev > cpu_level) return;

	for(i = 0; lookuptab[i].name[0]; i++) {
		if(table68k[opcode].mnemo == lookuptab[i].mnemo) break;
	}

	if(table68k[opcode].handler != -1) return;

	if(opcode_next_clev[rp] != cpu_level) {
		fprintf(stblfile, "{ CPUFUNC(op_%lx_%d), 0, %ld }, /* %s */\n", opcode,
		        opcode_last_postfix[rp], opcode, lookuptab[i].name);
		return;
	}
	fprintf(stblfile, "{ CPUFUNC(op_%lx_%d), 0, %ld }, /* %s */\n", opcode, postfix, opcode,
	        lookuptab[i].name);
	fprintf(headerfile, "extern cpuop_func op_%lx_%d_nf;\n", opcode, postfix);
	fprintf(headerfile, "extern cpuop_func op_%lx_%d_ff;\n", opcode, postfix);
	printf("unsigned long REGPARAM2 CPUFUNC(op_%lx_%d)(uae_u32 opcode) /* %s */\n{\n", opcode,
	       postfix, lookuptab[i].name);

	switch(table68k[opcode].stype) {
	case 0:
		smsk = 7;
		break;
	case 1:
		smsk = 255;
		break;
	case 2:
		smsk = 15;
		break;
	case 3:
		smsk = 7;
		break;
	case 4:
		smsk = 7;
		break;
	case 5:
		smsk = 63;
		break;
	case 7:
		smsk = 3;
		break;
	default:
		abort();
	}
	dmsk = 7;

	next_cpu_level = -1;
	if(table68k[opcode].suse && table68k[opcode].smode != imm && table68k[opcode].smode != imm0 &&
	   table68k[opcode].smode != imm1 && table68k[opcode].smode != imm2 && table68k[opcode].smode != absw &&
	   table68k[opcode].smode != absl && table68k[opcode].smode != PC8r && table68k[opcode].smode != PC16) {
		if(table68k[opcode].spos == -1) {
			if(((int)table68k[opcode].sreg) >= 128)
				printf("\tuae_u32 srcreg = (uae_s32)(uae_s8)%d;\n", (int)table68k[opcode].sreg);
			else
				printf("\tuae_u32 srcreg = %d;\n", (int)table68k[opcode].sreg);
		} else {
			char source[100];
			int pos = table68k[opcode].spos;

			if(pos)
				sprintf(source, "((opcode >> %d) & %d)", pos, smsk);
			else
				sprintf(source, "(opcode & %d)", smsk);

			if(table68k[opcode].stype == 3)
				printf("\tuae_u32 srcreg = imm8_table[%s];\n", source);
			else if(table68k[opcode].stype == 1)
				printf("\tuae_u32 srcreg = (uae_s32)(uae_s8)%s;\n", source);
			else
				printf("\tuae_u32 srcreg = %s;\n", source);
		}
	}
	if(table68k[opcode].duse
	   /* Yes, the dmode can be imm, in case of LINK or DBcc */
	   && table68k[opcode].dmode != imm && table68k[opcode].dmode != imm0 && table68k[opcode].dmode != imm1 &&
	   table68k[opcode].dmode != imm2 && table68k[opcode].dmode != absw && table68k[opcode].dmode != absl) {
		if(table68k[opcode].dpos == -1) {
			if(((int)table68k[opcode].dreg) >= 128)
				printf("\tuae_u32 dstreg = (uae_s32)(uae_s8)%d;\n", (int)table68k[opcode].dreg);
			else
				printf("\tuae_u32 dstreg = %d;\n", (int)table68k[opcode].dreg);
		} else {
			int pos = table68k[opcode].dpos;
#if 0
	    /* Check that we can do the little endian optimization safely.  */
	    if (pos < 8 && (dmsk >> (8 - pos)) != 0)
		abort ();
#endif
			if(pos)
				printf("\tuae_u32 dstreg = (opcode >> %d) & %d;\n", pos, dmsk);
			else
				printf("\tuae_u32 dstreg = opcode & %d;\n", dmsk);
		}
	}
	need_endlabel = 0;
	endlabelno++;
	sprintf(endlabelstr, "endlabel%d", endlabelno);
	gen_opcode(opcode);
	if(need_endlabel) printf("%s: ;\n", endlabelstr);
	printf("return %d;\n", insn_n_cycles * CYCLE_UNIT / 2);
	printf("}\n");
	opcode_next_clev[rp] = next_cpu_level;
	opcode_last_postfix[rp] = postfix;
}

static void generate_func(void) {
	int i, j, rp;

	using_prefetch = 0;
	using_exception_3 = 0;
	for(i = 0; i < 6; i++) {
		cpu_level = 4 - i;
		if(i == 5) {
			cpu_level = 0;
			using_prefetch = 1;
			using_exception_3 = 1;
			for(rp = 0; rp < nr_cpuop_funcs; rp++)
				opcode_next_clev[rp] = 0;
		}

		postfix = i;
		fprintf(stblfile, "struct cputbl CPUFUNC(op_smalltbl_%d)[] = {\n", postfix);

		/* sam: this is for people with low memory (eg. me :)) */
		printf("\n"
		       "#if !defined(PART_1) && !defined(PART_2) && "
		       "!defined(PART_3) && !defined(PART_4) && "
		       "!defined(PART_5) && !defined(PART_6) && "
		       "!defined(PART_7) && !defined(PART_8)"
		       "\n"
		       "#define PART_1 1\n"
		       "#define PART_2 1\n"
		       "#define PART_3 1\n"
		       "#define PART_4 1\n"
		       "#define PART_5 1\n"
		       "#define PART_6 1\n"
		       "#define PART_7 1\n"
		       "#define PART_8 1\n"
		       "#endif\n\n");

		rp = 0;
		for(j = 1; j <= 8; ++j) {
			int k = (j * nr_cpuop_funcs) / 8;
			printf("#ifdef PART_%d\n", j);
			for(; rp < k; rp++)
				generate_one_opcode(rp);
			printf("#endif\n\n");
		}

		fprintf(stblfile, "{ 0, 0, 0 }};\n");
	}
}

int main(int argc, char **argv) {
	read_table68k();
	do_merges();

	opcode_map = (int *)xmalloc(sizeof(int) * nr_cpuop_funcs);
	opcode_last_postfix = (int *)xmalloc(sizeof(int) * nr_cpuop_funcs);
	opcode_next_clev = (int *)xmalloc(sizeof(int) * nr_cpuop_funcs);
	counts = (unsigned long *)xmalloc(65536 * sizeof(unsigned long));
	read_counts();

	/* It would be a lot nicer to put all in one file (we'd also get rid of
	 * cputbl.h that way), but cpuopti can't cope.  That could be fixed, but
	 * I don't dare to touch the 68k version.  */

	headerfile = fopen("cputbl.h", "wb");
	stblfile = fopen("cpustbl.c", "wb");
	freopen("cpuemu.c", "wb", stdout);

	generate_includes(stdout);
	generate_includes(stblfile);

	generate_func();

	free(table68k);
	return 0;
}
