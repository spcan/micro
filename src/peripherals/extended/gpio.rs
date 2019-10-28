//! General Purpose I/O (GPIO)


use crate::common::{ asm, Register, GPIOPin, PortConfig, AltFunction, GPIOSpeed, PUPD, OutputType };
use crate::common::Pin;

pub const ADDRESS_A: u32 = 0x4002_0000;
pub const ADDRESS_B: u32 = 0x4002_0400;
pub const ADDRESS_C: u32 = 0x4002_0800;
pub const ADDRESS_D: u32 = 0x4002_0C00;
pub const ADDRESS_E: u32 = 0x4002_1000;
pub const ADDRESS_F: u32 = 0x4002_1C00;

pub const SIZE: usize = 10;

#[repr(C)]
pub struct Gpio {
	base: u32,
	pins: u32,
	block: &'static mut [Register<u32>; SIZE],
}

impl Gpio {
	/// Get the GPIO at `address`
	pub unsafe fn new(address: u32) -> Self {
		Gpio {
			base: address,
			pins: (1 << 16) - 1,
			block: &mut *(address as *mut _),
		}
	}

	/// Request access to the `n`th pin
	pub fn pin(&mut self, n: u32) -> Result<Pin, ()> {
		if n > 15 { return Err(()); }

		match self.pins & (1 << n) == 0 {
			true => Err(()),
			_ => {
				// This avoids data race conditions
				// If another thread tries to get this pin, it will block this one or the other
				self.pins ^= 1 << n;
				// Waits 20 cycles
				asm::delay(20);
				match self.pins & (1 << n) == 0 {
					true => 
					Ok(
						Pin::new(
							self.base,
							n
						)
					),
					_ => Err(()),
				}
			},
		}
	}
}

impl_rwio!(Gpio);

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
	pub fn set_ospeed(&mut self, pin: GPIOPin, speed: GPIOSpeed) -> &mut Self {
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
		self.block[6].write( 1u32 << pin as usize );
		self
	}

	/// Resets/Clears the given port
	pub fn reset_port(&mut self, pin: GPIOPin) -> &mut Self {
		self.block[6].write( 1u32 << (pin as usize + 16) );
		self
	}

	/// Sets the AltFunction for `pin`
	pub fn set_af(&mut self, pin: GPIOPin, af: AltFunction) -> &mut Self {
		let offsets = match pin as usize {
			0..=7  => (8,  pin as usize * 4),
			8..=15 => (9, (pin as usize - 8) * 4),
			_ => unreachable!(),
		};

		self.write_bits(offsets.0, offsets.1, af as u32, 4)
	}
}