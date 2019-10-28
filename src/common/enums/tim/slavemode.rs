//! TIM Slave mode

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SlaveMode {
	Disabled,
	Encoder1,
	Encoder2,
	Encoder3,
	Reset,
	Gated,
	Trigger,
	ExternalClockM1,
}
