//! External Interrupt/event register

use crate::common::{ Register, State, Trigger, enums::{ EXTILine } };

pub const ADDRESS: u32 = 0x4001_3C00;
pub const SIZE: usize = 6;

#[repr(C)]
pub struct Exti {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Exti {}

impl_rwio!(Exti);

impl Exti {
	/// Enable/Disable interrupt by masking it
	pub fn int_state(&mut self, int: EXTILine, s: bool) -> &mut Self {
		if s { self.set(0, int as usize) }
		else { self.clear(0, int as usize) }
	}

	/// Enable/Disable event by masking it
	pub fn event_state(&mut self, ev: EXTILine, s: bool) -> &mut Self {
		if s { self.set(1, ev as usize) }
		else { self.clear(1, ev as usize) }
	}

	/// Enables/Disables the type of trigger
	pub fn set_trigger(&mut self, line: EXTILine, trg: Trigger, s: bool) -> &mut Self {
		if s {
			match trg {
				Trigger::Rising => self.set(2, line as usize),
				Trigger::Falling => self.set(3, line as usize),
				Trigger::RiseFall => {
					self.set(2, line as usize);
					self.set(3, line as usize)
				},
			}
		} else {
			match trg {
				Trigger::Rising => self.clear(2, line as usize),
				Trigger::Falling => self.clear(3, line as usize),
				Trigger::RiseFall => {
					self.clear(2, line as usize);
					self.clear(3, line as usize)
				},
			}
		}
	}

	/// Requests a software interrupt if interrupts are enabled in that line
	/// 
	/// If the flag has already been raised, no irq is generated
	pub fn request_swie(&mut self, line: EXTILine) -> &mut Self {
		self.set(4, line as usize)
	}

	/// Clears a Software interrupt
	pub fn clear_swie(&mut self, line: EXTILine) -> &mut Self {
		self.block[5].write(1u32 << line as usize);
		self
	}

	/// Checks if the interrupt has been raised
	pub fn is_raised(&self, line: EXTILine) -> bool {
		self.is_set(5, line as usize)
	}

	/// Returns the PR register as a u32
	pub fn pending(&self) -> u32 {
		self.block[5].read()
	}
}