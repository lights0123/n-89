/* Hey EMACS -*- linux-c -*- */
/* $Id: images.h 2821 2009-05-04 20:06:12Z roms $ */

/*  TiEmu - Tiemu Is an EMUlator
 *
 *  Copyright (c) 2000-2001, Thomas Corvazier, Romain Lievin
 *  Copyright (c) 2001-2003, Romain Lievin
 *  Copyright (c) 2003, Julien Blache
 *  Copyright (c) 2004, Romain Li�vin
 *  Copyright (c) 2005, Romain Li�vin
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

#ifndef __TI68K_IMAGES__
#define __TI68K_IMAGES__

#include <stdint.h>
#include "ti68k_def.h"
/*
  Definitions
*/

#define IMG_SIGN "TiEmu img v2.00"
#define IMG_REV 2 // increase this number when changing the structure below

// Please update the docs/TiEmu_img_format.txt documentation when making changes
// on the structure below

// If this structure is modified, the SAV_REVISION number (state.c)
// has to be incremented.
G_BEGIN_DECLS
typedef struct {
	char signature[16];  // "TiEmu img v2.00" (dc)
	int32_t revision;    // structure revision (compatibility)
	int32_t header_size; // size of this structure and offset to pure data (dc)

	char calc_type;   // calculator type
	char version[5];  // firmware revision
	char flash;       // EPROM or FLASH
	char has_boot;    // FLASH upgrade does not have boot
	int32_t size;     // size of pure data
	char hw_type;     // hw1 or hw2
	uint8_t rom_base; // ROM base address (MSB)

	char fill[0x40 - 42]; // round up struct to 0x40 bytes
	char *data;           // pure data (temporary use, 8 bytes)
} IMG_INFO32;
// dc = don't care for rom/tib

typedef struct {
	char signature[16];  // "TiEmu img v2.00" (dc)
	int64_t revision;    // structure revision (compatibility)
	int64_t header_size; // size of this structure and offset to pure data (dc)

	char calc_type;   // calculator type
	char version[5];  // firmware revision
	char flash;       // EPROM or FLASH
	char has_boot;    // FLASH upgrade does not have boot
	int64_t size;     // size of pure data
	char hw_type;     // hw1 or hw2
	uint8_t rom_base; // ROM base address (MSB)

	char fill[0x40 - 42]; // round up struct to 0x40 bytes
	char *data;           // pure data (temporary use, 8 bytes)
} IMG_INFO64;
// dc = don't care for rom/tib
/**
 * DeviceType:
 *
 * An enumeration which contains some device IDs for FLASH apps:
 **/
typedef enum {
	DEVICE_TYPE_83P = 0x73,
	DEVICE_TYPE_73 = 0x74,
	DEVICE_TYPE_89 = 0x98,
	DEVICE_TYPE_92P = 0x88,
} DeviceType;
/**
 * CalcModel:
 *
 * An enumeration which contains several calculator models.
 *
 **/
typedef enum {
	CALC_NONE = 0,
	CALC_TI73,
	CALC_TI82,
	CALC_TI83,
	CALC_TI83P,
	CALC_TI84P,
	CALC_TI85,
	CALC_TI86,
	CALC_TI89,
	CALC_TI89T,
	CALC_TI92,
	CALC_TI92P,
	CALC_V200,
	CALC_TI84P_USB,
	CALC_TI89T_USB,
	CALC_NSPIRE,
	CALC_TI80,
	CALC_TI84PC,
	CALC_TI84PC_USB,
	CALC_TI83PCE_USB,
	CALC_TI84PCE_USB,
	CALC_TI82A_USB,
	CALC_TI84PT_USB,
	CALC_MAX_TILP
} CalcModel;
#define VARNAME_MAX 1024
/**
 * FlashPage:
 * @offset: FLASH offset (see TI link guide).
 * @page: FLASH page (see TI link guide).
 * @flag: see link guide.
 * @size: length of pure data (up to 16384 bytes)
 * @data: pure FLASH data.
 *
 * A generic structure used to store the content of a TI-Z80 memory page for FLASH.
 **/
typedef struct {
	uint16_t addr;
	uint16_t page;
	uint8_t flag;
	uint16_t size;
	uint8_t *data;
} FlashPage;
/**
 * FlashContent:
 * @model: a calculator model.
 * @revision_major:
 * @revision_minor:
 * @flags:
 * @object_type:
 * @revision_day:
 * @revision_month:
 * @revision_year:
 * @name: name of FLASH app or OS
 * @device_type: a device ID
 * @data_type: a type ID
 * @hw_id: hardware ID (used on TI-68k only, 0 otherwise)
 * @data_length: length of pure data
 * @data_part: pure FLASH data (TI-68k, TI-eZ80) or license or certificate
 * @num_pages: number of FLASH pages (TI-Z80 only)
 * @pages: NULL-terminated array of FLASH pages (TI-Z80 only)
 * @next: pointer to next structure (linked list of contents)
 *
 * A generic structure used to store the content of a FLASH file (os or app).
 **/
typedef struct _FlashContent FlashContent;
struct _FlashContent {
	CalcModel model;
	CalcModel model_dst;

	// FlashHeader	header;
	uint8_t revision_major;
	uint8_t revision_minor;
	uint8_t flags;
	uint8_t object_type;
	uint8_t revision_day;
	uint8_t revision_month;
	uint16_t revision_year;
	char name[VARNAME_MAX];
	uint8_t device_type;
	uint8_t data_type;
	uint8_t hw_id;
	uint32_t data_length;

	uint8_t *data_part;     // TI-68k and TI-eZ80 FlashApps.
	unsigned int num_pages; // TI-Z80 only
	FlashPage **pages;      // TI-Z80 only

	FlashContent *next; // TI-68k only
};

#define IMG_INFO IMG_INFO32

extern int img_loaded;
extern IMG_INFO img_infos;

/*
    Functions
*/

int ti68k_is_a_rom_file(const char *filename);
int ti68k_is_a_tib_file(const char *filename);
int ti68k_is_a_img_file(const char *filename);

void ti68k_display_rom_infos(IMG_INFO *rom);
void ti68k_display_tib_infos(IMG_INFO *tib);
void ti68k_display_img_infos(IMG_INFO *img);

int ti68k_get_rom_infos(const char *filename, IMG_INFO *rom, int preload);
int ti68k_get_tib_infos(const char *filename, IMG_INFO *tib, int preload);
int ti68k_get_img_infos(const char *filename, IMG_INFO *img);

int ti68k_convert_rom_to_image(const char *src, const char *dirname, char **dst);
int ti68k_convert_tib_to_image(const char *src, const char *dirname, char **dst, int hw_type);
int ti68k_merge_rom_and_tib_to_image(const char *srcname1, const char *srcname2, const char *dirname, char **dstname);

int ti68k_load_image(const char *filename);
int ti68k_unload_image_or_upgrade(void);
G_END_DECLS
#endif
