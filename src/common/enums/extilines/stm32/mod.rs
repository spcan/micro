//! STM32 EXTI lines

#[cfg(feature = "stm32f0")]
mod stm32f0;

#[cfg(feature = "stm32f1")]
mod stm32f1;

#[cfg(feature = "stm32f2")]
mod stm32f2;

#[cfg(feature = "stm32f3")]
mod stm32f3;

#[cfg(feature = "stm32f4")]
mod stm32f4;

#[cfg(feature = "stm32f7")]
mod stm32f7;


#[cfg(feature = "stm32f0")]
pub use self::stm32f0::*;

#[cfg(feature = "stm32f1")]
pub use self::stm32f1::*;

#[cfg(feature = "stm32f2")]
pub use self::stm32f2::*;

#[cfg(feature = "stm32f3")]
pub use self::stm32f3::*;

#[cfg(feature = "stm32f4")]
pub use self::stm32f4::*;

#[cfg(feature = "stm32f7")]
pub use self::stm32f7::*;