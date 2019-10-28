//! Common enums used in all implementations


reexport!{
	private:
		mod extilines;
		mod gpio;
		mod i2c;
		mod rcc;
		mod spi;
		mod tim;
		mod trigger;
}

#[derive(Debug, Copy, Clone)]
pub enum PeripheralBus {
	APB1,
	APB2,
	AHB,
}