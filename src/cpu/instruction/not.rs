use crate::cpu::{
	register::Register,
	CPU,
};
use super::Instruction;
use super::opcode::OpCode;

pub struct Not {
	dr: Register,
	sr: Register,
}

impl Not {
	pub fn new(dr: Register, sr: Register) -> Self {
		Self {
			dr, sr,
		}
	}
}

impl Instruction for Not {
	fn execute(&self) {
		let result = !CPU.read(self.sr);
		CPU.write(self.dr, result);
		CPU.update_condition_reg(result);
	}

	fn opcode(&self) -> OpCode {
		OpCode::NOT
	}
}
