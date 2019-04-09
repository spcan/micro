

#![cfg_attr(feature = "inline-asm", feature(asm))]


#![no_std]


#[macro_use]
pub mod common;

pub mod peripherals;
pub mod register;