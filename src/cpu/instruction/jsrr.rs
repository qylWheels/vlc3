use crate::cpu::{
	register::Register,
	CPU,
};
use super::Instruction;
use super::opcode::OpCode;

pub struct Jsrr {
	base_reg: Register,
}

impl Jsrr {
	pub fn new(base_reg: Register) -> Self {
		Self {
			base_reg,
		}
	}
}

impl Instruction for Jsrr {
	fn execute(&self) {
		let target = CPU.read(self.base_reg);
		CPU.write(Register::R7, CPU.read(Register::PC));
		CPU.write(Register::PC, target);
	}
	
	fn opcode(&self) -> OpCode {
		OpCode::JSRR
	}
}
