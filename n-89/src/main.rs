/*
 * main.rs: main interface to n-89
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

#![no_std]
#![no_main]
extern crate ndless_handler;

use failure::Fail;
use ndless::ffi::OsStrExt;
use ndless::fs::File;
use ndless::input::{get_keys, key_on_pressed, wait_no_key_pressed, Key};
use ndless::io;
use ndless::io::ErrorKind::InvalidInput;
use ndless::io::{Read, Write};
use ndless::msg::msg;
use ndless::path::{Path, PathBuf};
use ndless::prelude::*;
use ndless_sdl::video::{Surface, SurfaceFlag, VideoFlag};
use ndless_sdl::Rect;

use n_89::ffi::TiKey;
use n_89::{get_pixel, is_screen_on, run_cpu, set_key};

use crate::assets::GraphicsGlobal;
use crate::gui::{FilePicker, FunctionKeys};
use crate::util::DrawText;

use self::gui::KeyPicker;

mod assets;
mod gui;
mod util;

#[derive(Fail, Debug)]
pub enum Error {
	#[fail(display = "A File I/O error occured: {}", _0)]
	IoError(io::Error),
	#[fail(display = "An error occured in the emulator: {}", _0)]
	EmulatorError(n_89::error::Error),
}

impl From<io::Error> for Error {
	fn from(err: io::Error) -> Self {
		Error::IoError(err)
	}
}

impl From<n_89::error::Error> for Error {
	fn from(err: n_89::error::Error) -> Self {
		Error::EmulatorError(err)
	}
}

const CYCLES: u32 = 480_000;
const SCREEN_WIDTH: u16 = 320;
const SCREEN_HEIGHT: u16 = 240;
const DEFAULT_IMAGE_PATH: &str = "/documents/n-89-default-image.txt";

fn get_default_image_path() -> io::Result<(PathBuf, PathBuf)> {
	let mut f = File::open(DEFAULT_IMAGE_PATH)?;
	let mut buffer = String::new();
	f.read_to_string(&mut buffer)?;
	let path = PathBuf::from(buffer);
	if !path.exists() || !path.is_file() {
		Err(io::ErrorKind::NotFound)?
	}
	Ok((
		PathBuf::from(path.parent().ok_or(InvalidInput)?),
		PathBuf::from(path.file_name().ok_or(InvalidInput)?),
	))
}

fn get_image(screen: &Surface, assets: &GraphicsGlobal) -> io::Result<Option<PathBuf>> {
	if let Ok((folder, file)) = get_default_image_path() {
		if let Some(file) = FilePicker::new(&assets, folder, file)?.run(&screen) {
			return Ok(Some(file));
		}
	}
	Ok(FilePicker::new(&assets, "/documents", PathBuf::new())?.run(&screen))
}

fn run(screen: &Surface, assets: &GraphicsGlobal) -> Result<(), Error> {
	let image = get_image(screen, assets)?;
	let image = if let Some(image) = image {
		image
	} else {
		return Ok(());
	};
	File::create(DEFAULT_IMAGE_PATH)?.write_all(image.as_os_str().as_bytes())?;
	screen.fill(assets.bg);
	DrawText {
		text: "Initializing...",
		font: &assets.font,
		color: assets.fg,
		bg: assets.bg,
		size: 50,
		y: 125,
		x: 70,
		kerning: true,
		wrap: false,
	}
	.draw(&screen);
	screen.flip();
	initialize(image)?;
	main_loop(&screen, &assets)?;
	Ok(())
}

#[entry]
fn main() {
	ndless_sdl::init(&[ndless_sdl::InitFlag::Video]);
	ndless_sdl::mouse::set_cursor_visible(false);
	let screen = match ndless_sdl::video::set_video_mode(
		SCREEN_WIDTH as isize,
		SCREEN_HEIGHT as isize,
		if ndless::hw::has_colors() { 16 } else { 8 },
		&[SurfaceFlag::SWSurface],
		&[VideoFlag::NoFrame],
	) {
		Ok(screen) => screen,
		Err(err) => panic!("failed to set video mode: {}", err),
	};
	let assets = GraphicsGlobal::default();
	let result = run(&screen, &assets);
	let exit_result = n_89::exit();
	ndless_sdl::quit();
	wait_no_key_pressed();
	if let Err(err) = result {
		msg(
			"Error encountered",
			&format!(
				"The following error was encountered when running the emulator: {}",
				err
			),
		);
		wait_no_key_pressed();
	}
	if let Err(err) = exit_result {
		wait_no_key_pressed();
		msg(
			"Error when quitting",
			&format!(
				"The following error was encountered when quitting the emulator: {}",
				err
			),
		);
		wait_no_key_pressed();
	}
}

fn main_loop(screen: &Surface, assets: &GraphicsGlobal) -> n_89::error::Result<()> {
	let mut prev_keys: Vec<Key> = vec![];
	loop {
		let mut keys = get_keys();
		if key_on_pressed() && !keys.contains(&Key::On) {
			keys.push(Key::On);
		}
		if keys.contains(&Key::On) && keys.contains(&Key::Esc) {
			break Ok(());
		}
		if keys.contains(&Key::QuestionExclamation) {
			KeyPicker::new(&screen, &assets).run()?;
		}
		if keys.contains(&Key::Menu) {
			FunctionKeys::new(&screen, &assets).run()?;
		}
		if let Some(key) = keys
			.iter()
			.filter(|key| !prev_keys.contains(key))
			.cloned()
			.filter_map(map_alphabetic)
			.next()
		{
			let modifier = if keys.contains(&Key::Shift) {
				TiKey::TIKEY_SHIFT
			} else {
				TiKey::TIKEY_ALPHA
			};
			touch_key(&[modifier, key][..])?;
		};
		keys.iter()
			.filter(|key| !prev_keys.contains(key))
			.cloned()
			.filter_map(map_key)
			.try_for_each(|keys| touch_key(keys))?;
		run_cpu(CYCLES, CYCLES)?;
		draw_screen(&screen, assets);
		prev_keys = keys;
	}
}

fn touch_key(key: &[TiKey]) -> n_89::error::Result<()> {
	key.iter().for_each(|&key| set_key(key, true));
	run_cpu(CYCLES, CYCLES)?;
	key.iter().for_each(|&key| set_key(key, false));
	Ok(())
}

fn draw_screen(screen: &Surface, assets: &GraphicsGlobal) {
	if is_screen_on() {
		for x in 0..160 {
			for y in 0..100 {
				screen.fill_rect(
					Some(Rect {
						x: x * 2,
						y: y * 2,
						w: 2,
						h: 2,
					}),
					if get_pixel(x as usize, y as usize) {
						assets.fg
					} else {
						assets.bg
					},
				);
			}
		}
	} else {
		screen.fill_rect(
			Some(Rect {
				x: 0,
				y: 0,
				w: 320,
				h: 200,
			}),
			assets.bg,
		);
	}
	screen.fill_rect(
		Some(Rect {
			x: 0,
			y: 200,
			w: 320,
			h: 40,
		}),
		assets.bg,
	);
	screen.blit_at(&assets.bottom, 0, 200);
	screen.flip();
}

fn initialize(image: impl AsRef<Path>) -> n_89::error::Result<()> {
	n_89::load_default_configuration();
	n_89::load_image(image)?;
	n_89::init()?;
	n_89::reset();
	Ok(())
}

fn map_key(key: Key) -> Option<&'static [TiKey]> {
	match key {
		Key::On => Some(&[TiKey::TIKEY_ON]),
		Key::Esc => Some(&[TiKey::TIKEY_ESCAPE]),
		Key::Up => Some(&[TiKey::TIKEY_UP]),
		Key::Down => Some(&[TiKey::TIKEY_DOWN]),
		Key::Left => Some(&[TiKey::TIKEY_LEFT]),
		Key::Right => Some(&[TiKey::TIKEY_RIGHT]),
		Key::Key0 => Some(&[TiKey::TIKEY_0]),
		Key::Key1 => Some(&[TiKey::TIKEY_1]),
		Key::Key2 => Some(&[TiKey::TIKEY_2]),
		Key::Key3 => Some(&[TiKey::TIKEY_3]),
		Key::Key4 => Some(&[TiKey::TIKEY_4]),
		Key::Key5 => Some(&[TiKey::TIKEY_5]),
		Key::Key6 => Some(&[TiKey::TIKEY_6]),
		Key::Key7 => Some(&[TiKey::TIKEY_7]),
		Key::Key8 => Some(&[TiKey::TIKEY_8]),
		Key::Key9 => Some(&[TiKey::TIKEY_9]),
		Key::Enter => Some(&[TiKey::TIKEY_ENTER1]),
		Key::Ctrl => Some(&[TiKey::TIKEY_2ND]),
		Key::Plus => Some(&[TiKey::TIKEY_PLUS]),
		Key::Minus => Some(&[TiKey::TIKEY_MINUS]),
		Key::Multiply => Some(&[TiKey::TIKEY_MULTIPLY]),
		Key::Divide => Some(&[TiKey::TIKEY_DIVIDE]),
		Key::Period => Some(&[TiKey::TIKEY_PERIOD]),
		Key::Negative => Some(&[TiKey::TIKEY_NEGATE]),
		Key::Del => Some(&[TiKey::TIKEY_BACKSPACE]),
		Key::Catalog => Some(&[TiKey::TIKEY_CATALOG]),
		Key::Equals => Some(&[TiKey::TIKEY_EQUALS]),
		Key::EE => Some(&[TiKey::TIKEY_EE]),
		Key::LeftParenthesis => Some(&[TiKey::TIKEY_PALEFT]),
		Key::RightParenthesis => Some(&[TiKey::TIKEY_PARIGHT]),
		Key::Comma => Some(&[TiKey::TIKEY_COMMA]),
		Key::Exponent => Some(&[TiKey::TIKEY_POWER]),
		Key::EExp => Some(&[TiKey::TIKEY_DIAMOND, TiKey::TIKEY_X]),
		Key::Space => Some(&[TiKey::TIKEY_ALPHA, TiKey::TIKEY_NEGATE]),
		Key::Var => Some(&[TiKey::TIKEY_DIAMOND]),
		_ => None,
	}
}

fn map_alphabetic(key: Key) -> Option<TiKey> {
	match key {
		Key::A => Some(TiKey::TIKEY_EQUALS),
		Key::B => Some(TiKey::TIKEY_PALEFT),
		Key::C => Some(TiKey::TIKEY_PARIGHT),
		Key::D => Some(TiKey::TIKEY_COMMA),
		Key::E => Some(TiKey::TIKEY_DIVIDE),
		Key::F => Some(TiKey::TIKEY_PIPE),
		Key::G => Some(TiKey::TIKEY_7),
		Key::H => Some(TiKey::TIKEY_8),
		Key::I => Some(TiKey::TIKEY_9),
		Key::J => Some(TiKey::TIKEY_MULTIPLY),
		Key::K => Some(TiKey::TIKEY_EE),
		Key::L => Some(TiKey::TIKEY_4),
		Key::M => Some(TiKey::TIKEY_5),
		Key::N => Some(TiKey::TIKEY_6),
		Key::O => Some(TiKey::TIKEY_MINUS),
		Key::P => Some(TiKey::TIKEY_STORE),
		Key::Q => Some(TiKey::TIKEY_1),
		Key::R => Some(TiKey::TIKEY_2),
		Key::S => Some(TiKey::TIKEY_3),
		Key::U => Some(TiKey::TIKEY_PLUS),
		Key::V => Some(TiKey::TIKEY_0),
		Key::W => Some(TiKey::TIKEY_PERIOD),
		Key::T => Some(TiKey::TIKEY_T),
		Key::X => Some(TiKey::TIKEY_X),
		Key::Y => Some(TiKey::TIKEY_Y),
		Key::Z => Some(TiKey::TIKEY_Z),
		_ => None,
	}
}
