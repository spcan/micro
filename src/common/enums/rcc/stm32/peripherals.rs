//! RCC controlled peripherals for the STM32 devices

/// Peripherals

#[cfg(feature = "stm32f410")]
#[derive(Debug, Copy, Clone)]
pub enum RCCPeripheral {
	RNG   = 1055,	// RNG   @ AHB1 31
	DMA2  = 1046,	// DMA2  @ AHB1 22
	DMA1  = 1045,	// DMA1  @ AHB1 21
	CRC   = 1036,	// CRC   @ AHB1 12
	GPIOH = 1031,	// GPIOH @ AHB1  7
	GPIOC = 1026,	// GPIOH @ AHB1  2
	GPIOB = 1025,	// GPIOH @ AHB1  1
	GPIOA = 1024,	// GPIOH @ AHB1  0

	DAC    = 2077,	// DAC    @ APB1 29
	PWR    = 2076,	// PWR    @ APB1 28
	I2C4   = 2072,	// I2C4   @ APB1 24
	I2C2   = 2070,	// I2C4   @ APB1 22
	I2C1   = 2069,	// I2C4   @ APB1 21
	USART2 = 2065,  // USART2 @ APB1 17
	SPI2   = 2062,	// SPI2   @ APB1 14
	WWDG   = 2059,	// WWDG   @ APB1 11
	LPTIM1 = 2057,	// LPTIM1 @ APB1  9
	TIM6   = 2052,	// TIM6   @ APB1  4
	TIM5   = 2051,	// TIM5   @ APB1  3

	SPI5    = 2324,	// SPI5    @ APB2 20
	TIM11   = 2322,	// TIM11   @ APB2 18
	TIM9    = 2320,	// TIM9    @ APB2 16
	SYSCFG  = 2318,	// SYSCFG  @ APB2 14
	SPI1    = 2316,	// SPI1    @ APB2 12
	ADC1    = 2312,	// ADC1    @ APB2  8
	USART6  = 2309,	// USART6  @ APB2  5
	USART1  = 2308,	// USART1  @ APB2  4
	TIM1    = 2304,	// TIM1    @ APB2  0
}

#[cfg( any( feature = "stm324x5", feature = "stm32f4x7", feature = "stm32f4x9" ) )]
#[derive(Debug, Copy, Clone)]
pub enum RCCPeripheral {
	OTGHS  = 1053,	// OTG HS  @ AHB1 29
	ETHMAC = 1049,	// ETH MAC @ AHB1 25
	DMA2D  = 1047,	// DMA2D   @ AHB1 23
	DMA2   = 1046,	// DMA 2   @ AHB1 22
	DMA1   = 1045,	// DMA 1   @ AHB1 21
	CRC    = 1036,	// CRC     @ AHB1 12
	GPIOK  = 1034,	// GPIOK   @ AHB1 10
	GPIOJ  = 1033,	// GPIOJ   @ AHB1  9
	GPIOI  = 1032,	// GPIOI   @ AHB1  8
	GPIOH  = 1031,	// GPIOH   @ AHB1  7
	GPIOG  = 1030,	// GPIOH   @ AHB1  6
	GPIOF  = 1029,	// GPIOH   @ AHB1  5
	GPIOE  = 1028,	// GPIOH   @ AHB1  4
	GPIOD  = 1027,	// GPIOH   @ AHB1  3
	GPIOC  = 1026,	// GPIOH   @ AHB1  2
	GPIOB  = 1025,	// GPIOH   @ AHB1  1
	GPIOA  = 1024,	// GPIOH   @ AHB1  0

	OTGFS = 1287, // OTG FS @ AHB2 7
	RNG   = 1286, // RNG    @ AHB2 6
	HASH  = 1285, // HASH   @ AHB2 5
	CRYP  = 1284, // CRYP   @ AHB2 4
	DCMI  = 1280, // DCMI   @ AHB2 0

	#[cfg( any(feature = "stm32f469", feature = "stm32f479") )]
	QSPI = 1537, // Quad SPI @ AHB3 1
	FMC  = 1536, // FMC      @ AHB3 0

	USART8 = 2079,	// USART8 @ APB1 31
	USART7 = 2078,	// USART7 @ APB1 30
	DAC    = 2077,	// DAC    @ APB1 29
	PWR    = 2076,	// PWR    @ APB1 28
	CAN2   = 2074,	// CAN2   @ APB1 26
	CAN1   = 2073,	// CAN1   @ APB1 25
	I2C3   = 2071,	// I2C3   @ APB1 23
	I2C2   = 2070,	// I2C4   @ APB1 22
	I2C1   = 2069,	// I2C4   @ APB1 21
	USART5 = 2068,	// USART5 @ APB1 20
	USART4 = 2067,	// USART4 @ APB1 19
	USART3 = 2066,	// USART3 @ APB1 18
	USART2 = 2065,  // USART2 @ APB1 17
	SPI3   = 2063,	// SPI3   @ APB1 15
	SPI2   = 2062,	// SPI2   @ APB1 14
	WWDG   = 2059,	// WWDG   @ APB1 11
	TIM14  = 2056,	// TIM14  @ APB1  8
	TIM13  = 2055,	// TIM13  @ APB1  7
	TIM12  = 2054,	// TIM12  @ APB1  6
	TIM7   = 2053,	// TIM7   @ APB1  5
	TIM6   = 2052,	// TIM6   @ APB1  4
	TIM5   = 2051,	// TIM5   @ APB1  3
	TIM4   = 2050,	// TIM4   @ APB1  2
	TIM3   = 2049,	// TIM3   @ APB1  1
	TIM2   = 2048,	// TIM2   @ APB1  0

	#[cfg( any(feature = "stm32f469", feature = "stm32f479") )]
	DSI     = 2331,	// DSI     @ APB2 27
	LTDC    = 2330,	// LTDC    @ APB2 26
	SAI1    = 2326,	// SAI1    @ APB2 22
	SPI6    = 2325,	// SPI6    @ APB2 21
	SPI5    = 2324,	// SPI5    @ APB2 20
	TIM11   = 2322,	// TIM11   @ APB2 18
	TIM10   = 2321,	// TIM10   @ APB2 17
	TIM9    = 2320,	// TIM9    @ APB2 16
	SYSCFG  = 2318,	// SYSCFG  @ APB2 14
	SPI4    = 2317,	// SPI4    @ APB2 13
	SPI1    = 2316,	// SPI1    @ APB2 12
	SDIO    = 2315,	// SDIO    @ APB2 11
	ADC1    = 2312,	// ADC1    @ APB2  8
	USART6  = 2309,	// USART6  @ APB2  5
	USART1  = 2308,	// USART1  @ APB2  4
	TIM8    = 2305,	// TIM8    @ APB2  1
	TIM1    = 2304,	// TIM1    @ APB2  0
}

#[cfg(feature = "stm32f446")]
#[derive(Debug, Copy, Clone)]
pub enum RCCPeripheral {
	OTGHS = 1053,	// OTG HS  @ AHB1 29
	DMA2  = 1046,	// DMA2  @ AHB1 22
	DMA1  = 1045,	// DMA1  @ AHB1 21
	CRC   = 1036,	// CRC   @ AHB1 12
	GPIOH = 1031,	// GPIOH @ AHB1  7
	GPIOG = 1030,	// GPIOH @ AHB1  6
	GPIOF = 1029,	// GPIOH @ AHB1  5
	GPIOE = 1028,	// GPIOH @ AHB1  4
	GPIOD = 1027,	// GPIOH @ AHB1  3
	GPIOC = 1026,	// GPIOH @ AHB1  2
	GPIOB = 1025,	// GPIOH @ AHB1  1
	GPIOA = 1024,	// GPIOH @ AHB1  0

	OTGFS = 1287, // OTG FS @ AHB2 7
	DCMI  = 1280, // DCMI   @ AHB2 0

	QSPI = 1537, // Quad SPI @ AHB3 1
	FMC  = 1536, // FMC      @ AHB3 0

	DAC     = 2077,	// DAC     @ APB1 29
	PWR     = 2076,	// PWR     @ APB1 28
	CEC     = 2075,	// CEC     @ APB1 27
	CAN2    = 2074,	// CAN2    @ APB1 26
	CAN1    = 2073,	// CAN1    @ APB1 25
	FMPI2C1 = 2072,	// FMPI2C1 @ APB1 24
	I2C3    = 2071,	// I2C3    @ APB1 23
	I2C2    = 2070,	// I2C4    @ APB1 22
	I2C1    = 2069,	// I2C4    @ APB1 21
	USART5  = 2068,	// USART5  @ APB1 20
	USART4  = 2067,	// USART4  @ APB1 19
	USART3  = 2066,	// USART3  @ APB1 18
	USART2  = 2065,	// USART2  @ APB1 17
	SPDIFRX = 2064,	// SPDIFRX @ APB1 16
	SPI3    = 2063,	// SPI3    @ APB1 15
	SPI2    = 2062,	// SPI2    @ APB1 14
	WWDG    = 2059,	// WWDG    @ APB1 11
	TIM14   = 2056,	// TIM14   @ APB1  8
	TIM13   = 2055,	// TIM13   @ APB1  7
	TIM12   = 2054,	// TIM12   @ APB1  6
	TIM7    = 2053,	// TIM7    @ APB1  5
	TIM6    = 2052,	// TIM6    @ APB1  4
	TIM5    = 2051,	// TIM5    @ APB1  3
	TIM4    = 2050,	// TIM4    @ APB1  2
	TIM3    = 2049,	// TIM3    @ APB1  1
	TIM2    = 2048,	// TIM2    @ APB1  0

	SAI2    = 2327,	// SAI2    @ APB2 23
	SAI1    = 2326,	// SAI1    @ APB2 22
	TIM11   = 2322,	// TIM11   @ APB2 18
	TIM10   = 2321,	// TIM10   @ APB2 17
	TIM9    = 2320,	// TIM9    @ APB2 16
	SYSCFG  = 2318,	// SYSCFG  @ APB2 14
	SPI4    = 2317,	// SPI4    @ APB2 13
	SPI1    = 2316,	// SPI1    @ APB2 12
	SDIO    = 2315,	// SDIO    @ APB2 11
	ADC1    = 2312,	// ADC1    @ APB2  8
	USART6  = 2309,	// USART6  @ APB2  5
	USART1  = 2308,	// USART1  @ APB2  4
	TIM8    = 2305,	// TIM8    @ APB2  1
	TIM1    = 2304,	// TIM1    @ APB2  0
}

#[cfg( feature = "stm32f412" )]
#[derive(Debug, Copy, Clone)]
pub enum RCCPeripheral {
	DMA2  = 1046, // DMA2  @ AHB1 22
	DMA1  = 1045, // DMA1  @ AHB1 21
	CRC   = 1036, // CRC   @ AHB1 12
	GPIOH = 1031, // GPIOH @ AHB1  7
	GPIOG = 1030, // GPIOH @ AHB1  6
	GPIOF = 1029, // GPIOH @ AHB1  5
	GPIOE = 1028, // GPIOH @ AHB1  4
	GPIOD = 1027, // GPIOH @ AHB1  3
	GPIOC = 1026, // GPIOH @ AHB1  2
	GPIOB = 1025, // GPIOH @ AHB1  1
	GPIOA = 1024, // GPIOH @ AHB1  0

	OTGFS = 1287, // OTG FS @ AHB2 7
	RNG   = 1286, // RNG    @ AHB2 6

	QSPI = 1537, // Quad SPI @ AHB3 1
	FMC  = 1536, // FMC      @ AHB3 0

	PWR     = 2076,	// PWR     @ APB1 28
	CAN2    = 2074,	// CAN2    @ APB1 26
	CAN1    = 2073,	// CAN1    @ APB1 25
	FMPI2C1 = 2072,	// FMPI2C1 @ APB1 24
	I2C3    = 2071,	// I2C3    @ APB1 23
	I2C2    = 2070,	// I2C4    @ APB1 22
	I2C1    = 2069,	// I2C4    @ APB1 21
	USART3  = 2066,	// USART3  @ APB1 18
	USART2  = 2065,	// USART2  @ APB1 17
	SPI3    = 2063,	// SPI3    @ APB1 15
	SPI2    = 2062,	// SPI2    @ APB1 14
	WWDG    = 2059,	// WWDG    @ APB1 11
	TIM14   = 2056,	// TIM14   @ APB1  8
	TIM13   = 2055,	// TIM13   @ APB1  7
	TIM12   = 2054,	// TIM12   @ APB1  6
	TIM7    = 2053,	// TIM7    @ APB1  5
	TIM6    = 2052,	// TIM6    @ APB1  4
	TIM5    = 2051,	// TIM5    @ APB1  3
	TIM4    = 2050,	// TIM4    @ APB1  2
	TIM3    = 2049,	// TIM3    @ APB1  1
	TIM2    = 2048,	// TIM2    @ APB1  0

	DFSDM1  = 2328, // DFSDM1  @ APB2 24
	SPI5    = 2324,	// SPI5    @ APB2 20
	TIM11   = 2322,	// TIM11   @ APB2 18
	TIM10   = 2321,	// TIM10   @ APB2 17
	TIM9    = 2320,	// TIM9    @ APB2 16
	SYSCFG  = 2318,	// SYSCFG  @ APB2 14
	SPI4    = 2317,	// SPI4    @ APB2 13
	SPI1    = 2316,	// SPI1    @ APB2 12
	SDIO    = 2315,	// SDIO    @ APB2 11
	ADC1    = 2312,	// ADC1    @ APB2  8
	USART6  = 2309,	// USART6  @ APB2  5
	USART1  = 2308,	// USART1  @ APB2  4
	TIM8    = 2305,	// TIM8 @ APB2 1
	TIM1    = 2304,	// TIM1   @ APB2  0
}

#[cfg( any( feature = "stm32401", feature = "stm32f411") )]
#[derive(Debug, Copy, Clone)]
pub enum RCCPeripheral {
	DMA2  = 1046, // DMA2  @ AHB1 22
	DMA1  = 1045, // DMA1  @ AHB1 21
	CRC   = 1036, // CRC   @ AHB1 12
	GPIOH = 1031, // GPIOH @ AHB1  7
	GPIOE = 1028, // GPIOH @ AHB1  4
	GPIOD = 1027, // GPIOH @ AHB1  3
	GPIOC = 1026, // GPIOH @ AHB1  2
	GPIOB = 1025, // GPIOH @ AHB1  1
	GPIOA = 1024, // GPIOH @ AHB1  0

	OTGFS = 1287, //OTG FS @ AHB2 7

	PWR    = 2076,	// PWR    @ APB1 28
	I2C3   = 2071,	// I2C3   @ APB1 23
	I2C2   = 2070,	// I2C4   @ APB1 22
	I2C1   = 2069,	// I2C4   @ APB1 21
	USART2 = 2065,  // USART2 @ APB1 17
	SPI3   = 2063,	// SPI3   @ APB1 15
	SPI2   = 2062,	// SPI2   @ APB1 14
	WWDG   = 2059,	// WWDG   @ APB1 11
	TIM5   = 2051,	// TIM5   @ APB1  3
	TIM4   = 2050,	// TIM4   @ APB1  2
	TIM3   = 2049,	// TIM3   @ APB1  1
	TIM2   = 2048,	// TIM2   @ APB1  0

	#[cfg( feature = "stm32f411" )]
	SPI5   = 2324,	// SPI5   @ APB2 20
	TIM11  = 2322,	// TIM11  @ APB2 18
	TIM10  = 2321,	// TIM10  @ APB2 17
	TIM9   = 2320,	// TIM9   @ APB2 16
	SYSCFG = 2318,	// SYSCFG @ APB2 14
	SPI4   = 2317,	// SPI4   @ APB2 13
	SPI1   = 2316,	// SPI1   @ APB2 12
	SDIO   = 2315,	// SDIO   @ APB2 11
	ADC1   = 2312,	// ADC1   @ APB2  8
	USART6 = 2309,	// USART6 @ APB2  5
	USART1 = 2308,	// USART1 @ APB2  4
	TIM1   = 2304,	// TIM1   @ APB2  0
}


#[cfg( feature = "stm32f4x3" )]
#[derive(Debug, Copy, Clone)]
pub enum RCCPeripheral {
	DMA2  = 1046, // DMA2  @ AHB1 22
	DMA1  = 1045, // DMA1  @ AHB1 21
	CRC   = 1036, // CRC   @ AHB1 12
	GPIOH = 1031, // GPIOH @ AHB1  7
	GPIOG = 1030, // GPIOH @ AHB1  6
	GPIOF = 1029, // GPIOH @ AHB1  5
	GPIOE = 1028, // GPIOH @ AHB1  4
	GPIOD = 1027, // GPIOH @ AHB1  3
	GPIOC = 1026, // GPIOH @ AHB1  2
	GPIOB = 1025, // GPIOH @ AHB1  1
	GPIOA = 1024, // GPIOH @ AHB1  0

	OTGFS = 1287, // OTG FS @ AHB2 7
	RNG   = 1286, // RNG    @ AHB2 6
	HASH  = 1285, // HASH   @ AHB2 5
	CRYP  = 1284, // CRYP   @ AHB2 4
	DCMI  = 1280, // DCMI   @ AHB2 0

	QSPI = 1537, // Quad SPI @ AHB3 1
	FMC  = 1536, // FMC      @ AHB3 0

	USART8  = 2079, // USART8  @ APB1 31
	USART7  = 2078, // USART7  @ APB1 30
	DAC     = 2077,	// DAC     @ APB1 29
	PWR     = 2076,	// PWR     @ APB1 28
	CEC     = 2075,	// CEC     @ APB1 27
	CAN2    = 2074,	// CAN2    @ APB1 26
	CAN1    = 2073,	// CAN1    @ APB1 25
	I2C4    = 2072,	// I2C4    @ APB1 24
	I2C3    = 2071,	// I2C3    @ APB1 23
	I2C2    = 2070,	// I2C4    @ APB1 22
	I2C1    = 2069,	// I2C4    @ APB1 21
	USART5  = 2068,	// USART5  @ APB1 20
	USART4  = 2067,	// USART4  @ APB1 19
	USART3  = 2066,	// USART3  @ APB1 18
	USART2  = 2065, // USART2  @ APB1 17
	SPI3    = 2063,	// SPI3    @ APB1 15
	SPI2    = 2062,	// SPI2    @ APB1 14
	WWDG    = 2059,	// WWDG    @ APB1 11
	TIM14   = 2056,	// TIM14   @ APB1  8
	TIM13   = 2055,	// TIM13   @ APB1  7
	TIM12   = 2054,	// TIM12   @ APB1  6
	TIM7    = 2053,	// TIM7    @ APB1  5
	TIM6    = 2052,	// TIM6    @ APB1  4
	TIM5    = 2051,	// TIM5    @ APB1  3
	TIM4    = 2050,	// TIM4    @ APB1  2
	TIM3    = 2049,	// TIM3    @ APB1  1
	TIM2    = 2048,	// TIM2    @ APB1  0

	DFSDM2  = 2329, // DFSDM2  @ APB2 25
	DFSDM1  = 2328, // DFSDM1  @ APB2 24
	SAI1    = 2326,	// SAI1    @ APB2 22
	SPI5    = 2324,	// SPI5    @ APB2 20
	TIM11   = 2322,	// TIM11   @ APB2 18
	TIM10   = 2321,	// TIM10   @ APB2 17
	TIM9    = 2320,	// TIM9    @ APB2 16
	SYSCFG  = 2318,	// SYSCFG  @ APB2 14
	SPI4    = 2317,	// SPI4    @ APB2 13
	SPI1    = 2316,	// SPI1    @ APB2 12
	SDIO    = 2315,	// SDIO    @ APB2 11
	ADC1    = 2312,	// ADC1    @ APB2  8
	USART10 = 2311,	// USART10 @ APB2  7
	USART9  = 2310,	// USART9  @ APB2  6
	USART6  = 2309,	// USART6  @ APB2  5
	USART1  = 2308,	// USART1  @ APB2  4
	TIM8    = 2305,	// TIM8    @ APB2  1
	TIM1    = 2304,	// TIM1    @ APB2  0
}

impl RCCPeripheral {
	/// Extract offsets
	pub fn offsets(self) -> (usize, usize) {
		let data = self as usize;
		((data as usize >> 8) & 0xFF, data as usize & 0xFF)
	}
}