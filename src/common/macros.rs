//! Common macros

#[macro_export]
macro_rules! reexport {
	{private:$(mod $name:ident;)+} => {
		$(
			mod $name;
			pub use self::$name::*;
		)+
	};

	{public:$(mod $name:ident;)+} => {
		$(
			pub mod $name;
			pub use self::$name::*;
		)+
	};

	{private:$(mod $privname:ident;)+ public:$(mod $pubname:ident;)+} => {
		$(
			mod $privname;
			pub use self::$privname::*;
		)+

		$(
			pub mod $pubname;
			pub use self::$pubname::*;
		)+
	};

	{public:$(mod $pubname:ident;)+ private:$(mod $privname:ident;)+} => {
		$(
			mod $privname;
			pub use self::$privname::*;
		)+

		$(
			pub mod $pubname;
			pub use self::$pubname::*;
		)+
	};
}

#[macro_export]
macro_rules! mask {
	( $x:expr ) => {
		(1 << $x) - 1;
	};
}

#[macro_export]
macro_rules! read_partial {
	($r:expr, $size:expr, $offset:expr, $t:ty) => {
		(($r >> $offset) & mask!($size)) as $t
	};

	($r:expr, $size:expr, $offset:expr) => {
		(($r >> $offset) & mask!($size)).into()
	};
}

#[macro_export]
macro_rules! mask_from {
	( [ $( $x:expr ),* ] ) => {
		let mut out = 0;
		$(
			out |=  1 << $x;
		)*

		out
	};

	( $x:ident ) => {
		let mut out = 0;
		for i in $x {
			out |= 1 << i;
		}

		out
	};
}

#[macro_export]
macro_rules! impl_rwio {
	($name:ident<$life:tt>) => {
		impl<$life> $name<$life> {
			pub fn set(&mut self, b: usize, o: usize) -> &mut Self {
				self.block[b] |= 1 << o;
				self
			}

			pub fn clear(&mut self, b: usize, o: usize) -> &mut Self {
				self.block[b] &= !(1 << o);
				self
			}

			pub fn is_set(&self, r: usize, b: usize) -> bool {
				(self.block[r].read() >> b) & 1 == 1
			}

			pub fn write_bits(&mut self, b: usize, o: usize, data: u32, size: usize) -> &mut Self {
				let mask = (1u32 << size) - 1;
				let old = self.block[b].read();
				self.block[b].write( old & !(mask << o) | ((data & mask) << o) );
				self
			}

			pub fn modify(&mut self, b: usize, f: &dyn Fn(u32)->u32) -> &mut Self {
				let reg = self.block[b].read();
				let res = f(reg);
				self.block[b].write(res);

				self
			}
		}
	};

	($name:ident) => {
		impl $name {
			pub fn set(&mut self, b: usize, o: usize) -> &mut Self {
				self.block[b] |= 1 << o;
				self
			}

			pub fn clear(&mut self, b: usize, o: usize) -> &mut Self {
				self.block[b] &= !(1 << o);
				self
			}

			pub fn is_set(&self, r: usize, b: usize) -> bool {
				(self.block[r].read() >> b) & 1 == 1
			}

			pub fn write_bits(&mut self, b: usize, o: usize, data: u32, size: usize) -> &mut Self {
				let mask = (1u32 << size) - 1;
				let old = self.block[b].read();
				self.block[b].write( old & !(mask << o) | ((data & mask) << o) );
				self
			}

			pub fn modify(&mut self, b: usize, f: &dyn Fn(u32)->u32) -> &mut Self {
				let reg = self.block[b].read();
				let res = f(reg);
				self.block[b].write(res);

				self
			}
		}
	};
}