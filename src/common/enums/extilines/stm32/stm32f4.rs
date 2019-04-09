//! STM32 Line F4

/// Full EXTI 
#[cfg(any(feature = "stm32f4x5", feature = "stm32f4x7", feature = "stm32f4x9", feature = "stm32f446"))]
#[derive(Debug, Copy, Clone)]
pub enum EXTILine {
	Line0  =  0,
	Line1  =  1,
	Line2  =  2,
	Line3  =  3,
	Line4  =  4,
	Line5  =  5,
	Line6  =  6,
	Line7  =  7,
	Line8  =  8,
	Line9  =  9,
	Line10 = 10,
	Line11 = 11,
	Line12 = 12,
	Line13 = 13,
	Line14 = 14,
	Line15 = 15,
	Line16 = 16,
	Line17 = 17,
	Line18 = 18,
	Line19 = 19,
	Line20 = 20,
	Line21 = 21,
	Line22 = 22,
}

/// Partial EXTI
#[cfg(any(feature = "stm32f410", feature = "stm32f411", feature = "stm32f401", feature = "stm32f412", feature = "stm32f4x3"))]
#[derive(Debug, Copy, Clone)]
pub enum EXTILine {
	Line0  =  0,
	Line1  =  1,
	Line2  =  2,
	Line3  =  3,
	Line4  =  4,
	Line5  =  5,
	Line6  =  6,
	Line7  =  7,
	Line8  =  8,
	Line9  =  9,
	Line10 = 10,
	Line11 = 11,
	Line12 = 12,
	Line13 = 13,
	Line14 = 14,
	Line15 = 15,
	Line16 = 16,
	Line17 = 17,
	Line18 = 18,
	Line21 = 21,
	Line22 = 22,
	#[cfg(feature = "stm32f4x3")]
	Line23 = 23,
}