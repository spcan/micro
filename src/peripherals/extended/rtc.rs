//! Real Time Clock Peripheral

use crate::common::{ Timestamp, Weekday, Register, Clocks };

pub const ADDRESS: u32 = 0x4000_2800;
pub const SIZE: usize = 20;

#[repr(C)]
pub struct Rtc {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Rtc {}

impl_rwio!(Rtc);


impl Rtc {
	/// Unlock the RTC
	pub fn unlock(&mut self) -> &mut Self {
		self.write_bits(9, 0, 0xCA, 8)
			.write_bits(9, 0, 0x53, 8)
	}

	/// Initialize the RTC with the given Timestamp and Clocks
	pub fn init(&mut self, stamp: Timestamp, clocks: Clocks) -> &mut Self {
		// Set INIT in ISR
		self.set(3, 7);

		// Poll INITF until set
		while !self.is_set(3, 6) {}

		// ?? Write PPER sync and async (both )
		// Check which frequency we can give and try to get as close to 1 second as possible
		// Default for now to the LSE and get 1Hz clock
		const PRESCALERS: u32 = (mask!(7) << 16) | mask!(15);
		self.block[4].write(PRESCALERS);

		// Load the initial values to TR and DR, the shadow registers
		// Set 24 or 12 hour mode in CR, `time()` method applies the same
		// format as the given Timestamp
		self.block[0].write(u32::from( stamp.time() ));
		self.block[1].write(u32::from( stamp.date() ));
		// Clear the INIT

		self.clear(3, 7)
	}

	/// Gets the current time, date
	/// TODO : Check if it is necessary to do second reads
	pub fn timestamp(&self) -> Timestamp {
		let date = self.block[0].read();
		let time = self.block[1].read();

		let year: u16  = read_partial!(date, 4, 20, u16) * 10 + read_partial!(date, 4, 16, u16);
		let month: u16 = read_partial!(date, 1, 12, u16) * 10 + read_partial!(date, 4,  8, u16);
		let day: u16   = read_partial!(date, 2,  4, u16) * 10 + read_partial!(date, 4,  0, u16);

		let weekday = match read_partial!(date, 3, 13, u32) {
			1 => Weekday::Monday,
			2 => Weekday::Tuesday,
			3 => Weekday::Wednesday,
			4 => Weekday::Thursday,
			5 => Weekday::Friday,
			6 => Weekday::Saturday,
			7 => Weekday::Sunday,

			// Corrupted data, return empty Timestamp
			_ => return Timestamp::empty(),
		};

		let mut hour: u8  = read_partial!(time, 2, 20,  u8) * 10 + read_partial!(time, 4, 16,  u8);
		let minute: u8    = read_partial!(time, 3, 12,  u8) * 10 + read_partial!(time, 4,  8,  u8);
		let second: u16   = read_partial!(time, 3,  4, u16) * 10 + read_partial!(time, 4,  0, u16);

		if time & (1 << 22) != 0 {
			hour += 12;
		}

		Timestamp {
			year,
			month,
			day,
			weekday,
			hour,
			minute,
			second,
			subsecond: 0,
		}
	}

	/// Advances the RTC for the given amount of miliseconds
	/// Margin of error varies around 4-10 ms delays, though this is not garanteed
	pub fn advance(&mut self, m: u32) -> &mut Self {
		const DELAY1SEC: u32 = 1 << 31;
		if m > 1000 {
			for _ in 0..m/1000 {
				// Delay for 1 second
				self.block[11].write(DELAY1SEC);

				while self.is_set(3, 3) {}
			}
		}

		let prediv = read_partial!(self.block[4].read(), 15, 0, u32);

		let rest = m - ((m/1000)*1000);

		let subfs = (1-rest)*(prediv + 1);

		// Build the register
		let reg = (1 << 31) | (subfs & mask!(15));

		self.block[11].write(reg);

		self
	}

	/// Delays the RTC for the given amount of miliseconds
	/// TODO : Implementation
	pub fn delay(&mut self, m: u32) -> &mut Self {

		self
	}
}