//! Module for common utilities, structs and enums

#[macro_use]
pub mod macros;

pub mod frequency;
pub mod register;
pub mod config;


pub mod asm;

reexport!{
	private:
		mod clockspeed;
		mod timestamp;
	public:
		mod enums;
		mod structs;
}


pub mod State {
	pub const ON: bool = true;
	pub const OFF: bool = false;
}

pub use self::frequency::Frequency;
pub use self::register::{ Register, VolatileStruct };
pub use self::enums::*;
