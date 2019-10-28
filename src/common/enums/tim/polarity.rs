//! TIM polarity

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TIMPolarity {
	Rising,
	Falling,
	Both,
}