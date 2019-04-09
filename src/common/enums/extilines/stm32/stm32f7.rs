//! STM32 Line F7

/// Full EXTI 
#[cfg(any(feature = "stm32f7"))]
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
	Line23 = 23,
	#[cfg(any(feature = "stm32f76", feature = "stm32f77"))]
	Line24 = 24,
}

