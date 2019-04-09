//! Abstraction over a Read/Write register

#[cfg(feature = "std")]
use std::{ ptr, ops };

#[cfg(not(feature = "std"))]
use core::{ ptr, ops };


#[repr(C)]
pub struct Register<T>(T);

impl<T> Register<T> {
	/// Read the register value
	pub fn read(&self) -> T {
		unsafe {
			ptr::read_volatile(&self.0)
		}
	}

	/// Writes a value to the register
	pub fn write(&mut self, data: T) {
		unsafe {
			ptr::write_volatile(&mut self.0, data)
		}
	}
}

impl<T> ops::BitOrAssign<T> for Register<T>
	where T: ops::BitOr<T, Output=T>
{
	fn bitor_assign(&mut self, other: T) {
		let new = self.read() | other;
		self.write(new);
	}
}

impl<T> ops::BitAndAssign<T> for Register<T>
	where T: ops::BitAnd<T, Output=T>
{
	fn bitand_assign(&mut self, other: T) {
		let new = self.read() & other;
		self.write(new);
	}
}

pub trait VolatileStruct: Sized {
	unsafe fn from_addr(addr: u32) -> &'static mut Self {
		Self::from_ptr(addr as *mut Self)
	}

	unsafe fn from_ptr(addr: *mut Self) -> &'static mut Self {
		&mut *addr
	}
}