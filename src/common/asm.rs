//! Miscelaneous assembly functions

/// Inserts a breakpoint
#[inline(always)]
pub fn bkpt() {
	#[cfg(all(cortex_m, feature = "inline-asm"))]
	{
		unsafe {
			asm!("bkpt" :::: "volatile")
		}
	}

	#[cfg(all(cortex_m, not(feature = "inline-asm")))]
	{
		unsafe {
			extern "C" {
				fn __bkpt();
			}

			__bkpt();
		}
	}

	#[cfg(not(cortex_m))]
	{
		unimplemented!();
	}
}

/// Blocks the program for `n` instruction cycles
#[inline]
pub fn delay(n: u32) {
	#[cfg(all(cortex_m, feature = "inline-asm"))]
	{
		asm!("1:
			  nop
			  subs $0, $$1
			  bne.n 1b"
			: "+r"(n/4 + 1)
			:
			:
			: "volatile");
	}

	#[cfg(all(cortex_m, not(feature = "inline-asm")))]
	{
		unsafe {
			extern "C" {
				fn __delay(n: u32);
			}

			__delay(n/4 + 1);
		}
	}

	#[cfg(not(cortex_m))]
	{
		unimplemented!();
	}
}

/// Inserts a no-operation
#[inline(always)]
pub fn nop() {
	#[cfg(all(cortex_m, feature = "inline-asm"))]
	{
		unsafe {
			asm!("nop" :::: "volatile")
		}
	}

	#[cfg(all(cortex_m, not(feature = "inline-asm")))]
	{
		unsafe {
			extern "C" {
				fn __nop();
			}

			__nop();
		}
	}

	#[cfg(not(cortex_m))]
	{
		unimplemented!();
	}
}

/// Wait For Event
#[inline(always)]
pub fn wfe() {
	#[cfg(all(cortex_m, feature = "inline-asm"))]
	{
		unsafe {
			asm!("wfe" :::: "volatile")
		}
	}

	#[cfg(all(cortex_m, not(feature = "inline-asm")))]
	{
		unsafe {
			extern "C" {
				fn __wfe();
			}

			__wfe();
		}
	}

	#[cfg(not(cortex_m))]
	{
		unimplemented!();
	}
}

/// Wait For Interrupt
#[inline(always)]
pub fn wfi() {
	#[cfg(all(cortex_m, feature = "inline-asm"))]
	{
		unsafe {
			asm!("wfi" :::: "volatile")
		}
	}

	#[cfg(all(cortex_m, not(feature = "inline-asm")))]
	{
		unsafe {
			extern "C" {
				fn __wfi();
			}

			__wfi();
		}
	}

	#[cfg(not(cortex_m))]
	{
		unimplemented!();
	}
}

/// Send evend
#[inline(always)]
pub fn sev() {
	#[cfg(all(cortex_m, feature = "inline-asm"))]
	{
		unsafe {
			asm!("sev" :::: "volatile")
		}
	}

	#[cfg(all(cortex_m, not(feature = "inline-asm")))]
	{
		unsafe {
			extern "C" {
				fn __sev();
			}

			__sev();
		}
	}

	#[cfg(not(cortex_m))]
	{
		unimplemented!();
	}
}

/// Instruction Synchronization Barrier
///
/// Flushes the pipeline in the processor, so that all instructions following the `ISB` are fetched
/// from cache or memory, after the instruction has been completed.
#[inline]
pub fn isb() {
	#[cfg(all(cortex_m, feature = "inline-asm"))]
	{
		unsafe {
			asm!("isb 0xF" ::: "memory" : "volatile")
		}
	}

	#[cfg(all(cortex_m, not(feature = "inline-asm")))]
	{
		unsafe {
			extern "C" {
				fn __isb();
			}
			__isb()
		}
	}

	#[cfg(not(cortex_m))]
	{
		unimplemented!()
	}
}

/// Data Synchronization Barrier
///
/// Acts as a special kind of memory barrier. No instruction in program order after this instruction
/// can execute until this instruction completes. This instruction completes only when both:
///
///  * any explicit memory access made before this instruction is complete
///  * all cache and branch predictor maintenance operations before this instruction complete
#[inline]
pub fn dsb() {
	#[cfg(all(cortex_m, feature = "inline-asm"))]
	{
		unsafe {
			asm!("dsb 0xF" ::: "memory" : "volatile")
		}
	}

	#[cfg(all(cortex_m, not(feature = "inline-asm")))]
	{
		unsafe {
			extern "C" {
				fn __dsb();
			}
			__dsb()
		}
	}

	#[cfg(not(cortex_m))]
	{
		unimplemented!()
	}
}

/// Data Memory Barrier
///
/// Ensures that all explicit memory accesses that appear in program order before the `DMB`
/// instruction are observed before any explicit memory accesses that appear in program order
/// after the `DMB` instruction.
#[inline]
pub fn dmb() {
	#[cfg(all(cortex_m, feature = "inline-asm"))]
	{
		unsafe {
			asm!("dmb 0xF" ::: "memory" : "volatile")
		}
	}

	#[cfg(all(cortex_m, not(feature = "inline-asm")))]
	{
		unsafe {
			extern "C" {
				fn __dmb();
			}
			__dmb()
		}
	}

	#[cfg(not(cortex_m))]
	{
		unimplemented!()
	}
}