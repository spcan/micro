//! Advanced timer 

#[cfg(feature = "std")]
use std::{ ptr };

#[cfg(not(feature = "std"))]
use core::{ ptr };

use crate::common::{ asm, Register, Frequency, TIMChannel, TIMPolarity, ICFilter, ICPrescaler, ICChConfig, SlaveMode, TIMTrigger, TIMInterrupt };

pub const ADDRESS_A: u32 = 0x4002_0000;
pub const ADDRESS_B: u32 = 0x4002_0400;
pub const ADDRESS_C: u32 = 0x4002_0800;
pub const ADDRESS_D: u32 = 0x4002_0C00;
pub const ADDRESS_E: u32 = 0x4002_1000;
pub const ADDRESS_F: u32 = 0x4002_1C00;

pub const SIZE: usize = 10;

#[repr(C)]
pub struct AdvTim {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for AdvTim {}

impl_rwio!(AdvTim);

// TODO : Documentation
// Select channel
// Polarity = Rising
// Select direction 
// Set prescaler (0)
// Set ICFilter = 0
// Set channel 1 with polarity and selction
// Set channel 2 with inversse polarity and selection
impl AdvTim {
	/// Captures the period and the duty cycle of a signal
	/// To capture the period of a PWM signal, the steps to do are:
	///   - Configure the channel :
	///     - Disable the desired channel
	///     - Set the filter for input mode. The filter simply regulates the speed at which the TIM
	///       samples the signal and the number of times the rise of the signal is detected to change
	///       the output. This means that if `N` (the number of ignored signals) = 2, this will measure
	///       2 periods, instead of 1. This is useful for very short period PWMs.
	/// The register CCx with x = `period`(`TIMChannel`) is the register in which the period will be stored
	/// The register CCx with x = `duty_cycle`(`TIMChannel`) is the register in which the duty cycle will be stored
	/// To gather the data, the user must set up the corresponding PWM interrupt. In the interrupt, read the values
	/// of the two registers and perform the next operation:
	pub fn pwm_capture(&mut self, period: TIMChannel, pin1: ICChConfig, duty_cycle: TIMChannel, pin2: ICChConfig) -> Result<&mut Self, ()> {
		match pin1 == pin2 || period == duty_cycle {
			true => return Err(()),
			_ => (),
		}
		// Configure pin 1
		// Polarity rising - Trigger when there's a rise
		// pinx - Map to whatever pin was given
		// No FIlter - Sample at TIM Counter speed (TIM Counter speed = Bus speed / (CNT_Prescaler + 1))
		self.ic_channel_config(period, TIMPolarity::Rising, pin1, ICFilter::NoFilter)?
		// Set prescaler 1
		// The prescaler indicates if some events are ignored or not
		// Can be set to trigger every event, every 2, every 4 or every 8
		// If it's a continuous square wave, limiting the number of changes to the values
		// can reduce blocking time due to interrupts
			.ic_prescaler(ICPrescaler::NoIgnore)
		// Configure pin 2 with inverse settings
			.ic_channel_config(duty_cycle, TIMPolarity::Falling, pin2, ICFilter::NoFilter)?
		// Set prescaler 2
			.ic_prescaler(ICPrescaler::NoIgnore)
		// Select type of trigger
			.trigger(TIMTrigger::TI2FP2)
		// Select Slave Mode
			.slave_mode(SlaveMode::Reset)
		// Enable Master/Slave mode
			.set(2, 7)
		// Enable Counter
			.set(0, 0)
		// Enable interrupts on the Period Channel
			.int_state(true, match period {
				TIMChannel::Ch1 => TIMInterrupt::CC1,
				TIMChannel::Ch2 => TIMInterrupt::CC2,
				TIMChannel::Ch3 => TIMInterrupt::CC3,
				TIMChannel::Ch4 => TIMInterrupt::CC4,
			});

		Ok( self )
	}

	/// Enable/Disable interrupt
	pub fn int_state(&mut self, s: bool, int: TIMInterrupt) -> &mut Self {
		match s {
			true => self.set(3, int as usize),
			_ => self.clear(3, int as usize),
		}
	}

	/// Returns `true` if the flag is raised
	pub fn is_raised(&mut self, int: TIMInterrupt) -> bool {
		self.is_set(3, int as usize)
	}

	/// Sets the slave mode
	pub fn slave_mode(&mut self, sm: SlaveMode) -> &mut Self {
		self.write_bits(2, 0, sm as u32, 3)
	}

	/// Sets the trigger of the TIM
	pub fn trigger(&mut self, trigger: TIMTrigger) -> &mut Self {
		self.write_bits(2, 4, trigger as u32, 3)
	}

	/// Configure the number of events ignored by TIM
	pub fn ic_prescaler(&mut self, psc: ICPrescaler) -> &mut Self {
		self.write_bits(6, 2, psc as u32, 2)
	}

	/// Configure one Input channel
	pub fn ic_channel_config(&mut self, ch: TIMChannel, polarity: TIMPolarity, cfg: ICChConfig, filter: ICFilter) -> Result<&mut Self, ()> {
		// TODO : Investigate option 0b11 IC1 is mapped to TRC

		// Disable Channel
		self.clear(CCER, 0)
		// Set input filter
			.icfilter(filter)
		// Set which PIN is it mapped to
			.ic_channel_map(cfg)?
		// Disable output if present
			.clear(CCER, 1)
		// Write the polarity
			.write_bits(CCER, 1, polarity as u32, 2)
		// Initiate the Pin
			.set(CCER, 0);

		Ok( self )
	}

	/// Configure Input Capture filter
	pub fn icfilter(&mut self, filter: ICFilter) -> &mut Self {
		self.write_bits(6, 4, filter as u32, 4)
	}

	/// Configure where the channel is mapped
	pub fn ic_channel_map(&mut self, cfg: ICChConfig) -> Result<&mut Self, ()> {
		match cfg {
			ICChConfig::TI1 => Ok( self.write_bits(7, 0, 0b01, 2) ),
			ICChConfig::TI2 => Ok( self.write_bits(7, 0, 0b10, 2) ),
			// TODO : Implement TRC mapping
			_ => Err(())
		}
	}
}