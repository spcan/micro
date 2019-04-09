# micro
Cortex-M system peripherals

Part of this work has been taken from/based on the [cortex-m](https://github.com/rust-embedded/cortex-m) crate, and as such, this crate will be licensed the same as the original crate. If you are interested in contributing, please head to the Rust [Embedded WG](https://github.com/rust-embedded)

## About this crate
This crate aims to make it possible to target all Cortex-M devices from a single crate. This is done via conditional compilation of the different peripherals available in each device.

## Features
- [x] Target the core peripherals of all Cortex-M devices (NVIC, SCB, etc...) and some unique ones (FPU, etc...)
- [ ] **WIP** Target most common peripherals that can be used by the normal user (RCC, PWR, GPIO, etc...)
- [ ] Allow for multithreaded applications by creating a "one instance only" system for the peripherals.
- [ ] Compatible with [`embedded-hal`](https://github.com/rust-embedded/embedded-hal).
- [ ] Create some examples.
## License
Licensed under dual license [APACHE-2.0]() and [MIT]()
