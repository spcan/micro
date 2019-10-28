

mod interrupts;
mod flags;
mod errors;

pub use self::interrupts::*;
pub use self::flags::*;
pub use self::errors::*;

#[derive(Debug, Copy, Clone)]
pub enum MasterMode {
	SM,
	FM,
}

#[derive(Debug, Copy, Clone)]
pub enum I2CBitMode {
	Bit7,
	Bit10,
}

#[derive(Debug, Copy, Clone)]
pub enum DutyCycle {
	D2,
	D169,
}

#[derive(Debug, Copy, Clone)]
pub enum DualAddress {
	Addr1,
	Addr2,
}