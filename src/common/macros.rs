//! Common macros

#[macro_export]
macro_rules! mask {
	( $x:expr ) => {
		(1u32 << $x) - 1;
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