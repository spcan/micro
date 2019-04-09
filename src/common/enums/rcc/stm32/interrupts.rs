//! RCC Interrupts

/// Available interrupts
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RCCInterrupt {
	CSSC,
	PLLI2SRDY,
	PLLRDY,
	HSERDY,
	HSIRDY,
	LSERDY,
	LSIRDY,
	BORRST,
	PINRST,
	SFTRST,
	PORRST,
	IWDGRST,
	WWDGRST,
	LPWRRST,
}
