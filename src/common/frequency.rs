//! Abstraction over Frequency
//! 
//! 

#[cfg(feature = "std")]
use std::{cmp, ops, convert};

#[cfg(not(feature = "std"))]
use core::{cmp, ops, convert};

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
		match *self {
			Frequency::Hz(a)  => a,
			Frequency::KHz(a) => a * 1_000,
			Frequency::MHz(a) => a * 1_000_000,
			Frequency::GHz(a) => a * 1_000_000_000,
		}
	}

	/// Returns the frequency value in kHz
	pub fn khz(&self) -> u32 {
		match *self {
			Frequency::Hz(a)  => a / 1000,
			Frequency::KHz(a) => a,
			Frequency::MHz(a) => a * 1_000,
			Frequency::GHz(a) => a * 1_000_000,
		}
	}

	/// Returns the frequency value in kHz
	pub fn mhz(&self) -> u32 {
		match *self {
			Frequency::Hz(a)  => a / 1_000_000,
			Frequency::KHz(a) => a / 1_000,
			Frequency::MHz(a) => a,
			Frequency::GHz(a) => a * 1_000,
		}
	}

	/// Returns the frequency value in kHz
	pub fn ghz(&self) -> u32 {
		match *self {
			Frequency::Hz(a)  => a / 1_000_000_000,
			Frequency::KHz(a) => a / 1_000_000,
			Frequency::MHz(a) => a / 1_000,
			Frequency::GHz(a) => a,
		}
	}
}

impl convert::From<u32> for Frequency {
	fn from(o: u32) -> Frequency {
		match o {
			0..=499_999 => Frequency::Hz(o),
			500_000..=499_999_999 => Frequency::KHz(o / 1000),
			500_000_000..=4_294_967_295 => Frequency::MHz(o / 1_000_000),
		}
	}
}

impl convert::From<Frequency> for u32 {
	fn from(f: Frequency) -> u32 {
		f.hz()
	}
}

impl convert::From<&'static Frequency> for u32 {
	fn from(f: &Frequency) -> u32 {
		f.hz()
	}
}

impl PartialEq<u32> for Frequency {
	fn eq(&self, other: &u32) -> bool {
		self.hz() == *other
	}

	fn ne(&self, other: &u32) -> bool {
		self.hz() != *other
	}
}


impl PartialEq<Frequency> for Frequency {
	fn eq(&self, other: &Frequency) -> bool {
		self.hz() == other.hz()
	}

	fn ne(&self, other: &Frequency) -> bool {
		self.hz() != other.hz()
	}
}

impl Eq for Frequency {}


impl<T> ops::Div<T> for Frequency
	where u32: From<T> {
	type Output = Frequency;

	fn div(self, other: T) -> Frequency {
		let new = self.hz() / u32::from(other);

		match new {
			0..=999 => Frequency::Hz(new),
			1000..=999_999 => Frequency::KHz(new / 1000),
			1000_000..=999_999_999 => Frequency::MHz(new / 1000_000),
			_ => Frequency::GHz(new / 1000_000_000),
		}
	}
}


impl<T> ops::Mul<T> for Frequency
	where u32: From<T> {
	type Output = Frequency;

	fn mul(self, other: T) -> Frequency {
		let new = self.hz() * u32::from(other);

		match new {
			0..=999 => Frequency::Hz(new),
			1000..=999_999 => Frequency::KHz(new / 1000),
			1000_000..=999_999_999 => Frequency::MHz(new / 1000_000),
			_ => Frequency::GHz(new / 1000_000_000),
		}
	}
}

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