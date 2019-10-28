//! I2C dependent interrupts

#[derive(Debug, Copy, Clone)]
pub enum I2CInterrupt {
	DMARequest = 267,
	BufferInt  = 266,
	Event      = 265,
	Error      = 264,
}

impl I2CInterrupt {
	pub fn offsets(self) -> (usize, usize) {
		let data = self as usize;

		((data >> 8) & 0b1111_1111, data & 0b1111_1111)
	}
}