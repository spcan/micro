//! I2C Flags

#[derive(Debug, Copy, Clone)]
pub enum I2CFlags {
	Start             = 1280,
	AddressSent           = 1281,
	TransferComplete  = 1282,
	Header10Bit       = 1283,
	Stop              = 1284,
	RxNotEmpty        = 1286,
	TxEmpty           = 1287,
	BusError          = 1288,
	ArbitrationLost   = 1289,
	ACKFailure        = 1290,
	OverUnder         = 1291,
	PECReceptionError = 1292,
	Timeout           = 1294,
	SMBusAlert        = 1295,

	GenCall    = 1540,
	SMBDefault = 1541,
	SMBHost    = 1542,
}

impl I2CFlags {
	pub fn offsets(self) -> (usize, usize) {
		let data = self as usize;

		((data >> 8 ) & 0b1111_1111, data & 0b1111_1111 )
	}
}