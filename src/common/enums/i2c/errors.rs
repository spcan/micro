//! I2C Errors

#[derive(Copy, Clone)]
pub enum I2CError {
	WrongDataFormat,
	FrequencyNotAllowed,
	NotIn10BitMode,
	NACK,
	InvalidBusSpeed,
	Address2NotAllowed,
}