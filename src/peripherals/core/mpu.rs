//! Memory Protectino Unit

use crate::common::{ Register };

pub const ADDRESS: u32 = 0xE000_ED90;
pub const SIZE: usize = 11;

#[repr(C)]
pub struct Mpu {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Mpu {}
