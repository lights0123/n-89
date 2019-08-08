/*
 * convert.rs: program to create ROM images from TI upgrade files
 * Copyright (C) 2019 Ben Schattinger
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111 USA
 */

use alloc::boxed::Box;

use byteorder::BigEndian;
use ndless::io::{Read, Seek};
use ndless::prelude::*;
use zerocopy::byteorder::{U16, U32};
use zerocopy::AsBytes;

use crate::error::Error;
use crate::ffi::{
	img_changed, img_infos, img_loaded, DeviceType_DEVICE_TYPE_89, DeviceType_DEVICE_TYPE_92P,
};

const TI89_AMS: u8 = 0x23;
const TI89_APPL: u8 = 0x24;
const TI89_CERTIF: u8 = 0x25;
const TI89_LICENSE: u8 = 0x3E;
/// system privileged part
const SPP_LENGTH: usize = 0x12000;
/// offset from SPP to boot
const BOOT_OFFSET: usize = 0x88;
const FLASH_ROM: u8 = 2;

#[derive(AsBytes)]
#[repr(C)]
struct HwParamBlock {
	len: U16<BigEndian>,
	hardware_id: U32<BigEndian>,
	hardware_revision: U32<BigEndian>,
	boot_major: U32<BigEndian>,
	boot_revision: U32<BigEndian>,
	boot_build: U32<BigEndian>,
	gate_array: U32<BigEndian>,
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum CalcType {
	TI89 = (1 << 1),
	TI92p = (1 << 2),
	V200 = (1 << 3),
	TI89t = (1 << 4),
}

impl CalcType {
	fn hardware_id(self) -> u32 {
		match self {
			CalcType::TI89 => 3,
			CalcType::TI92p => 1,
			CalcType::V200 => 8,
			CalcType::TI89t => 9,
		}
	}
	fn hardware_revision(self) -> u32 {
		match self {
			CalcType::TI89 => 2,
			CalcType::TI92p => 1,
			CalcType::V200 => 2,
			CalcType::TI89t => 2,
		}
	}
}

pub fn load_upgrade<T: Read + Seek>(src: &mut T) -> Result<(), Error> {
	// 0x00 - 0x07: signature ("**TIFL**")
	//        0x08: major revision number
	//        0x09: minor revision number
	//        0x0A: flags
	//        0x0B: object type
	//        0x0C: revision day
	//        0x0D: revision month
	// 0x0E - 0x0F: revision year
	// 0x11 - 0x18: name
	//        0x30: device type
	//        0x31: data type
	//        0x49: HW ID
	// 0x4A - 0x4D: data length
	let mut header = [0; 0x4E];
	let mut first_byte = [0;1];
	src.read_exact(&mut first_byte)?;
	if &first_byte == b"*" {
		header[0] = b'*';
	} else {
		src.read_exact(&mut header[0..1])?;
	}
	src.read_exact(&mut header[1..])?;
	if &header[0..8] != b"**TIFL**" {
		Err(Error::InvalidUpgrade)?
	}
	if header[0x31] != TI89_LICENSE
		&& ![
			DeviceType_DEVICE_TYPE_89 as u8,
			DeviceType_DEVICE_TYPE_92P as u8,
		]
		.contains(&header[0x30])
	{
		Err(Error::InvalidImage)?
	}
	if ![TI89_AMS, TI89_APPL, TI89_CERTIF, TI89_LICENSE].contains(&header[0x31]) {
		Err(Error::InvalidImage)?
	}
	let data_size = {
		let mut data_size = [0; 4];
		data_size.copy_from_slice(&header[0x4A..0x4E]);
		u32::from_le_bytes(data_size) as usize
	};
	if data_size > 4 * 1024 * 1024 - 65536 {
		Err(Error::InvalidUpgrade)?
	}
	let mut data_vec = vec![0xff; data_size + SPP_LENGTH].into_boxed_slice();
	let data: &mut [u8] = &mut data_vec[..];
	src.read_exact(&mut data[SPP_LENGTH..])?;
	let rom_base: u8 = data[SPP_LENGTH + BOOT_OFFSET + 5] & 0xf0;
	let calc_type = match (header[0x30] as u32, rom_base) {
		(DeviceType_DEVICE_TYPE_89, 0x20) => CalcType::TI89,
		(DeviceType_DEVICE_TYPE_89, 0x80) => CalcType::TI89t,
		(DeviceType_DEVICE_TYPE_92P, 0x20) => CalcType::V200,
		(DeviceType_DEVICE_TYPE_92P, 0x40) => CalcType::TI92p,
		_ => Err(Error::InvalidUpgrade)?,
	};
	let hw_type = if calc_type == CalcType::TI89t { 3u8 } else { 2 };
	// ROM layout:
	// data[BOOT_OFFSET..BOOT_OFFSET+256]
	// 0xFEEDBABE
	// 0x00
	// rom_base
	// 0x0108
	// HwParamBlock
	// 0xFF up to end of SPP_LENGTH
	// data[SPP_LENGTH..]
	{
		let (boot, system) = data.split_at_mut(SPP_LENGTH);
		boot[..256].copy_from_slice(&system[BOOT_OFFSET..BOOT_OFFSET + 256]);
	}
	data[256..260].copy_from_slice(&0xFEED_BABEu32.to_be_bytes()[..]);
	data[260] = 0;
	data[261] = rom_base;
	data[262..264].copy_from_slice(&0x0108u16.to_be_bytes()[..]);
	data[264..290].copy_from_slice(
		HwParamBlock {
			len: U16::new(24),
			hardware_id: U32::new(calc_type.hardware_id()),
			hardware_revision: U32::new(calc_type.hardware_revision()),
			boot_major: U32::new(1),
			boot_revision: U32::new(1),
			boot_build: U32::new(1),
			gate_array: U32::new(hw_type as u32),
		}
		.as_bytes(),
	);
	unsafe {
		img_changed = 1;
		img_loaded = 1;
		img_infos.calc_type = calc_type as u8;
		img_infos.rom_base = rom_base;
		img_infos.flash = FLASH_ROM;
		img_infos.hw_type = hw_type;
		img_infos.version = *b"?.??\0";
		img_infos.size = data_vec.len() as i32;
		img_infos.data = Box::leak(data_vec).as_mut_ptr() as _;
	}
	Ok(())
}
