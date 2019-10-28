

#![cfg_attr(feature = "inline-asm", feature(asm))]

#![feature(const_fn)]

#![no_std]


#[macro_use]
pub mod common;

pub mod peripherals;
pub mod register;
