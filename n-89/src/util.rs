/*
 * util.rs: provides small utilities used by n-89
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

use ndless::prelude::*;
use ndless_freetype::face::KerningMode;
use ndless_freetype::Face;
use ndless_sdl::video::Color::{RGB, RGBA};
use ndless_sdl::video::{Color, Surface};
use ndless_sdl::Rect;
use unicode_segmentation::UnicodeSegmentation;

pub fn color_mix(color: Color, bg: Color, alpha: u8) -> Color {
	let (bg_r, bg_g, bg_b, bg_alpha) = match bg {
		Color::RGB(r, g, b) => (u32::from(r), u32::from(g), u32::from(b), 255),
		Color::RGBA(r, g, b, alpha) => (u32::from(r), u32::from(g), u32::from(b), u32::from(alpha)),
	};
	let (r, g, b, alpha) = match color {
		RGB(r, g, b) => (u32::from(r), u32::from(b), u32::from(g), u32::from(alpha)),
		RGBA(r, g, b, col_alpha) => (
			u32::from(r),
			u32::from(b),
			u32::from(g),
			u32::from(col_alpha) * u32::from(alpha) / 255,
		),
	};
	let r = (r * alpha / 255) + (bg_r * bg_alpha * (255 - alpha) / (255 * 255));
	let g = (g * alpha / 255) + (bg_g * bg_alpha * (255 - alpha) / (255 * 255));
	let b = (b * alpha / 255) + (bg_b * bg_alpha * (255 - alpha) / (255 * 255));
	RGB(r as u8, g as u8, b as u8)
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct DrawText<'a> {
	pub text: &'a str,
	pub font: &'a Face,
	pub color: Color,
	pub bg: Color,
	pub size: isize,
	pub y: usize,
	pub x: usize,
	pub kerning: bool,
	pub wrap: bool,
}

impl DrawText<'_> {
	pub fn draw(&self, scr: &Surface) {
		let text = self.text;
		let font = self.font;
		let color = self.color;
		let bg = self.bg;
		let mut pen = ndless_freetype::Vector { x: 0, y: 0 };
		let max_width = scr.get_width();
		let max_height = scr.get_height();
		font.set_char_size(self.size * 64, 0, 50, 0)
			.expect("failed to set character size");
		let mut mat = ndless_freetype::Matrix {
			xx: (0f64.cos() * f64::from(0x10000)) as ndless_freetype::FT_Fixed,
			xy: (-(0f64.sin()) * f64::from(0x10000)) as ndless_freetype::FT_Fixed,
			yx: (0f64.sin() * f64::from(0x10000)) as ndless_freetype::FT_Fixed,
			yy: (0f64.cos() * f64::from(0x10000)) as ndless_freetype::FT_Fixed,
		};
		let mut formatted_text = String::with_capacity(text.len());

		let mut max_char_height = 0;
		let mut min_char_height = 0;
		'outer: for word in text.split_word_bounds() {
			let graphemes: Vec<_> = word.graphemes(true).collect();
			for (index, &letter) in graphemes.iter().enumerate() {
				font.set_transform(&mut mat, &mut pen);
				let character = letter.chars().next().expect("failed to get character") as usize;
				font.load_char(character, ndless_freetype::face::LoadFlag::RENDER)
					.expect(concat!("FAILED ", line!()));
				let glyph = font.glyph();
				let bitmap = glyph.bitmap();
				let x = self.x + glyph.bitmap_left() as usize;
				let width = bitmap.width() as usize;
				let x_max = x + width;
				let cbox = glyph
					.get_glyph()
					.expect("failed to get character glyph size")
					.get_cbox(0);
				if cbox.yMin / 64 < min_char_height {
					min_char_height = cbox.yMin / 64
				}
				if cbox.yMax / 64 > max_char_height {
					max_char_height = cbox.yMax / 64
				}
				if x_max >= max_width as usize {
					if !self.wrap {
						break 'outer;
					}
					formatted_text.push('\n');
					pen.x = 0;
					break;
				}
				pen.x += glyph.advance().x;
				if self.kerning && font.has_kerning() {
					if let Some(next) = graphemes.get(index + 1) {
						let left = font.get_char_index(character);
						let next_character =
							next.chars().next().expect("failed to get character") as usize;
						let right = font.get_char_index(next_character);
						let kerning = font
							.get_kerning(left, right, KerningMode::KerningDefault)
							.expect("Failed to get kerning info");
						pen.x += kerning.x;
					}
				}
			}
			formatted_text.push_str(word);
		}
		let max_full_height = -min_char_height + max_char_height;
		let mut pen = ndless_freetype::Vector { x: 0, y: 0 };
		let graphemes: Vec<_> = formatted_text.graphemes(true).collect();
		for (index, &letter) in graphemes.iter().enumerate() {
			if letter == "\n" {
				pen.x = 0;
				pen.y -= max_full_height * 64;
				continue;
			}
			font.set_transform(&mut mat, &mut pen);
			let character = letter.chars().next().expect("failed to get character") as usize;
			font.load_char(character, ndless_freetype::face::LoadFlag::RENDER)
				.expect(concat!("FAILED ", line!()));
			let glyph = font.glyph();
			let bitmap = glyph.bitmap();
			let x = self.x + glyph.bitmap_left() as usize;
			let y = self.y - glyph.bitmap_top() as usize;
			let width = bitmap.width() as usize;
			let x_max = x + width;
			let y_max = y + bitmap.rows() as usize;
			for (row, x_scaled) in (x..x_max).enumerate() {
				let mut col = 0;
				for y_scaled in y..y_max {
					if x_scaled < max_width as usize && y_scaled < max_height as usize {
						let alpha = bitmap.buffer()[col * width + row];
						if alpha != 0 {
							scr.fill_rect(
								Some(Rect {
									x: x_scaled as i16,
									y: y_scaled as i16,
									w: 1,
									h: 1,
								}),
								color_mix(color, bg, alpha),
							);
						}
						col += 1;
					}
				}
			}
			pen.x += glyph.advance().x;
			pen.y += glyph.advance().y;
			if self.kerning && font.has_kerning() {
				if let Some(next) = graphemes.get(index + 1) {
					let left = font.get_char_index(character);
					let next_character =
						next.chars().next().expect("failed to get character") as usize;
					let right = font.get_char_index(next_character);
					let kerning = font
						.get_kerning(left, right, KerningMode::KerningDefault)
						.expect("Failed to get kerning info");
					pen.x += kerning.x;
					pen.y += kerning.y;
				}
			}
		}
	}
}
