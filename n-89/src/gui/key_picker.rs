/*
 * key_picker.rs: displays a GUI for picking keys to press
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

use ndless::alloc::string::String;
use ndless::input::{get_keys, wait_key_pressed, wait_no_key_pressed, Key};
use ndless::prelude::*;
use ndless_sdl::gfx::primitives::Graphics;
use ndless_sdl::video::Color::RGB;
use ndless_sdl::video::{Color, Surface};
use ndless_sdl::Rect;

use n_89::ffi::TiKey;

use crate::assets::GraphicsGlobal;
use crate::util::DrawText;

type KeyMapping = (&'static [&'static str], &'static [TiKey]);

const KEY_MAPPINGS: &[KeyMapping] = &[
	(&["F1"], &[TiKey::TIKEY_F1]),
	(&["F2"], &[TiKey::TIKEY_F2]),
	(&["F3"], &[TiKey::TIKEY_F3]),
	(&["F4"], &[TiKey::TIKEY_F4]),
	(&["F5"], &[TiKey::TIKEY_F5]),
	(&["F6"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_F1]),
	(&["F7"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_F2]),
	(&["F8"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_F3]),
	(&["Y="], &[TiKey::TIKEY_2ND, TiKey::TIKEY_F1]),
	(&["Window"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_F2]),
	(&["Graph"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_F3]),
	(
		&["Tableset", "Tblset"],
		&[TiKey::TIKEY_2ND, TiKey::TIKEY_F4],
	),
	(&["Table"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_F5]),
	(&["2nd", "Second"], &[TiKey::TIKEY_2ND]),
	(&["Cut"], &[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_2ND]),
	(&["Shift"], &[TiKey::TIKEY_SHIFT]),
	(&["Copy"], &[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_SHIFT]),
	(&["Escape"], &[TiKey::TIKEY_ESCAPE]),
	(&["Quit"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_ESCAPE]),
	(&["Paste"], &[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_ESCAPE]),
	(&["Diamond"], &[TiKey::TIKEY_DIAMOND]),
	(&["Alpha"], &[TiKey::TIKEY_ALPHA]),
	(
		&["Alpha lock", "A lock"],
		&[TiKey::TIKEY_2ND, TiKey::TIKEY_ALPHA],
	),
	(&["Apps"], &[TiKey::TIKEY_APPS]),
	(&["Switch Apps"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_APPS]),
	(&["Home"], &[TiKey::TIKEY_HOME]),
	(&["Custom"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_HOME]),
	(&["Mode"], &[TiKey::TIKEY_MODE]),
	(&["Triangle"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_MODE]),
	(&["Underscore"], &[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_MODE]),
	(&["Catalog"], &[TiKey::TIKEY_CATALOG]),
	(&["i"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_CATALOG]),
	(&["Infinity"], &[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_CATALOG]),
	(&["Backspace"], &[TiKey::TIKEY_BACKSPACE]),
	(&["Insert"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_BACKSPACE]),
	(&["Delete"], &[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_BACKSPACE]),
	(&["Clear"], &[TiKey::TIKEY_CLEAR]),
	(&["Natural Log", "ln"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_X]),
	(&["Ex"], &[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_X]),
	(&["sin"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_Y]),
	(&["sin-1"], &[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_Y]),
	(&["cos"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_Z]),
	(&["cos-1"], &[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_Z]),
	(&["tan"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_T]),
	(&["tan-1"], &[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_T]),
	(&["Exponent", "Caret", "Power"], &[TiKey::TIKEY_POWER]),
	(&["Pi"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_POWER]),
	(&["Theta"], &[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_POWER]),
	(&["Equals"], &[TiKey::TIKEY_EQUALS]),
	(&["Apostrophe"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_POWER]),
	(&["Left Parenthesis"], &[TiKey::TIKEY_PALEFT]),
	(
		&[
			"Left Curly Bracket",
			"Left Curly Brace",
			"Left Brace",
			"Opening Curly Bracket",
			"Opening Curly Brace",
			"Opening Brace",
		],
		&[TiKey::TIKEY_2ND, TiKey::TIKEY_PALEFT],
	),
	(&["Right Parenthesis"], &[TiKey::TIKEY_PARIGHT]),
	(
		&[
			"Right Curly Bracket",
			"Right Curly Brace",
			"Right Brace",
			"Closing Curly Bracket",
			"Closing Curly Brace",
			"Closing Brace",
		],
		&[TiKey::TIKEY_2ND, TiKey::TIKEY_PARIGHT],
	),
	(&["Comma"], &[TiKey::TIKEY_COMMA]),
	(
		&["Left Bracket", "Opening Bracket"],
		&[TiKey::TIKEY_2ND, TiKey::TIKEY_COMMA],
	),
	(&["Divide"], &[TiKey::TIKEY_DIVIDE]),
	(
		&["Right Bracket", "Closing Bracket"],
		&[TiKey::TIKEY_2ND, TiKey::TIKEY_DIVIDE],
	),
	(&["Pipe"], &[TiKey::TIKEY_PIPE]),
	(
		&["Degrees", "Circle"],
		&[TiKey::TIKEY_2ND, TiKey::TIKEY_PIPE],
	),
	(
		&["Integral", "Curved F"],
		&[TiKey::TIKEY_2ND, TiKey::TIKEY_7],
	),
	(
		&["Derivative", "Curved D"],
		&[TiKey::TIKEY_2ND, TiKey::TIKEY_8],
	),
	(&["Semicolon"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_9]),
	(&["Multiply", "Times"], &[TiKey::TIKEY_MULTIPLY]),
	(
		&["Square Root", "Radical"],
		&[TiKey::TIKEY_2ND, TiKey::TIKEY_MULTIPLY],
	),
	(&["EE"], &[TiKey::TIKEY_EE]),
	(&["Angle"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_EE]),
	(&["Key"], &[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_EE]),
	(&["Colon"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_4]),
	(&["Math"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_5]),
	(&["Mem"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_6]),
	(&["Minus", "Subtract"], &[TiKey::TIKEY_MINUS]),
	(&["Var Link"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_MINUS]),
	(&["Store"], &[TiKey::TIKEY_STORE]),
	(&["Recall", "Rcl"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_STORE]),
	(&["Quote"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_1]),
	(&["Backslash"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_2]),
	(&["Units"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_3]),
	(&["Plus", "Add"], &[TiKey::TIKEY_PLUS]),
	(&["Char"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_PLUS]),
	(&["On"], &[TiKey::TIKEY_ON]),
	(&["Off"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_ON]),
	(&["Less Than"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_0]),
	(&["Period", "Dot", "Decimal"], &[TiKey::TIKEY_PERIOD]),
	(&["Greater Than"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_PERIOD]),
	(&["Negative", "Negate"], &[TiKey::TIKEY_NEGATE]),
	(&["Answer"], &[TiKey::TIKEY_2ND, TiKey::TIKEY_NEGATE]),
	(&["Enter"], &[TiKey::TIKEY_ENTER1]),
	(
		&["Approximate"],
		&[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_ENTER1],
	),
];

pub struct KeyPicker<'a> {
	screen: &'a Surface,
	assets: &'a GraphicsGlobal,
}

const SELECTED_COLOR: Color = RGB(54, 123, 240);
const WHITE: Color = RGB(255, 255, 255);
const BLACK: Color = RGB(0, 0, 0);
const WIDTH: u16 = 280;
const HEIGHT: u16 = 200;
const SCR_WIDTH: u16 = 320;
const SCR_HEIGHT: u16 = 240;
const LEFT: i16 = ((SCR_WIDTH - WIDTH) / 2) as i16;
const TOP: i16 = ((SCR_HEIGHT - HEIGHT) / 2) as i16;
const LINE_HEIGHT: i16 = 16;
const MAX_ITEMS_ON_SCREEN: usize = ((HEIGHT - 6) / (LINE_HEIGHT) as u16) as usize - 1;

fn filter_items<'a>(search_term: &'a str) -> impl Iterator<Item = &'static KeyMapping> + 'a {
	KEY_MAPPINGS.iter().filter(move |(terms, _)| {
		terms
			.iter()
			.any(|term| term.to_ascii_lowercase().contains(search_term))
	})
}

impl<'a> KeyPicker<'a> {
	pub fn new(screen: &'a Surface, assets: &'a GraphicsGlobal) -> Self {
		KeyPicker { screen, assets }
	}
	fn setup_screen(&self) {
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
		self.screen.draw_rectangle(
			(LEFT + 5, TOP + 5),
			(LEFT + WIDTH as i16 - 7, TOP + 5 + LINE_HEIGHT),
			WHITE,
		);
	}
	pub fn run(&self) -> n_89::error::Result<()> {
		let mut index = 0;
		let mut top_index = 0;
		let mut search_term = String::new();
		wait_no_key_pressed();
		let key = loop {
			self.setup_screen();
			let items: Vec<_> = filter_items(&search_term).collect();
			let on_screen = MAX_ITEMS_ON_SCREEN.min(items.len());
			let mut y = TOP + 5 + LINE_HEIGHT + 1;
			DrawText {
				text: &search_term,
				font: &self.assets.font,
				color: WHITE,
				bg: BLACK,
				size: 20,
				y: (TOP + 6 + 11) as usize,
				x: (LEFT + 7) as usize,
				kerning: true,
				wrap: false,
			}
			.draw(self.screen);
			for (i, item) in items.iter().enumerate().skip(top_index).take(on_screen) {
				let bg = if i == index {
					self.screen.fill_rect(
						Some(Rect {
							x: LEFT + 5,
							y: y - 2,
							w: WIDTH - 4 - 7,
							h: LINE_HEIGHT as u16,
						}),
						SELECTED_COLOR,
					);
					SELECTED_COLOR
				} else {
					BLACK
				};
				DrawText {
					text: &item.0[0],
					font: &self.assets.font,
					color: if i == index { BLACK } else { WHITE },
					bg,
					size: 20,
					y: y as usize + 11,
					x: (LEFT + 7) as usize,
					kerning: true,
					wrap: false,
				}
				.draw(self.screen);
				y += LINE_HEIGHT;
			}
			self.screen.flip();
			wait_key_pressed();
			let keys = get_keys();
			if let Some(&key) = keys.get(0) {
				match key {
					Key::Esc => break None,
					Key::Enter => break items.get(index).cloned(),
					Key::Down => {
						if index < items.len() - 1 {
							index += 1;
							if index >= top_index + MAX_ITEMS_ON_SCREEN {
								top_index += 1;
							}
						} else {
							index = 0;
							top_index = 0;
						}
					}
					Key::Up => {
						if index > 0 {
							index -= 1;
							if index < top_index {
								top_index -= 1;
							}
						} else {
							index = items.len() - 1;
							top_index = index.checked_sub(MAX_ITEMS_ON_SCREEN - 1).unwrap_or(0);
						}
					}
					Key::Del => {
						search_term.pop();
					}
					other => {
						if let Some(key) = key_to_char(other) {
							search_term.push(key);
							index = 0;
							top_index = 0;
						};
					}
				}
				wait_no_key_pressed();
			}
		};
		if let Some(key) = key {
			crate::touch_key(key.1)?;
			wait_no_key_pressed();
		}
		Ok(())
	}
}

fn key_to_char(key: Key) -> Option<char> {
	match key {
		Key::A => Some('a'),
		Key::B => Some('b'),
		Key::C => Some('c'),
		Key::D => Some('d'),
		Key::E => Some('e'),
		Key::F => Some('f'),
		Key::G => Some('g'),
		Key::H => Some('h'),
		Key::I => Some('i'),
		Key::J => Some('j'),
		Key::K => Some('k'),
		Key::L => Some('l'),
		Key::M => Some('m'),
		Key::N => Some('n'),
		Key::O => Some('o'),
		Key::P => Some('p'),
		Key::Q => Some('q'),
		Key::R => Some('r'),
		Key::S => Some('s'),
		Key::T => Some('t'),
		Key::U => Some('u'),
		Key::V => Some('v'),
		Key::W => Some('w'),
		Key::X => Some('x'),
		Key::Y => Some('y'),
		Key::Z => Some('z'),
		Key::Key0 => Some('0'),
		Key::Key1 => Some('1'),
		Key::Key2 => Some('2'),
		Key::Key3 => Some('3'),
		Key::Key4 => Some('4'),
		Key::Key5 => Some('5'),
		Key::Key6 => Some('6'),
		Key::Key7 => Some('7'),
		Key::Key8 => Some('8'),
		Key::Key9 => Some('9'),
		Key::Space => Some(' '),
		_ => None,
	}
}
