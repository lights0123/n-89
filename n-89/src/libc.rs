pub use crate::emulator::core::dbg::bkpts::*;
pub use crate::emulator::core::ti68k_def::*;
pub use crate::emulator::core::ti_sw::er_codes::ercodes_get_name;
pub use crate::emulator::core::ti_sw::handles::{heap_get_block_addr, heap_search_for_address};
pub use crate::emulator::core::ti_sw::romcalls::{romcalls_get_name, romcalls_is_addr};

extern "C" {
	pub fn vasnprintf(
		arg1: *mut cty::c_char,
		arg2: *mut usize,
		arg3: *const cty::c_char,
		__VALIST: core::ffi::VaList,
	) -> *mut cty::c_char;
	pub fn printf(format: *const cty::c_char, _: ...) -> cty::c_int;
}
