//! External Interrupt/event register

use crate::common::{ asm, Register, State, Frequency, PeripheralBus };

use crate::common::enums::{ RCCInterrupt, DeviceClock, MClockOutput, RCCPeripheral, RCCRegister };

use crate::common::{ SrcClock, Clocks, ClockCfg };

//pub mod helper;

pub const ADDRESS: u32 = 0x4002_3800;
pub const SIZE: usize = 35;

// TODO : Set up HSI and HSE speed depending on board and chip
pub const HSIF: Frequency = Frequency::MHz(16);
pub const HSEF: Frequency = Frequency::MHz(8);
pub const LSIF: Frequency = Frequency::KHz(32);
pub const LSEF: Frequency = Frequency::Hz(32768);


#[repr(C)]
pub struct Rcc {
	clocks: Clocks,
	block: &'static mut [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Rcc {}

impl Rcc {
	pub fn clocks(&self) -> Clocks {
		self.clocks
	}
	/// Get the RCC from the given address
	pub fn from_addr(address: u32) -> Self {
		Rcc {
			clocks: Clocks::default(),
			block: unsafe{ &mut *(address as *mut _) },
		}
	}

	
}

impl_rwio!(Rcc);

/*impl Rcc {
	/// Sets bit at block and offset given
	pub fn set(&mut self, b: usize, o: usize) -> &mut Self {
		self.block[b] |= 1 << o;
		self
	}

	/// Clears bit at block and offset given
	pub fn clear(&mut self, b: usize, o: usize) -> &mut Self {
		self.block[b] &= !(1 << o);
		self
	}

	/// Checks if bit is set
	pub fn is_set(&self, r: usize, b: usize) -> bool {
		(self.block[r].read() >> b) & 1 == 1
	}

	pub fn write_bits(&mut self, b: usize, o: usize, data: u32, size: usize) -> &mut Self {
		let mask = (1u32 << size) - 1;
		let old = self.block[b].read();
		self.block[b].write( old & !(mask << o) | ((data & mask) << o) );
		self
	}
}*/

// VCO = PLLINPUT / PLLM
// PLLOUT = VCO *PLLN / PLLP
// USB/SDIO = VCO * PLLN / PLLQ

impl Rcc {
	/// Freezes the clocks, making it effective
	pub fn freeze(&mut self, cfg: ClockCfg) -> Result<(), ()> {
		// Configure if needed the PLL
		let (haspll, pllout, vco) = if let Some(pll) = cfg.pllcfg {
			self.clear(0, 24);

			// pllcfg has a layout of
			// ( (u32, u32, u32, u32?), SrcClock )
			// Last u32 depends on hardware
			let (n, m, p, q) = self.set_pll_cfg((pll.0).0, (pll.0).1, (pll.0).2, (pll.0).3);

			// If the clock selected is not the HSE, default to HSI
			match pll.1 {
				SrcClock::HSE => self.set(1, 22).set(0, 24),
				_ => self.clear(1, 22).set(0, 24),
			};

			let (pll, vco) = match pll.1 {
				SrcClock::HSE => ((HSEF.hz() * n / m) / p, HSEF.hz() / m),
				_ => (( (HSIF.hz() / m) * n ) / p, HSIF.hz() / m),
			};

			(true, Frequency::from( pll ), Frequency::from( vco ))
		} else {
			(false, Frequency::Hz(0), Frequency::Hz(0))
		};

		#[cfg(feature = "debug")]
		{
			use cortex_m_semihosting::hprintln;

			if haspll {
				hprintln!("Has PLL. PLL Frequency = {:?}. VCO Frequency = {:?}", pllout, vco);
			} else {
				hprintln!("Does not have PLL");
			}
		}

		// With PLL configured, set the correct SYSCLK
		let sysf = match cfg.sysclk.0 {
			SrcClock::HSI => {
				self.set_sysclk_source(0b00);
				HSIF
			},
			SrcClock::HSE => {
				self.set_sysclk_source(0b01);
				HSEF
			},
			SrcClock::PLL if haspll => {
				#[cfg(feature = "debug")]
				{
					use cortex_m_semihosting::hprintln;
					hprintln!("SEtting PLL as SYSCLK");
				}
				self.set_sysclk_source(0b10);

				pllout
			},
			_ => return Err(()),
		};

		#[cfg(feature = "debug")]
		{
			use cortex_m_semihosting::hprintln;
			hprintln!("System Frequency = {:?}", sysf);
			hprintln!("Source: {:b}", self.block[2].read() & 0b1111);
		}

		// Here we have SYSCLK set up and PLL (if it is enabled)
		// Now configure buses (AHB, APB1, APB2)
		// First AHB1
		let ahb1f = Frequency::from( sysf.hz() / self.set_hpre1(cfg.hpre.0) );

		#[cfg(feature = "debug")]
		{
			use cortex_m_semihosting::hprintln;

			hprintln!("AHB1 frequency = {:?}", ahb1f);
		}

		let mut ahb2f = Frequency::Hz(0);
		let mut ahb3f = Frequency::Hz(0);

		// Conditional compilation for ahb2 & ahb3
		#[cfg(feature = "ahb3")]
		{
			ahb3f = sysf / self.set_hpre3(cfg.hpre.2);
		}
		#[cfg(feature = "ahb2")]
		{
			ahb2f = sysf / self.set_hpre2(cfg.hpre.1);
		}

		// All possible AHB buses set
		// Now start setting all APB buses
		let apb1f = Frequency::Hz( ahb1f.hz() / self.set_ppre1(cfg.ppre.0) );

		#[cfg(feature = "debug")]
		{
			use cortex_m_semihosting::hprintln;

			hprintln!("APB1 frequency = {:?}", apb1f);
		}

		let mut apb2f = Frequency::Hz(0);
		let mut apb3f = Frequency::Hz(0);

		// TODO : Check documentation for the rest of the APB buses

		let mut i2sf = Frequency::Hz(0);

		// Set PLLI2S if enabled
		#[cfg(feature = "plli2s")]
		{
			// This will get trimmed
			let m =
				if cfg.i2scfg.1 < 2 { 2 }
				else { cfg.i2scfg.1 };
			let n = 
				if cfg.i2scfg.0 < 50 { 50 }
				else if cfg.i2scfg.0 > 432 { 432 }
				else { cfg.i2scfg.0 };

			// This will get trimmed
			let r = 
				if cfg.i2scfg.2 < 2 { 2 }
				else { cfg.i2scfg.2 };

			// Disable, set, enable
			self.clear(0, 26)
				.write_bits(19,  0, m, 6)
				.write_bits(19,  6, n, 9)
				.write_bits(19, 28, r, 3)
				.set(0, 26);

			while !self.is_set(0, 27) {}

			i2sf = (vco * n / m) / r;
		}

		// All clocks and buses enabled

		// TODO : Check that no frequency goes higher than allowed in debug mode

		self.clocks = Clocks {
			sysf,
			ahb1f,
			apb1f,
			apb2f,
			apb3f,
			pllout,
			i2sf,
		};

		Ok(())

	}
}

impl Rcc {
	#[cfg(not(any(feature = "sdio", feature = "usbotg")))]
	#[inline]
	fn set_pll_cfg(&mut self, plln: u32, pllm: u32, pllp: u32, _pllq: u32) -> (u32, u32, u32, u32) {
		// Clear the registers
		const CLEARINT: u32 = 0b111111 | (0b111111111 << 6) | (0b11 << 16);

		#[cfg(debug_assertions)]
		{
			match plln {
				50..=432 => (),
				_ => panic!(),
			}

			match pllm {
				2..=50 => (),
				_ => panic!(),
			}
		}

		let (p, reg) = match pllp {
			0..=2 => (2, 0b00),
			3 | 4 => (4, 0b01),
			5 | 6 => (6, 0b10),
			_ => (8, 0b11),
		};

		// Set the registers 
		let setint = pllm | (plln << 6) | (reg << 16);

		self.block[1] &= !(CLEARINT);
		self.block[1] |= setint;

		#[cfg(feature = "debug")]
		use cortex_m_semihosting::hprintln;
		{
			hprintln!("CREATED PLLCFG regsiter = {:b}", setint);
		}

		(plln, pllm, p, 0)
	}

	#[cfg(any(feature = "sdio", feature = "usbotg"))]
	#[inline]
	fn set_pll_cfg(&mut self, plln: u32, pllm: u32, pllp: u32, pllq: u32) -> (u32, u32, u32, u32) {
		// Clear the registers
		const CLEARINT: u32 = 0b111111 | (0b111111111 << 6) | (0b11 << 16) | (0b1111 << 24);

		// Set the registers 
		let setint = pllm | (plln << 6) | (pllp << 16) | (pllq << 24);

		self.block[1] &= !(CLEARINT);
		self.block[1] |= setint;

		(plln, pllm, pllp, pllq)
	}

	#[inline]
	fn set_ppre1(&mut self, ppre: u32) -> u32 {
		if ppre < 2 {
			self.clear(2, 12);
			1
		} else if ppre < 4 {
			self.write_bits(2, 10, 0b100, 3);
			2
		} else if ppre < 8 {
			self.write_bits(2, 10, 0b101, 3);
			4
		} else if ppre < 16 {
			self.write_bits(2, 10, 0b110, 3);
			8
		} else {
			self.write_bits(2, 10, 0b111, 3);
			16
		}
	}

	#[inline]
	fn set_hpre1(&mut self, hpre: u32) -> u32 {
		if hpre < 2 {
			self.clear(2, 7);
			1
		} else if hpre < 4 {
			self.write_bits(2, 4, 0b1000, 4);
			2
		} else if hpre < 8 {
			self.write_bits(2, 4, 0b1001, 4);
			4
		} else if hpre < 16 {
			self.write_bits(2, 4, 0b1010, 4);
			8
		} else if hpre < 64 {
			self.write_bits(2, 4, 0b1011, 4);
			16
		} else if hpre < 128 {
			self.write_bits(2, 4, 0b1100, 4);
			64
		}else if hpre < 256 {
			self.write_bits(2, 4, 0b1101, 4);
			128
		} else if hpre < 512 {
			self.write_bits(2, 4, 0b1110, 4);
			256
		} else {
			self.write_bits(2, 4, 0b1111, 4);
			512
		}
	}

	#[inline]
	fn set_sysclk_source(&mut self, value: u32) -> &mut Self {
		self.write_bits(2, 0, value, 2);
		while (self.block[2].read() & 0b1100) >> 2 != value {}
		self
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
	pub fn int_state(&mut self, s: bool, int: RCCInterrupt) -> &mut Self {
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

		if s { self.set(3, offset)   }
		else { self.clear(3, offset) }
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
		let offsets = peripheral.offsets();

		self.set(offsets.0, offsets.1);
		// 10 cycles delay
		asm::delay(9);
		self.clear(offsets.0, offsets.1)
	}

	/// Enables/Disables the given peripheral
	pub fn peripheral_state(&mut self, s: bool, peripheral: RCCPeripheral) -> &mut Self {
		let offsets = peripheral.offsets();

		if s { self  .set(offsets.0 + 8, offsets.1) }
		else { self.clear(offsets.0 + 8, offsets.1) }
	}

	/// Enables/Disables the given peripheral when in Low Power mode
	pub fn lp_peripheral_state(&mut self, s: bool, peripheral: RCCPeripheral) -> &mut Self {
		let offsets = peripheral.offsets();

		if s { self  .set(offsets.0 + 16, offsets.1) }
		else { self.clear(offsets.0 + 16, offsets.1) }
	}
}

/// Backup Domain Control Register (BDCR)
impl Rcc {
	/// Resets the Backup Domain
	pub fn reset_bck_domain(&mut self) -> &mut Self {
		self.set(RCCRegister::BDCR as usize, 16)
	}

	/// Enable/Disable RTC clock
	pub fn rtc_state(&mut self, s: bool) -> &mut Self {
		if s { self.set(RCCRegister::BDCR as usize, 15) }
		else { self.clear(RCCRegister::BDCR as usize, 15) }
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
	pub fn lse_high_drive_state(&mut self, s: bool) -> &mut Self {
		if s { self.set(RCCRegister::BDCR as usize, 3) }
		else {self.clear(RCCRegister::BDCR as usize, 3) }
	}

	/// Enable/Disable LSE Bypass with an external clock
	pub fn lse_bypass_state(&mut self, s: bool) -> &mut Self {
		if s { self.set(RCCRegister::BDCR as usize, 2) }
		else { self.clear(RCCRegister::BDCR as usize, 2) }
	}

	/// Clears **ALL** the Reset Flags
	pub fn clear_rst_flags(&mut self) -> &mut Self {
		self.set(RCCRegister::CSR as usize, 24)
	}
}

/// Spread Spectrum Clock Generator Register (SSCGR)
impl Rcc {
	/// Enables/Disables the SSCG
	/// Fails if it's disabled before the PLL or enabled after the PLL
	pub fn sscg_state(&mut self, s: bool) -> Result<&mut Self, ()> {
		if s & !self.is_set(0, 24) { Ok( self.set(RCCRegister::SSCGR as usize, 31)   )}
		else if self.is_set(0, 24) { Ok( self.clear(RCCRegister::SSCGR as usize, 31) )}
		else { Err(()) }
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
	pub fn timpre_state(&mut self, s: bool) -> &mut Self {
		if s { self.set(RCCRegister::DCKCFGR as usize, 24) }
		else { self.clear(RCCRegister::DCKCFGR as usize, 24) }
	}
}