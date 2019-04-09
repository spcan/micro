//! General Purpose I/O (GPIO)

use crate::common::{ Register, GPIOPin, PortConfig, AltFunction, GPIOSpeed, PUPD, OutputType };

pub const ADDRESS_A: u32 = 0x4002_0000;
pub const ADDRESS_B: u32 = 0x4002_0400;
pub const ADDRESS_C: u32 = 0x4002_0800;
pub const ADDRESS_D: u32 = 0x4002_0C00;
pub const ADDRESS_E: u32 = 0x4002_1000;
pub const ADDRESS_F: u32 = 0x4002_1C00;

pub const SIZE: usize = 10;

#[repr(C)]
pub struct Gpio {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Gpio {}

impl Gpio {
	/// Sets bit at block and offset given
	#[inline]
	pub fn set(&mut self, b: usize, o: usize) -> &mut Self {
		self.block[b] |= 1 << o;
		self
	}

	/// Clears bit at block and offset given
	#[inline]
	pub fn clear(&mut self, b: usize, o: usize) -> &mut Self {
		self.block[b] &= !(1 << o);
		self
	}

	/// Checks if bit is set
	#[inline]
	pub fn is_set(&self, r: usize, b: usize) -> bool {
		(self.block[r].read() >> b) & 1 == 1
	}

	#[inline]
	pub fn write_bits(&mut self, b: usize, o: usize, data: u32, size: usize) -> &mut Self {
		let mask = (1u32 << size) - 1;
		let old = self.block[b].read();
		self.block[b].write( old & !(mask << o) | ((data & mask) << o) );
		self
	}
}

impl Gpio {
	/// Set up port mode
	pub fn set_mode(&mut self, pin: GPIOPin, mode: PortConfig) -> &mut Self {
		self.write_bits( 0, pin as usize * 2, mode as u32, 2)
	}

	/// Set port output type
	pub fn set_otype(&mut self, pin: GPIOPin, otype: OutputType) -> &mut Self {
		match otype {
			OutputType::PushPull => self.clear(1, pin as usize),
			OutputType::OpenDrain => self.set(1, pin as usize),
		}
	}

	/// Set port output speed
	pub fn set_speed(&mut self, pin: GPIOPin, speed: GPIOSpeed) -> &mut Self {
		self.write_bits(2, pin as usize * 2, speed as u32, 2)
	}

	/// Set port PUPD mode
	pub fn set_pupd(&mut self, pin: GPIOPin, pupd: PUPD) -> &mut Self {
		self.write_bits(3, pin as usize * 2, pupd as u32, 2)
	}

	/// Reads input
	pub fn read(&mut self) -> u32 {
		self.block[4].read() & mask!(16)
	}

	/// Outputs `value`
	pub fn send(&mut self, value: u32) -> &mut Self {
		self.write_bits(5, 0, value, 16)
	}

	/// Sets the given port
	pub fn set_port(&mut self, pin: GPIOPin) -> &mut Self {
		self.block[6].write( 1 << pin as usize );
		self
	}

	/// Resets/Clears the given port
	pub fn reset_port(&mut self, pin: GPIOPin) -> &mut Self {
		self.block[6].write( 1 << (pin as usize * 2) );
		self
	}

	/// Sets the AltFunction for `pin`
	pub fn set_af(&mut self, pin: GPIOPin, af: AltFunction) -> &mut Self {
		let offsets = match pin as usize {
			0...7  => (8,  pin as usize * 4),
			8...15 => (9, (pin as usize - 8) * 4),
			_ => unreachable!(),
		};

		self.write_bits(offsets.0, offsets.1, af as u32, 4)
	}
}