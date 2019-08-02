/* Hey EMACS -*- linux-c -*- */
/* $Id: images.c 2821 2009-05-04 20:06:12Z roms $ */

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
    This module handles loading of images or upgrades.
    Images can be:
    - ROM dump
    - FLASH upgrade as a ROM dump

    Note:0x12000 is the beginning of the system privileged part.
*/

#include <assert.h>
#include <ctype.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

#include "uae/libuae.h"

#include "hwpm.h"
#include "images.h"
#include "../misc/intl.h"
#include "../src/logging.h"
#include "ti68k_def.h"
#include "ti68k_err.h"
#include "ti68k_int.h"

#define is_num(c) isdigit(c)
#define is_alnum(c) isalnum(c)

#define SPP 0x12000 // system privileged part
#define BO 0x88     // offset from SPP to boot

IMG_INFO img_infos;
int img_loaded = 0;
int img_changed = 0;

static int get_rom_version(char *ptr, int size, char *version);

/*
    Utility functions
*/
int ti68k_is_a_rom_file(const char *filename) {
	char *ext;

	ext = strrchr(filename, '.');
	if(ext == NULL)
		return 0;
	else if(!strcasecmp(ext, ".rom"))
		return !0;

	return 0;
}

int ti68k_is_a_tib_file(const char *filename) {
	return TRUE;
}

int ti68k_is_a_img_file(const char *filename) {
	char *ext;

	ext = strrchr(filename, '.');
	if(ext == NULL)
		return 0;
	else if(!strcasecmp(ext, ".img"))
		return !0;

	return 0;
}

int ti68k_is_a_sav_file(const char *filename) {
	char *ext;

	ext = strrchr(filename, '.');
	if(ext == NULL)
		return 0;
	else if(!strcasecmp(ext, ".sav"))
		return !0;

	return 0;
}

/*
    Display information
*/
void ti68k_display_rom_infos(IMG_INFO *s) {
	tiemu_info(_("ROM information:"));
	tiemu_info(_("  Calculator  : %s"), ti68k_calctype_to_string(s->calc_type));
	tiemu_info(_("  Firmware    : %s"), s->version);
	tiemu_info(_("  Memory type : %s"), ti68k_romtype_to_string(s->flash));
	tiemu_info(_("  Memory size : %iMB (%i bytes)"), s->size >> 20, s->size);
	tiemu_info(_("  ROM base    : %02x"), s->rom_base & 0xff);
	tiemu_info(_("  Hardware    : %i"), s->hw_type);
}

void ti68k_display_tib_infos(IMG_INFO *s) {
	tiemu_info(_("TIB information:"));
	tiemu_info(_("  Calculator  : %s"), ti68k_calctype_to_string(s->calc_type));
	tiemu_info(_("  Firmware    : %s"), s->version);
	tiemu_info(_("  Memory type : %s"), ti68k_romtype_to_string(s->flash));
	tiemu_info(_("  Memory size : %iMB (%i bytes)"), s->size >> 20, s->size);
	tiemu_info(_("  ROM base    : %02x"), s->rom_base & 0xff);
}

void ti68k_display_img_infos(IMG_INFO *s) {
	tiemu_info(_("Image information:"));
	tiemu_info(_("  Calculator  : %s"), ti68k_calctype_to_string(s->calc_type));
	tiemu_info(_("  Firmware    : %s"), s->version);
	tiemu_info(_("  Memory type : %s"), ti68k_romtype_to_string(s->flash));
	tiemu_info(_("  Memory size : %iMB (%i bytes)"), s->size >> 20, s->size);
	tiemu_info(_("  ROM base    : %02x"), s->rom_base & 0xff);
	tiemu_info(_("  Hardware    : %i"), s->hw_type);
	tiemu_info(_("  Has boot    : %s"), s->has_boot ? _("yes") : _("no"));
}

/*
    Get some information on the ROM dump:
    - size
    - ROM base address
    - FLASH/EPROM
    - os version
    - calc type
    Note: if the data field is NULL, memory is allocated.
    Otherwise, data is overwritten.
    Thanks to Kevin for HW2 detection code.
*/
int ti68k_get_rom_infos(const char *filename, IMG_INFO *rom, int preload) {
	FILE *file;
	HW_PARM_BLOCK hwblock;

	// No filename, exits
	if(!strcmp(g_basename(filename), "")) return ERR_CANT_OPEN;

	// Open file
	file = fopen(filename, "rb");
	if(file == NULL) {
		tiemu_info(_("Unable to open this file: <%s>"), filename);
		return ERR_CANT_OPEN;
	}

	// Retrieve ROM size
	fseek(file, 0, SEEK_END);
	rom->size = ftell(file);
	fseek(file, 0, SEEK_SET);

	if(rom->size < 256) return ERR_INVALID_ROM_SIZE;
	if(rom->size == 8 * MB) {
		// TiLP used to dump 8 MB images for HW4, try to load them anyway.
		tiemu_info(_("Warning: truncating 8 MB image to 4 MB: <%s>"), filename);
		rom->size = 4 * MB;
	}
	if(rom->size > 4 * MB) return ERR_INVALID_ROM_SIZE;

	if(rom->data == NULL) rom->data = malloc(rom->size + 4);
	if(rom->data == NULL) return ERR_MALLOC;
	memset(rom->data, 0xff, rom->size);
	if(fread(rom->data, 1, rom->size, file) < (size_t)rom->size) {
		tiemu_info(_("Failed to read from file: <%s>"), filename);
		fclose(file);
		return ERR_CANT_OPEN;
	}
	if(fclose(file)) {
		tiemu_info(_("Failed to close file: <%s>"), filename);
		return ERR_CANT_OPEN;
	}

	rom->has_boot = 1;
	rom->rom_base = rom->data[0x05] & 0xf0;
	rom->flash = (rom->data[0x65] & 0x0f) ? 0 : FLASH_ROM;

	get_rom_version(rom->data, rom->size, rom->version);

	if(!rom->flash) {
		rom->calc_type = TI92;
		rom->hw_type = HW1;
	} else {
		// Get hw param block to determine calc type & hw type
		if(ti68k_get_hw_param_block((uint8_t *)rom->data, rom->rom_base, &hwblock) == -1)
			return ERR_INVALID_ROM;
		ti68k_display_hw_param_block(&hwblock);

		switch(hwblock.hardwareID) {
		case HWID_TI92P:
			rom->calc_type = TI92p;
			break;
		case HWID_TI89:
			rom->calc_type = TI89;
			break;
		case HWID_V200:
			rom->calc_type = V200;
			break;
		case HWID_TI89T:
			rom->calc_type = TI89t;
			break;
		default:
			break;
		}

		if(rom->flash) {
			if(hwblock.len < 24)
				rom->hw_type = HW1;
			else
				rom->hw_type = (char)hwblock.gateArray;
		}
	}

	if(!preload) free(rom->data);

	return 0;
}

/*
    Try to get some information on the ROM dump:
    - size
    - ROM base address
    - FLASH/EPROM
    - soft version
    - calc type
*/
int ti68k_get_img_infos(const char *filename, IMG_INFO *ri) {
	FILE *f;
	IMG_INFO32 ri32;
	IMG_INFO64 ri64;

	// No filename, exits
	if(!strcmp(g_basename(filename), "")) return ERR_CANT_OPEN;

	// Check file
	if(!ti68k_is_a_img_file(filename)) {
		tiemu_warning("Images must have '.img' extension (%s).\n", filename);
		return ERR_CANT_OPEN;
	}

	// Open dest file
	f = fopen(filename, "rb");
	if(f == NULL) {
		tiemu_warning("Unable to open this file: <%s>\n", filename);
		return ERR_CANT_OPEN;
	}

	// Read header
	if(fread(&ri32, sizeof(IMG_INFO32), 1, f) < 1) {
		tiemu_warning("Failed to read from file: <%s>\n", filename);
		fclose(f);
		return ERR_CANT_OPEN;
	}
	*ri = ri32;

	// below is patch from Lionel
	if(strcmp(ri->signature, IMG_SIGN) || ri->size > 4 * MB || ri->calc_type > CALC_MAX ||
	   ri->header_size == 0 || ri->hw_type > 4 || ri->rom_base == 0) {
		// In addition to plain invalid files, this may happen if the image was
		// created on a 64-bit platform with TIEmu <= 3.03.
		// Try to read an IMG_INFO structure as it used to be written by those
		// 64-bit platforms.
		fseek(f, 0, SEEK_SET);
		if(fread(&ri64, sizeof(IMG_INFO64), 1, f) < 1) {
			tiemu_warning("Failed to read from file: <%s>\n", filename);
			fclose(f);
			return ERR_CANT_OPEN;
		} else {
			memcpy(ri->signature, &(ri64.signature), sizeof(ri64.signature));
			ri->revision = (int32_t)(ri64.revision);
			ri->header_size = (int32_t)(ri64.header_size);

			ri->calc_type = ri64.calc_type;
			memcpy(ri->version, &(ri64.version), sizeof(ri64.version));
			ri->flash = ri64.flash;
			ri->has_boot = ri64.has_boot;
			ri->size = (int32_t)(ri64.size);
			ri->hw_type = ri64.hw_type;
			ri->rom_base = ri64.rom_base;

			if(strcmp(ri->signature, IMG_SIGN) || ri->size > 4 * MB || ri->calc_type > CALC_MAX ||
			   ri->header_size == 0 || ri->hw_type > 4 || ri->rom_base == 0) {
				// Nope, it still doesn't seem to be a TIEmu image.
				tiemu_warning("Bad image: <%s>\n", filename);
				return ERR_INVALID_UPGRADE;
			} else {
				tiemu_info("Found a reasonably valid 64-bit IMG_INFO in <%s>\n", filename);
			}
		}
	}

	// Close file
	if(fclose(f)) {
		tiemu_warning("Failed to close file: <%s>\n", filename);
		return ERR_CANT_OPEN;
	}

	return 0;
}

/*
    This function loads an image.
*/
int ti68k_load_image(const char *filename) {
	IMG_INFO *img = &img_infos;
	FILE *f;
	int err;

	// Clear infos
	memset(img, 0, sizeof(IMG_INFO));

	// No filename, exits
	if(!strcmp(g_basename(filename), "")) return ERR_CANT_OPEN;

	// Load infos
	err = ti68k_get_img_infos(filename, img);
	if(err) {
		tiemu_info(_("Unable to get information on image: %s"), filename);
		return err;
	}
	ti68k_display_img_infos(img);

	// Open file
	f = fopen(filename, "rb");
	if(f == NULL) {
		tiemu_warning("Unable to open this file: <%s>\n", filename);
		return ERR_CANT_OPEN;
	}

	// Read pure data
	if(fseek(f, img->header_size, SEEK_SET)) {
		tiemu_warning("Failed to read from file: <%s>\n", filename);
		fclose(f);
		return ERR_CANT_OPEN;
	}

	img->data = malloc(img->size + 4);
	if(img->data == NULL) return ERR_MALLOC;
	if(fread(img->data, 1, img->size, f) < (size_t)img->size) {
		tiemu_warning("Failed to read from file: <%s>\n", filename);
		fclose(f);
		return ERR_CANT_OPEN;
	}

#if 1
	{
		HW_PARM_BLOCK hwblock;

		ti68k_get_hw_param_block((uint8_t *)img->data, img->rom_base, &hwblock);
		ti68k_display_hw_param_block(&hwblock);
	}
#endif

	if(fclose(f)) {
		tiemu_warning("Failed to close file: <%s>\n", filename);
		return ERR_CANT_OPEN;
	}

	img_loaded = 1;
	img_changed = 1;

	return 0;
}

/*
    Unload an image (free memory).
*/
int ti68k_unload_image_or_upgrade(void) {
	IMG_INFO *img = &img_infos;

	if(!img_loaded) return -1;

	img->data = NULL;
	img_loaded = 0;

	return 0;
}



/*
    Search the version string in the ROM
    Arguments:
    - ptr: a ROM or update image
    - size: the size of the buffer
    - version: the returned string version
*/
static int get_rom_version(char *ptr, int size, char *version) {
	int i;

	strcpy(version, "?.??");

	for(i = SPP; i < size - 16; i += 2) {
		if(is_num(ptr[i]) && (ptr[i + 1] == '.') && is_num(ptr[i + 2]) && (ptr[i + 3] == 0) &&
		   is_alnum(ptr[i + 4]) && is_alnum(ptr[i + 5]) && (ptr[i + 6] == '/') && is_alnum(ptr[i + 7]) &&
		   is_alnum(ptr[i + 8]) && (ptr[i + 9] == '/') && is_alnum(ptr[i + 10]) && is_alnum(ptr[i + 11]))
			break;

		if(is_num(ptr[i]) && (ptr[i + 1] == '.') && is_num(ptr[i + 2]) && is_num(ptr[i + 3]) &&
		   (ptr[i + 4] == 0) && is_alnum(ptr[i + 5]) && is_alnum(ptr[i + 6]) &&
		   (ptr[i + 7] == '/') && is_alnum(ptr[i + 8]) && is_alnum(ptr[i + 9]) &&
		   (ptr[i + 10] == '/') && is_alnum(ptr[i + 11]) && is_alnum(ptr[i + 12]))
			break;

		if(is_num(ptr[i]) && (ptr[i + 1] == '.') && is_num(ptr[i + 2]) && (ptr[i + 3] == 0) &&
		   is_alnum(ptr[i + 4]) && is_alnum(ptr[i + 5]) && is_alnum(ptr[i + 6]) && is_alnum(ptr[i + 7]) &&
		   is_alnum(ptr[i + 8]) && is_alnum(ptr[i + 9]) && is_alnum(ptr[i + 10]) && is_alnum(ptr[i + 11]))
			break;

		if(is_num(ptr[i]) && (ptr[i + 1] == '.') && is_num(ptr[i + 2]) && is_alnum(ptr[i + 3]) &&
		   (ptr[i + 4] == 0) && is_alnum(ptr[i + 5]) && is_alnum(ptr[i + 6]) && is_alnum(ptr[i + 7]) &&
		   is_alnum(ptr[i + 8]) && is_alnum(ptr[i + 9]) && is_alnum(ptr[i + 10]) && is_alnum(ptr[i + 11]))
			break;
	}

	if(i < size - 16) {
		int n;

		for(n = i; n < i + 16; n++) {
			if(ptr[n] == 0) {
				strcpy(version, ptr + i);
				(version)[n - i] = 0;

				return 0;
			}
		}
	}

	return 0;
}
