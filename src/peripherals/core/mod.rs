//! Core peripherals of the STM32 devices



#[cfg(fpu)]
pub mod fpu;

#[cfg(not(armv6m))]
pub mod cbp;
#[cfg(not(armv6m))]
pub mod fpb;
#[cfg(not(armv6m))]
pub mod itm;
#[cfg(not(armv6m))]
pub mod tpiu;

pub mod cpuid;
pub mod dcb;
pub mod dwt;
pub mod mpu;
pub mod nvic;
pub mod scb;
pub mod syst;