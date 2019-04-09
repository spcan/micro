//! EXTI Trigger options

#[derive(Debug, Copy, Clone)]
pub enum Trigger {
	Rising,
	Falling,
	RiseFall,
}