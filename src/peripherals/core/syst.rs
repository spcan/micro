//! SysTick - System Timer

use crate::common::{ Register, State };

pub const ADDRESS: u32 = 0xE000_E010;
pub const SIZE: usize = 4;

#[repr(C)]
pub struct SysTick {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for SysTick {}

impl_rwio!(SysTick);

impl SysTick {
	/// Clears current value to 0
	/// 
	/// After clearing, the wrapper bit turns to 0
	pub fn clear_current(&mut self) -> &mut Self {
		self.block[2].write(0);
		self
	}

	/// Enables/Disables the counter
	/// "The SysTick counter reload and current value are undefined at reset, the correct
	/// initialization sequence for the SysTick counter is:
	///
	/// - Program reload value
	/// - Clear current value
	/// - Program Control and Status register"
	///
	/// The sequence translates to `self.set_reload(x); self.clear_current(); self.counter_state(State::ON)`
	pub fn counter_state(&mut self, s: bool) -> &mut Self {
		if s { self.set(0, 0) }
		else { self.clear(0, 0) }
	}

	/// Enables/Disables SysTick interrupt
	pub fn interrupt_state(&mut self, s: bool) -> &mut Self {
		if s { self.set(0, 1) }
		else { self.clear(0, 1) }
	}

	/// Gets clock source
	/// 
	/// It can clear the wrapper bit
	pub fn get_clock_source(&mut self) -> SysTClock {
		match self.is_set(0, 2) {
			false => SysTClock::External,
			true => SysTClock::Core,
		}
	}

	/// Gets current counter value
	pub fn get_current(&self) -> u32 {
		self.block[2].read()
	}

	/// Gets reload value
	pub fn get_reload(&self) -> u32 {
		self.block[1].read()
	}

	/// Returns the reload value with which the counter would wrap every 10 ms
	pub fn get_ticks_per_10ms(&self) -> u32 {
		self.block[3].read() & 0x00FFFFFF
	}

	/// Checks if an external reference clock is available
	pub fn has_ref_clock(&self) -> bool {
		!self.is_set(3, 31)
	}

	/// Checks if the counter wrapped
	/// 
	/// This operation clears the wrapper bit
	pub fn has_wrapped(&self) -> bool {
		self.is_set(0, 16)
	}

	/// Checks if counter is enabled
	/// 
	/// This operation clears the wrapper bit
	pub fn is_counter_enabled(&self) -> bool {
		self.is_set(0, 0)
	}

	/// Checks if interrupt is enabled
	/// 
	/// This operation clears the wrapper bit
	pub fn is_interrupt_enabled(&self) -> bool {
		self.is_set(0, 1)
	}

	/// Checks if the calibration is precise
	/// 
	/// Returns `false` if using the reload value returned by
	/// `get_ticks_per_10ms()` may result in a period significantly deviating
	/// from 10 ms.
	pub fn is_precise(&self) -> bool {
		!self.is_set(3, 30) 
	}

	/// Sets clock source
	pub fn set_clock_source(&mut self, src: SysTClock) -> &mut Self {
		match src {
			SysTClock::External => self.clear(0, 2),
			SysTClock::Core => self.set(0, 2),
		}
	}

	/// Sets reload value
	///
	/// Valid values are between `1` and `0x00FF_FFFF`.
	///
	/// *NOTE* To make the timer wrap every `N` ticks set the reload value to `N - 1`
	pub fn set_reload(&mut self, value: u32) -> &mut Self {
		self.block[1].write(value & 0x00FFFFFF);
		self
	}
}

#[derive(Debug, Copy, Clone)]
pub enum SysTClock {
	Core,
	External,
}