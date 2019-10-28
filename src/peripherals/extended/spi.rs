//! SPI interface peripheral
//! Implements `embedded-hal` traits

use crate::common::{ Register, config::SPIConfig, SPIInterrupt, PeripheralBus, SPIFlag, SPIError, DFFormat, FrameFormat };
use crate::common::structs::Pin;
use crate::common::Frequency;
use crate::common::{ RCCPeripheral, DeviceClock };

pub const SPI1: u32 = 0x4001_3000;
pub const SPI2: u32 = 0x4000_3800;
pub const SPI3: u32 = 0x4000_3C00;
pub const SPI4: u32 = 0x4001_3400;
pub const SPI5: u32 = 0x4001_5000;

pub const SIZE: usize = 9;


/// This struct is not a direct abstraction over the hardware peripheral
/// This is due to communication protocal constraints and checks that must not 
/// be performed during communication
#[repr(C)]
pub struct Spi {
	id: RCCPeripheral,
	pins: Option<[Pin; 3]>,
	block: &'static mut [Register<u32>; SIZE],
}

impl_rwio!(Spi);

impl Spi {
	pub fn read(&mut self) -> Result<u8, SPIError> {
		let sr = self.block[2].read();

		if sr & 0b111_0000 != 0 {
			Err( SPIError::ReadErr )
		} else {
			if self.is_set(2, 0) {
				Ok( self.read_data() )
			} else {
				Err( SPIError::ReadErr )
			}
		}
	}

	pub fn send(&mut self, byte: u8) -> Result<&mut Self, SPIError> {
		let sr = self.block[2].read();

		if sr & 0b111_0000 != 0 {
			Err( SPIError::SendErr )
		} else {
			if self.is_set(2, 1) {
				Ok( self.write_data(byte) )
			} else {
				Err( SPIError::SendErr )
			}
		}
	}
}

impl Spi {
	/// Get the SPI at `address`
	pub unsafe fn from_addr(address: u32) -> Result<Self, ()> {
		let id = match address {
			SPI1 => RCCPeripheral::SPI1,
			SPI2 => RCCPeripheral::SPI2,
			SPI3 => RCCPeripheral::SPI3,
			SPI4 => RCCPeripheral::SPI4,
			SPI5 => RCCPeripheral::SPI5,
			_ => return Err(()),
		};

		Ok( Spi {
			id,
			pins: None,
			block: &mut *(address as *mut _),
		} )
	}

	/// Initialize the SPI interface in master mode
	pub fn init_master(&mut self, pins: [Pin; 3], freq: Frequency, rcc: &mut super::rcc::Rcc, lsb: bool) -> Result<&mut Self, SPIError> {
		// Disable, enable and reset, get CLOCK
		rcc.peripheral_state(true, self.id)
			.reset_peripheral(self.id);

		// Activate 8-bit transfer size
		// Disable Slave Select
		self.clear(1, 2)
			.clear(1, 4);

		// Get rate
		let br = match u32::from(rcc.clocks().apb1f / freq ) {
			0 => return Err(SPIError::InvalidBus),
			1..=2    => 0b000,
			3..=5    => 0b001,
			6..=11   => 0b010,
			12..=23  => 0b011,
			24..=39  => 0b100,
			40..=95  => 0b101,
			96..=191 => 0b110,
			_        => 0b111,
		};

		// Create lambda for modification
		// Creates less read/writes, less latency, less time wasted in IO
		let setup = move |mut reg| {
			// Set Motorola or 8-bit mode
			reg &= !(1 << 11);
			// Set master mode
			reg |= 1 << 2;
			// Set Baud rate
			reg &= !(mask!(3) << 3);
			reg |= (br & mask!(3)) << 3;
			// Enable preripheral
			reg |= 1 << 6;
			// Set lsb or msb
			if lsb { reg |= 1 << 7 }
			else   { reg &= !(1 << 7) }
			// Set SSM and SSI (software managed slave select)
			reg |= 0b11 << 8;
			// Disable CRC and FUll-duplex mode
			reg &= !(0b101 << 13);
			reg
		};

		self.modify(0, &setup);

			// Set Motorola or 8-bit mode
		/*self.clear(0, 11)
			// Seet master mode
			.set(0, 2)
			// Set Baud rate
			.write_bits(0, 3, br, 3)
			// Enable peripheral
			.set(0, 6)
			// Set LSB or MSB
			.lsb(lsb)
			// Set SSM and SSI (software managed Slave Select)
			.set(0, 9)
			.set(0, 8)
			// Disable CRC
			.clear(0, 13)
			// Full-duplex mode
			.clear(0, 15);*/

		self.pins = Some( pins );

		Ok( self )
	}

	/// Stop execution of the SPI interface
	pub fn deinit(&mut self) -> [Pin; 3] {
		let tmp = match &self.pins {
			Some(p) => [p[0], p[1], p[2]],
			_ => panic!("Cannot deinit an uninitialized peripheral"),
		};

		self.clear(0, 6);

		self.pins = None;
		[tmp[0], tmp[1], tmp[2]]
	}

	fn lsb(&mut self, s: bool) -> &mut Self {
		if s { self  .set(0, 7) }
		else { self.clear(0, 7) }
	}

	/*
	/// Sets up a new SPI interface with the given configuration
	pub fn init(&mut self, cfg: SPIConfig, rcc: &mut super::rcc::Rcc) -> Result<&mut Self, SPIError> {
		// Enable peripheral in RCC
		let pclk = match cfg.spibus {
			PeripheralBus::APB1 => {
				rcc.peripheral_state(true, cfg.id)
					.get_speed(PeripheralBus::APB1)
			},

			PeripheralBus::APB2 => {
				rcc.peripheral_state(true, cfg.id)
					.get_speed(PeripheralBus::APB2)
			},

			_ => return Err(SPIError::InvalidBus),
		};

		// Disable SS output
		self.clear(1, 2);

		// Select Baud Rate
		let br = match (pclk / cfg.freq).into() {
			0 => return Err(SPIError::FreqHigherThanBus),
			1...2 => 0b000,
			3...5 => 0b001,
			6...11 => 0b010,
			12...23 => 0b011,
			24...47 => 0b100,
			48...95 => 0b101,
			96...191 => 0b110,
			_ => 0b111,
		};

		self.write_bits(0, 0, cfg.cr1(), 16);

		self.write_bits(0, 3, br, 3);

		Ok( self.set(0, 6) )
	}
	*/
}


/// Configuration methods
impl Spi {
	/// Enable/Disable bidirectional mode
	pub fn bidi_state(&mut self, s: bool) -> &mut Self {
		match s {
			true => self.clear(0, 15),
			_ => self.set(0, 15),
		}
	}

	/// Enable/Disable full-duplex mode
	pub fn full_duplex_state(&mut self, s: bool) -> &mut Self {
		match s {
			true => self.clear(0, 10),
			_ => self.set(0, 10),
		}
	}

	/// Set transmit mode
	pub fn transmit(&mut self) -> &mut Self {
		self.set(0, 14)
	}

	/// Set receive mode
	pub fn receive(&mut self) -> &mut Self {
		self.clear(0, 14)
	}

	/// Enable/Disable CRC calculation
	pub fn crc_state(&mut self, s: bool) -> Result<&mut Self, ()> {
		if self.is_set(0, 6) {
			Err(())
		} else {
			match s {
				true => Ok( self.set(0, 13) ),
				_ => Ok( self.clear(0, 13) ),
			}
		}
	}

	/// Enable a CRC transfer next
	/// When SPI is configured in full-duplex or tx only mode,
	/// CRCNEXT must be set as soon as the last data is written to SPI_DR
	/// When SPI is configured in rx only mode, CRCNEXT must be set after
	/// the second last data reception.
	/// This bit should be kept cleared when the transfers are managed by DMA
	pub fn crc_transfer(&mut self) -> &mut Self {
		self.set(0, 12)
	}

	pub fn disable_crc_transfer(&mut self) -> &mut Self {
		self.clear(0, 12)
	}

	/// Set the Data Frame Format
	pub fn set_dff(&mut self, dff: DFFormat) -> Result<&mut Self, ()> {
		if self.is_set(0, 6) {
			Err(())
		} else {
			match dff {
				DFFormat::Bit8 => Ok( self.clear(0, 11) ),
				_ => Ok( self.set(0, 11) ),
			}
		}
	}

	/// Set the frame format
	pub fn set_ff(&mut self, ff: FrameFormat) -> Result<&mut Self, ()> {
		if self.is_set(2, 7) {
			Err(())
		} else {
			match ff {
				FrameFormat::MSB => Ok( self.clear(0, 7) ),
				_ => Ok( self.set(0, 7) ),
			}
		}
	}
}


/// Interupts and events methods
impl Spi {
	/// Enable/Disable interrupt
	pub fn int_state(&mut self, s: bool, int: SPIInterrupt) -> &mut Self {
		let offset = match int {
			SPIInterrupt::RXDMA => 0,
			SPIInterrupt::TXDMA => 1,
			SPIInterrupt::ERR => 5,
			SPIInterrupt::RXNE => 6,
			SPIInterrupt::TXE => 7,
		};

		match s {
			true => self.set(1, offset),
			_ => self.clear(1, offset),
		}
	}

	/// Returns tru eif the flag is raised
	pub fn is_flag_raised(&self, flag: SPIFlag) -> bool {
		self.is_set(2, flag as usize)
	}

	/// Reads the data in the RX buffer
	pub fn read_data(&self) -> u8 {
		self.block[3].read() as u8
	}

	/// Writes to the TX buffer
	pub fn write_data(&mut self, data: u8) -> &mut Self {
		self.write_bits(3, 0, data as u32, 8)
	}
}
