//! Input Compare Pin Config

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ICChConfig {
	TI1,
	TI2,
	TRC,
	Output,
}