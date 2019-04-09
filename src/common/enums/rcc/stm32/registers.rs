//! RCC Registers for STM32


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RCCRegister {
	CR = 0,
	PLLCFGR = 1,
	CFGR = 2,
	CIR = 3,

	AHB1RST = 4,
	#[cfg(feature = "ahb2")]
	AHB2RST = 5,
	#[cfg(feature = "ahb3")]
	AHB3RST = 6,

	APB1RST = 8,
	APB2RST = 9,

	AHB1EN = 12,
	#[cfg(feature = "ahb2")]
	AHB2EN = 13,
	#[cfg(feature = "ahb3")]
	AHB3EN = 14,

	APB1EN = 16,
	APB2EN = 17,

	AHB1LPEN = 20,
	#[cfg(feature = "ahb2")]
	AHB2LPEN = 21,
	#[cfg(feature = "ahb3")]
	AHB3LPEN = 22,

	APB1LPEN = 24,
	APB2LPEN = 25,

	BDCR = 28,
	CSR = 29,
	SSCGR = 32,
	#[cfg(feature = "plli2s")]
	PLLI2SCFGR = 33,
	#[cfg(feature = "pllsai")]
	PLLSAICFGR = 34,
	DCKCFGR = 35,

	#[cfg(any(feature = "stm32f4x3", feature = "stm32f412"))]
	CKGATEN = 36,
	#[cfg(any(feature = "stm32f4x3", feature = "stm32f412", feature = "stm32f410"))]
	DCKCFGR2 = 37,
}

