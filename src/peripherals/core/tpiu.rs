//! Trace Port Interface Unit

use crate::common::{ Register };

pub const ADDRESS: u32 = 0xE004_0000;
pub const SIZE: usize = 1011;

#[repr(C)]
pub struct Tpiu {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Tpiu {}
