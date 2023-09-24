use crate::cpu::{
	register::Register,
	CPU,
};
use crate::memory::MEMORY;
use super::Instruction;
use super::opcode::OpCode;

pub struct Ld {
	dr: Register,
	offset: u16,
}

impl Ld {
	pub fn new(dr: Register, offset: u16) -> Self {
		Self {
			dr, offset,
		}
	}
}

impl Instruction for Ld {
	fn execute(&self) {
		let target_addr = CPU
			.read(Register::PC)
			.wrapping_add(self.offset);
		let data = MEMORY.read(target_addr);
		CPU.write(self.dr, data);
		CPU.update_condition_reg(data);
	}

	fn opcode(&self) -> OpCode {
		OpCode::LD
	}
}
