use crate::cpu::{
	register::Register,
	CPU,
};
use crate::memory::MEMORY;
use super::Instruction;
use super::opcode::OpCode;

pub struct St {
	sr: Register,
	offset: u16,
}

impl St {
	pub fn new(sr: Register, offset: u16) -> Self {
		Self {
			sr, offset,
		}
	}
}

impl Instruction for St {
	fn execute(&self) {
		let addr = CPU
			.read(Register::PC)
			.wrapping_add(self.offset);
		MEMORY.write(addr, CPU.read(self.sr));
	}

	fn opcode(&self) -> OpCode {
		OpCode::ST
	}
}
