use crate::cpu::{
	register::Register,
	CPU,
};
use super::Instruction;
use super::opcode::OpCode;

pub struct Jmp {
	base_reg: Register,
}

impl Jmp {
	pub fn new(base_reg: Register) -> Self {
		Self {
			base_reg,
		}
	}
}

impl Instruction for Jmp {
	fn execute(&self) {
		let target_addr = CPU.read(self.base_reg);
		CPU.write(Register::PC, target_addr);
	}

	fn opcode(&self) -> OpCode {
		OpCode::JMP
	}
}
