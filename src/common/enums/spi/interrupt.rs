//! SPI Interrupts

#[derive(Debug, Copy, Clone)]
pub enum SPIInterrupt {
	RXDMA = 0,
	TXDMA = 1,
	ERR   = 5,
	RXNE  = 6,
	TXE   = 7,
}