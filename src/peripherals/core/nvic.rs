//! Nested Vector Interrupt Controller

use crate::common::{ Register, State };

pub const ADDRESS: u32 = 0xE000_E100;
#[cfg(not(armv6m))]
pub const SIZE: usize = 897;
#[cfg(armv6m)]
pub const SIZE: usize = 200;

#[repr(C)]
pub struct Nvic {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Nvic {}

impl Nvic {
	#[inline]
	pub fn set(&mut self, b: usize, o: usize) -> &mut Self {
		self.block[b] |= 1<<o;
		self
	}

	#[inline]
	pub fn clear(&mut self, b: usize, o: usize) -> &mut Self {
		self.block[b] &= !(1 << o);
		self
	}
	#[inline]
	pub fn write_bits(&mut self, b: usize, o: usize, data: u32, size: usize) -> &mut Self {
		let mask = (1u32 << size) - 1;
		let old = self.block[b].read();
		self.block[b].write( old & !(mask << o) | ((data & mask) << o) );
		self
	}
}

impl Nvic {
	/// Request an IRQ in software
	///
	/// Writing a value to the INTID field is the same as manually pending an interrupt by setting
	/// the corresponding interrupt bit in an Interrupt Set Pending Register. This is similar to
	/// `set_pending`.
	///
	/// This method is not available on ARMv6-M chips.
	#[cfg(not(armv6m))]
	pub fn request<I>(&mut self, int: u32) {
		self.block[NVICRegs::STIR as usize].write(int);
	}

	/// Enable/Disable the interrupt `int`
	pub fn int_state(&mut self, int: u32, s: State) {
		match s {
			State::ON  => self.block[     int as usize / 32].write( 1 << (int as usize % 32) ),
			State::OFF => self.block[32 + int as usize / 32].write( 1 << (int as usize % 32) ),
		}
	}

	/// Returns the NVIC priority of interrupt
	pub fn get_priority<I>(&mut self, int: u32) -> u8 {
		#[cfg(not(armv6m))]
		{
			(self.block[NVICRegs::IPR as usize + (int as usize / 4)].read() & 0b1111_1111 << (int as usize % 4) ) as u8
		}

		#[cfg(armv6m)]
		{
			// TODO
		}
	}

	/// Is `int` active or pre-empted and stacked
	#[cfg(not(armv6m))]
	pub fn is_active<I>(&self, int: u32) -> bool {
		let mask = 1 << (int as usize % 32);
		self.block[NVICRegs::IABR as usize + (int as usize / 32)].read() & mask == mask
	}

	/// Checks if `int` is enabled
	pub fn is_enabled<I>(&self, int: u32) -> bool {
		let mask = 1 << (int as usize % 32);
		self.block[NVICRegs::ISER as usize + (int as usize / 32)].read() & mask == mask
	}

	/// Checks if `int` is pending
	pub fn is_pending<I>(&self, int: u32) -> bool {
		let mask = 1 << (int as usize % 32);
		self.block[NVICRegs::ISPR as usize + (int as usize / 32)].read() & mask == mask
	}

	/// Forces `int` into pending state
	pub fn pend<I>(&mut self, int: u32) {
		self.block[NVICRegs::ISPR as usize + (int as usize / 32)].write(1 << (int as usize % 32));
	}

	/// Sets the "priority" of `interrupt` to `prio`
	///
	/// *NOTE* See [`get_priority`](struct.NVIC.html#method.get_priority) method for an explanation
	/// of how NVIC priorities work.
	///
	/// On ARMv6-M, updating an interrupt priority requires a read-modify-write operation. On
	/// ARMv7-M, the operation is performed in a single atomic write operation.
	///
	/// # Unsafety
	///
	/// Changing priority levels can break priority-based critical sections (see
	/// [`register::basepri`](../register/basepri/index.html)) and compromise memory safety.
	pub unsafe fn set_priority<I>(&mut self, int: u32, prio: u32) {
		#[cfg(not(armv6m))]
		{
			self.write_bits( NVICRegs::IPR as usize + (int as usize / 4), (int as usize % 4) * 8, prio, 8);
		}

		#[cfg(armv6m)]
		{
			self.write_bits( NVICRegs::IPR as usize + (int as usize / 4), (int as usize % 4) * 8, prio, 8 );
		}
	}

	/// Clears `int` pending state
	pub fn unpend<I>(&mut self, int: u32) {
		self.block[NVICRegs::ICPR as usize + (int as usize / 32)].write( 1 << (int as usize % 32));
	}
}

#[derive(Debug, Copy, Clone)]
pub enum NVICRegs {
	ISER = 0,
	ICER = 32,
	ISPR = 64,
	ICPR = 96,
	IABR = 128,
	IPR = 192,
	STIR = 896,
}