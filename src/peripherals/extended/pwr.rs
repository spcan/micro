//! Power Management Peripheral

use crate::common::{ Register };

pub const ADDRESS: u32 = 0x4000_7000;
pub const SIZE: usize = 6;

#[repr(C)]
pub struct Pwr {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Pwr {}

impl_rwio!(Pwr);

impl Pwr {
	/// Set the voltage at which the PWR set the flag for low voltage
	/// Accepts values from 2.2 V to 2.9 V
	/// Value are encoded with a prescaler of *10 (e.g. 2.2 -> 22)
	pub fn pvdlevel(&mut self, pvd: u32) -> &mut Self {
		let value = match pvd {
			22 => 0b000,
			23 => 0b001,
			24 => 0b010,
			25 => 0b011,
			26 => 0b100,
			27 => 0b101,
			28 => 0b110,
			29 => 0b111,
			_ => return self,
		};

		self.write_bits(0, 5, value, 3)
	}

	/// Enable the PVD
	#[inline]
	pub fn enable_pvd(&mut self) -> &mut Self {
		self.set(0, 4)
	}

	/// Disable the PVD
	#[inline]
	pub fn disable_pvd(&mut self) -> &mut Self {
		self.clear(0, 4)
	}

	/// Returns `true` if the voltage is higher than the threshold
	/// and the PVD is enabled
	pub fn voltlevel(&self) -> bool {
		self.is_set(1, 2)
	}

	/// Enable/Disable the Wakeup pin
	pub fn wakeup_state(&mut self, s: bool) -> &mut Self {
		if s { self.set(1, 8) }
		else { self.clear(1, 8) }
	}

	/// Enable/Disable the Backup Regulator
	pub fn bckpreg_state(&mut self, s: bool) -> &mut Self {
		if s { self.set(1, 9) }
		else { self.clear(1, 9) }
	}

	/// Call this function once after reset to enable the RTC
	pub fn unlockrtc(&mut self) -> &mut Self {
		self.set(0, 8)
	}
}