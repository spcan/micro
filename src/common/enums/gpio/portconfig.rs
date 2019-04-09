//!! GPIO Port Configuration

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PortConfig {
	Input,
	Output,
	AltFunction,
	Analog,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum OutputType {
	PushPull,
	OpenDrain,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GPIOSpeed {
	Low = 0,
	Medium = 1,
	Fast = 2,
	High = 3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PUPD {
	NONE = 0,
	PullUp = 1,
	PullDown = 2,
}