//! Independent Watchdog

use crate::common::{ Register };

use embedded_hal::watchdog::*;

pub const ADDRESS: u32 = 0x4001_3C00;
pub const SIZE: usize = 6;

#[repr(C)]
pub struct Iwdg {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Iwdg {}

impl_rwio!(Iwdg);

impl Watchdog for Iwdg {
	fn feed(&mut self) {
		self.reload()
	}
}

impl Iwdg {
	/// Sets the value to be loaded into the register
	/// This operation blocks until the value has been registered in the
	/// VDD voltage domain (up to 5 cycles)
	pub fn set_countdown(&mut self, value: u32) {
		self.block[2].write(value & mask!(11));
		while self.is_set(3, 1) {}
	}

	/// Sets the prescaler into the register
	/// This operation blocks until the value has been registered in the
	/// VDD voltage domain (up to 5 cycles)
	pub fn set_prescaler(&mut self, value: u32) {
		self.block[2].write(value & mask!(3));
		while self.is_set(3, 0) {}
	}

	/// Resets the countdown, preventing it from reseting the MCU
	pub fn reload(&mut self) {
		self.block[0].write(0xAAAA)
	}

	/// Unlocks the IWDG registers
	/// Must be performed before a write to the value or prescaler
	pub fn unlock(&mut self) {
		self.block[0].write(0x5555)
	}

	/// Starts the IWDG
	pub fn start(&mut self) {
		self.block[0].write(0xCCCC)
	}
}