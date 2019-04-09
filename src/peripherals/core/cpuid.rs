//! CPUID 

use crate::common::{ Register, asm };

pub const ADDRESS: u32 = 0xE000_ED00;

#[cfg(not(armv6m))]
pub const SIZE: usize = 34;

#[cfg(armv6m)]
pub const SIZE: usize = 30;

#[repr(C)]
pub struct CpuId {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for CpuId {}

impl CpuId {
	/// Reads the device info
	pub fn info(&self) -> u32 {
		self.block[0].read()
	}
}

#[cfg(not(armv6m))]
impl CpuId {
	/// Selects the current CCSIDR
	/// 
	/// * `level`: the required cache level minus 1, e.g. 0 for L1
	/// * `ind`: select instruction cache or data/unified cache
	/// 
	/// `level` is masked to be between 0 and 7
	pub fn select_cache(&mut self, level: u32, ind: CsselrCacheType) {
		self.block[CPUIDRegs::CSSELR as usize].write(
			(((level as u32) << 1) & (0x7 << 1)) |
			(((ind   as u32) << 0) &  1)
		)
	}

	/// Returns the number of sets and ways in the selected cache
	pub fn cache_num_sets_ways(&mut self, level: u32, ind: CsselrCacheType) -> (u16, u16) {
		self.select_cache(level, ind);

		asm::dsb();

		let ccsidr = self.block[CPUIDRegs::CCSIDR as usize].read();

		(
			(1 + ((ccsidr >> 13) & 0x7FFF ) as u16 ),
			(1 + ((ccsidr >>  3) &  0x3FF ) as u16 )
		)
	}
}

/// Type of cache to select on CSSELR writes.
#[cfg(not(armv6m))]
#[derive(Copy, Clone)]
pub enum CsselrCacheType {
	/// Select DCache or unified cache
	DataOrUnified = 0,
	/// Select ICache
	Instruction = 1,
}

/// Registers in CPUID
#[cfg(not(armv6m))]
#[derive(Copy, Clone)]
pub enum CPUIDRegs {
	Base = 0,
	PFR = 16,
	DFR = 18,
	AFR = 19,
	MMFR = 20,
	ISAR = 24,
	CLIDR = 30,
	CTR = 31,
	CCSIDR = 32,
	CSSELR = 33,
}
