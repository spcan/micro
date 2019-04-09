//! Floating Point Unit
//! 
//! Only present in some models

use crate::common::{ Register };

pub const ADDRESS: u32 = 0xE000_EF30;
pub const SIZE: usize = 7;

#[repr(C)]
pub struct Fpu {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Fpu {}