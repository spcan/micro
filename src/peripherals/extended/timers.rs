//! Control of all timer functionalities




#[derive(Debug)]
pub struct TimControl {


	core_functions: ,
}




macro_rules! timers {
	{$ts:tt} => {
		lazy_static! {
			pub static ref TIMERS: TimControl = {
				let mut control: TimControl = Default::default();
				timers_inner!($ts)
			};
		}
	};
}

macro_rules! timers_inner {
	($($control:ident => {$ts:tt},)+) => {
		$(timers_inner($control, $ts))+
	};

	(TIM1, $($ch:ident => $ts:tt,)+,) => {
		{
			let mut tim = AdvTim::from_address(TIM1);
			$(ch_inner!($ch => $ts))+
		}
	};
}

macro_rules! ch_inner {
	(inputCapture => [ period = $period:expr, duty = $duty:expr, reset = $reset:expr ]) => {
		tim.ic_channel_config(period, )
	};
}

macro_rules! ch_setup {
	($x:expr, inputCapture, period = $period:expr, duty = $duty:expr) => {

	};
}

timers! {
	TIM1 => {
		inputCapture => [
			period = 1,
			duty = 2,
		],
	},

	TIME2 => {
		Ch1 => [
			outPWM,
			period = 1,
			duty = 2,
		],

	},
}


