//! I2C Peripheral

use crate::common::{ Register };


pub const SIZE: usize = 6;

#[repr(C)]
pub struct FlashIface {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for FlashIface {}

impl FlashIface {
	/// Sets bit at block and offset given
	pub fn set(&mut self, b: usize, o: usize) -> &mut Self {
		self.block[b] |= 1 << o;
		self
	}

	/// Clears bit at block and offset given
	pub fn clear(&mut self, b: usize, o: usize) -> &mut Self {
		self.block[b] &= !(1 << o);
		self
	}

	/// Checks if bit is set
	pub fn is_set(&self, r: usize, b: usize) -> bool {
		(self.block[r].read() >> b) & 1 == 1
	}

	pub fn write_bits(&mut self, b: usize, o: usize, data: u32, size: usize) -> &mut Self {
		let mask = (1u32 << size) - 1;
		let old = self.block[b].read();
		self.block[b].write( old & !(mask << o) | ((data & mask) << o) );
		self
	}
}


impl FlashIface {
	/// Unlock the Flash interface
	pub fn unlock(&mut self) {
		self.block[1].write(0x45670123);
		while self.is_set(3, 16) {}
		self.block[1].write(0xCDEF89AB);
		while self.is_set(3, 16) {}
	}

	/// Unlock the Flash OTP bits
	pub fn unlock_otp(&mut self) {
		self.block[2].write(0x08192A3B);
		while self.is_set(3, 16) {}
		self.block[2].write(0x4C5D6E7F);
		while self.is_set(3, 16) {}
	}

	/// Set latency
	pub fn set_latency(&mut self, latency: u32) -> &mut Self {
		self.write_bits(0, 0, latency, 4)
	}
}