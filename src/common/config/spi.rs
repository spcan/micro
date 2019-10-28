//! SPI Configuration Struct

#[derive(Debug, Copy, Clone)]
pub struct SPIConfig {
	pub spibus: super::super::PeripheralBus,
	pub freq: super::super::Frequency,
	pub id: super::super::RCCPeripheral,
	pub dline: bool,
	pub crc: bool,
	pub bit8: bool,
	pub rxonly: bool,
	pub lsb: bool,
	pub master: bool,
	pub idle_high: bool,
	pub first_trans: bool,
}

impl SPIConfig {
	pub fn cr1(&self) -> u32 {
		0 |  if self.dline  { 1 << 15 } else { 0 }
		| if self.rxonly    { 0 }       else { 1 << 14}
		| if self.crc       { 1 << 13}  else { 0 }
		| if self.bit8      { 0 }       else { 1 << 11 }
		| if self.rxonly    { 1 << 10 } else { 0 }
		| if self.lsb       { 1 <<  7 } else { 0 }
		| if self.master    { 1 <<  2 } else { 0 }
		| if self.idle_high { 1 <<  1 } else { 0 }
		| if self.first_trans { 1 }     else { 0 }
		| if self.master    { 0b11 << 8 } else { 0 }
	}
}