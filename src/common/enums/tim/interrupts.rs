//! TIM interrupts

#[derive(Debug, Copy, Clone)]
pub enum TIMInterrupt {
	Update     =  0,
	CC1        =  1,
	CC2        =  2,
	CC3        =  3,
	CC4        =  4,
	COM        =  5,
	Trigger    =  6,
	Break      =  7,
	UpdateDMA  =  8,
	CC1DMA     =  9,
	CC2DMA     = 10,
	CC3DMA     = 11,
	CC4DMA     = 12,
	COMDMA     = 13,
	TriggerDMA = 14,

}