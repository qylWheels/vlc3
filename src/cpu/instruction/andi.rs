use crate::cpu::{
	register::Register,
	CPU,
};
use super::Instruction;
use super::opcode::OpCode;

pub struct Andi {
	dr: Register,
	sr1: Register,
	imm: u16,
}

impl Andi {
	pub fn new(dr: Register, sr1: Register, imm: u16) -> Self {
		Self {
			dr, sr1, imm,
		}
	}
}

impl Instruction for Andi {
	fn execute(&self) {
		let result = CPU.read(self.sr1) & self.imm;
		CPU.write(self.dr, result);
		CPU.update_condition_reg(result);
	}
	
	fn opcode(&self) -> OpCode {
		OpCode::ANDI
	}
}
