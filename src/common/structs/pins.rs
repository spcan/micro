//! Abstraction over a Pin
//! Allows to set it or clear it

#[cfg(feature = "std")]
use std::{ ptr };

#[cfg(not(feature = "std"))]
use core::{ ptr };

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Pin {
	base: u32,
	n: u32,
}

impl Pin {
	/// Sets up a new GPIO pin
	pub fn new(base: u32, n: u32) -> Self {
		Self {
			base,
			n,
		}
	}

	/// Set the pin
	pub fn set(&self) {
		unsafe { ptr::write_volatile((self.base + 0x18) as *mut _, 1u32 << self.n) }
		//self.reg.write(1 << self.n);
	}

	/// Reset the pin
	pub fn reset(&self) {
		unsafe { ptr::write_volatile((self.base + 0x18) as *mut _, 1u32 << (self.n + 16)) }
		//self.reg.write(1 << (self.n + 16))
	}

	/// Set Alternative function
	pub fn altfn(&self, af: u32) -> &Self {
		let offsets = match self.n {
			0..=7 => (self.base + 0x20,  self.n * 4      ),
			_     => (self.base + 0x24, (self.n - 8) * 4 ),
		};

		let og: u32 = unsafe { ptr::read_volatile(offsets.0 as *mut _) };

		unsafe {
			ptr::write_volatile(offsets.0 as *mut _, (og & !(0b1111 << offsets.1)) | (af << offsets.1) );
		}

		self
	}

	/// Set PullUp PullDown
	pub fn pupd(&self, pupd: u32) -> &Self {
		let dir = self.base + 0x0C;
		let og: u32 = unsafe { ptr::read_volatile(dir as *mut _) };

		unsafe {
			ptr::write_volatile( dir as *mut _, (og & !(0b11 << (self.n*2))) | (pupd << (self.n*2)) );
		}

		self
	}

	/// Set output speed
	pub fn speed(&self, ospeed: u32) -> &Self {
		let dir = self.base + 0x08;
		let og: u32 = unsafe { ptr::read_volatile(dir as *mut _) };

		unsafe {
			ptr::write_volatile( dir as *mut _, (og & !(0b11 << (self.n*2))) | (ospeed << (self.n*2)) );
		}

		self
	}

	/// Set the mode
	pub fn mode(&self, mode: u32) -> &Self {
		let og: u32 = unsafe { ptr::read_volatile(self.base as *mut _) };

		unsafe {
			ptr::write_volatile( self.base as *mut _, (og & !(0b11 << (self.n*2))) | (mode << (self.n*2)) );
		}

		self
	}

	/// Set the output type
	pub fn otype(&self, otype: u32) -> &Self {
		let dir = self.base + 0x04;
		let og: u32 = unsafe { ptr::read_volatile(dir as *mut _) };

		unsafe {
			ptr::write_volatile( dir as *mut _, (og & !(0b11 << self.n)) | (otype << self.n) );
		}

		self
	}
}