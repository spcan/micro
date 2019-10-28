mod flags;
mod errors;
mod interrupt;

pub use self::flags::*;
pub use self::errors::*;
pub use self::interrupt::*;

#[derive(Debug, Copy, Clone)]
pub enum FrameFormat {
	MSB,
	LSB,
}

#[derive(Debug, Copy, Clone)]
pub enum DFFormat {
	Bit16,
	Bit8,
}