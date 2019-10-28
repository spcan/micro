
use super::Frequency;

#[cfg(any(
	feature = "stm32f4disco", feature = "stm32f410", feature = "stm32f411",
	feature = "stm32f412",    feature = "stm32f4x3"
))]
pub mod clockspeed {
	use super::Frequency;

	pub const HSI: Frequency = Frequency::MHz(16);
	pub const LSI: Frequency = Frequency::KHz(32);
	pub const LSE: Frequency = Frequency::Hz(32768);

	#[cfg(feature = "stm32f4disco")]
	pub const HSE: Frequency = Frequency::MHz(8);

	pub const SYSCLKMAX: Frequency = Frequency::MHz(100);
	pub const APB1MAX: Frequency = Frequency::MHz(50);
	pub const APB2MAX: Frequency = Frequency::MHz(100);
	pub const AHBMAX: Frequency = Frequency::MHz(100);
}

#[cfg(feature = "stm32f401")]
pub mod clockspeed {
	use super::Frequency;

	pub const HSI: Frequency = Frequency::MHz(16);
	pub const LSI: Frequency = Frequency::KHz(32);
	pub const LSE: Frequency = Frequency::Hz(32768);

	pub const SYSCLKMAX: Frequency = Frequency::MHz(84);
	pub const APB1MAX: Frequency = Frequency::MHz(42);
	pub const APB2MAX: Frequency = Frequency::MHz(84);
	pub const AHBMAX: Frequency = Frequency::MHz(84);
}

#[cfg(any(
	feature = "stm32f446", feature = "stm32f4x9",
	feature = "stm32f4x7", feature = "stm32f4x5"
))]
pub mod clockspeed {
	use super::Frequency;

	pub const HSI: Frequency = Frequency::MHz(16);
	pub const LSI: Frequency = Frequency::KHz(32);
	pub const LSE: Frequency = Frequency::Hz(32768);

	pub const SYSCLKMAX: Frequency = Frequency::MHz(180);
	pub const APB1MAX: Frequency = Frequency::MHz(45);
	pub const APB2MAX: Frequency = Frequency::MHz(90);
	pub const AHBMAX: Frequency = Frequency::MHz(180);
}

#[cfg(feature = "stm32f2")]
pub mod clockspeed {
	use super::Frequency;

	pub const HSI: Frequency = Frequency::MHz(16);
	pub const LSI: Frequency = Frequency::KHz(32);
	pub const LSE: Frequency = Frequency::Hz(32768);

	pub const SYSCLKMAX: Frequency = Frequency::MHz(168);
	pub const APB1MAX: Frequency = Frequency::MHz(30);
	pub const APB2MAX: Frequency = Frequency::MHz(60);
	pub const AHBMAX: Frequency = Frequency::MHz(120);
}

#[cfg(any(feature = "stm32f72x", feature = "stm32f73x", feature = "stm32f75x", feature = "stm32f74x"))]
pub mod clockspeed {
	use super::Frequency;

	pub const HSI: Frequency = Frequency::MHz(16);
	pub const LSI: Frequency = Frequency::KHz(32);
	pub const LSE: Frequency = Frequency::Hz(32768);

	pub const SYSCLKMAX: Frequency = Frequency::MHz(216);
	pub const APB1MAX: Frequency = Frequency::MHz(54);
	pub const APB2MAX: Frequency = Frequency::MHz(108);
	pub const AHBMAX: Frequency = Frequency::MHz(216);
}

#[cfg(any(feature = "stm32f76x", feature = "stm32f77x"))]
pub mod clockspeed {
	use super::Frequency;

	pub const HSI: Frequency = Frequency::MHz(16);
	pub const LSI: Frequency = Frequency::KHz(32);
	pub const LSE: Frequency = Frequency::Hz(32768);

	pub const SYSCLKMAX: Frequency = Frequency::MHz(216);
	pub const APB1MAX: Frequency = Frequency::MHz(45);
	pub const APB2MAX: Frequency = Frequency::MHz(90);
	pub const AHBMAX: Frequency = Frequency::MHz(216);
}

#[cfg(any(feature = "stm32h7"))]
pub mod clockspeed {
	use super::Frequency;

	pub const HSI: Frequency = Frequency::MHz(64);
	pub const HSI48: Frequency = Frequency::MHz(48);
	pub const CSI: Frequency = Frequency::MHz(4);
	pub const LSI: Frequency = Frequency::KHz(32);
	pub const LSE: Frequency = Frequency::Hz(32768);

	pub const CPU1MAX: Frequency = Frequency::MHz(480);
	pub const CPU2MAX: Frequency = Frequency::MHz(240);

	pub const AXIMAX: Frequency = Frequency::MHz(240);

	pub const AHB1MAX: Frequency = Frequency::MHz(240);
	pub const AHB2MAX: Frequency = Frequency::MHz(240);
	pub const AHB3MAX: Frequency = Frequency::MHz(240);
	pub const AHB4MAX: Frequency = Frequency::MHz(240);

	pub const APB1MAX: Frequency = Frequency::MHz(120);
	pub const APB2MAX: Frequency = Frequency::MHz(120);
	pub const APB3MAX: Frequency = Frequency::MHz(120);
	pub const APB4MAX: Frequency = Frequency::MHz(120);
}
