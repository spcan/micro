//! SPI Errors

#[derive(Debug, Copy, Clone)]
pub enum SPIError {
	InvalidBus,
	FreqHigherThanBus,
	SendErr,
	ReadErr,
}