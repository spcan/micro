//! Flash Patch and Breakpoint Unit

use crate::common::{ Register };

pub const ADDRESS: u32 = 0xE000_2000;
pub const SIZE: usize = 1006;

#[repr(C)]
pub struct Fpb {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Fpb {}
