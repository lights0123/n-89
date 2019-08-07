/*
 * assets.rs: manages the assets required by n-89
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

use ndless_freetype::Face;
use ndless_sdl::image::load_mem_gif;
use ndless_sdl::video::Color::RGB;
use ndless_sdl::video::{Color, Surface};

#[derive(Clone, Debug)]
pub struct GraphicsGlobal {
	pub folder: Surface,
	pub file: Surface,
	pub bottom: Surface,
	pub fg: Color,
	pub bg: Color,
	pub font: Face,
}

impl Default for GraphicsGlobal {
	fn default() -> Self {
		let library = ndless_freetype::Library::init().expect("failed to create freetype library");
		GraphicsGlobal {
			folder: load_mem_gif(FOLDER).expect("failed to load folder.gif"),
			file: load_mem_gif(FILE).expect("failed to load disk.gif"),
			bottom: load_mem_gif(BOTTOM).expect("failed to load bottom.gif"),
			fg: RGB(255, 255, 255),
			bg: RGB(0, 0, 0),
			font: library
				.new_static_face(ROBOTO, 0)
				.expect("failed to load font"),
		}
	}
}

pub const FOLDER: &[u8] = include_bytes!("assets/folder.gif");
pub const FILE: &[u8] = include_bytes!("assets/disk.gif");
pub const BOTTOM: &[u8] = include_bytes!("assets/bottom.gif");
/// See https://stackoverflow.com/a/44440992 for how the reduced file was generated
/// Also turn off "PS Glyph Names" in FontForge
pub const ROBOTO: &[u8] = include_bytes!("assets/Roboto-Light-reduced.ttf");
