//! Time stamp abstraction
//! Represents the time stamp provided by the MCU

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum Weekday {
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
	Sunday,
}

impl From<Weekday> for u32 {
	fn from(w: Weekday) -> u32 {
		match w {
			Weekday::Monday => 1,
			Weekday::Tuesday => 2,
			Weekday::Wednesday => 3,
			Weekday::Thursday => 4,
			Weekday::Friday => 5,
			Weekday::Saturday => 6,
			Weekday::Sunday => 7,
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Timestamp {
	pub year: u16,
	pub month: u16,
	pub day: u16,
	pub weekday: Weekday,

	pub hour: u8,
	pub minute: u8,
	pub second: u16,
	pub subsecond: u16,
}

impl Timestamp {
	pub fn date(&self) -> Date {
		Date {
			year: self.year,
			month: self.month,
			day: self.day,
			weekday: self.weekday,
		}
	}

	pub fn time(&self) -> Time {
		Time {
			hour: self.hour,
			minute: self.minute,
			second: self.second,
			subsecond: self.subsecond,
		}
	}

	pub fn empty() -> Timestamp {
		Timestamp {
			year: 0,
			month: 0,
			day: 0,
			weekday: Weekday::Monday,

			hour: 0,
			minute: 0,
			second: 0,
			subsecond: 0,
		}
	}
}

impl From<Timestamp> for (u32, u32) {
	fn from(t: Timestamp) -> (u32, u32) {
		(u32::from(t.date()), u32::from(t.time()))
	}
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Date {
	year: u16,
	month: u16,
	day: u16,
	weekday: Weekday,
}

impl From<Date> for u32 {
	fn from(d: Date) -> u32 {
		let mut result = 0;

		let yt = d.year / 10;

		result |= u32::from(yt & mask!(4)) << 20;
		result |= u32::from((d.year - (yt*10)) & mask!(4)) << 16;

		let mt = d.month / 10;

		result |= u32::from(mt & mask!(1)) << 12;
		result |= u32::from((d.month - (mt*10)) & mask!(4)) << 8;

		let dt = d.day / 10;

		result |= u32::from(dt & mask!(2)) << 4;
		result |= u32::from((d.day - (dt*10)) & mask!(4));

		result |= u32::from(d.weekday) << 13;

		result
	}
}



#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Time {
	hour: u8,
	minute: u8,
	second: u16,
	subsecond: u16,
}

impl From<Time> for u32 {
	fn from(t: Time) -> u32 {
		let mut result = 0;

		let ht = t.hour / 10;

		result |= u32::from(ht & mask!(2)) << 20;
		result |= u32::from((t.hour - (ht*10)) & mask!(4)) << 16;

		let mt = t.minute / 10;

		result |= u32::from(mt & mask!(3)) << 12;
		result |= u32::from((t.minute - (mt*10)) & mask!(4)) << 8;

		let st = t.second / 10;

		result |= u32::from(st & mask!(3)) << 4;
		result |= u32::from((t.second - (st*10)) & mask!(4));

		if t.hour > 24 {
			result |= 1 << 22;
		}

		result
	}
}
