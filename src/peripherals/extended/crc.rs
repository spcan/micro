//! CRC Peripheral

use crate::common::{ Register };

pub const ADDRESS: u32 = 0x4002_3000;
pub const SIZE: usize = 6;

#[repr(C)]
pub struct Crc {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Crc {}

impl_rwio!(Crc);

impl Crc {
	/// Write the next data to process
	/// WARNING!! This overwrites the result of the previous calculation
	pub fn write(&mut self, data: u32) -> &mut Self {
		self.block[0].write(data);
		self
	}

	/// Read the calculation result
	pub fn read(&self) -> u32 {
		self.block[0].read()
	}

	/// Write to the independent register
	pub fn writeI(&mut self, data: u8) -> &mut Self {
		self.block[1].write(data as u32);
		self
	}

	/// Read the independent register
	pub fn readI(&self) -> u32 {
		self.block[1].read()
	}

	/// Resets the calculation
	pub fn reset(&mut self) -> &mut Self {
		self.set(2, 0)
	}
}
