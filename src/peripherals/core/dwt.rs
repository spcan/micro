//! Data Watchpoint and Trace unit

use crate::common::{ Register };

pub const ADDRESS: u32 = 0xE000_1000;

#[cfg(not(armv6m))]
pub const SIZE: usize = 1006;

#[cfg(armv6m)]
pub const SIZE: usize = 16;

#[repr(C)]
pub struct Dwt {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Dwt {}

#[cfg(not(armv6m))]
impl Dwt {
	/// Enables the cycle counter
	pub fn enable_cycle_counter(&mut self) {
		self.block[0] |= 1;
	}

	/// Returns the current clock cycle count
	pub fn get_cycle_count(&self) -> u32 {
		self.block[1].read()
	}
}