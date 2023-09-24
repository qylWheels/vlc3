use crate::cpu::{
	register::Register,
	CPU,
};
use crate::memory::MEMORY;
use super::Instruction;
use super::opcode::OpCode;

pub struct Ldi {
	dr: Register,
	offset: u16,
}

impl Ldi {
	pub fn new(dr: Register, offset: u16) -> Self {
		Self {
			dr, offset,
		}
	}
}

impl Instruction for Ldi {
	fn execute(&self) {
		let target_addr = MEMORY.read(
			CPU.read(Register::PC).wrapping_add(self.offset)
		);
		let result = MEMORY.read(target_addr);
		CPU.write(self.dr, result);
		CPU.update_condition_reg(result);
	}

	fn opcode(&self) -> OpCode {
		OpCode::LDI
	}
}
