use crate::cpu::{
	register::Register,
	CPU,
};
use crate::memory::MEMORY;
use super::Instruction;
use super::opcode::OpCode;

#[derive(Debug)]
pub struct Str {
	sr: Register,
	base_reg: Register,
	offset: u16,
}

impl Str {
	pub fn new(sr: Register, base_reg: Register, offset: u16) -> Self {
		Self {
			sr, base_reg, offset,
		}
	}
}

impl Instruction for Str {
	fn execute(&self) {
		let addr = CPU
			.read(self.base_reg)
			.wrapping_add(self.offset);
		MEMORY.write(addr, CPU.read(self.sr));
	}

	fn opcode(&self) -> OpCode {
		OpCode::STR
	}
}
