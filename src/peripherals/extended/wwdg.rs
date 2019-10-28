//! Window Watchdog

use crate::common::{ Register };

use embedded_hal::watchdog::*;

pub const ADDRESS: u32 = 0x4001_3C00;
pub const SIZE: usize = 6;

#[repr(C)]
pub struct Wwdg {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Wwdg {}

impl_rwio!(Wwdg);

impl Wwdg {
	/// The user must give a reset value
	pub fn reset(&mut self, value: u32) -> &mut Self {
		self.write_bits(0, 0, value, 7)
	}

	/// Start. The user must give an initial value.
	pub fn start(&mut self, value: u32) -> &mut Self {
		self.block[0].write( (1 << 7) | (value & mask!(7)) );
		self
	}


	/// Enables/Disables early wakeup
	/// Early wakeup creates an interrupt one tick away from reset
	pub fn ewi_enabled(&mut self, state: bool) -> &mut Self {
		if state { self.set(1, 9) }
		else     { self.clear(1, 9) }
	}

	/// Sets a divisor for the PCLK1
	/// the frequency of counting will be (PCLK1 / 4096) / div
	/// 00: div = 1
	/// 01: div = 2
	/// 10: div = 4
	/// 11: div = 8
	pub fn set_divisor(&mut self, div: u32) -> &mut Self {
		self.write_bits(1, 7, div, 2)
	}

	/// Sets the upper bound
	/// If a WWDG reset is sent before the downcounter is less than
	/// this value a reset is generated
	/// meaning if upper_bound > downcounter(when reset is sent) > 0x3F everything is good
	/// Else a reset is/has been sent
	pub fn set_bound(&mut self, bound: u32) -> &mut Self {
		self.write_bits(1, 0, bound, 7)
	}

	/// Clears EWI flag
	pub fn clear_ewi(&mut self) {
		self.block[2].write(0)
	}
}