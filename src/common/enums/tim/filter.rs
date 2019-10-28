//! TIM filter

/// TIM Input Mode filter
#[derive(Debug, Copy, Clone)]
pub enum ICFilter {
	NoFilter     = 0b0000,

	Div1Ignore2  = 0b0001,
	Div1Ignore4  = 0b0010,
	Div1Ignore8  = 0b0011,

	Div2Ignore6  = 0b0100,
	Div2Ignore8  = 0b0101,

	Div4Ignore6  = 0b0110,
	Div4Ignore8  = 0b0111,

	Div8Ignore6  = 0b1000,
	Div8Ignore8  = 0b1001,

	Div16Ignore5 = 0b1010,
	Div16Ignore6 = 0b1011,
	Div16Ignore8 = 0b1100,

	Div32Ignore5 = 0b1101,
	Div32Ignore6 = 0b1110,
	Div32Ignore8 = 0b1111,
}