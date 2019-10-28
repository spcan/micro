//! TIM trigger modes

#[derive(Copy, Clone, PartialEq)]
pub enum TIMTrigger {
	/// Internal0
	ITR0,    
	/// Internal1
	ITR1,    
	/// Internal2
	ITR2,    
	/// Internal3
	ITR3,    
	/// EdgeDetectorTI1
	TI1F_ED, 
	/// FilteredInput1
	TI1FP1,  
	/// FilteredInput2
	TI2FP2,  
	/// ExternalTriggerInput
	ETRF,    
}
