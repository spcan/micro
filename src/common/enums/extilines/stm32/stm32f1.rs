//! STM32 Line F1
#[cfg(feature = "stm32f1")]
#[derive(Debug, Copy, Clone)]
pub enum EXTILine {
	Line0 = 0,
	Line1 = 1,
	Line2 = 2,
	Line3 = 3,
	Line4 = 4,
	Line5 = 5,
	Line6 = 6,
	Line7 = 7,
	Line8 = 8,
	Line9 = 9,
	Line10 = 10,
	Line11 = 11,
	Line12 = 12,
	Line13 = 13,
	Line14 = 14,
	Line15 = 15,
	Line16 = 16,
	Line17 = 17,
	#[cfg(not(feature = "stm32f100"))]
	Line18 = 18,
	#[cfg(not(feature = "stm32f100"))]
	Line19 = 19,
}
