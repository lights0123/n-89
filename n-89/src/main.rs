#![no_std]
#![no_main]
extern crate ndless_handler;

use ndless::input::{get_keys, Key, key_on_pressed};
use ndless::prelude::*;
use ndless_sdl::nsdl::{Font, FontOptions};
use ndless_sdl::Rect;
use ndless_sdl::video::{Color, Surface, SurfaceFlag, VideoFlag};
use ndless_sdl::video::Color::RGB;

use n_89::{get_pixel, is_screen_on, run, set_key};
use n_89::ffi::TiKey;

use crate::key_picker::KeyPicker;

mod key_picker;

const CYCLES: u32 = 480_000;
const SCREEN_WIDTH: u16 = 320;
const SCREEN_HEIGHT: u16 = 240;
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
	let fg = RGB(255, 255, 255);
	let bg = RGB(0, 0, 0);
	let font = Font::new(FontOptions::ThinType, 255, 255, 255);
	let width = font.get_width("Initializing");
	let height = font.get_width("Initializing");
	font.draw(screen.raw, "Initializing", (SCREEN_WIDTH as i32 - width) / 2, (SCREEN_HEIGHT as i32 - height) / 2);
	screen.flip();
	if initialize().is_ok() {
		main_loop(&screen, &font, fg, bg);
		n_89::exit();
	}
	ndless_sdl::quit();
}

fn main_loop(screen: &Surface, font: &Font, fg: Color, bg: Color) -> n_89::error::Result<()> {
	let mut prev_keys = vec![];
	loop {
		let keys = get_keys();
		if key_on_pressed() {
			return Ok(());
		}
		if keys.contains(&Key::QuestionExclamation)
			&& !prev_keys.contains(&Key::QuestionExclamation)
		{
			KeyPicker::new(&screen).run()?;
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
		run(CYCLES, CYCLES)?;
		draw_screen(&screen, &font, fg, bg);
		prev_keys = keys;
	}
}

fn touch_key(key: &[TiKey]) -> n_89::error::Result<()> {
	key.iter().for_each(|&key| set_key(key, true));
	run(CYCLES, CYCLES)?;
	key.iter().for_each(|&key| set_key(key, false));
	Ok(())
}

fn draw_screen(screen: &Surface, font: &Font, fg: Color, bg: Color) {
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
						fg
					} else {
						bg
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
			bg,
		);
	}
	screen.fill_rect(
		Some(Rect {
			x: 0,
			y: 200,
			w: 320,
			h: 40,
		}),
		bg,
	);
//	font.draw(screen.raw, "Press ?! for key picker", 0, 203);
//	font.draw(screen.raw, "Press On to exit", 0, 212);
	screen.flip();
}

fn initialize() -> n_89::error::Result<()> {
	n_89::load_default_configuration();
	n_89::load_image("image.img.tns")?;
	n_89::init()?;
	n_89::reset();
	Ok(())
}

fn map_key(key: Key) -> Option<&'static [TiKey]> {
	match key {
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
		Key::Scratchpad => Some(&[TiKey::TIKEY_F1]),
		Key::Tab => Some(&[TiKey::TIKEY_F2]),
		Key::Doc => Some(&[TiKey::TIKEY_F3]),
		Key::Menu => Some(&[TiKey::TIKEY_F4]),
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
