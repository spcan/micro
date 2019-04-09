//! Abstraction over Frequency
//! 
//! 

#[cfg(feature = "std")]
use std::cmp;

#[cfg(not(feature = "std"))]
use core::cmp;

#[derive(Debug, Copy, Clone)]
pub enum Frequency {
	Hz(u32),
	KHz(u32),
	MHz(u32),
	GHz(u32),
}

impl Frequency {
	/// Returns the frequency value in Hz
	pub fn hz(&self) -> u32 {
		match self {
			Frequency::Hz(a)  => a.clone(),
			Frequency::KHz(a) => a.clone() * 1_000,
			Frequency::MHz(a) => a.clone() * 1_000_000,
			Frequency::GHz(a) => a.clone() * 1_000_000_000,
		}
	}

	/// Returns the frequency value in kHz
	pub fn khz(&self) -> u32 {
		self.hz() / 1_000
	}

	/// Returns the frequency value in kHz
	pub fn mhz(&self) -> u32 {
		self.hz() / 1_000_000
	}

	/// Returns the frequency value in kHz
	pub fn ghz(&self) -> u32 {
		self.hz() / 1_000_000_000
	}
}

impl cmp::PartialEq for Frequency {
	fn eq(&self, other: &Self) -> bool {
		self.hz() == other.hz()
	}

	fn ne(&self, other: &Self) -> bool {
		self.hz() != other.hz()
	}
}

impl cmp::Eq for Frequency {}

impl cmp::Ord for Frequency {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		self.hz().cmp(&other.hz())
	}
}

impl cmp::PartialOrd for Frequency {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}