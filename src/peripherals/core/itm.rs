//! Instrumentation Trace Macrocell

use crate::common::{ Register };

pub const ADDRESS: u32 = 0xE000_0000;
pub const SIZE: usize = 1006;

#[repr(C)]
pub struct Dwt {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Dwt {}
