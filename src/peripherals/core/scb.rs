//! System Control Block

use crate::common::{ Register, State, asm, VolatileStruct };

#[cfg(not(armv6m))]
use super::cpuid::CsselrCacheType;

pub const ADDRESS: u32 = 0xE000_ED04;

pub const SIZE: usize = 34;

#[derive(Copy, Clone, PartialEq)]
pub enum SCBRegs {
	ICSR = 0,
	VTOR = 1,
	AIRCR = 2,
	SCR = 3,
	CCR = 4,
	SHPR = 5,
	SHCSR = 8,
	CFSR = 9,
	HFSR = 10,
	DFSR = 11,
	MMFAR = 12,
	BFAR = 13,
	AFSR = 14,
	CPACR = 33,
}

#[repr(C)]
pub struct Scb {
	block: [Register<u32>; SIZE],
}

impl crate::common::VolatileStruct for Scb {}

impl Scb {

	#[inline]
	pub fn set(&mut self, b: usize, o: usize) -> &mut Self {
		self.block[b] |= 1<<o;
		self
	}

	#[inline]
	pub fn clear(&mut self, b: usize, o: usize) -> &mut Self {
		self.block[b] &= !(1 << o);
		self
	}
	#[inline]
	pub fn write_bits(&mut self, b: usize, o: usize, data: u32, size: usize) -> &mut Self {
		let mask = (1u32 << size) - 1;
		let old = self.block[b].read();
		self.block[b].write( old & !(mask << o) | ((data & mask) << o) );
		self
	}
}

#[cfg(has_fpu)]
#[derive(Clone, Copy)]
pub enum FpuAccessMode {
	/// FPU is not accessible
	Disabled,
	/// FPU is accessible in Priviliged and User mode
	Enabled,
	/// FPU is accessible only in Priviliged mode
	Priviliged,
}

#[cfg(has_fpu)]
mod fpu_consts {
	pub const SCB_CPACR_FPU_MASK: u32 = 0b11_11 << 20;
	pub const SCB_CPACR_FPU_ENABLE: u32 = 0b01_01 << 20;
	pub const SCB_CPACR_FPU_USER: u32 = 0b10_10 << 20;
}

#[cfg(has_fpu)]
use self::fpu_consts::*;

#[cfg(has_fpu)]
impl Scb {
	/// Disable FPU
	pub fn disable_fpu(&mut self) {
		self.set_fpu_access(FpuAccessMode::Disabled)
	}

	/// Enable FPU in Priviliged and User mode
	pub fn enable_fpu(&mut self) {
		self.set_fpu_access(FpuAccessMode::Enabled)
	}

	/// Get FPU access mode
	pub fn fpu_access_mode(&self) -> FpuAccessMode {
		let cpacr = self.block[SCBRegs::CPACR as usize].read();

		// CPACR & FPU_ENABLE
		match cpacr & SCB_CPACR_FPU_MASK {
			// PRIVILIGED   |  USER
			SCB_CPACR_FPU_ENABLE | SCB_CPACR_FPU_USER => FpuAccessMode::Enabled,
			// ONLY PRIVILIGED
			SCB_CPACR_FPU_ENABLE => FpuAccessMode::Priviliged,
			_ => FpuAccessMode::Disabled,
		}
	}

	/// Sets FPU access mode
	pub fn set_fpu_access(&mut self, mode: FpuAccessMode) {
		self.block[SCBRegs::CPACR as usize] &= !(0b11_11 << 20);

		match mode {
			FpuAccessMode::Disabled => (),
			FpuAccessMode::Priviliged => self.block[SCBRegs::CPACR as usize] |= SCB_CPACR_FPU_ENABLE,
			FpuAccessMode::Enabled => self.block[SCBRegs::CPACR as usize] |= SCB_CPACR_FPU_ENABLE | SCB_CPACR_FPU_USER,			
		}
	}
}

impl Scb {
	/// Returns the active exception number
	pub fn vect_active(&self) -> VectActive {
		match self.block[SCBRegs::ICSR as usize].read() as u8 {
			0 => VectActive::ThreadMode,
			2 => VectActive::Exception(Exception::NonMaskableInt),
			3 => VectActive::Exception(Exception::HardFault),

			#[cfg(not(armv6m))]
			4 => VectActive::Exception(Exception::MemoryManagement),

			#[cfg(not(armv6m))]
			5 => VectActive::Exception(Exception::BusFault),

			#[cfg(not(armv6m))]
			6 => VectActive::Exception(Exception::UsageFault),

			#[cfg(any(armv8m, target_arch = "x86_64"))]
			7 => VectActive::Exception(Exception::SecureFault),

			11 => VectActive::Exception(Exception::SVCall),

			#[cfg(not(armv6m))]
			12 => VectActive::Exception(Exception::DebugMonitor),

			14 => VectActive::Exception(Exception::PendSV),
			15 => VectActive::Exception(Exception::SysTick),
			irqn => VectActive::Interrupt( irqn - 16 ),
		}
	}
}

/// Processor core exceptions (internal interrupts)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Exception {
	/// Non maskable interrupt
	NonMaskableInt,

	/// Hard fault interrupt
	HardFault,

	/// Memory management interrupt (not present on Cortex-M0 variants)
	#[cfg(not(armv6m))]
	MemoryManagement,

	/// Bus fault interrupt (not present on Cortex-M0 variants)
	#[cfg(not(armv6m))]
	BusFault,

	/// Usage fault interrupt (not present on Cortex-M0 variants)
	#[cfg(not(armv6m))]
	UsageFault,

	/// Secure fault interrupt (only on ARMv8-M)
	#[cfg(any(armv8m, target_arch = "x86_64"))]
	SecureFault,

	/// SV call interrupt
	SVCall,

	/// Debug monitor interrupt (not present on Cortex-M0 variants)
	#[cfg(not(armv6m))]
	DebugMonitor,

	/// Pend SV interrupt
	PendSV,

	/// System Tick interrupt
	SysTick,
}

impl Exception {
	/// Returns the IRQ number of this `Exception`
	///
	/// The return value is always within the closed range `[-1, -14]`
	pub fn irqn(&self) -> i8 {
		match *self {
			Exception::NonMaskableInt => -14,
			Exception::HardFault => -13,
			#[cfg(not(armv6m))]
			Exception::MemoryManagement => -12,
			#[cfg(not(armv6m))]
			Exception::BusFault => -11,
			#[cfg(not(armv6m))]
			Exception::UsageFault => -10,
			#[cfg(any(armv8m, target_arch = "x86_64"))]
			Exception::SecureFault => -9,
			Exception::SVCall => -5,
			#[cfg(not(armv6m))]
			Exception::DebugMonitor => -4,
			Exception::PendSV => -2,
			Exception::SysTick => -1,
		}
	}
}

/// Active exception number
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VectActive {
	/// Thread mode
	ThreadMode,

	/// Processor core exception (internal interrupts)
	Exception(Exception),

	/// Device specific exception (external interrupts)
	Interrupt(u8),
}

impl VectActive {
	/// Converts a `byte` into `VectActive`
	pub fn from(vect_active: u8) -> Option<Self> {
		Some(match vect_active {
			0 => VectActive::ThreadMode,
			2 => VectActive::Exception(Exception::NonMaskableInt),
			3 => VectActive::Exception(Exception::HardFault),

			#[cfg(not(armv6m))]
			4 => VectActive::Exception(Exception::MemoryManagement),

			#[cfg(not(armv6m))]
			5 => VectActive::Exception(Exception::BusFault),

			#[cfg(not(armv6m))]
			6 => VectActive::Exception(Exception::UsageFault),

			#[cfg(any(armv8m, target_arch = "x86_64"))]
			7 => VectActive::Exception(Exception::SecureFault),

			11 => VectActive::Exception(Exception::SVCall),

			#[cfg(not(armv6m))]
			12 => VectActive::Exception(Exception::DebugMonitor),

			14 => VectActive::Exception(Exception::PendSV),
			15 => VectActive::Exception(Exception::SysTick),

			irqn if (irqn >= 16) && (irqn < 240) => VectActive::Interrupt(irqn),

			_ => return None,
		})
	}
}

#[cfg(not(armv6m))]
mod scb_consts {
	pub const SCB_CCR_IC_MASK: u32 = (1 << 17);
	pub const SCB_CCR_DC_MASK: u32 = (1 << 16);
}

#[cfg(not(armv6m))]
use self::scb_consts::*;

#[cfg(not(armv6m))]
impl Scb {
	/// Enable I-Cache if disabled
	#[inline]
	pub fn icache_state(&mut self, s: State) -> &mut Self {

		match s {
			State::ON if self.icache_enabled() => return self,
			State::OFF if !self.icache_enabled() => return self,

			State::ON => {
				let cbp = unsafe { super::cbp::Cbp::from_addr( super::cbp::ADDRESS ) };
				cbp.iciallu();
				self.block[SCBRegs::CCR as usize] |= SCB_CCR_IC_MASK;
			},

			State::OFF => {
				self.block[SCBRegs::CCR as usize] &= !SCB_CCR_IC_MASK;
				let cbp = unsafe { super::cbp::Cbp::from_addr( super::cbp::ADDRESS ) };
				cbp.iciallu();
			},
		}

		asm::dsb();
		asm::isb();

		self
	}

	/// Returns true if the I-Cache is enabled
	#[inline]
	pub fn icache_enabled(&self) -> bool {
		asm::dsb();
		asm::isb();

		self.block[SCBRegs::CCR as usize].read() & SCB_CCR_IC_MASK == SCB_CCR_IC_MASK
	}

	/// Invalidate I-Cache
	pub fn invalidate_icache(&self)  {
		let cbp = unsafe { super::cbp::Cbp::from_addr( super::cbp::ADDRESS ) };

		cbp.iciallu();

		asm::dsb();
		asm::isb();
	}

	/// Enables/Disables D-Cache
	#[inline]
	pub fn dcache_state(&mut self, s: State) -> &mut Self {
		match s {
			State::ON if self.dcache_enabled() => return self,
			State::OFF if !self.dcache_enabled() => return self,

			State::ON => {
				self.invalidate_dcache( unsafe { super::cpuid::CpuId::from_addr(super::cpuid::ADDRESS) } );
				self.block[SCBRegs::CCR as usize] |= SCB_CCR_DC_MASK;
				asm::dsb();
				asm::isb();
			},

			State::OFF => {
				self.block[SCBRegs::CCR as usize] &= !SCB_CCR_DC_MASK;
				self.clean_invalidate_dcache( unsafe { super::cpuid::CpuId::from_addr(super::cpuid::ADDRESS) } );
			},
		}

		self
	}

	/// Returns true if D-Cache is enabled
	#[inline]
	pub fn dcache_enabled(&self) -> bool {
		asm::dsb();
		asm::isb();

		self.block[SCBRegs::CCR as usize].read() & SCB_CCR_DC_MASK == SCB_CCR_DC_MASK
	}

	/// Invalidates D-cache
	///
	/// Note that calling this while the dcache is enabled will probably wipe out your
	/// stack, depending on optimisations, breaking returning to the call point.
	/// It's used immediately before enabling the dcache, but not exported publicly.
	#[inline]
	fn invalidate_dcache(&mut self, cpuid: &mut super::cpuid::CpuId) {
		// Read number of sets and ways
		let (sets, ways) = cpuid.cache_num_sets_ways(0, CsselrCacheType::DataOrUnified);

		let cbp = unsafe { super::cbp::Cbp::from_addr( super::cbp::ADDRESS ) };

		// Invalidate entire D-Cache
		for set in 0..sets {
			for way in 0..ways {
				cbp.dcisw(set, way);
			}
		}

		asm::dsb();
		asm::isb();
	}

	/// Cleans D-cache
	#[inline]
	pub fn clean_dcache(&mut self, cpuid: &mut super::cpuid::CpuId) {
		// Read number of sets and ways
		let (sets, ways) = cpuid.cache_num_sets_ways(0, CsselrCacheType::DataOrUnified);

		let cbp = unsafe { super::cbp::Cbp::from_addr( super::cbp::ADDRESS ) };

		for set in 0..sets {
			for way in 0..ways {
				cbp.dccsw(set, way);
			}
		}

		asm::dsb();
		asm::isb();
	}

	/// Cleans and invalidates D-cache
	#[inline]
	pub fn clean_invalidate_dcache(&mut self, cpuid: &mut super::cpuid::CpuId) {
		// Read number of sets and ways
		let (sets, ways) = cpuid.cache_num_sets_ways(0, CsselrCacheType::DataOrUnified);

		let cbp = unsafe { super::cbp::Cbp::from_addr( super::cbp::ADDRESS ) };

		for set in 0..sets {
			for way in 0..ways {
				cbp.dccisw(set, way);
			}
		}

		asm::dsb();
		asm::isb();
	}

	/// Invalidates D-cache by address
	///
	/// `addr`: the address to invalidate
	/// `size`: size of the memory block, in number of bytes
	///
	/// Invalidates cache starting from the lowest 32-byte aligned address represented by `addr`,
	/// in blocks of 32 bytes until at least `size` bytes have been invalidated.
	#[inline]
	pub fn invalidate_dcache_by_address(&mut self, addr: usize, size: usize) {
		// No-op zero sized operations
		if size == 0 {
			return;
		}

		asm::dsb();

		// Cache lines are fixed to 32 bit on Cortex-M7 and not present in earlier Cortex-M
		const LINESIZE: usize = 32;
		let num_lines = ((size - 1) / LINESIZE) + 1;

		let mut addr = addr & 0xFFFF_FFE0;

		let cbp = unsafe { super::cbp::Cbp::from_addr( super::cbp::ADDRESS ) };

		for _ in 0..num_lines {
			cbp.dcimvac(addr as u32);
			addr += LINESIZE;
		}

		asm::dsb();
		asm::isb();
	}

	/// Cleans D-cache by address
	///
	/// `addr`: the address to clean
	/// `size`: size of the memory block, in number of bytes
	///
	/// Cleans cache starting from the lowest 32-byte aligned address represented by `addr`,
	/// in blocks of 32 bytes until at least `size` bytes have been cleaned.
	#[inline]
	pub fn clean_dcache_by_address(&mut self, addr: usize, size: usize) {
		// No-op zero sized operations
		if size == 0 {
			return;
		}

		asm::dsb();

		// Cache lines are fixed to 32 bit on Cortex-M7 and not present in earlier Cortex-M
		const LINESIZE: usize = 32;
		let num_lines = ((size - 1) / LINESIZE) + 1;

		let mut addr = addr & 0xFFFF_FFE0;

		let cbp = unsafe { super::cbp::Cbp::from_addr( super::cbp::ADDRESS ) };

		for _ in 0..num_lines {
			cbp.dccmvac(addr as u32);
			addr += LINESIZE;
		}

		asm::dsb();
		asm::isb();
	}

	/// Cleans and invalidates D-cache by address
	///
	/// `addr`: the address to clean and invalidate
	/// `size`: size of the memory block, in number of bytes
	///
	/// Cleans and invalidates cache starting from the lowest 32-byte aligned address represented
	/// by `addr`, in blocks of 32 bytes until at least `size` bytes have been cleaned and
	/// invalidated.
	#[inline]
	pub fn clean_invalidate_dcache_by_address(&mut self, addr: usize, size: usize) {
		// No-op zero sized operations
		if size == 0 {
			return;
		}
		
		asm::dsb();

		// Cache lines are fixed to 32 bit on Cortex-M7 and not present in earlier Cortex-M
		const LINESIZE: usize = 32;
		let num_lines = ((size - 1) / LINESIZE) + 1;

		let mut addr = addr & 0xFFFF_FFE0;

		let cbp = unsafe { super::cbp::Cbp::from_addr( super::cbp::ADDRESS ) };

		for _ in 0..num_lines {
			cbp.dccimvac(addr as u32);
			addr += LINESIZE;
		}

		asm::dsb();
		asm::isb();
	}
}

impl Scb {
	/// Set/Reset the SLEEPDEEP bit in the SCR register
	pub fn sleepdeep_state(&mut self, s: State) -> &mut Self {
		match s {
			State::ON => self.set(SCBRegs::SCR as usize, 2),
			State::OFF => self.clear(SCBRegs::SCR as usize, 2),
		}
	}

	/// Set/Reset the SLEEPONEXIT bit in the SCR register
	pub fn sleeponexit_state(&mut self, s: State) -> &mut Self {
		match s {
			State::ON => self.set(SCBRegs::SCR as usize, 1),
			State::OFF => self.clear(SCBRegs::SCR as usize, 1),
		}
	}
}

impl Scb {
	/// Initiate a system reset request
	pub fn system_reset(&mut self) -> ! {
		asm::dsb();

		let old = self.block[SCBRegs::AIRCR as usize].read();
		self.block[SCBRegs::AIRCR as usize].write( (0x05FA << 16) | (old & (0x5 << 8)) | (1 << 2) );

		asm::dsb();

		loop {
			asm::nop();
		}
	}
}

impl Scb {
	/// Set/Reset the PENDSVSET bit in the ICSR register which will pend the PendSV interrupt
	pub fn pendsv_state(&mut self, s: State) -> &mut Self {
		match s {
			State::ON  => self.set(SCBRegs::ICSR as usize, 28),
			State::OFF => self.set(SCBRegs::ICSR as usize, 27),
		}
	}

	/// Returns true if PENDSVSET bit in ICSR is set
	pub fn is_pendsv_pending(&self) -> bool {
		self.block[SCBRegs::ICSR as usize].read() & (1 << 28) == (1 << 28)
	}

	/// Set the PENDSTCLR bit in the ICSR register which will clear a pending SysTick interrupt
	#[inline]
	pub fn pendst_state(&mut self, s: State) -> &mut Self {
		match s {
			State::ON  => self.set(SCBRegs::ICSR as usize, 26),
			State::OFF => self.set(SCBRegs::ICSR as usize, 25),
		}
	}

	#[inline]
	/// Returns true if PENDSTSET bit in ICSR is set
	pub fn is_pendst_pending(&self) -> bool {
		self.block[SCBRegs::ICSR as usize].read() & (1 << 26) == (1 << 26)
	}
}

/// System handlers and exceptions
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemHandler {
	#[cfg(not(armv6m))]
	MemManagement,

	#[cfg(not(armv6m))]
	BusFault,

	#[cfg(not(armv6m))]
	UsageFault,

	#[cfg(any(armv8m, target_arch = "x86_64"))]
	SecureFault,

	SVCall,

	#[cfg(not(armv6m))]
	DebugMonitor,

	PendSV,

	SysTick,
}

impl SystemHandler {
	fn index(&self) -> u8 {
		match *self {
			#[cfg(not(armv6m))]
			SystemHandler::MemManagement => 4,
			#[cfg(not(armv6m))]
			SystemHandler::BusFault => 5,
			#[cfg(not(armv6m))]
			SystemHandler::UsageFault => 6,
			#[cfg(any(armv8m, target_arch = "x86_64"))]
			SystemHandler::SecureFault => 7,
			SystemHandler::SVCall => 11,
			#[cfg(not(armv6m))]
			SystemHandler::DebugMonitor => 12,
			SystemHandler::PendSV => 14,
			SystemHandler::SysTick => 15,
		}
	}
}

impl Scb {
	/// Returns the hardware priority of `handler`
	pub fn get_priority(&self, handler: SystemHandler) -> u8 {
		#[cfg(not(armv6m))]
		{
			return match handler {
				SystemHandler::MemManagement => self.block[SCBRegs::SHPR as usize].read()       & 0b1111_1111,
				SystemHandler::BusFault   => (self.block[SCBRegs::SHPR as usize ].read() >>  8) & 0b1111_1111,
				SystemHandler::UsageFault => (self.block[SCBRegs::SHPR as usize ].read() >> 16) & 0b1111_1111,
				SystemHandler::PendSV  => (self.block[SCBRegs::SHPR as usize + 2].read() >> 16) & 0b1111_1111,
				SystemHandler::SysTick => (self.block[SCBRegs::SHPR as usize + 2].read() >> 24) & 0b1111_1111,
				SystemHandler::SVCall  => (self.block[SCBRegs::SHPR as usize + 1].read() >> 24) & 0b1111_1111,
				_ => 0,
			} as u8;
		}

		#[cfg(armv6m)]
		{
			return match handler {
				SystemHandler::PendSV  => (self.block[SCBRegs::SHPR as usize + 2].read() >> 16) & 0b1111_1111,
				SystemHandler::SysTick => (self.block[SCBRegs::SHPR as usize + 2].read() >> 24) & 0b1111_1111,
				SystemHandler::SVCall  => (self.block[SCBRegs::SHPR as usize + 1].read() >> 24) & 0b1111_1111,
			} as u8;
		}
	}

	/// Returns the hardware priority of `handler`
	pub fn set_priority(&mut self, handler: SystemHandler, priority: u32) -> &mut Self {
		#[cfg(not(armv6m))]
		{
			match handler {
				SystemHandler::MemManagement => self.write_bits(SCBRegs::SHPR as usize, 0, priority, 8),
				SystemHandler::BusFault    => self.write_bits(SCBRegs::SHPR as usize,   8, priority, 8),
				SystemHandler::UsageFault  => self.write_bits(SCBRegs::SHPR as usize,  16, priority, 8),
				SystemHandler::PendSV   => self.write_bits(SCBRegs::SHPR as usize + 2, 16, priority, 8),
				SystemHandler::SysTick  => self.write_bits(SCBRegs::SHPR as usize + 2, 24, priority, 8),
				SystemHandler::SVCall   => self.write_bits(SCBRegs::SHPR as usize + 1, 24, priority, 8),
				_ => self,
			}
		}

		#[cfg(armv6m)]
		{
			match handler {
				SystemHandler::PendSV   => self.write_bits(SCBRegs::SHPR as usize + 2, 16, priority, 8),
				SystemHandler::SysTick  => self.write_bits(SCBRegs::SHPR as usize + 2, 24, priority, 8),
				SystemHandler::SVCall   => self.write_bits(SCBRegs::SHPR as usize + 1, 24, priority, 8),
			}
		}
	}
}