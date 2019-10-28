//! Helper module with compile time calculations

#[cfg(feature = "std")]
use std::{cmp, ops, convert, f32};

#[cfg(not(feature = "std"))]
use core::{cmp, ops, convert, f32};


use crate::common::ClockSpeeds::*;
use crate::common::ClockSpeeds::APB1MAX;
use crate::common::frequency::Frequency;

pub struct Clocks {
	pub sysclk: Frequency,
	pub ahb: Frequency,
	pub apb1: Frequency,
	#[cfg(feature = "apb2")]
	pub apb2: Frequency,
	#[cfg(feature = "apb3")]
	pub apb3: Frequency,
}

impl Clocks {
	pub const fn new() -> Clocks {
		Clocks {
			sysclk: Frequency::Hz(0),
			ahb: Frequency::Hz(0),
			apb1: Frequency::Hz(0),
			#[cfg(feature = "apb2")]
			apb2: Frequency::Hz(0),
			#[cfg(feature = "apb3")]
			apb3: Frequency::Hz(0),
		}
	}
}

pub struct RccClocks {
	pub sysclk: Option<Frequency>,
	pub ahb: Option<Frequency>,
	pub apb1: Option<Frequency>,
	#[cfg(feature = "apb2")]
	pub apb2: Option<Frequency>,
	#[cfg(feature = "apb3")]
	pub apb3: Option<Frequency>,
}

/// This is a compile-time calculation of a RCC Clock configuration
/// Using this allows for very optimized clock configurations 
/// with almost no cost in runtime.
pub const fn clock_calculation(cfg: RccClocks, lp: bool) -> ([u32; 12], Clocks) {
	// Array
	//  0: SYSCLK source -> 0 HSI 1 HSE 2 PLL
	//  1: AHB prescaler
	//  2: APB1 prescaler
	//  3: APB2 prescaler
	//  4: APB3 prescaler
	//  8: PLLP
	//  9: PLLN
	// 10: PLLM
	let mut array = [0; 6];

	let mut clocks = Clocks::new();

	match cfg.sysclk {
		// If there is a predefined SYSCLK frequency, try to achieve it
		Some(sys) => defined_sysclk(cfg, lp),
		// If not, continue with the AHB
		None => match cfg.ahb {
			// There is a predefined AHBCLK
			Some(ahb) => defined_ahb(cfg, lp),

			// SYSCLK undefined and AHBCLK undefined
			// Basically, no info
			None => {
				let mut defined = false;

				match cfg.apb1 {
					Some(_) => defined = true,
					_ => (),
				}

				#[cfg(feature = "apb2")]
				match cfg.apb2 {
					Some(_) => defined = true,
					_ => (),
				}

				#[cfg(feature = "apb3")]
				match cfg.apb3 {
					Some(_) => defined = true,
					_ => (),
				}

				match defined {
					true => defined_apb(cfg, lp),
					_ => no_defines(cfg, lp),
				}

			},
		},
	}
}

const fn pllerror(vco: Frequency, ahb: Frequency) -> (u32, u32, f32) {
	// Get the least error configuration
	let mut previous = (0, 0, 1000.0);
	let mut i = 1;
	let mut j = 50;

	// WTF no ranged loops in const fn????
	'outer: loop {
		'inner: loop {
			let ahb_real = (vco * j / (i * 2)).hz() as f32;

			// WTF no .abs() in core?????
			let error = match ((ahb_real / (ahb.hz() as f32)) - 1.0) {
				a if a < 0.0 => -a,
				b => b,
			};

			if error < previous.2 {
				previous = (i*2, j, error);
			}

			j += 1;

			if j == 433 {
				break 'inner;
			}
		}

		i += 1;

		if i == 5 {
			break 'outer;
		}
	}


	previous
}


const fn defined_sysclk(cfg: RccClocks, _: bool) -> ([u32; 12], Clocks) {
	let mut clocks = Clocks::new();
	let mut array = [0u32; 12];

	match cfg.sysclk {
		// Already checked
		None => unreachable!(),
		// Unwraping
		Some(sys) => {
			// CSet up SYSCLK
			match sys {
				// Easy and energy performant
				HSI => {
					array[0] = 0;
					clocks.sysclk = HSI;
				},
				// Anything else will go through PLL
				_ => {
					// Get VCO
					let pllm = match ((HSI.mhz() / 2), (HSI.hz() % 2)) {
						(m, 0) => m,
						(m, _) => m + 1,
					};

					let vco = HSI / pllm;

					// Get best PLL approximation
					let (pllp, plln, _) = pllerror(vco, sys);

					// Set SYSCLK source as PLL
					array[0] = 2;
					clocks.sysclk = vco * plln / pllp;
				}
			}

			// Set up AHB
			match cfg.ahb {
				// Predefined AHB
				Some(ahb) => {
					let (ratio, bytes): (u32, u32) = match u32::from(clocks.sysclk / cmp::min(ahb, AHBMAX)) {
						0..=1     => (  1, 0b0000),
						2..=3     => (  2, 0b1000),
						4..=7     => (  4, 0b1001),
						8..=15    => (  8, 0b1010),
						16..=63   => ( 16, 0b1011),
						64..=127  => ( 64, 0b1100),
						128..=255 => (128, 0b1101),
						256..=511 => (256, 0b1110),
						_         => (512, 0b1111),
					};

					// Set up the AHB prescaler
					array[1] = bytes;
					clocks.ahb = clocks.sysclk / ratio;
				},
				// Undefined AHB
				None => {
					// Set the same as SYSCLK
					array[1] = 0;
					clocks.ahb = clocks.sysclk;
				},
			}

			// Set up APB1 and APB2 and APB3 if they exist
			{
				let (ratio, bytes): (u32, u32) = match cfg.apb1 {
					Some(apb) => match u32::from(clocks.ahb / cmp::min(apb, APB1MAX)) {
						0..=1  => ( 1, 0b000),
						2..=3  => ( 2, 0b100),
						4..=8  => ( 4, 0b101),
						8..=15 => ( 8, 0b110),
						_      => (16, 0b111),
					},
					None => match u32::from(clocks.ahb / APB1MAX) {
						0..=1  => ( 1, 0b000),
						2..=3  => ( 2, 0b100),
						4..=8  => ( 4, 0b101),
						8..=15 => ( 8, 0b110),
						_      => (16, 0b111),
					}
				};

				// Set the values
				array[2] = bytes;
				clocks.apb1 = clocks.ahb / ratio;
			}

			#[cfg(feature = "apb2")]
			{
				let (ratio, bytes): (u32, u32) = match cfg.apb2 {
					Some(apb) => match u32::from(clocks.ahb / cmp::min(apb, APB2MAX)) {
						0..=1  => ( 1, 0b000),
						2..=3  => ( 2, 0b100),
						4..=8  => ( 4, 0b101),
						8..=15 => ( 8, 0b110),
						_      => (16, 0b111),
					},
					None => match u32::from(clocks.ahb / APB2MAX) {
						0..=1  => ( 1, 0b000),
						2..=3  => ( 2, 0b100),
						4..=8  => ( 4, 0b101),
						8..=15 => ( 8, 0b110),
						_      => (16, 0b111),
					}
				};

				// Set the values
				array[3] = bytes;
				clocks.apb2 = clocks.ahb / ratio;
			}

			#[cfg(feature = "apb3")]
			{
				let (ratio, bytes): (u32, u32) = match cfg.apb3 {
					Some(apb) => match u32::from(clocks.ahb / cmp::min(apb, APB3MAX)) {
						0..=1  => ( 1, 0b000),
						2..=3  => ( 2, 0b100),
						4..=8  => ( 4, 0b101),
						8..=15 => ( 8, 0b110),
						_      => (16, 0b111),
					},
					None => match u32::from(clocks.ahb / APB3MAX) {
						0..=1  => ( 1, 0b000),
						2..=3  => ( 2, 0b100),
						4..=8  => ( 4, 0b101),
						8..=15 => ( 8, 0b110),
						_      => (16, 0b111),
					}
				};

				// Set the values
				array[4] = bytes;
				clocks.apb3 = clocks.ahb / ratio;
			}
		},
	}

	(array, clocks)
}

const fn defined_ahb(cfg: RccClocks, lp: bool) -> ([u32; 12], Clocks) {
	let mut clocks = Clocks::new();
	let mut array = [0u32; 12];

	const HSIHZ: u32 = HSI.hz();

	match cfg.ahb {
		None => unreachable!(),
		Some(ahb) => {
			match cmp::min(ahb.hz(), AHBMAX.hz()) {
				// If AHBCLK <= HSI -> SYSCLK = HSI
				0..=HSIHZ => {
					let (ratio, bytes): (u32, u32) = match u32::from(HSI / ahb) {
						0..=1     => (  1, 0b0000),
						2..=3     => (  2, 0b1000),
						4..=7     => (  4, 0b1001),
						8..=15    => (  8, 0b1010),
						16..=63   => ( 16, 0b1011),
						64..=127  => ( 64, 0b1100),
						128..=255 => (128, 0b1101),
						256..=511 => (256, 0b1110),
						_         => (512, 0b1111),
					};

					array[0] = 0;
					array[1] = bytes;

					clocks.sysclk = HSI;
					clocks.ahb = HSI / ratio;
				},
				_ => {
					let pllm = match (HSI.mhz() / 2, HSI.hz() % 2) {
						(m, 0) => m,
						(m, _) => m + 1,
					};

					let vco = HSI / pllm;

					// If it's low power mode and 1.5 * AHBCLK < SYSCLKMAX 
					// it defaults to SYSCLK = AHBCLK to save power
					match lp && u32::from(SYSCLKMAX / cmp::min(ahb, AHBMAX)) < 2
					         && (SYSCLKMAX.hz() % cmp::min(ahb.hz(), AHBMAX.hz()) < (cmp::min(ahb.hz(), AHBMAX.hz()) / 2)) {
						true => {
							let (pllp, plln, _) = pllerror(vco, ahb);

							array[0] = 2;
							array[1] = 0;
							array[8] = match pllp {
								2 => 0b00,
								4 => 0b01,
								6 => 0b10,
								8 => 0b11,
								_ => unreachable!(),
							};
							array[9] = plln;
							array[10] = pllm;

							clocks.sysclk = vco * plln / pllp;
							clocks.ahb = clocks.sysclk;
						},

						false => {
							let (ahbpre, bytes): (u32, u32) = match u32::from(SYSCLKMAX / ahb) {
								0..=1     => (  1, 0b0000),
								2..=3     => (  2, 0b1000),
								4..=7     => (  4, 0b1001),
								8..=15    => (  8, 0b1010),
								16..=63   => ( 16, 0b1011),
								64..=127  => ( 64, 0b1100),
								128..=255 => (128, 0b1101),
								256..=511 => (256, 0b1110),
								_         => (512, 0b1111),
							};

							let (pllp, plln, _) = pllerror(vco, ahb * ahbpre);
							array[0] = 2;
							array[1] = bytes;
							array[8] = match pllp {
								2 => 0b00,
								4 => 0b01,
								6 => 0b10,
								8 => 0b11,
								_ => unreachable!(),
							};
							array[9] = plln;
							array[10] = pllm;

							clocks.sysclk = vco * plln / plln;
							clocks.ahb = clocks.sysclk / ahbpre;
						},
					}
				},
			}

			// Set up APB1 and APB2 and APB3 if they exist
			{
				let (ratio, bytes): (u32, u32) = match cfg.apb1 {
					Some(apb) => match u32::from(clocks.ahb / cmp::min(apb, APB1MAX)) {
						0..=1  => ( 1, 0b000),
						2..=3  => ( 2, 0b100),
						4..=8  => ( 4, 0b101),
						8..=15 => ( 8, 0b110),
						_      => (16, 0b111),
					},
					None => match u32::from(clocks.ahb / APB1MAX) {
						0..=1  => ( 1, 0b000),
						2..=3  => ( 2, 0b100),
						4..=8  => ( 4, 0b101),
						8..=15 => ( 8, 0b110),
						_      => (16, 0b111),
					}
				};

				// Set the values
				array[2] = bytes;
				clocks.apb1 = clocks.ahb / ratio;
			}

			#[cfg(feature = "apb2")]
			{
				let (ratio, bytes): (u32, u32) = match cfg.apb2 {
					Some(apb) => match u32::from(clocks.ahb / cmp::min(apb, APB2MAX)) {
						0..=1  => ( 1, 0b000),
						2..=3  => ( 2, 0b100),
						4..=8  => ( 4, 0b101),
						8..=15 => ( 8, 0b110),
						_      => (16, 0b111),
					},
					None => match u32::from(clocks.ahb / APB2MAX) {
						0..=1  => ( 1, 0b000),
						2..=3  => ( 2, 0b100),
						4..=8  => ( 4, 0b101),
						8..=15 => ( 8, 0b110),
						_      => (16, 0b111),
					}
				};

				// Set the values
				array[3] = bytes;
				clocks.apb2 = clocks.ahb / ratio;
			}

			#[cfg(feature = "apb3")]
			{
				let (ratio, bytes): (u32, u32) = match cfg.apb3 {
					Some(apb) => match u32::from(clocks.ahb / cmp::min(apb, APB3MAX)) {
						0..=1  => ( 1, 0b000),
						2..=3  => ( 2, 0b100),
						4..=8  => ( 4, 0b101),
						8..=15 => ( 8, 0b110),
						_      => (16, 0b111),
					},
					None => match u32::from(clocks.ahb / APB3MAX) {
						0..=1  => ( 1, 0b000),
						2..=3  => ( 2, 0b100),
						4..=8  => ( 4, 0b101),
						8..=15 => ( 8, 0b110),
						_      => (16, 0b111),
					}
				};

				// Set the values
				array[4] = bytes;
				clocks.apb3 = clocks.ahb / ratio;
			}
		}
	}

	(array, clocks)
}

const fn defined_apb(cfg: RccClocks, lp: bool) -> ([u32; 12], Clocks) {
	let mut clocks = Clocks::new();
	let mut array = [0u32; 12];


	// Get max APB speed
	const APBC: [Frequency; 3] = [Frequency::Hz(0), Frequency::Hz(0), Frequency::Hz(0)];
	let mut apb = APBC;

	match cfg.apb1 {
		Some(a) if a <= APB1MAX => apb[0] = a,
		_ => apb[0] = APB1MAX,
	}

	#[cfg(feature = "apb2")]
	match cfg.apb2 {
		Some(a) if a <= APB1MAX => apb[1] = a,
		_ => apb[1] = APB1MAX,
	}

	#[cfg(feature = "apb3")]
	match cfg.apb3 {
		Some(a) if a <= APB3MAX => apb[2] = a,
		_ => apb[2] = APB3MAX,
	}

	let max = apb.iter().max();


	match max {
		// Everything defaults to HSI
		Some( Frequency::Hz(0) ) | None => {
			array[0] = 0;
			array[1] = 0;
			array[2] = 0;
			array[3] = 0;
			array[4] = 0;

			clocks.sysclk = HSI;
			clocks.ahb = HSI;
			clocks.apb1 = HSI;
			#[cfg(feature = "apb2")]
			{
				clocks.apb2 = HSI;
			}

			#[cfg(feature = "apb3")]
			{
				clocks.apb3 = HSI;
			}
		},

		// If it's not default -> max SYSCLK speed & max AHB speed
		Some( a ) => {
			// Get VCO
			let pllm = match (u32::from(HSI / 2u32), (HSI.hz() % 2u32)) {
				(m, 0) => m,
				(m, _) => m + 1,
			};

			let vco = HSI / pllm;

			let (pllp, plln, error) = pllerror(vco, SYSCLKMAX);

			array[0] = 2;
			array[8] = match pllp {
				2 => 0b00,
				4 => 0b01,
				6 => 0b10,
				8 => 0b11,
				_ => unreachable!(),
			};
			array[9] = plln;
			array[10] = pllm;

			clocks.sysclk = vco * plln / pllp;

			let mut best = (0, 0, 1000.0);

			for i in &[1u32, 2, 4, 8, 16, 64, 128, 256, 512] {
				for j in &[1u32, 2, 4, 8, 16] {
					let mut error = 0.0;

					error += match (u32::from((clocks.sysclk / *i) / *j) as f32 / (apb[0].hz() as f32) - 1.0) {
						a if a < 0.0 => -a,
						b => b,
					};
					#[cfg(feature = "apb2")]
					{
						error += match (u32::from((clocks.sysclk / *i) / *j) as f32 / (apb[1].hz() as f32) - 1.0) {
							a if a < 0.0 => -a,
							b => b,
						};
					}
					#[cfg(feature = "apb3")]
					{
						error += match (u32::from((clocks.sysclk / *i) / *j) as f32 / (apb[2].hz() as f32) - 1.0) {
							a if a < 0.0 => -a,
							b => b,
						};
					}

					if error < best.2 {
						best = (*i, *j, error);
					}
				}
			}

			array[1] = match best.0 {
				1   => 0b0000,
				2   => 0b1000,
				4   => 0b1001,
				8   => 0b1010,
				16  => 0b1011,
				64  => 0b1100,
				128 => 0b1101,
				256 => 0b1110,
				_   => 0b1111,
			};

			clocks.sysclk = HSI;
			clocks.ahb = HSI / best.0;

			match u32::from(((HSI / best.0) / best.1) / apb[0] ) {
				0..=1  => { array[2] = 0b000; clocks.apb1 = (HSI / best.0) /  1u32;},
				2..=3  => { array[2] = 0b100; clocks.apb1 = (HSI / best.0) /  2u32;},
				4..=7  => { array[2] = 0b101; clocks.apb1 = (HSI / best.0) /  4u32;},
				8..=15 => { array[2] = 0b110; clocks.apb1 = (HSI / best.0) /  8u32;},
				_      => { array[2] = 0b111; clocks.apb1 = (HSI / best.0) / 16u32;},
			};

			#[cfg(feature = "apb2")]
			{
				match u32::from(((HSI / best.0) / best.1) / apb[1] ) {
					0..=1  => { array[3] = 0b000; clocks.apb2 = (HSI / best.0) /  1u32;},
					2..=3  => { array[3] = 0b100; clocks.apb2 = (HSI / best.0) /  2u32;},
					4..=7  => { array[3] = 0b101; clocks.apb2 = (HSI / best.0) /  4u32;},
					8..=15 => { array[3] = 0b110; clocks.apb2 = (HSI / best.0) /  8u32;},
					_      => { array[3] = 0b111; clocks.apb2 = (HSI / best.0) / 16u32;},
				};
			}

			#[cfg(feature = "apb3")]
			{
				match u32::from(((HSI / best.0) / best.1) / apb[2] ) {
					0..=1  => { array[4] = 0b000; clocks.apb3 = (HSI / best.0) /  1u32;},
					2..=3  => { array[4] = 0b100; clocks.apb3 = (HSI / best.0) /  2u32;},
					4..=7  => { array[4] = 0b101; clocks.apb3 = (HSI / best.0) /  4u32;},
					8..=15 => { array[4] = 0b110; clocks.apb3 = (HSI / best.0) /  8u32;},
					_      => { array[4] = 0b111; clocks.apb3 = (HSI / best.0) / 16u32;},
				};
			}

		},
	}

	(array, clocks)
}

// Everything defaults to HSI
const fn no_defines(cfg: RccClocks, lp: bool) -> ([u32; 12], Clocks) {
	let mut clocks = Clocks::new();
	let mut array = [0u32; 12];

	clocks.sysclk = HSI;
	clocks.ahb = HSI;
	clocks.apb1 = HSI;

	#[cfg(feature = "apb2")]
	{
		clocks.apb2 = HSI;
	}

	#[cfg(feature = "apb3")]
	{
		clocks.apb3 = HSI;
	}

	(array, clocks)
}
