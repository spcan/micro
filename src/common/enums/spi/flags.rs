//! SPI Flags

#[derive(Debug, Copy, Clone)]
pub enum SPIFlag {
	FFError   = 8,
	Busy      = 7,
	Overrun   = 6,
	ModeFault = 5,
	CRCErr    = 4,
	Underrun  = 3,
	TXE       = 1,
	RXNE      = 0,
}