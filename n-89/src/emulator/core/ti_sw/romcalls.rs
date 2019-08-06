use ndless::prelude::*;

use crate::emulator::core::ti_sw::timem::rd_long;
use crate::ffi::tihw;
use crate::libc::*;

#[no_mangle]
pub unsafe extern "C" fn romcalls_is_addr(_addr: u32) -> i32 {
	-1
}

#[no_mangle]
pub unsafe extern "C" fn romcalls_get_name(_id: i32) -> *const cty::c_char {
	"not loaded\0".as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn romcalls_get_table_infos(base: *mut uint32_t, size: *mut uint32_t) {
	*base = 0;
	*size = 0;
	if tihw.calc_type == 1 << 0 { return };
	*base = rd_long(tihw.rom.offset(0x12000 + 0x88 + 0xC8));
	*size = rd_long(tihw.rom.offset(((*base - 4) & 0x0fffff) as _));
}

#[no_mangle]
pub unsafe extern "C" fn romcalls_get_symbol_address(id: i32, addr: *mut u32) {
	let base = rd_long(tihw.rom.offset(0x12000 + 0x88 + 0xC8));
	*addr = rd_long(tihw.rom.offset(((base & 0x0fffff) as i32 + id * 4) as _));
}
