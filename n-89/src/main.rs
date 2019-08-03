#![no_std]
#![no_main]
extern crate ndless_handler;

use ndless::input::{get_keys, Key};
use ndless::prelude::*;
use ndless_sdl::Rect;
use ndless_sdl::video::{SurfaceFlag, VideoFlag};
use ndless_sdl::video::Color::RGB;

use n_89::{get_pixel, is_screen_on, run, set_key};
use n_89::ffi::TiKey;

const CYCLES: u32 = 480000;

#[entry]
fn main() {
	ndless_sdl::init(&[ndless_sdl::InitFlag::Video]);
	ndless_sdl::mouse::set_cursor_visible(false);
	let screen = match ndless_sdl::video::set_video_mode(
		320,
		240,
		if ndless::hw::has_colors() { 16 } else { 8 },
		&[SurfaceFlag::SWSurface],
		&[VideoFlag::NoFrame],
	) {
		Ok(screen) => screen,
		Err(err) => panic!("failed to set video mode: {}", err),
	};
	let fg = RGB(255, 255, 255);
	let bg = RGB(0, 0, 0);
	if initialize().is_ok() {
		let mut prev_keys = vec![];
		loop {
			let keys = get_keys();
			if keys.contains(&Key::Esc) { break; }
			prev_keys
				.iter()
				.filter(|key| !keys.contains(key))
				.cloned()
				.filter_map(map_key)
				.for_each(|key| set_key(key, false));
			keys
				.iter()
				.filter(|key| !prev_keys.contains(key))
				.cloned()
				.filter_map(map_key)
				.for_each(|key| set_key(key, true));
			if run(CYCLES, CYCLES).is_err() { break; }
			if is_screen_on() {
				for x in 0..160 {
					for y in 0..100 {
						screen.fill_rect(Some(Rect {
							x: x * 2,
							y: y * 2,
							w: 2,
							h: 2,
						}), if get_pixel(x as usize, y as usize) { fg } else { bg });
					}
				}
			} else {
				screen.fill(bg);
			}
			screen.flip();
			prev_keys = keys;
		}
		n_89::exit();
	}
	ndless_sdl::quit();
}

fn initialize() -> n_89::error::Result<()> {
	n_89::load_default_configuration();
	n_89::load_image("image.img.tns")?;
	n_89::init()?;
	n_89::reset();
	Ok(())
}

fn map_key(key: Key) -> Option<TiKey> {
	match key {
		Key::Up => Some(TiKey::TIKEY_UP),
		Key::Down => Some(TiKey::TIKEY_DOWN),
		Key::Left => Some(TiKey::TIKEY_LEFT),
		Key::Right => Some(TiKey::TIKEY_RIGHT),
		Key::Key0 => Some(TiKey::TIKEY_0),
		Key::Key1 => Some(TiKey::TIKEY_1),
		Key::Key2 => Some(TiKey::TIKEY_2),
		Key::Key3 => Some(TiKey::TIKEY_3),
		Key::Key4 => Some(TiKey::TIKEY_4),
		Key::Key5 => Some(TiKey::TIKEY_5),
		Key::Key6 => Some(TiKey::TIKEY_6),
		Key::Key7 => Some(TiKey::TIKEY_7),
		Key::Key8 => Some(TiKey::TIKEY_8),
		Key::Key9 => Some(TiKey::TIKEY_9),
		Key::Enter => Some(TiKey::TIKEY_ENTER1),
		Key::Ctrl => Some(TiKey::TIKEY_2ND),
		Key::Plus => Some(TiKey::TIKEY_PLUS),
		Key::Minus => Some(TiKey::TIKEY_MINUS),
		Key::Multiply => Some(TiKey::TIKEY_MULTIPLY),
		Key::Divide => Some(TiKey::TIKEY_DIVIDE),
		Key::Period => Some(TiKey::TIKEY_PERIOD),
		Key::Negative => Some(TiKey::TIKEY_NEGATE),
		Key::Del => Some(TiKey::TIKEY_BACKSPACE),
		Key::Catalog => Some(TiKey::TIKEY_CATALOG),
		Key::Equals => Some(TiKey::TIKEY_EQUALS),
		Key::EE => Some(TiKey::TIKEY_EE),
		Key::X => Some(TiKey::TIKEY_X),
		Key::Y => Some(TiKey::TIKEY_Y),
		Key::Z => Some(TiKey::TIKEY_Z),
		Key::LeftParenthesis => Some(TiKey::TIKEY_PALEFT),
		Key::RightParenthesis => Some(TiKey::TIKEY_PARIGHT),
		Key::Comma => Some(TiKey::TIKEY_COMMA),
		_ => None,
	}
}
