//! Extended peripherals

reexport!{
	private:
		mod crc; // ADDR
		mod exti;
		mod iwdg;
		mod wwdg;
		mod pwr; // ADDR
		mod rtc; // ADDR
	public:
		mod gpio;
}

pub mod rcc;

//pub mod i2c;

pub mod spi;

//pub mod advtim;

//pub mod timers;

pub mod flashiface;
