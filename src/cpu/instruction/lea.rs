use crate::cpu::{
	register::Register,
	CPU,
};
use super::Instruction;
use super::opcode::OpCode;

pub struct Lea {
	dr: Register,
	offset: u16,
}

impl Lea {
	pub fn new(dr: Register, offset: u16) -> Self {
		Self {
			dr, offset,
		}
	}
}

impl Instruction for Lea {
	fn execute(&self) {
		let result = CPU
			.read(Register::PC)
			.wrapping_add(self.offset);
		CPU.write(self.dr, result);
		CPU.update_condition_reg(result);
	}

	fn opcode(&self) -> OpCode {
		OpCode::LEA
	}
}
