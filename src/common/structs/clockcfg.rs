
use crate::common::enums::SrcClock;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ClockCfg {
	/// PLL Configuration
	/// ((plln, pllm, pllp, pllq), Source Clock)
	pub pllcfg: Option<((u32, u32, u32, u32), SrcClock)>,

	/// System Clock configuration
	/// (Source Clock, _)
	pub sysclk: (SrcClock, u32),

	/// AHB Prescalers
	pub hpre: (u32, u32, u32),

	/// APB Prescalers
	pub ppre: (u32, u32, u32),

	/// I2S Configuration
	/// (i2sn, i2sm, i2sr)
	pub i2scfg: (u32, u32, u32)
}