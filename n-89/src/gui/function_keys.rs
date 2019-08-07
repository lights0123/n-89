/*
 * function_keys.rs: displays a GUI for pressing function keys
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

use ndless::input::{get_keys, wait_key_pressed, wait_no_key_pressed, Key};
use ndless_sdl::video::Color::RGB;
use ndless_sdl::video::{Color, Surface};
use ndless_sdl::Rect;

use n_89::ffi::TiKey;

use crate::assets::GraphicsGlobal;
use crate::util::DrawText;

pub struct FunctionKeys<'a> {
	screen: &'a Surface,
	assets: &'a GraphicsGlobal,
}

const SELECTED_COLOR: Color = RGB(54, 123, 240);
const WHITE: Color = RGB(255, 255, 255);
const BLACK: Color = RGB(0, 0, 0);
const WIDTH: u16 = 250;
const HEIGHT: u16 = 45;
const SCR_WIDTH: u16 = 320;
const SCR_HEIGHT: u16 = 240;
const LEFT: i16 = ((SCR_WIDTH - WIDTH) / 2) as i16;
const TOP: i16 = ((SCR_HEIGHT - HEIGHT) / 2) as i16;

impl<'a> FunctionKeys<'a> {
	pub fn new(screen: &'a Surface, assets: &'a GraphicsGlobal) -> Self {
		FunctionKeys { screen, assets }
	}
	pub fn run(&self) -> n_89::error::Result<()> {
		wait_no_key_pressed();
		self.screen.fill_rect(
			Some(Rect {
				x: LEFT,
				y: TOP,
				w: WIDTH,
				h: HEIGHT,
			}),
			SELECTED_COLOR,
		);
		self.screen.fill_rect(
			Some(Rect {
				x: LEFT + 2,
				y: TOP + 2,
				w: WIDTH - 4,
				h: HEIGHT - 4,
			}),
			BLACK,
		);
		DrawText {
			text: "Press function key 1-8",
			font: &self.assets.font,
			color: WHITE,
			bg: BLACK,
			size: 30,
			y: (TOP as u16 + HEIGHT - 15) as usize,
			x: (LEFT + 20) as usize,
			kerning: true,
			wrap: false,
		}
		.draw(self.screen);
		self.screen.flip();
		let key = loop {
			wait_key_pressed();
			let keys = get_keys();
			if let Some(&key) = keys.get(0) {
				match key {
					Key::Esc => break None,
					other => {
						if let Some(key) = map_keys(other) {
							break Some(key);
						};
					}
				}
				wait_no_key_pressed();
			}
		};
		if let Some(key) = key {
			crate::touch_key(key)?;
		}
		Ok(())
	}
}

fn map_keys(key: Key) -> Option<&'static [TiKey]> {
	match key {
		Key::Key1 => Some(&[TiKey::TIKEY_F1]),
		Key::Key2 => Some(&[TiKey::TIKEY_F2]),
		Key::Key3 => Some(&[TiKey::TIKEY_F3]),
		Key::Key4 => Some(&[TiKey::TIKEY_F4]),
		Key::Key5 => Some(&[TiKey::TIKEY_F5]),
		Key::Key6 => Some(&[TiKey::TIKEY_2ND, TiKey::TIKEY_F1]),
		Key::Key7 => Some(&[TiKey::TIKEY_2ND, TiKey::TIKEY_F2]),
		Key::Key8 => Some(&[TiKey::TIKEY_2ND, TiKey::TIKEY_F3]),
		_ => None,
	}
}
