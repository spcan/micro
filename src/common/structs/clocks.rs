//! RCC clocks

#[cfg(feature = "std")]
use std::{ default };

#[cfg(not(feature = "std"))]
use core::{ default };

use crate::common::{ Frequency, asm, VolatileStruct };

use crate::peripherals::core::syst::{ ADDRESS, SysTick, SysTClock };

use embedded_hal::blocking::delay::{ DelayMs, DelayUs };


#[derive(Debug, Copy, Clone)]
pub struct Clocks {
	pub sysf: Frequency,
	
	pub ahb1f: Frequency,
	
	pub apb1f: Frequency,
	
	pub apb2f: Frequency,
	
	pub apb3f: Frequency,

	pub pllout: Frequency,

	pub i2sf: Frequency,
}

impl DelayMs<u32> for Clocks {
	fn delay_ms(&mut self, ms: u32) {
		self.delay_us(ms * 1000);
	}
}

impl DelayUs<u32> for Clocks {
	fn delay_us(&mut self, us: u32) {
		let rvr = us * self.ahb1f.mhz();

		let mut SYST = unsafe { SysTick::from_addr(ADDRESS) };

		SYST.set_clock_source(SysTClock::Core);

		SYST.set_reload(rvr);
		SYST.clear_current();
		SYST.counter_state(true);

		while !SYST.has_wrapped() {}

		SYST.counter_state(false);
	}
}

impl default::Default for Clocks {
	fn default() -> Clocks {
		Clocks {
			sysf: Frequency::MHz(0),
			
			ahb1f: Frequency::MHz(0),
			
			apb1f: Frequency::MHz(0),
			
			apb2f: Frequency::MHz(0),
			
			apb3f: Frequency::MHz(0),

			pllout: Frequency::MHz(0),

			i2sf: Frequency::MHz(0),
		}
	}
}