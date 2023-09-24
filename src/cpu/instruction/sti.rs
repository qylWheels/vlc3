use crate::cpu::{
	register::Register,
	CPU,
};
use crate::memory::MEMORY;
use super::Instruction;
use super::opcode::OpCode;

#[derive(Debug)]
pub struct Sti {
	sr: Register,
	offset: u16,
}

impl Sti {
	pub fn new(sr: Register, offset: u16) -> Self {
		Self {
			sr, offset,
		}
	}
}

impl Instruction for Sti {
	fn execute(&self) {
		let target_addr = MEMORY
			.read(CPU.read(Register::PC))
			.wrapping_add(self.offset);

		MEMORY.write(target_addr, CPU.read(self.sr));
	}

	fn opcode(&self) -> OpCode {
		OpCode::STI
	}
}
