//! External Interrupt/event register

use crate::common::{ asm, Register, State, RCCInterrupt, DeviceClock, MClockOutput, RCCPeripheral, RCCRegister };

pub const ADDRESS: u32 = 0x4002_3800;
pub const SIZE: usize = 35;

#[repr(C)]
pub struct Rcc {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Rcc {}

impl Rcc {
	/// Sets bit at block and offset given
	#[inline]
	pub fn set(&mut self, b: usize, o: usize) -> &mut Self {
		self.block[b] |= 1 << o;
		self
	}

	/// Clears bit at block and offset given
	#[inline]
	pub fn clear(&mut self, b: usize, o: usize) -> &mut Self {
		self.block[b] &= !(1 << o);
		self
	}

	/// Checks if bit is set
	#[inline]
	pub fn is_set(&self, r: usize, b: usize) -> bool {
		(self.block[r].read() >> b) & 1 == 1
	}

	#[inline]
	pub fn write_bits(&mut self, b: usize, o: usize, data: u32, size: usize) -> &mut Self {
		let mask = (1u32 << size) - 1;
		let old = self.block[b].read();
		self.block[b].write( old & !(mask << o) | ((data & mask) << o) );
		self
	}
}

impl Rcc {
	/// Checks wether a clock is being used so it can be turned off
	pub fn is_used(&self, clock: DeviceClock) -> bool {
		match clock {
			DeviceClock::HSI => {
				if !self.is_set(RCCRegister::PLLCFGR as usize, 22) {
					match self.block[RCCRegister::CFGR as usize].read() & 0b11 {
						0 => true,
						_ => false,
					}
				} else {
					true
				}
			},

			DeviceClock::HSE => {
				if self.is_set(RCCRegister::PLLCFGR as usize, 22) {
					true
				} else {
					match self.block[RCCRegister::CFGR as usize].read() & 0b11 {
						1 => true,
						_ => match ( self.block[RCCRegister::BDCR as usize].read() >> 8 ) & 0b11 {
							3 => true,
							_ => false,
						},
					}
				}
			},

			DeviceClock::PLL => match self.block[RCCRegister::CFGR as usize].read() & 0b11 {
				2 => true,
				_ => false,
			},

			DeviceClock::LSE => match ( self.block[RCCRegister::BDCR as usize].read() >> 8) & 0b11 {
				1 => true,
				_ => false,
			},

			DeviceClock::LSI => match ( self.block[RCCRegister::BDCR as usize].read() >> 8) & 0b11 {
				2 => true,
				_ => false,
			},

			DeviceClock::PLLI2S => false,

			DeviceClock::SYSCLK => true,
		}
	}
}


/// Register 1 methods
/// Control Register (CR)
impl Rcc {
	/// Enable/Disable `clock`
	pub fn clock_state(&mut self, s: State, clock: DeviceClock) -> Result<&mut Self, ()> {
		match s {
			State::ON => {
				let bit = clock.offsets();
				Ok( self.set(bit.0, bit.1) )
			},

			State::OFF => {
				if !self.is_used(clock) {
					let bit = clock.offsets();
					Ok( self.clear(bit.0, bit.1) )
				} else {
					Err(())
				}
			},
		}
	}

	/// Enable/Disable Clock Security System
	#[inline]
	pub fn css_state(&mut self, s: State) -> &mut Self {
		match s {
			State::ON => self.set(0, 19),
			State::OFF => self.clear(0, 19),
		}
	}

	/// Enable/Disable HSE Bypass with an external clock
	#[inline]
	pub fn hse_bypass_state(&mut self, s: State) -> &mut Self {
		match s {
			State::ON => self.set(0, 18),
			State::OFF => self.clear(0, 18),
		}
	}

	/// Read HISCAL
	#[inline]
	pub fn hsi_cal(&self) -> u32 {
		( self.block[0].read() >> 7 ) & 0b1111_1111
	}

	/// Read HSITRIM
	#[inline]
	pub fn hsi_trim(&self) -> u32 {
		( self.block[0].read() >> 3) & 0b1_1111
	}

	/// Sets the HSITRIM
	#[inline]
	pub fn set_hsi_trim(&mut self, trim: u32) -> &mut Self {
		self.write_bits(0, 3, trim, 5)
	}
}

/// Register 2 methods
/// PLL Configuration Register (PLLCFGR)
impl Rcc {
	/// Sets the PLL source.
	/// WARNING!: This operation must be done before enabling the PLL
	/// In order for this method to not fail, it defaults to HSI
	#[inline]
	pub fn pll_src(&mut self, clock: DeviceClock) -> Result<&mut Self, ()> {
		match clock {
			DeviceClock::HSE => Ok( self.set(1, 22) ),
			DeviceClock::HSI => Ok( self.clear(1, 22) ),
			_ => {
				self.clear(1, 22);
				Err(())
			},
		}
	}

	/// Sets the PLL `N` multiplication factor
	#[inline]
	pub fn plln(&mut self, n: u32) -> &mut Self {
		self.write_bits(1, 6, n, 9)
	}

	/// Sets the PLL `M` multiplication factor
	#[inline]
	pub fn pllm(&mut self, m: u32) -> &mut Self {
		self.write_bits(1, 0, m, 6)
	}

	/// Sets the PLL `P` multiplication factor
	#[inline]
	pub fn pllp(&mut self, p: u32) -> &mut Self {
		self.write_bits(1, 16, p, 2)
	}

	/// Sets the PLL `Q` multiplication factor
	#[inline]
	pub fn pllq(&mut self, q: u32) -> &mut Self {
		self.write_bits(1, 24, q, 4)
	}
}

/// Register 3 methods
/// Clock Configuration Register (CFGR)
impl Rcc {
	/// Sets the system clock to `clock`
	/// Defaults to `HSI`
	/// It will loop 1000 times to wait for hardware ready flag and return `Err(())` for a timeout
	pub fn sys_clock_src(&mut self, clock: DeviceClock) -> Result<&mut Self, ()> {
		let value = match clock {
			DeviceClock::PLL => 2,
			DeviceClock::HSE => 1,
			DeviceClock::HSI => 0,
			_ => 0,
		};

		self.write_bits(2, 0, value, 2);
		for _ in 0..1000 {
			if self.block[2].read() & 0b11 == value {
				return Ok( self );
			}
		}

		Err(())
	}

	/// Sets the `mco` source `clock`
	pub fn set_mco_src(&mut self, mco: MClockOutput, clock: DeviceClock) -> &mut Self {
		let value = match clock {
			DeviceClock::PLL => 3,
			DeviceClock::HSE => 2,
			DeviceClock::PLLI2S => 1,
			DeviceClock::LSE => 1,
			DeviceClock::SYSCLK => 0,
			DeviceClock::HSI => 0,
			_ => 0,
		};

		match mco {
			MClockOutput::MCO1 => self.write_bits(2, 21, value, 2),
			MClockOutput::MCO2 => self.write_bits(2, 30, value, 2),
		}
	}

	/// Sets the `mco` prescaler
	/// Possible value:
	/// 0xx: No division
	/// 100: Division by 2
	/// 101: Division by 3
	/// 110: Division by 4
	/// 111: Division by 5
	pub fn set_mco_pre(&mut self, mco: MClockOutput, pre: u32) -> &mut Self {
		match mco {
			MClockOutput::MCO1 => self.write_bits(2, 24, pre, 3),
			MClockOutput::MCO2 => self.write_bits(2, 27, pre, 3),
		}
	}

	/// Select wether I2S takes an external clock
	pub fn i2s_external(&mut self, external: bool) -> &mut Self {
		if external {
			self.set(2, 23)
		} else {
			self.clear(2, 23)
		}
	}

	/// Selects the Real Time Clock prescaler
	/// `pre` > 1 or else it defaults to 2
	pub fn set_rtc_pre(&mut self, pre: u32) -> &mut Self {
		self.write_bits(2, 16, if pre > 1 { pre } else { 2 }, 5)
	}

	/// Sets the corresponding AP Bus High Speed prescaler
	/// Divides the AHB bus by the corresponding `pre`
	/// Possible values:
	/// 0xx: No division
	/// 100: Division by 2
	/// 101: Division by 4
	/// 110: Division by 8
	/// 111: Division by 16
	pub fn set_apb2_prescaler(&mut self, pre: u32) -> &mut Self {
		self.write_bits(2, 13, pre, 3)
	}

	/// Sets the corresponding AP Bus Low Speed prescaler
	/// Divides the AHB bus by the corresponding `pre`
	/// Possible values:
	/// 0xx: No division
	/// 100: Division by 2
	/// 101: Division by 4
	/// 110: Division by 8
	/// 111: Division by 16
	pub fn set_apb1_prescaler(&mut self, pre: u32) -> &mut Self {
		self.write_bits(2, 10, pre, 3)
	}

	/// Sets the AH Bus prescaler
	/// Divides the System Clock by the corresponding `pre`
	/// Possible values:
	/// 0xxx: No division
	/// 1000: Division by 2
	/// 1001: Division by 4
	/// 1010: Division by 8
	/// 1011: Division by 16
	/// 1100: Division by 64
	/// 1101: Division by 128
	/// 1110: Division by 256
	/// 1111: Division by 512
	pub fn set_ahb_prescaler(&mut self, pre: u32) -> &mut Self {
		self.write_bits(2, 4, pre, 4)
	}
}

/// Register 4 methods
/// Clock Interrupt Register (CIR)
impl Rcc {
	/// Clears `flag`
	pub fn clear_flag(&mut self, flag: RCCInterrupt) -> &mut Self {
		let f = match flag {
			RCCInterrupt::CSSC => 23,
			RCCInterrupt::PLLI2SRDY => 21,
			RCCInterrupt::PLLRDY => 20,
			RCCInterrupt::HSERDY => 19,
			RCCInterrupt::HSIRDY => 18,
			RCCInterrupt::LSERDY => 17,
			RCCInterrupt::LSIRDY => 16,
			_ => return self,
		};
		self.set(3, f)
	}

	/// Enable/Disable `int`
	pub fn int_state(&mut self, s: State, int: RCCInterrupt) -> &mut Self {
		let offset = match int {
			RCCInterrupt::CSSC => return self,
			RCCInterrupt::PLLI2SRDY => 13,
			RCCInterrupt::PLLRDY => 12,
			RCCInterrupt::HSERDY => 11,
			RCCInterrupt::HSIRDY => 10,
			RCCInterrupt::LSERDY => 9,
			RCCInterrupt::LSIRDY => 8,
			_ => return self,
		};

		match s {
			State::ON => self.set(3, offset),
			State::OFF => self.clear(3, offset),
		}
	}

	/// Returns `true` if the flag is raised
	pub fn is_raised(&mut self, flag: RCCInterrupt) -> bool {
		let offsets = match flag {
			RCCInterrupt::CSSC => (RCCRegister::CIR as usize , 7),
			RCCInterrupt::PLLI2SRDY => (RCCRegister::CIR as usize , 5),
			RCCInterrupt::PLLRDY => (RCCRegister::CIR as usize , 4),
			RCCInterrupt::HSERDY => (RCCRegister::CIR as usize , 3),
			RCCInterrupt::HSIRDY => (RCCRegister::CIR as usize , 2),
			RCCInterrupt::LSERDY => (RCCRegister::CIR as usize , 1),
			RCCInterrupt::LSIRDY => (RCCRegister::CIR as usize , 0),

			RCCInterrupt::LPWRRST => (RCCRegister::CSR as usize, 31),
			RCCInterrupt::WWDGRST => (RCCRegister::CSR as usize, 30),
			RCCInterrupt::IWDGRST => (RCCRegister::CSR as usize, 29),
			RCCInterrupt::SFTRST  => (RCCRegister::CSR as usize, 28),
			RCCInterrupt::PORRST  => (RCCRegister::CSR as usize, 27),
			RCCInterrupt::PINRST  => (RCCRegister::CSR as usize, 26),
			RCCInterrupt::BORRST  => (RCCRegister::CSR as usize, 25),
		};

		self.is_set(offsets.0, offsets.1)
	}
}

/// AHB1 Peripheral Reset Register AHB1RSTR
/// AHB2 Peripheral Reset Register AHB2RSTR
/// AHB3 Peripheral Reset Register AHB3RSTR
/// APB1 Peripheral Reset Register APB1RSTR
/// APB2 Peripheral Reset Register APB2RSTR
/// APB3 Peripheral Reset Register APB3RSTR
impl Rcc {
	/// Resets the given `peripheral`
	pub fn reset_peripheral(&mut self, peripheral: RCCPeripheral) -> &mut Self {
		let offsets = match peripheral {
			RCCPeripheral::DMA2  => (RCCRegister::AHB1RST as usize, 22),
			RCCPeripheral::DMA1  => (RCCRegister::AHB1RST as usize, 21),
			RCCPeripheral::CRC   => (RCCRegister::AHB1RST as usize, 12),
			RCCPeripheral::GPIOH => (RCCRegister::AHB1RST as usize,  7),
			RCCPeripheral::GPIOE => (RCCRegister::AHB1RST as usize,  4),
			RCCPeripheral::GPIOD => (RCCRegister::AHB1RST as usize,  3),
			RCCPeripheral::GPIOC => (RCCRegister::AHB1RST as usize,  2),
			RCCPeripheral::GPIOB => (RCCRegister::AHB1RST as usize,  1),
			RCCPeripheral::GPIOA => (RCCRegister::AHB1RST as usize,  0),

			RCCPeripheral::USB => (RCCRegister::AHB2RST as usize, 7),

			RCCPeripheral::PWR    => (RCCRegister::APB1RST as usize, 28),
			RCCPeripheral::I2C3   => (RCCRegister::APB1RST as usize, 23),
			RCCPeripheral::I2C2   => (RCCRegister::APB1RST as usize, 22),
			RCCPeripheral::I2C1   => (RCCRegister::APB1RST as usize, 21),
			RCCPeripheral::USART2 => (RCCRegister::APB1RST as usize, 17),
			RCCPeripheral::SPI3   => (RCCRegister::APB1RST as usize, 15),
			RCCPeripheral::SPI2   => (RCCRegister::APB1RST as usize, 14),
			RCCPeripheral::WWDG   => (RCCRegister::APB1RST as usize, 11),
			RCCPeripheral::TIM5   => (RCCRegister::APB1RST as usize,  3),
			RCCPeripheral::TIM4   => (RCCRegister::APB1RST as usize,  2),
			RCCPeripheral::TIM3   => (RCCRegister::APB1RST as usize,  1),
			RCCPeripheral::TIM2   => (RCCRegister::APB1RST as usize,  0),

			RCCPeripheral::SPI5   => (RCCRegister::APB2RST as usize, 20),
			RCCPeripheral::TIM11  => (RCCRegister::APB2RST as usize, 18),
			RCCPeripheral::TIM10  => (RCCRegister::APB2RST as usize, 17),
			RCCPeripheral::TIM9   => (RCCRegister::APB2RST as usize, 16),
			RCCPeripheral::SYSCFG => (RCCRegister::APB2RST as usize, 14),
			RCCPeripheral::SPI4   => (RCCRegister::APB2RST as usize, 13),
			RCCPeripheral::SPI1   => (RCCRegister::APB2RST as usize, 12),
			RCCPeripheral::SDIO   => (RCCRegister::APB2RST as usize, 11),
			RCCPeripheral::ADC1   => (RCCRegister::APB2RST as usize,  8),
			RCCPeripheral::USART6 => (RCCRegister::APB2RST as usize,  5),
			RCCPeripheral::USART1 => (RCCRegister::APB2RST as usize,  4),
			RCCPeripheral::TIM1   => (RCCRegister::APB2RST as usize,  0),
		};

		self.set(offsets.0, offsets.1);
		asm::delay(99);
		self.clear(offsets.0, offsets.1)
	}

	/// Enables/Disables the given peripheral
	pub fn peripheral_state(&mut self, s: State, peripheral: RCCPeripheral) -> &mut Self {
		let offsets = match peripheral {
			RCCPeripheral::DMA2  => (RCCRegister::AHB1EN as usize, 22),
			RCCPeripheral::DMA1  => (RCCRegister::AHB1EN as usize, 21),
			RCCPeripheral::CRC   => (RCCRegister::AHB1EN as usize, 12),
			RCCPeripheral::GPIOH => (RCCRegister::AHB1EN as usize,  7),
			RCCPeripheral::GPIOE => (RCCRegister::AHB1EN as usize,  4),
			RCCPeripheral::GPIOD => (RCCRegister::AHB1EN as usize,  3),
			RCCPeripheral::GPIOC => (RCCRegister::AHB1EN as usize,  2),
			RCCPeripheral::GPIOB => (RCCRegister::AHB1EN as usize,  1),
			RCCPeripheral::GPIOA => (RCCRegister::AHB1EN as usize,  0),

			RCCPeripheral::USB => (RCCRegister::AHB2EN as usize, 7),

			RCCPeripheral::PWR    => (RCCRegister::APB1EN as usize, 28),
			RCCPeripheral::I2C3   => (RCCRegister::APB1EN as usize, 23),
			RCCPeripheral::I2C2   => (RCCRegister::APB1EN as usize, 22),
			RCCPeripheral::I2C1   => (RCCRegister::APB1EN as usize, 21),
			RCCPeripheral::USART2 => (RCCRegister::APB1EN as usize, 17),
			RCCPeripheral::SPI3   => (RCCRegister::APB1EN as usize, 15),
			RCCPeripheral::SPI2   => (RCCRegister::APB1EN as usize, 14),
			RCCPeripheral::WWDG   => (RCCRegister::APB1EN as usize, 11),
			RCCPeripheral::TIM5   => (RCCRegister::APB1EN as usize,  3),
			RCCPeripheral::TIM4   => (RCCRegister::APB1EN as usize,  2),
			RCCPeripheral::TIM3   => (RCCRegister::APB1EN as usize,  1),
			RCCPeripheral::TIM2   => (RCCRegister::APB1EN as usize,  0),

			RCCPeripheral::SPI5   => (RCCRegister::APB2EN as usize, 20),
			RCCPeripheral::TIM11  => (RCCRegister::APB2EN as usize, 18),
			RCCPeripheral::TIM10  => (RCCRegister::APB2EN as usize, 17),
			RCCPeripheral::TIM9   => (RCCRegister::APB2EN as usize, 16),
			RCCPeripheral::SYSCFG => (RCCRegister::APB2EN as usize, 14),
			RCCPeripheral::SPI4   => (RCCRegister::APB2EN as usize, 13),
			RCCPeripheral::SPI1   => (RCCRegister::APB2EN as usize, 12),
			RCCPeripheral::SDIO   => (RCCRegister::APB2EN as usize, 11),
			RCCPeripheral::ADC1   => (RCCRegister::APB2EN as usize,  8),
			RCCPeripheral::USART6 => (RCCRegister::APB2EN as usize,  5),
			RCCPeripheral::USART1 => (RCCRegister::APB2EN as usize,  4),
			RCCPeripheral::TIM1   => (RCCRegister::APB2EN as usize,  0),
		};

		match s {
			State::ON => self.set(offsets.0, offsets.1),
			State::OFF => self.clear(offsets.0, offsets.1),
		}
	}

	/// Enables/Disables the given peripheral when in Low Power mode
	pub fn lp_peripheral_state(&mut self, s: State, peripheral: RCCPeripheral) -> &mut Self {
		let offsets = match peripheral {
			RCCPeripheral::DMA2  => (RCCRegister::AHB1LPEN as usize, 22),
			RCCPeripheral::DMA1  => (RCCRegister::AHB1LPEN as usize, 21),
			RCCPeripheral::CRC   => (RCCRegister::AHB1LPEN as usize, 12),
			RCCPeripheral::GPIOH => (RCCRegister::AHB1LPEN as usize,  7),
			RCCPeripheral::GPIOE => (RCCRegister::AHB1LPEN as usize,  4),
			RCCPeripheral::GPIOD => (RCCRegister::AHB1LPEN as usize,  3),
			RCCPeripheral::GPIOC => (RCCRegister::AHB1LPEN as usize,  2),
			RCCPeripheral::GPIOB => (RCCRegister::AHB1LPEN as usize,  1),
			RCCPeripheral::GPIOA => (RCCRegister::AHB1LPEN as usize,  0),

			RCCPeripheral::USB => (RCCRegister::AHB2LPEN as usize, 7),

			RCCPeripheral::PWR    => (RCCRegister::APB1LPEN as usize, 28),
			RCCPeripheral::I2C3   => (RCCRegister::APB1LPEN as usize, 23),
			RCCPeripheral::I2C2   => (RCCRegister::APB1LPEN as usize, 22),
			RCCPeripheral::I2C1   => (RCCRegister::APB1LPEN as usize, 21),
			RCCPeripheral::USART2 => (RCCRegister::APB1LPEN as usize, 17),
			RCCPeripheral::SPI3   => (RCCRegister::APB1LPEN as usize, 15),
			RCCPeripheral::SPI2   => (RCCRegister::APB1LPEN as usize, 14),
			RCCPeripheral::WWDG   => (RCCRegister::APB1LPEN as usize, 11),
			RCCPeripheral::TIM5   => (RCCRegister::APB1LPEN as usize,  3),
			RCCPeripheral::TIM4   => (RCCRegister::APB1LPEN as usize,  2),
			RCCPeripheral::TIM3   => (RCCRegister::APB1LPEN as usize,  1),
			RCCPeripheral::TIM2   => (RCCRegister::APB1LPEN as usize,  0),

			RCCPeripheral::SPI5   => (RCCRegister::APB2LPEN as usize, 20),
			RCCPeripheral::TIM11  => (RCCRegister::APB2LPEN as usize, 18),
			RCCPeripheral::TIM10  => (RCCRegister::APB2LPEN as usize, 17),
			RCCPeripheral::TIM9   => (RCCRegister::APB2LPEN as usize, 16),
			RCCPeripheral::SYSCFG => (RCCRegister::APB2LPEN as usize, 14),
			RCCPeripheral::SPI4   => (RCCRegister::APB2LPEN as usize, 13),
			RCCPeripheral::SPI1   => (RCCRegister::APB2LPEN as usize, 12),
			RCCPeripheral::SDIO   => (RCCRegister::APB2LPEN as usize, 11),
			RCCPeripheral::ADC1   => (RCCRegister::APB2LPEN as usize,  8),
			RCCPeripheral::USART6 => (RCCRegister::APB2LPEN as usize,  5),
			RCCPeripheral::USART1 => (RCCRegister::APB2LPEN as usize,  4),
			RCCPeripheral::TIM1   => (RCCRegister::APB2LPEN as usize,  0),
		};

		match s {
			State::ON => self.set(offsets.0, offsets.1),
			State::OFF => self.clear(offsets.0, offsets.1),
		}
	}
}

/// Backup Domain Control Register (BDCR)
impl Rcc {
	/// Resets the Backup Domain
	pub fn reset_bck_domain(&mut self) -> &mut Self {
		self.set(RCCRegister::BDCR as usize, 16)
	}

	/// Enable/Disable RTC clock
	pub fn rtc_state(&mut self, s: State) -> &mut Self {
		match s {
			State::ON => self.set(RCCRegister::BDCR as usize, 15),
			State::OFF => self.clear(RCCRegister::BDCR as usize, 15),
		}
	}

	/// Sets the RTC clock source
	/// Defaults to the LSI
	pub fn rtc_src(&mut self, src: Option<DeviceClock>) -> &mut Self {
		self.write_bits( RCCRegister::BDCR as usize, 8,
			match src {
				Some(clock) => match clock {
					DeviceClock::LSE => 1,
					DeviceClock::LSI => 2,
					DeviceClock::HSE => 3,
					_ => 2,
				},
				None => 0,
			}
		, 2)
	}

	/// Enables/Disables the LSE oscillator "high drive" mode
	#[inline]
	pub fn lse_high_drive_state(&mut self, s: State) -> &mut Self {
		match s {
			State::ON => self.set(RCCRegister::BDCR as usize, 3),
			State::OFF => self.clear(RCCRegister::BDCR as usize, 3),
		}
	}

	/// Enable/Disable LSE Bypass with an external clock
	#[inline]
	pub fn lse_bypass_state(&mut self, s: State) -> &mut Self {
		match s {
			State::ON => self.set(RCCRegister::BDCR as usize, 2),
			State::OFF => self.clear(RCCRegister::BDCR as usize, 2),
		}
	}

	/// Clears **ALL** the Reset Flags
	#[inline]
	pub fn clear_rst_flags(&mut self) -> &mut Self {
		self.set(RCCRegister::CSR as usize, 24)
	}
}

/// Spread Spectrum Clock Generator Register (SSCGR)
impl Rcc {
	/// Enables/Disables the SSCG
	/// Fails if it's disabled before the PLL or enabled after the PLL
	pub fn sscg_state(&mut self, s: State) -> Result<&mut Self, ()> {
		match s {
			State::ON if !self.is_set(0, 24) => Ok( self.set(RCCRegister::SSCGR as usize, 31) ),
			State::OFF if self.is_set(0, 24) => Ok( self.clear(RCCRegister::SSCGR as usize, 31) ),
			_ => Err(())
		}
	}

	/// Enable SSCG Centre spread
	pub fn sscg_center_spread(&mut self) -> Result<&mut Self, ()> {
		if self.is_set(0, 24) {
			Err(())
		} else {
			Ok( self.clear(RCCRegister::SSCGR as usize, 30) )
		}
	}

	/// Enable SSCG Down spread
	pub fn sscg_down_spread(&mut self) -> Result<&mut Self, ()> {
		if self.is_set(0, 24) {
			Err(())
		} else {
			Ok( self.set(RCCRegister::SSCGR as usize, 30) )
		}
	}

	/// Sets the Modulation Amplitude of the SSCG
	pub fn sscg_amplitude(&mut self, amplitude: u32) -> Result<&mut Self, ()> {
		if self.is_set(0, 24) {
			Err(())
		} else {
			Ok( self.write_bits(RCCRegister::SSCGR as usize, 13, amplitude, 15) )
		}
	}

	/// Sets the Modulation Period of the SSCG
	pub fn sscg_period(&mut self, period: u32) -> Result<&mut Self, ()> {
		if self.is_set(0, 24) {
			Err(())
		} else {
			Ok( self.write_bits(RCCRegister::SSCGR as usize, 0, period, 13) )
		}
	}
}

/// PLLI2S Configuration Register (PLLI2SCFGR)
impl Rcc {
	/// Sets the PLLI2S R division factor
	pub fn plli2sr(&mut self, r: u32) -> &mut Self {
		self.write_bits(RCCRegister::PLLI2SCFGR as usize, 28, r, 3)
	}

	/// Sets the PLLI2S N multiplication factor
	pub fn plli2sn(&mut self, n: u32) -> &mut Self {
		self.write_bits(RCCRegister::PLLI2SCFGR as usize, 6, n, 9)
	}

	/// Sets the PLLI2S M division factor
	pub fn plli2sm(&mut self, m: u32) -> &mut Self {
		self.write_bits(RCCRegister::PLLI2SCFGR as usize, 0, m, 6)
	}
}

/// Dedicated Clocks Configuration Register (DCKCFGR)
impl Rcc {
	/// Sets/Clears the TIMPRE bit
	/// Refer to Reference Manual for the behaviour
	#[inline]
	pub fn timpre_state(&mut self, s: State) -> &mut Self {
		match s {
			State::ON => self.set(RCCRegister::DCKCFGR as usize, 24),
			State::OFF => self.clear(RCCRegister::DCKCFGR as usize, 24),
		}
	}
}