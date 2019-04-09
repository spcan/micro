//! Debug Control Block

use crate::common::{ Register };

pub const ADDRESS: u32 = 0xE000_EDF0;
pub const SIZE: usize = 4;

#[repr(C)]
pub struct Dcb {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Dcb {}

impl Dcb {
	/// Enables TRACE. This is for example required by the
	/// `peripheral::DWT` cycle counter to work properly.
	/// As by STM documentation, this flag is not reset on
	/// soft-reset, only on power reset.
	pub fn enable_trace(&mut self) {
	// Set bit 24
		self.block[3] |= (1 << 24);
	}

	/// Disables TRACE. See `DCB::enable_trace()` for more details
	pub fn disable_trace(&mut self) {
	// Reset bit 24
		self.block[3] &= !(1 << 24);
	}

	/// Returns true if there is a debugger attached. It may not work properly. See note below
	///
	/// Note: This function is [reported not to
	/// work](http://web.archive.org/web/20180821191012/https://community.nxp.com/thread/424925#comment-782843)
	/// on Cortex-M0 devices. Per the ARM v6-M Architecture Reference Manual, "Access to the DHCSR
	/// from software running on the processor is IMPLEMENTATION DEFINED". Indeed, from the
	/// [Cortex-M0+ r0p1 Technical Reference Manual](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0484c/BABJHEIG.html), "Note Software cannot access the debug registers."
	pub fn is_debugger_attached(&self) -> bool {
		self.block[0].read() as u8 & 1 == 1
	}
}