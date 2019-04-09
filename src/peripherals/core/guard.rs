//! Guarding struct
//! Guards the core peripherals so no two instances of a peripheral
//! can live at the same time. This excludes system calls, which
//! have a higher priority level and can force create an instance.

#[derive(Debug)]
pub struct CoreGuard {
	created: u32;
	instances: [u32; 10],
	forced: [u32; 10],
	scb: Option<&mut scb::Scb>,
}

impl CoreGuard {
	/// Initializes the requested peripheral
	pub fn init(&mut self, p: super::Peripheral) -> Result<(), ()> {
		match p {
			Peripheral::Scb => {
				self.scb = Some( scb::Scb::from_addr( scb::ADDRESS ) );
				Ok(())
			},
		}
	}

	/// Request access to the SCB
	pub fn scb(&mut self) -> Option<&mut Scb> {
		match self.scb {
			None => None, // Core peripheral has not been initialized
			Some(a) => {
				match instances[SCB_POS] {
					// There are no current instances of the peripheral
					0 => {
						// Flip the corresponding bit
						self.created ^= SCB_MASK;
						// Wait
						asm::delay(20);
						// Check if the peripheral has been requested by someone else in a data race
						if self.created & SCB_MASK == 0 {
							None
						} else {
							self.instances[SCB_POS] += 1;
							Some(a)
						}
					},
					_ => None,
				}
			},
		}
	}

	/// Force access to the SCB
	/// 
	/// Remember that this instance must be manually killed with `kill` method
	pub fn force_scb(&mut self) -> &mut Scb {
		self.forced[SCB_POS] += 1;
		scb::Scb::from_addr( scb::ADDRESS )
	}
}