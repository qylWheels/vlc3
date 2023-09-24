use crate::cpu::{
	register::Register,
	CPU,
};
use super::Instruction;
use super::opcode::OpCode;

pub struct Addi {
	dr: Register,
	sr1: Register,
	imm: u16,
}

impl Addi {
	pub fn new(dr: Register, sr1: Register, imm: u16) -> Self {
		Self {
			dr, sr1, imm,
		}
	}
}

impl Instruction for Addi {
	fn execute(&self) {
		let result = CPU
			.read(self.sr1)
			.wrapping_add(self.imm);
		CPU.write(self.dr, result);
		CPU.update_condition_reg(result);
	}

	fn opcode(&self) -> OpCode {
		OpCode::ADDI
	}
}
