# micro
Cortex-M system peripherals

## About this crate
This crate provides a common lightweight framework to target most Cortex-M devices from a single crate. This is done via conditional compilation of the different peripherals available in each device.

## Features
- [x] Target the core peripherals of all Cortex-M devices (NVIC, SCB, etc...) and some unique ones (FPU, etc...)
- [ ] **WIP** Target most common peripherals that can be used by the normal user (RCC, PWR, GPIO, etc...)
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

## License
Licensed under dual license [APACHE-2.0]() and [MIT]()
