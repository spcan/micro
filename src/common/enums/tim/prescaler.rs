//! TIM prescalers

/// TIM Input Mode prescaler
#[derive(Debug, Copy, Clone)]
pub enum ICPrescaler {
	NoIgnore,
	Every2,
	Every4,
	Every8,
}