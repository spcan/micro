//! I2C Peripheral

use crate::common::{ Register, Frequency, I2CInterrupt, I2CFlags, I2CBitMode, MasterMode, DutyCycle, DualAddress };
use crate::common::enums::RCCPeripheral;
use crate::common::structs::pins::Pin;

use embedded_hal::blocking::i2c::{ Read, Write, WriteRead };

use crate::peripherals::extended::{ gpio::Gpio, rcc::Rcc };

pub const I2C1: u32 = 0x4000_5400;
pub const I2C2: u32 = 0x4000_5800;
pub const I2C3: u32 = 0x4000_5C00;

pub const SIZE: usize = 10;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum I2CError {
	/// NACK received
	NACK,
	/// Bus error
	Bus,
	/// Arbitration loss
	Arbitration,
	/// Overrun - Slave mode only
	Overrun,
	/// PEC - SMBUS mode only
	PEC,
	/// Timeout - SMBUS mode only
	Timeout,
	/// Alert - SMBUS mode only
	Alert,
	Other,
}

#[repr(C)]
pub struct I2c {
	#[repr(C)]
	block: &'static [Register<u32>; SIZE],
	pins: (Pin, Pin),
}

impl I2c {
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
}

impl I2c {
	/// Set up as master
	pub fn master<'a>(address: u32, pins: (Pin, Pin), rcc: &'a Rcc, clocks: Clocks, speed: Frequency) -> Result<Self, I2CError> {
		let i2cid = match address {
			I2C1 => RCCPeripheral::I2C1,
			I2C2 => RCCPeripheral::I2C2,
			I2C3 => RCCPeripheral::I2C3,
			_ => return Err(I2CError::Other),
		};

		let new = I2c {
			block: &mut *(address as *mut _),
			pins,
		};

		let (sda, scl) = pins;

		// TODO : Change to each board
		sda.altfn(4)
			.speed(HIGH);
		scl.altfn(4)
			.speed(HIGH);

		rcc.peripheral_state(true, i2cid);
		rcc.reset_peripheral(i2cid);

		// TODO set up RCC clocks

		// Disable he peripheral
		// by clearing PE bit in CR1
		new.clear(0, 0);
		// Calculate settings for I2C speed modes
		// If the user used the RCC given clocks, APB clock is legal

		// Configure bus frequency into I2C peripheral
		new.write_bits(1, 0, clocks.apb1.mhz(), 6);

		let trise = if speed <= Frequency::KHz(100) {
			clocks.apb1.mhz() + 1
		} else {
			((clocks.apb.mhz() * 300) / 1000) + 1
		};

		// Configure correct rise times
		new.write_bits(8, 0, trise, 6);

		// I2C clock control calculation
		// If in slow mode
		if speed <= Frequency::KHz(100) {
			let ccr = match clocks.apb.hz() / (speed.hz() * 2) {
				0...3 => 4,
				n => n,
			};

			// Set clock to standard mode with appropiate parameters for selected speed
			new.clear(7, 15)
				.clear(7, 14)
				.write_bits(7, 0, ccr, 12);
		} else {
			// Fast mode
			// Defaults for now to 2:1 duty cycle
			if true {
				let ccr = match clocks.apb1.hz() / (speed.hz() * 3) {
					0 => 1,
					n => n,
				};

				new.set(7, 15)
					.clear(7, 14)
					.write_bits(7, 0, ccr, 12);
			} else {
				// 16:9 duty cycle
				let ccr = match clocks.apb1.hz() / (speed.hz() * 25) {
					0 => 1,
					n => n,
				};

				new.set(7, 15)
					.set(7, 14)
					.write_bits(7, 0, ccr, 12);
			}
		}

		new.set(0, 0);

		Ok( new )
	}

	/// Stop the peripheral and release the pins
	pub fn free(&mut self) -> (Pin, Pin) {
		self.clear(0, 0);
		self.pins
	}
}


/*
impl I2c {
	/// Scans for devices and returns all the addresses it found connected
	pub fn scan(&mut self) -> Vec<u8> {
		let mut addresses = Vec::new();
		let mut void = &[0];

		for i in 0..128 {
			match self.read(i, void) {
				Ok(()) => addresses.push(i),
				_ => (),
			}
		}

		addresses
	}
}
*/
impl Read for I2c {
	type Error = I2CError;

	/// Read bytes into buffer
	/// This function is based on MASTER mode
	/// WARNING!
	/// `unsafe` function (but now marked as such). This function may leave the sender hanging 
	/// if the sender sends more bytes than what the buffer can hold.
	/// This is due to no STOP signal being sent back.
	fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), I2CError> {
		let last = buffer.len() - 1;

		// Send start condition and ACK bit
		self.start()
			.ack();
		// Wait until START condition is generated
		while !self.is_set(5, 0) {}
		// Wait until all devices are listening to us (bus is free)
		while !self.is_set(6, 0) && !self.is_set(6, 1) {}

		// Set up current address to talk to
		self.write_data(((addr as u32) << 1) + 1);
		// wait until address was sent
		while !self.is_set(5, 1) {}

		// Clear condition by reading SR2
		let _ = self.block[6].read();

		// Store bytes
		for i in 0..last {
			buffer[i] = self.recv_byte()?;
		}

		self.nack()
			.stop();

		// Read last byte
		buffer[last] = self.recv_byte()?;

		Ok(())
	}
}

impl Write for I2c {
	type Error = I2CError;

	/// Send a buffer of bytes
	fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), I2CError> {
		// Send START condition
		self.start();
		// Wait until START condition is generated
		while !self.is_set(5, 0) {}
		// Wait until all devices are listening to us (bus is free)
		while !self.is_set(6, 0) && !self.is_set(6, 1) {}

		// Set up current address to talk to
		self.write_data((addr as u32) << 1);
		// wait until address was sent
		while !self.is_set(5, 1) {}

		// Clear condition by reading SR2
		// let _ = ptr::read_volatile(self as u32 + 0x18);
		let _ = self.block[6].read();

		// Send the bytes
		for b in bytes {
			self.send_byte(*b)?;
		}

		Ok(())
	}
}

impl WriteRead for I2c {
	type Error = I2CError;

	/// Writes some bytes then reads some bytes
	fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), I2CError> {
		self.write(addr, bytes)?;
		self.read(addr, buffer)
	}
}

impl I2c {
	/// Sends a byte
	pub fn send_byte(&mut self, byte: u8) -> Result<&mut Self, I2CError> {
		// Wait until TX buffer is empty
		while !self.is_raised(I2CFlags::TxEmpty) {}

		self.write_data(byte as u32);

		while {
			if self.is_raised(I2CFlags::ACKFailure) {
				return Err(I2CError::NACK);
			}

			!self.is_raised(I2CFlags::TransferComplete)
		} {}

		Ok( self )
	}

	/// Receive a byte
	pub fn recv_byte(&self) -> Result<u8, I2CError> {
		while !self.is_raised(I2CFlags::RxNotEmpty) {}
		Ok( self.read_data() )
	}
}

impl I2c {
	/// Enable the sending of ACK signal after byte transfer
	pub fn ack(&mut self) -> &mut Self {
		self.set(0, 10)
	}

	/// Disable the sending of ACK signal (effectively sending a NACK) after byte transfer
	pub fn nack(&mut self) -> &mut Self {
		self.clear(0, 10)
	}

	/// Stop generation
	/// 0: No stop generation
	/// 1: Slave Mode - Release the SCL and SDA lines after current byte transfer
	///    Master Mode - Stop generation after the current byte transfer or current Start condition is sent
	pub fn stop(&mut self) -> &mut Self {
		self.set(0, 9)
	}

	/// Start generation
	/// 0: No start generation
	/// 1: Slave Mode - Start generation when bus id free
	///    Master Mode - Repeated start generation
	pub fn start(&mut self) -> &mut Self {
		self.set(0, 8)
	}

	/// Enable/Disable peripheral
	pub fn state(&mut self, s: bool) -> &mut Self {
		match s {
			true => self.set(0, 0),
			_ => self.clear(0, 0),
		}
	}

	/// If enabled the next byte will received in shift register
	pub fn receive_in_shift(&mut self) -> &mut Self {
		self.set(0, 11)
	}

	/// Starts Packet Error Checking (PEC) for the next transfer
	pub fn start_pec(&mut self) -> &mut Self {
		self.set(0, 12)
	}

	/// Resets the peripheral
	pub fn reset(&mut self) -> &mut Self {
		// TODO : check lines are free
		self.stop()
			.set(0, 15)
	}

	/// Sets the frequency of the transfer
	pub fn set_frequency(&mut self, f: Frequency) -> Result<&mut Self, I2CError> {
		match f.mhz() {
			2...50 => Ok( self.write_bits(1, 0, f.mhz() as u32, 6) ),
			_ => Err(I2CError::InvalidBusSpeed),
		}
	}

	/// Indicate this is the last trasnfer
	pub fn last_transfer(&mut self) -> &mut Self {
		self.set(1, 12)
	}

	/// Enable/Disable interrupt
	pub fn int_state(&mut self, s: bool, int: I2CInterrupt) -> &mut Self {
		let offsets = int.offsets();

		match s {
			true => self.set(offsets.0, offsets.1),
			_ => self.clear(offsets.0, offsets.1),
		}
	}

	/// Sets the addressing mode between 7-bit and 10-bit
	pub fn address_mode(&mut self, a: I2CBitMode) -> &mut Self {
		match a {
			I2CBitMode::Bit7 => self.clear(2, 15),
			_ => self.set(2, 15),
		}
	}

	/// Writes the interface address 1
	/// To be set **after** the interface bit size is set (7-bit or 10-bit)
	pub fn set_address_1(&mut self, addr: u32) -> &mut Self {
		match self.is_set(2, 15) {
			true => self.write_bits(2, 0, addr, 10),
			_ => self.write_bits(2, 1, addr, 7),
		}
	}

	/// Writes the interface address 2
	/// Returns an error if not in 7 bit mode
	pub fn set_address_2(&mut self, addr: u32) -> Result<&mut Self, I2CError> {
		match self.is_set(2, 15) {
			true => Err( I2CError::Address2NotAllowed),
			_ => Ok( self.write_bits(3, 1, addr, 7) ),
		}
	}

	/// Enable/Disable dual addressing mode
	pub fn dual_address_state(&mut self, s: bool) -> &mut Self {
		match s {
			true => self.set(3, 0),
			_ => self.clear(3, 0),
		}
	}

	/// Read received byte
	pub fn read_data(&self) -> u8 {
		self.block[4].read() as u8
	}

	/// Write data to be transmitted
	pub fn write_data(&mut self, data: u32) -> &mut Self {
		self.write_bits(4, 0, data, 8)
	}

	/// Returns true if the flag is raised
	pub fn is_raised(&self, f: I2CFlags) -> bool {
		let offsets = f.offsets();

		self.is_set( offsets.0, offsets.1 )
	}

	/// Returns true if the device is master
	pub fn is_master(&self) -> bool {
		self.is_set(6, 0)
	}

	/// Returns true if the bus is busy
	pub fn is_bus_busy(&self) -> bool {
		self.is_set(6, 1)
	}

	/// Returns true if the TRA bit is set
	pub fn is_tra_set(&self) -> bool {
		self.is_set(6, 2)
	}

	/// Returns which Dual Address has matched
	pub fn which_addr(&self) -> DualAddress {
		match self.is_set(6, 7) {
			true => DualAddress::Addr2,
			_ => DualAddress::Addr1,
		}
	}

	/// Returns the PEC register
	pub fn pec(&self) -> u32 {
		(self.block[6].read() >> 8) & 0b1111_1111
	}

	/// Clear the given flag
	/// If the flag is cleared by hardware, it does nothing
	pub fn clear_flag(&mut self, f: I2CFlags) -> &mut Self {
		match f.offsets() {
			(5, o) => match o {
				8...15 => self.clear(5, o),
				_ => self
			},
			_ => self
		}
	}

	/// Set CCR 
	/// Refer to the STM32F4 user manual
	pub fn set_ccr(&mut self, data: u32) -> &mut Self {
		self.write_bits(7, 0, data, 12)
	}

	/// Set Master Mode
	/// Refer to the STM32F4 user manual
	pub fn set_master_mode(&mut self, mode: MasterMode) -> &mut Self {
		match mode {
			MasterMode::SM => self.clear(7, 15),
			MasterMode::FM => self.set(7, 15),
		}
	}

	/// Set duty cycle
	/// Refer to the STM32F4 user manual
	pub fn set_duty_cycle(&mut self, d: DutyCycle) -> &mut Self {
		match d {
			DutyCycle::D2 => self.clear(7, 14),
			DutyCycle::D169 => self.set(7, 14),
		}
	}

	/// Set maximum rise time
	pub fn max_rise_time(&mut self, data: u32) -> &mut Self {
		self.write_bits(8, 0, data, 6)
	}

	/// Enable/Disable Analog Filter
	pub fn analog_filter_state(&mut self, s: bool) -> &mut Self {
		match s {
			true => self.clear(9, 4),
			_ => self.set(9, 4),
		}
	}

	/// Sets the Digital Noise Filter
	pub fn digital_noise_filter(&mut self, d: Option<u32>) -> &mut Self {
		match d {
			None => self.write_bits(9, 0, 0, 4),
			Some(a) => self.write_bits(9, 0, a, 4),
		}
	}
}

impl I2c {

	/// Set master mode (Standard or Fast)
	pub fn master_mode(&mut self, mode: MasterMode) -> &mut Self {
		match mode {
			MasterMode::SM => self.clear(7, 15),
			_ => self.set(7, 15),
		}
	}

	/// Write Interface Address
	pub fn set_address(&mut self, address: u32) -> &mut Self {
		let cons = if self.is_set(2, 15) { (0, 10) } else { (1, 7) };
		self.write_bits(2, cons.0, address, cons.1)
	}

	/// Set secondary address
	pub fn set_secondary_address(&mut self, address: u32) -> &mut Self {
		self.write_bits(3, 1, address, 7)
	}
}