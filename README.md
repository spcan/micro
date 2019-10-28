# micro
Cortex-M peripherals framework

## About this crate
This crate provides a common lightweight framework to target most Cortex-M devices from a single crate. This is done via conditional compilation of the different peripherals available in each device.

## Features
- [x] Target the core peripherals of all Cortex-M devices (NVIC, SCB, etc...) and some unique ones (FPU, etc...)
- [ ] **WIP** Target most common peripherals that can be used by the user (RCC, PWR, GPIO, etc...)
    - [x] RCC
    - [x] GPIO
    - [x] IWDG
    - [x] WWDG
    - [x] CRC
    - [ ] **WIP** I2C
    - [ ] **WIP** SPI
    - [ ] **WIP** RTC
    - [ ] **WIP** PWR
    - [ ] TIMx
    - [ ] USB OTG
    - [ ] SDIO
    - [ ] Ethernet
    - [ ] Flash
    - [ ] RNG

- [ ] Allow for multithreaded applications by creating a "one instance only" system for the peripherals.
- [ ] **WIP** Compatible with [`embedded-hal`](https://github.com/rust-embedded/embedded-hal).
    - [x] Delay
    - [x] Watchdog (PARTIAL)
    - [ ] **WIP** Read/Write for SPI, I2C
- [ ] Create some examples.

## Showcase
```
#![no_main]
#![no_std]

extern crate micro;
#[macro_use]
extern crate cortex_m_rt;
extern crate panic_halt;

use micro::common::{ ClockCfg, SrcClock };
use micro::common::{ VolatileStruct, asm, State };
use micro::common::{ PortConfig, OutputType, PUPD };

use micro::peripherals::extended::{ rcc, gpio, Gpio };

#[entry]
unsafe fn main() -> ! {

	let mut rcc = rcc::Rcc::from_addr(rcc::ADDRESS);

    // This part is only used to configure the SYSCLK, it can be skipped
	let cfg = ClockCfg {
        //  (plln, pllm, pllp, pllq)
		pllcfg: Some( ((200, 16, 2, 4), SrcClock::HSI) ),
		sysclk: (SrcClock::PLL, 0),
		hpre: (1, 0, 0),
		ppre: (10, 10, 0),
		i2scfg: (100, 2, 2),
	};

	let clocks = rcc.freeze(cfg);

	let mut gpiod = Gpio::new(gpio::ADDRESS_D);

	rcc.peripheral_state(State::ON, common::RCCPeripheral::GPIOD);

	let green  = gpiod.pin(12).unwrap();
	let orange = gpiod.pin(13).unwrap();
	let red    = gpiod.pin(14).unwrap();
	let blue   = gpiod.pin(15).unwrap();

	green.mode(PortConfig::Output as u32)
		.otype(OutputType::PushPull as u32)
		.pupd(PUPD::PullUp as u32);

	orange.mode(PortConfig::Output as u32)
		.otype(OutputType::PushPull as u32)
		.pupd(PUPD::PullUp as u32);

	red.mode(PortConfig::Output as u32)
		.otype(OutputType::PushPull as u32)
		.pupd(PUPD::PullUp as u32);

	blue.mode(PortConfig::Output as u32)
		.otype(OutputType::PushPull as u32)
		.pupd(PUPD::PullUp as u32);

	loop {
		green.set();
		asm::delay(10_000_000);
		orange.set();
		asm::delay(10_000_000);
		red.set();
		asm::delay(10_000_000);
		blue.set();
		asm::delay(10_000_000);

		green.reset();
		asm::delay(10_000_000);
		orange.reset();
		asm::delay(10_000_000);
		red.reset();
		asm::delay(10_000_000);
		blue.reset();
		asm::delay(10_000_000);
	}
}
```

## License
Licensed under dual license [APACHE-2.0]() and [MIT]()
