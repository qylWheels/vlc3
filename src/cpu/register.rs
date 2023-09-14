use std::convert::From;

#[derive(Clone, Copy, Debug)]
pub enum Register {
	R0,
	R1,
	R2,
	R3,
	R4,
	R5,
	R6,
	R7,
	PC,
	Cond,
	Count,
}

impl From<u16> for Register {
	fn from(value: u16) -> Self {
		match value {
			0 => Self::R0,
			1 => Self::R1,
			2 => Self::R2,
			3 => Self::R3,
			4 => Self::R4,
			5 => Self::R5,
			6 => Self::R6,
			7 => Self::R7,
			8 => Self::PC,
			9 => Self::Cond,
			10 => Self::Count,
			_ => panic!("Invalid register number"),
		}
	}
}
