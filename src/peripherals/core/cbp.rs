//! Cache and Branch Predictor 
//!
//! Available only on ARMv7-M

use crate::common::{ Register };

pub const ADDRESS: u32 = 0xE000_EF50;
pub const SIZE: usize = 11;

#[repr(C)]
pub struct Cbp {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Cbp {}

impl Cbp {
	/// I-Cache invalidate all to PoU
	#[inline]
	pub fn iciallu(&mut self) {
		self.block[0].write(0);
	}

	/// I-Cache invalidate by MVA to PoU
	#[inline]
	pub fn icimvau(&mut self, mva: u32) {
		self.block[2].write(mva);
	}

	/// D-Cache invalidate by MVA to PoC
	#[inline]
	pub fn dcimvac(&mut self, mva: u32) {
		self.block[3].write(mva);
	}

	/// D-Cache invalidate by set-way
	#[inline]
	pub fn dcisw(&mut self, set: u16, way: u16) {
		self.block[4].write(  ((way as u32 & 0x3) << 30) | ((set as u32 & 0x1FF) << 5)  );
	}

	/// D-Cache clean by MVA to PoU
	#[inline]
	pub fn dccmvau(&mut self, mva: u32) {
		self.block[5].write(mva);
	}

	/// D-Cache clean by MVA to PoC
	#[inline]
	pub fn dccmvac(&mut self, mva: u32) {
		self.block[6].write(mva);
	}

	/// D-Cache clean by set-way
	#[inline]
	pub fn dccsw(&mut self, set: u16, way: u16) {
		self.block[7].write(  ((way as u32 & 0x3) << 30) | ((set as u32 & 0x1FF) << 5)  );
	}

	/// D-Cache clean and invalidate by MVA to PoC
	#[inline]
	pub fn dccimvac(&mut self, mva: u32) {
		self.block[8].write(mva);
	}

	// D-Cache clean and invalidate by set-way
	#[inline]
	pub fn dccisw(&mut self, set: u16, way: u16) {
		self.block[9].write(  ((way as u32 & 0x3) << 30) | ((set as u32 & 0x1FF) << 5)  );
	}

	/// Branch Predictor invalidate all
	#[inline]
	pub fn bpiall(&mut self) {
		self.block[10].write(0);
	}
}