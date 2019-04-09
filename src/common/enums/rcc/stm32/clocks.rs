//! Available clocks in the STM32 devices

/// Available Clocks
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeviceClock {
	SYSCLK,
	PLL,
	HSE,
	HSI,
	LSE,
	LSI,
	PLLI2S,
}

impl DeviceClock {
	/// Returns the clock offsets for ON / OFF
	pub fn offsets(&self) -> (usize, usize) {
		use super::RCCRegister;

		match *self {
			DeviceClock::SYSCLK => (0,0),
			DeviceClock::PLL => (RCCRegister::CR as usize, 24),
			DeviceClock::PLLI2S => (RCCRegister::CR as usize, 26),
			DeviceClock::HSI => (RCCRegister::CR as usize, 0),
			DeviceClock::HSE => (RCCRegister::CR as usize, 16),
			DeviceClock::LSI => (RCCRegister::CSR as usize, 0),
			DeviceClock::LSE => (RCCRegister::BDCR as usize, 0),
		}
	}
}
